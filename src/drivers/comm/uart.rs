//! UART Peripheral abstracted driver.


use crate::drivers::Data;



/// UART Driver common trait.
pub trait UartDriver {
    /// Error type to be implemented.
    type Error;

    /// Type to be returned when an operation completes or starts(in the case of async operations).
    type Handle;
}


/// UART Driver for data transmission.
/// To be implemented by blocking and non blocking drivers.
pub trait UartTxDriver: UartDriver {
    /// Function to send data.
    /// Returns the number of data packets sent.
    fn send<D: Data>(&self, data: &[D]) -> Result<Self::Handle, Self::Error>;
}


/// UART Driver for data reception.
/// To be implemented by blocking and non blocking drivers.
pub trait UartRxDriver: UartDriver {
    /// Function to receive data.
    /// Returns the number of data packets received.
    fn recv<D: Data>(&self, data: &mut [D]) -> Result<Self::Handle, Self::Error>;
}


/// UART Driver for data transmission and reception.
/// To be implemented by blocking and non blocking drivers.
pub trait UartDuplexDriver: UartTxDriver + UartRxDriver {
    /// Function to receive and send data simultaneously.
    /// Returns the number of data packets sent and received.
    fn send_recv<DTX: Data, DRX: Data>(&self, send: &[DTX], recv: &mut [DRX]) -> Result<Self::Handle, Self::Error>;

    /// Function to send and then receive data sequentially.
    /// Returns the number of data packets sent and received.
    fn send_then_recv<DTX: Data, DRX: Data>(&self, send: &[DTX], recv: &mut [DRX]) -> Result<Self::Handle, Self::Error>;
}
