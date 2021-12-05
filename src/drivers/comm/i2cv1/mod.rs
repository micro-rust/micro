//! I2C Driver module.


use core::future::Future;

pub use embedded_hal::i2c::{
    SevenBitAddress, TenBitAddress, AddressMode,
};



/// Common trait for all I2C Interfaces.
pub trait I2CInterface {
    type Error;

    //type Handle : I2CHandle;
}

/// Trait for I2C interfaces with write capabilites.
pub trait Write<A: AddressMode, H: I2CHandle> : I2CInterface {
    /// Writes data to the I2C bus.
    fn write(&mut self, addr: A, bytes: &[u8], stop: bool) -> Result<H, Self::Error>;
}


/// Trait for I2C interfaces with write-read capabilities.
pub trait WriteRead<A: AddressMode, H: I2CHandle> : I2CInterface {
    /// Writes data to the I2C bus then reads data from it.
    fn write_read(&mut self, addr: A, bytes: &[u8], buffer: &mut [u8], restart: bool) -> Result<H, Self::Error>;
}


/// Common trait for I2C async handles.
/// Asynchronous and blocking interfaces must return a type implementing this
/// trait after each operation.
pub trait I2CHandle: Future {
    /// Returns `true` if the operation has completed.
    /// By default it returns `true` (hardcoded).
    /// For blocking handles, this can be left as the default implementation.
    fn done(&self) -> bool {
        true
    }

    /// Blocks the thread until this handle has completed.
    fn join(&self);
}
