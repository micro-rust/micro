//! Acquire and Release procedures for system resources.



/// Release trait.
/// Must be iplemented by all system resources that do not support multithreading.
pub trait Release {
	type Error;

	type Output;

	/// Failable release of a resource.
	/// A resource must fail to release if other resources that depend on it
	/// are still in use.
	fn release(&mut self) -> Result<Self::Output, Self::Error>;
}



/// Acquire trait.
pub trait Acquire: Sized + Release {
	/// Failable aquisition of a resource.
	fn acquire() -> Result<Self, Self::Error>;
}