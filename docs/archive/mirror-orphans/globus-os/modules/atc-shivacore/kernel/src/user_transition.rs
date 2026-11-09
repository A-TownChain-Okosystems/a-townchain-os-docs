// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! First controlled ring-3 transition for the GlobusOS init process.
//!
//! This bootstrap uses the active page table and two USER_ACCESSIBLE pages.
//! It is deliberately not final per-process MMU isolation.

use core::arch::asm;
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
    VirtAddr,
};

use crate::{
    gdt,
    memory::{AddressSpace, BootInfoFrameAllocator},
};

const USER_CODE: u64 = 0x0040_0000;
const USER_STACK: u64 = 0x0080_0000;

// rax = Yield syscall ID, then int 0x80. The loop keeps init alive after
// returning from the kernel. This exercises the real register-based ABI.
static USER_PROGRAM: [u8; 13] = [
    0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1
    0xCD, 0x80, // int 0x80
    0xEB, 0xFE, // jmp $-2
    0x90, 0x90, // padding
];

pub unsafe fn enter_init(
    address_space: &mut AddressSpace,
    frame_allocator: &mut BootInfoFrameAllocator,
    physical_memory_offset: u64,
) -> ! {
    let code_page = Page::<Size4KiB>::containing_address(VirtAddr::new(USER_CODE));
    let stack_page = Page::<Size4KiB>::containing_address(VirtAddr::new(USER_STACK));

    let code_frame = frame_allocator
        .allocate_frame()
        .expect("ShivaCore: no frame available for GlobusOS init code");
    let stack_frame = frame_allocator
        .allocate_frame()
        .expect("ShivaCore: no frame available for GlobusOS init stack");

    let code_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
    let stack_flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

    address_space
        .mapper()
        .map_to(code_page, code_frame, code_flags, frame_allocator)
        .expect("ShivaCore: failed to map GlobusOS init code")
        .flush();
    address_space
        .mapper()
        .map_to(stack_page, stack_frame, stack_flags, frame_allocator)
        .expect("ShivaCore: failed to map GlobusOS init stack")
        .flush();

    let code_dst = VirtAddr::new(physical_memory_offset) + code_frame.start_address().as_u64();
    core::ptr::copy_nonoverlapping(
        USER_PROGRAM.as_ptr(),
        code_dst.as_mut_ptr::<u8>(),
        USER_PROGRAM.len(),
    );

    let user_cs = u64::from(gdt::user_code_selector().0);
    let user_ss = u64::from(gdt::user_data_selector().0);
    let user_rsp = USER_STACK + 4096 - 16;
    let rflags = 0x202u64;

    address_space.activate();
    x86_64::instructions::interrupts::disable();
    asm!("push {ss}", "push {rsp}", "push {rflags}", "push {cs}", "push {rip}", "iretq",
        ss = in(reg) user_ss, rsp = in(reg) user_rsp, rflags = in(reg) rflags,
        cs = in(reg) user_cs, rip = in(reg) USER_CODE, options(noreturn));
}
