//! System Control Block general functionality.

use core::ptr::read_volatile as read;
use core::ptr::write_volatile as write;

pub struct Scb;

impl Scb {
    /// Returns `true` if the processor is using big endianess.
    #[inline(always)]
    pub fn be(&self) -> bool {
        unsafe { read(0xE000ED0C as *const u32) & (1 << 15) != 0 }
    }

    /// Returns `true` if the processor is using little endianess.
    #[inline(always)]
    pub fn le(&self) -> bool {
        unsafe { read(0xE000ED0C as *const u32) & (1 << 15) == 0 }
    }

    /// Requests a system reset.
    /// UNSAFETY : This function resets the whole Core and its state.
    #[inline(always)]
    pub unsafe fn sysreset(&self) {
        write(0xE000ED0C as *mut u32, (0x05FA << 16) | (1 << 2))
    }

    /// Enables or disables waking up the core after an interrupt or event.
    #[cfg(feature = "scr")]
    #[inline(always)]
    pub fn sevonpend(&self) {
        const CONST : *const u32 = 0xE000ED10 as *const u32;
        const MUT : *mut u32 = 0xE000ED10 as *mut u32;

        if s { unsafe { write(MUT, read(CONST) |  (1 << 4)) } }
        else { unsafe { write(MUT, read(CONST) & !(1 << 4)) } }
    }

    /// Enables or disables deep sleep when a WFI or WFE instruction occurs.
    #[cfg(feature = "scr")]
    #[inline(always)]
    pub fn deepsleep(&self) {
        const CONST : *const u32 = 0xE000ED10 as *const u32;
        const MUT : *mut u32 = 0xE000ED10 as *mut u32;

        if s { unsafe { write(MUT, read(CONST) |  (1 << 2)) } }
        else { unsafe { write(MUT, read(CONST) & !(1 << 2)) } }
    }

    /// Enables or disables deep sleep when coming out of an ISR.
    #[cfg(feature = "scr")]
    #[inline(always)]
    pub fn sleeponexit(&self) {
        const CONST : *const u32 = 0xE000ED10 as *const u32;
        const MUT : *mut u32 = 0xE000ED10 as *mut u32;

        if s { unsafe { write(MUT, read(CONST) |  (1 << 1)) } }
        else { unsafe { write(MUT, read(CONST) & !(1 << 1)) } }
    }

    /// Returns `true` if unaligned accesses are allowed.
    #[inline(always)]
    pub fn unaligned(&self) -> bool {
        unsafe { read(0xE000ED14 as *const u32) & (1 << 3) == 0 }
    }
}