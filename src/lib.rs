//! Common library for the `micro`framework ecosystem.

#![no_std]

#![deny(warnings)]

#![feature(asm)]
#![feature(const_fn_trait_bound)]

pub mod freq;
pub mod reg;

pub mod drivers;

mod per;

#[cfg(feature = "arm")]
mod arm;


#[cfg(feature = "arm")]
pub use self::arm::*;


pub use self::reg::Register;
pub use self::reg::XType;


pub use self::per::{ Peripheral, Single };