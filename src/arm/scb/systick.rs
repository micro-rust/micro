//! System timer peripheral.


use crate::Register;



#[repr(C)]
pub struct Systick<R: 'static + Register<u32>>(&'static mut [R; 4]);

impl<R: Register<u32>> Systick<R> {
	/// Sets the clock source as the external clock.
	#[inline(always)]
	pub fn external(&mut self) {
		self.0[0].clear(1 << 2)
	}

	/// Sets the clock source as the processor's clock.
	#[inline(always)]
	pub fn processor(&mut self) {
		self.0[0].set(1 << 2)
	}

	/// Enables / Disables the Systick interrupt.
	#[inline(always)]
	pub fn int(&mut self, s: bool) {
		if s { self.0[0].set(1 << 1) }
		else { self.0[0].clear(1 << 1) }
	}

	/// Enables the system timer.
	#[inline(always)]
	pub fn enable(&mut self) {
		self.0[0].set(1)
	}

	/// Disables the system timer.
	#[inline(always)]
	pub fn disable(&mut self) {
		self.0[0].clear(1)
	}

	/// Reloads the system timer with this countdown value.
	#[inline(always)]
	pub fn reload(&mut self, rvr: u32) {
		self.0[1].write(rvr & 0xFFFFFF)
	}
}
