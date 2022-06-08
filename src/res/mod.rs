//! Common traits for system resources.


mod acquire;
mod allocate;


pub use self::acquire::{ Acquire, Release };
pub use self::allocate::*;