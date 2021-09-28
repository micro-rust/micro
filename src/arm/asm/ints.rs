//! Interrupt and Event related ARM assembly instructions.



/// Wait for Interrupt instruction.
#[inline(always)]
pub fn wfi() {
	unsafe { asm!("wfi") }
}

/// Wait for Event instruction.
#[inline(always)]
pub fn wfe() {
	unsafe { asm!("wfi") }
}

/// Send Global Event instruction.
#[inline(always)]
pub fn sev() {
	unsafe { asm!("sev") }
}

/// Send Local Event instruction.
#[cfg(not(any(target_arch="thumbv6", target_arch="thumbv7")))]
#[inline(always)]
pub fn sevl() {
	unsafe { asm!("sevl") }
}

/// Supervisor Call instruction.
#[inline(always)]
pub fn svc() {
	unsafe { asm!("svc") }
}

/// Breakpoint.
#[inline(always)]
pub fn bkpt<const N: u8>() {
	unsafe { asm!("bkpt {x}", x = const N) }
}

/// Disable interrupts.
#[inline(always)]
pub fn cpsid_i() {
	unsafe { asm!("cpsid i") }
}

/// Enable interrupts.
#[inline(always)]
pub fn cpsie_i() {
	unsafe { asm!("cpsie i") }
}

/// Disable faults.
#[cfg(not(target_arch="thumbv6"))]
#[inline(always)]
pub fn cpsid_f() {
	unsafe { asm!("cpsid i") }
}

/// Enable faults.
#[cfg(not(target_arch="thumbv6"))]
#[inline(always)]
pub fn cpsie_f() {
	unsafe { asm!("cpsie i") }
}
