//! Communication peripheral drivers.



mod i2c;
//mod uart;



//pub use self::uart::UartDriver;
pub use i2c::{I2CDriver, I2CAsyncDriver};



/// Trait that must be implemented for all communication interfaces to define
/// their custom error types.
pub trait CommunicationError {
    type Error;
}

impl<T: CommunicationError> CommunicationError for &mut T {
    type Error = T::Error;
}
