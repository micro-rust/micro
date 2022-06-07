//! Core ARM peripherals module.


use crate::reg::{ DefaultRegister, Register };


pub use self::int::InterruptControl;
pub use self::mpu::{ MPU, MPUConfiguration, MPURegionSize, MPUPermissions };
pub use self::scb::SystemControl;
pub use self::systick::Systick;



/// NVIC and Interrupt Control peripherals.
mod int;

/// MPU peripheral.
mod mpu;

/// System Control Block peripheral (SCB).
mod scb;

/// System Timer peripheral (SysTick).
mod systick;





/// Reads the CPUID register.
#[inline(always)]
pub fn cpuid() -> u32 {
	// Reference to the CPUID register.
	let cpuid = DefaultRegister::at(0xE000ED00);

	cpuid.read()
}
