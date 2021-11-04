//! Module for the Exception and Interrupt tables.


mod exc;
mod irq;


pub use self::exc::EXCTable;
pub use self::irq::IRQTable;
