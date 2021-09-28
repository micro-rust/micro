//! Common library for the `micro`framework ecosystem.

#![no_std]

#![deny(warnings)]

#![feature(asm)]

pub mod freq;
pub mod reg;

#[cfg(feature = "arm")]
mod arm;


#[cfg(feature = "arm")]
pub use self::arm::*;


pub use self::reg::Register;
pub use self::reg::XType;
