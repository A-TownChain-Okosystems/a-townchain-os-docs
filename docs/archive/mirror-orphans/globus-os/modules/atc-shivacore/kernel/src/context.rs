// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.
//! x86_64 kernel and userspace context representations.
//!
//! Kernel context switching and userspace interrupt return state are modeled
//! separately. A kernel Context cannot safely represent a ring-3 interrupt
//! frame because the CPU additionally owns RIP/CS/RFLAGS/RSP/SS.

#![cfg(target_arch = "x86_64")]

use alloc::boxed::Box;
use alloc::vec;
use core::arch::asm;

const STACK_SIZE: usize = 16 * 1024;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub rsp: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl Context {
    pub const fn empty() -> Self {
        Self {
            rsp: 0,
            rbx: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }
}

/// Complete general-purpose register state saved by the ShivaCore syscall
/// trampoline. The layout must remain identical to the push order in
/// interrupts.rs.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UserRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

/// CPU-owned return frame present after a ring-3 int 0x80/interrupt.
/// This structure is intentionally separate from UserRegisters.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UserInterruptFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl UserInterruptFrame {
    pub fn is_ring3(&self) -> bool {
        (self.cs & 0x3) == 3 && (self.ss & 0x3) == 3
    }
}

/// Complete restorable userspace execution state.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UserContext {
    pub registers: UserRegisters,
    pub frame: UserInterruptFrame,
}

impl UserContext {
    pub fn is_ring3(&self) -> bool {
        self.frame.is_ring3()
    }
}

pub struct BootstrapProcess {
    stack: Box<[u8]>,
    context: Context,
}

impl BootstrapProcess {
    pub fn new(entry: extern "C" fn() -> !) -> Self {
        let mut stack = vec![0u8; STACK_SIZE].into_boxed_slice();
        let top = stack.as_mut_ptr() as usize + stack.len();
        let stack_top = (top & !0xFusize).saturating_sub(8);
        unsafe {
            (stack_top as *mut u64).write(entry as usize as u64);
        }

        Self {
            stack,
            context: Context {
                rsp: stack_top as u64,
                ..Context::empty()
            },
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }
}

/// Switches the CPU context from `current` to `next` (x86-64).
///
/// # Safety
///
/// `current` and `next` must point to valid, non-overlapping contexts and
/// must not be moved or dropped while the switch is in flight. The stack
/// saved in `next` must be mapped, writable and aligned; `next` must have
/// been initialized by [`Context::bootstrap`] or a previous `switch`.
#[inline(never)]
pub unsafe fn switch(current: &mut Context, next: &Context) {
    asm!(
        "push r15",
        "push r14",
        "push r13",
        "push r12",
        "push rbp",
        "push rbx",
        "mov [rdi + 0x00], rsp",
        "mov rsp, [rsi + 0x00]",
        "pop rbx",
        "pop rbp",
        "pop r12",
        "pop r13",
        "pop r14",
        "pop r15",
        "ret",
        in("rdi") current,
        in("rsi") next,
        options(preserves_flags)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn never_returns() -> ! {
        loop {
            core::hint::spin_loop();
        }
    }

    #[test]
    fn bootstrap_context_has_stack_and_entry() {
        let process = BootstrapProcess::new(never_returns);
        assert_eq!(process.stack_size(), STACK_SIZE);
        assert_ne!(process.context().rsp, 0);
    }

    #[test]
    fn ring3_frame_is_distinct_from_kernel_context() {
        let frame = UserInterruptFrame {
            rip: 0x0040_0000,
            cs: 0x1b,
            rflags: 0x202,
            rsp: 0x0080_1000,
            ss: 0x23,
        };
        assert!(frame.is_ring3());
        assert_ne!(frame.rip, 0);
        assert_ne!(frame.rsp, 0);
    }

    #[test]
    fn user_context_contains_registers_and_cpu_frame() {
        let context = UserContext::default();
        assert!(!context.is_ring3());
        assert_eq!(context.registers.rax, 0);
    }
}
