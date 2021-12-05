//! Cortex MPU.
//! Only implemented the Cortex M0+ MPU.


use crate::Register;


use core::marker::PhantomData;


pub struct MPU<R: Register<u32>>(PhantomData<R>);

impl<R: Register<u32>> MPU<R> {
    /// Creates an empty MPU abstraction.
    pub fn empty() -> Self {
        Self(PhantomData)
    }

    /// Enables the MPU.
    #[inline(always)]
    pub fn enable(&mut self) {
        let ctrl = unsafe { &mut *(0xE000ED94 as *mut R) };

        ctrl.write( (1 << 2) | 1 );
    }

    /// Disables the MPU.
    #[inline(always)]
    pub fn disable(&mut self) {
        let ctrl = unsafe { &mut *(0xE000ED94 as *mut R) };

        ctrl.write( 0 );
    }

    /// Configures a region.
    pub fn region(&mut self, n: u8, addr: u8, size: MPURegionSize, sub: u8, perm: MPUPermissions) {
        let rbar = unsafe { &mut *(0xE000ED9C as *mut R) };
        let rasr = unsafe { &mut *(0xE000EDA0 as *mut R) };

        rbar.write( (addr & !0xFF) | (1 << 4) | (n & 0x0F) );
        rasr.write( ((perm as u8) << 16) | (srd << 8) | ((size as u8) << 1) | 1 );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MPURegionSize {
    Bytes256 = 7,
    Bytes512 = 8,

    KBytes1   =  9,
    KBytes2   = 10,
    KBytes4   = 11,
    KBytes8   = 12,
    KBytes16  = 13,
    KBytes32  = 14,
    KBytes64  = 15,
    KBytes128 = 16,
    KBytes256 = 17,
    KBytes512 = 18,

    MBytes1   = 19,
    MBytes2   = 20,
    MBytes4   = 21,
    MBytes8   = 22,
    MBytes16  = 23,
    MBytes32  = 24,
    MBytes64  = 25,
    MBytes128 = 26,
    MBytes256 = 27,
    MBytes512 = 28,

    GBytes1 = 29,
    GBytes2 = 30,
    GBytes4 = 31,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MPUPermissions {
    None = 0b000,
    ReadWrite_None = 0b001,
    ReadWrite_ReadOnly = 0b010,
    ReadWrite_ReadWrite = 0b011,

    ReadOnly_None = 0b100,
    ReadOnly_ReadOnly = 0b111,
}
