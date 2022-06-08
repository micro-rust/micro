//! Miscellaneous ARM assembly instructions.



use core::arch::asm;



/// No OP instruction.
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop") }
}

/// Yield instruction.
#[inline(always)]
pub fn r#yield() {
    unsafe { asm!("yield") }
}


/// Debug hint.
#[cfg(not(target_arch="thumbv6"))]
#[inline(always)]
pub fn dbg() {
    unsafe { asm!("dbg") }
}
