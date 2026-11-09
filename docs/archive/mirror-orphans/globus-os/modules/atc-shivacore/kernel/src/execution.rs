// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.
//! Kernel execution bridge: Process -> CPU context -> address space.
//!
//! The scheduler owns the authoritative current PID used by the syscall boundary.
//! Timer interrupts only request preemption; context switching remains outside
//! the interrupt handler until the complete interrupt-frame switch path is active.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use crate::ats1000::Pid;
// Boot-only pieces depend on the x86 address-space path; the library/test
// build exposes only the scheduling facade below.
#[cfg(feature = "x86-boot")]
use crate::context::{self, BootstrapProcess, Context, UserContext};
#[cfg(feature = "x86-boot")]
use crate::memory::AddressSpace;
#[cfg(feature = "x86-boot")]
use alloc::collections::VecDeque;
#[cfg(feature = "x86-boot")]
use x86_64::registers::control::{Cr3, Cr3Flags};
#[cfg(feature = "x86-boot")]
use x86_64::structures::paging::PhysFrame;

static CURRENT_PID: AtomicU32 = AtomicU32::new(0);
static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);
static PREEMPT_REQUESTED: AtomicBool = AtomicBool::new(false);

pub fn current_pid() -> Option<Pid> {
    match CURRENT_PID.load(Ordering::Acquire) {
        0 => None,
        pid => Some(Pid(pid)),
    }
}

pub fn on_timer_tick() {
    TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
    if current_pid().is_some() {
        PREEMPT_REQUESTED.store(true, Ordering::Release);
    }
}

pub fn timer_ticks() -> u64 {
    TIMER_TICKS.load(Ordering::Relaxed)
}

pub fn take_preemption_request() -> bool {
    PREEMPT_REQUESTED.swap(false, Ordering::AcqRel)
}

#[cfg(feature = "x86-boot")]
pub struct ScheduledProcess {
    pub pid: Pid,
    process: BootstrapProcess,
    user_context: UserContext,
    root_frame: PhysFrame,
    cr3_flags: Cr3Flags,
}

#[cfg(feature = "x86-boot")]
impl ScheduledProcess {
    pub fn new(pid: Pid, process: BootstrapProcess, address_space: &AddressSpace) -> Self {
        Self {
            pid,
            process,
            user_context: UserContext::default(),
            root_frame: address_space.root_frame(),
            cr3_flags: address_space.cr3_flags(),
        }
    }

    pub fn context(&self) -> &Context {
        self.process.context()
    }
    pub fn context_mut(&mut self) -> &mut Context {
        self.process.context_mut()
    }
    pub fn user_context(&self) -> &UserContext {
        &self.user_context
    }
    pub fn user_context_mut(&mut self) -> &mut UserContext {
        &mut self.user_context
    }

    pub unsafe fn activate_address_space(&self) {
        Cr3::write(self.root_frame, self.cr3_flags);
    }
}

#[cfg(feature = "x86-boot")]
pub struct ProcessScheduler {
    ready: VecDeque<ScheduledProcess>,
    current: Option<ScheduledProcess>,
}

#[cfg(feature = "x86-boot")]
impl ProcessScheduler {
    pub const fn new() -> Self {
        Self {
            ready: VecDeque::new(),
            current: None,
        }
    }

    pub fn enqueue(&mut self, process: ScheduledProcess) {
        self.ready.push_back(process);
    }

    pub fn ready_len(&self) -> usize {
        self.ready.len()
    }

    pub fn current_pid(&self) -> Option<Pid> {
        self.current.as_ref().map(|p| p.pid)
    }

    pub unsafe fn run_next(&mut self, current_context: &mut Context) -> ! {
        let process = self
            .ready
            .pop_front()
            .expect("ShivaCore: scheduler run requested with empty ready queue");

        self.current = Some(process);
        let current = self.current.as_ref().expect("current process missing");

        CURRENT_PID.store(current.pid.0, Ordering::Release);
        serial_println!("ShivaCore: scheduler selected pid={}.", current.pid.0);
        current.activate_address_space();

        context::switch(current_context, current.context());
        panic!("ShivaCore: scheduled process returned unexpectedly");
    }

    pub fn requeue_current(&mut self) {
        if let Some(process) = self.current.take() {
            self.ready.push_back(process);
        }
    }
}
