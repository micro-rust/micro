//! I2C Driver trait.


use crate::drivers::Handle;

pub use embedded_hal::i2c::{
    SevenBitAddress, TenBitAddress, AddressMode,
};


/// Common trait for all I2C Drivers.
pub trait I2CDriver {
    type Error: core::fmt::Debug;
}

/// Trait for I2C interfaces with write capabilites.
pub trait I2CWrite<A: AddressMode> : I2CDriver {
    /// Writes data to the I2C bus.
    fn write<'a>(&mut self, addr: A, bytes: &[u8], stop: bool) -> Result<&'a mut Handle, <Self as I2CDriver>::Error>;
}


/// Trait for I2C interfaces with write-read capabilities.
pub trait I2CWriteRead<A: AddressMode> : I2CDriver + I2CWrite<A> {
    /// Writes data to the I2C bus then reads data from it.
    fn write_read<'a>(&mut self, addr: A, bytes: &[u8], buffer: &mut [u8], restart: bool) -> Result<&'a mut Handle, <Self as I2CDriver>::Error>;
}
