//! Common library for the `micro`framework ecosystem.

#![no_std]

#![deny(warnings)]

#![feature(asm_const)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(generic_associated_types)]



/// Buffer utilities.
pub mod buffer;

/// Interrupt utilities.
pub mod int;

/// Abstraction for Hardware Registers.
pub mod reg;

/// Common traits for system resources.
pub mod res;

/// Drivers for different peripherals (internal and external).
pub mod drivers;


#[cfg(feature = "arm")]
mod arm;


#[cfg(feature = "arm")]
pub use self::arm::*;
