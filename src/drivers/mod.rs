//! Drivers module.
//! Contains the drivers / traits / abstractions over the different types
//! of peripherals in a MCU.




pub mod comm;

pub mod dma;

pub mod usb;



/// Common trait for data types that can be operated on.
pub trait Data: Sized {
	/// Size in bytes of the data type.
	const SIZE: usize = core::mem::size_of::<Self>();
}


impl Data for i8  {}
impl Data for u8  {}
impl Data for i16 {}
impl Data for u16 {}
impl Data for i32 {}
impl Data for u32 {}
impl Data for i64 {}
impl Data for u64 {}


/// Common trait for asynchronous peripherals.
pub trait AsyncPeripheral {
	/// Handle type to check for completion of an asynchronous operation.
	type Handle;

	/// Callback type to be implemented by each asynchronous peripheral.
	type Callback;

	/// Method to set the peripheral's callback.
	fn callback(&self, f: Self::Callback);
}

