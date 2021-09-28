//! Common abstraction for Frequency.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frequency(u32);

impl Frequency {
	const KHZ : u32 = 1000;
	const MHZ : u32 = Self::KHZ * 1000;
	const GHZ : u32 = Self::MHZ * 1000;

	pub const fn zero() -> Frequency { Frequency(0) }

	pub const fn hz(val: u32)  -> Frequency { Frequency(val)             }
	pub const fn khz(val: u32) -> Frequency { Frequency(val * Self::KHZ) }
	pub const fn mhz(val: u32) -> Frequency { Frequency(val * Self::MHZ) }
	pub const fn ghz(val: u32) -> Frequency { Frequency(val * Self::GHZ) }

	pub const fn as_hz(&self)  -> u32 { self.0             }
	pub const fn as_khz(&self) -> u32 { self.0 / Self::KHZ }
	pub const fn as_mhz(&self) -> u32 { self.0 / Self::MHZ }
	pub const fn as_ghz(&self) -> u32 { self.0 / Self::GHZ }

	pub const fn is_zero(&self) -> bool { self.0 == 0 }
}

impl core::ops::Mul<u32> for Frequency {
	type Output = Frequency;

	fn mul(self, rhs: u32) -> Frequency {
		Frequency(self.0 * rhs)
	}
}

impl core::ops::Div<u32> for Frequency {
	type Output = Frequency;

	fn div(self, rhs: u32) -> Frequency {
		Frequency(self.0 / rhs)
	}
}

impl core::convert::Into<u32> for Frequency {
	fn into(self) -> u32 {
		self.0
	}
}
