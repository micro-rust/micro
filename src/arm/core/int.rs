//! NVIC and Interrupt Control peripherals.



use crate::reg::{ DefaultRegister, Register };
use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};



/// The `InterruptControl` trait includes all the necessary methods to interact with the
/// NVIC and other Interrupt Control peripherals. This trait must be implemented
/// by an empty struct in the HAL crates.
/// This struct must be protected by a locking mechanism.
pub trait InterruptControl {
    /// Relocates the Vector Table.
    /// UNSAFETY : This function may cause the interrupts to stop working
    /// or cause corruption.
    #[inline(never)]
    unsafe fn relocate(&mut self, mut from: *const usize, base: *mut usize) {
        // Disable interrupts.
        crate::asm::cpsid_i();

        // Create a destination pointer.
        let mut to = base;

        // Relocate the Vector Table.
        for _ in 0..=16 {
            write(to, read(from));

            from = from.offset(1);
            to = to.offset(1);
        }

        // Reference to the VTOR.
        let vtor = DefaultRegister::at(0xE000ED08);

        // Write the new VTOR.
        vtor.write( base as u32 );

        // Set memory barriers.
        crate::asm::dmb();
        crate::asm::dsb();
        crate::asm::isb();

        // Enable interrupts.
        crate::asm::cpsie_i();
    }
}