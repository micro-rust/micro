//! I2C Driver trait.



use super::CommunicationError;



/// I2C communication driver.
pub trait I2CDriver: CommunicationError {
    /// Sends a buffer through the I2C bus.
    fn send(&mut self, buffer: &[u8], stop: bool) -> Result<(), Self::Error>;

    /// Sends a byte buffer then receives bytes until a buffer is full.
    fn transfer(&mut self, send: &[u8], receive: &mut [u8]) -> Result<(), Self::Error>;
}



/// Asynchronous I2C communication driver.
pub trait I2CAsyncDriver: CommunicationError {
    /// Sends a buffer through the I2C bus.
    async fn send(&mut self, data: &[u8], stop: bool) -> Result<(), Self::Error>;

    /// Sends a byte buffer then receives bytes until a buffer is full.
    async fn transfer(&mut self, send: &[u8], recv: &mut [u8]) -> Result<(), Self::Error>;
}
