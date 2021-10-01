//! CPUID register access.

use core::ptr::read_volatile as read;

pub fn cpuid() -> u32 {
    unsafe { read(0xE000ED00 as *const u32) }
}