//! I2C Address module.


use crate::drivers::Data;


/// Common trait for I2C Address types.
pub trait I2CAddress: Sized + Data + Into<u16> + Into<u32> + PartialEq + Eq {}


/// 7-bit I2C address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2CAddress7bit(pub u8);

impl I2CAddress7bit {
    /// Creates a new I2c Address in 7 bit mode.
    pub const fn new(addr: u8) -> Self {
        I2CAddress7bit(addr & 0x7F)
    }
}

impl core::convert::From<I2CAddress7bit>  for u16 {
    #[inline(always)]
    fn from(addr: I2CAddress7bit) -> u16 {
        addr.0 as u16
    }
}

impl core::convert::From<I2CAddress7bit>  for u32 {
    #[inline(always)]
    fn from(addr: I2CAddress7bit) -> u32 {
        addr.0 as u32
    }
}



/// 10-bit I2C address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2CAddress10bit(pub u16);

impl I2CAddress10bit {
    /// Creates a new I2c Address in 7 bit mode.
    pub const fn new(addr: u16) -> Self {
        I2CAddress10bit(addr & 0x3FF)
    }
}

impl core::convert::From<I2CAddress10bit>  for u16 {
    #[inline(always)]
    fn from(addr: I2CAddress10bit) -> u16 {
        addr.0
    }
}

impl core::convert::From<I2CAddress10bit>  for u32 {
    #[inline(always)]
    fn from(addr: I2CAddress10bit) -> u32 {
        addr.0 as u32
    }
}



impl Data for I2CAddress7bit {}
impl Data for I2CAddress10bit {}

impl I2CAddress for I2CAddress7bit {}
impl I2CAddress for I2CAddress10bit {}
