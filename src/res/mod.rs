//! Common traits for system resources.


mod acquire;

#[macro_use]
mod allocate;


pub use self::acquire::{ Acquire, Release };
