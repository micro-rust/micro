//! Data and instruction synchronization.



/// Data Memory Barrier instruction.
#[inline(always)]
pub fn dmb() {
	unsafe { asm!("dmb") }
}

/// Data Synchronization Barrier instruction.
#[inline(always)]
pub fn dsb() {
	unsafe { asm!("dsb") }
}

/// Instruction Synchronization Barrier instruction.
#[inline(always)]
pub fn isb() {
	unsafe { asm!("isb") }
}
