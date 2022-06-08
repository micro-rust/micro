//! Common traits for system resources.


mod acquire;
pub mod allocate;


pub use self::acquire::{ Acquire, Release };
