//! MPU Core peripheral.


use crate::reg::{ DefaultRegister, Register };



/// The `MemoryProtection` trait contains the necessary methods to configure th
/// MPU peripheral in the ARM Cores.This trait must be implemented by an empty struct in the
/// HAL crates. This struct must be protected by a locking mechanism.
pub trait MPU {
    /// Configures the given memory region and enables it.
    fn configure(&mut self, cfg: MPUConfiguration) {
        // Reference to the MPU registers.
        let mpu = DefaultRegister::array::<5>(0xE000ED90);

        // Write the RBAR.
        mpu[3].write(cfg.rbar);

        // Write the RBAR.
        mpu[4].write(cfg.rasr | 1);
    }

    /// Disables the MPU.
    /// UNSAFETY : Calling this method breaks memory guarantees.
    unsafe fn disable(&mut self) {
        // Reference to the MPU registers.
        let mpu = DefaultRegister::array::<5>(0xE000ED90);

        // Disable MPU.
        mpu[1].clear(1);
    }

    /// Enables the MPU.
    /// UNSAFETY : Calling this method breaks memory guarantees.
    unsafe fn enable(&mut self) {
        // Reference to the MPU registers.
        let mpu = DefaultRegister::array::<5>(0xE000ED90);

        // Disable MPU.
        mpu[1].set(1);
    }

    /// Disables the given region.
    /// UNSAFETY : Calling this method breaks memory guarantees.
    unsafe fn disableregion(&mut self, n: u8) {
        // Reference to the MPU registers.
        let mpu = DefaultRegister::array::<5>(0xE000ED90);

        // Set which region is being modified.
        mpu[2].write(n as u32);

        // Disable region.
        mpu[4].clear(1);
    }

    /// Enables the given region.
    /// UNSAFETY : Calling this method breaks memory guarantees.
    unsafe fn enableregion(&mut self, n: u8) {
        // Reference to the MPU registers.
        let mpu = DefaultRegister::array::<5>(0xE000ED90);

        // Set which region is being modified.
        mpu[2].write(n as u32);

        // Enable region.
        mpu[4].set(1);
    }
}


/// MPU Configuration struct. 
pub struct MPUConfiguration {
    /// Region Base Address Register.
    pub(self) rbar: u32,

    /// Region Attribute and Size Register.
    pub(self) rasr: u32,
}

impl MPUConfiguration {
    /// Creates a new empty MPU Configuration.
    pub const fn new() -> Self {
        MPUConfiguration {
            rbar: 0,
            rasr: 0,
        }
    }

    /// Configures the region's address, size and region number.
    /// This method fails if the region address does not align with the region size.
    pub const fn region(mut self, n: u8, addr: u32, size: MPURegionSize) -> Result<Self, ()> {
        if (addr & !size.mask()) == addr {
            self.rbar = addr | (1 << 4) | (n as u32);
            self.rasr = (self.rasr & !0x3E) | u32::from(size);

            Ok(self)
        } else {
            Err(())
        }
    }

    /// Configures the region's subregion disable.
    #[inline]
    pub const fn subregions(mut self, mask: u8) -> Self {
        self.rasr = (self.rasr & !(0xFF << 8)) | ((mask as u32) << 8);
        self
    }

    /// Configures the region's permissions.
    #[inline]
    pub const fn permissions(mut self, p: MPUPermissions, u: MPUPermissions) -> Self {
        self.rasr = self.rasr & !(0x7 << 24);

        match (p, u) {
            (MPUPermissions::None, MPUPermissions::None) => (),

            (MPUPermissions::RW,   MPUPermissions::None) => self.rasr |= 0b001 << 24,
            (MPUPermissions::RW,   MPUPermissions::RO  ) => self.rasr |= 0b010 << 24,
            (MPUPermissions::RW,   MPUPermissions::RW  ) => self.rasr |= 0b011 << 24,

            (MPUPermissions::RO,   MPUPermissions::None) => self.rasr |= 0b101 << 24,
            (MPUPermissions::RO,   MPUPermissions::RO  ) => self.rasr |= 0b111 << 24,

            _ => (),
        }

        self
    }

    /// Enables execution from this region.
    #[inline]
    pub const fn execute(mut self) -> Self {
        self.rasr &= !(1 << 28);
        self
    }

    /// Disables execution from this region.
    #[inline]
    pub const fn noexecute(mut self) -> Self {
        self.rasr |= 1 << 28;
        self
    }
}


/// All possible permissions of a region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPUPermissions {
    RW,
    RO,
    None,
}


/// All possible MPU Region Sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPURegionSize {
    Bytes256,
    Bytes512,

    KBytes1,
    KBytes2,
    KBytes4,
    KBytes8,
    KBytes16,
    KBytes32,
    KBytes64,
    KBytes128,
    KBytes256,
    KBytes512,

    MBytes1,
    MBytes2,
    MBytes4,
    MBytes8,
    MBytes16,
    MBytes32,
    MBytes64,
    MBytes128,
    MBytes256,
    MBytes512,

    GBytes1,
    GBytes2,
    GBytes4,
}

impl MPURegionSize {
    /// Address mask needed for the Region Base Address.
    pub const fn mask(&self) -> u32 {
        match *self {
            MPURegionSize::Bytes256 => (1 << 7) - 1,
            MPURegionSize::Bytes512 => (1 << 8) - 1,

            MPURegionSize::KBytes1   => (1 <<  9) - 1,
            MPURegionSize::KBytes2   => (1 << 10) - 1,
            MPURegionSize::KBytes4   => (1 << 11) - 1,
            MPURegionSize::KBytes8   => (1 << 12) - 1,
            MPURegionSize::KBytes16  => (1 << 13) - 1,
            MPURegionSize::KBytes32  => (1 << 14) - 1,
            MPURegionSize::KBytes64  => (1 << 15) - 1,
            MPURegionSize::KBytes128 => (1 << 16) - 1,
            MPURegionSize::KBytes256 => (1 << 17) - 1,
            MPURegionSize::KBytes512 => (1 << 18) - 1,

            MPURegionSize::MBytes1   => (1 << 19) - 1,
            MPURegionSize::MBytes2   => (1 << 20) - 1,
            MPURegionSize::MBytes4   => (1 << 21) - 1,
            MPURegionSize::MBytes8   => (1 << 22) - 1,
            MPURegionSize::MBytes16  => (1 << 23) - 1,
            MPURegionSize::MBytes32  => (1 << 24) - 1,
            MPURegionSize::MBytes64  => (1 << 25) - 1,
            MPURegionSize::MBytes128 => (1 << 26) - 1,
            MPURegionSize::MBytes256 => (1 << 27) - 1,
            MPURegionSize::MBytes512 => (1 << 28) - 1,

            MPURegionSize::GBytes1 => (1 << 29) - 1,
            MPURegionSize::GBytes2 => (1 << 30) - 1,
            MPURegionSize::GBytes4 => (1 << 31) - 1,
        }
    }
}

impl const core::convert::From<MPURegionSize> for u32 {
    fn from(s: MPURegionSize) -> u32 {
        match s {
            MPURegionSize::Bytes256 => 7 << 1,
            MPURegionSize::Bytes512 => 8 << 1,

            MPURegionSize::KBytes1   =>  9 << 1,
            MPURegionSize::KBytes2   => 10 << 1,
            MPURegionSize::KBytes4   => 11 << 1,
            MPURegionSize::KBytes8   => 12 << 1,
            MPURegionSize::KBytes16  => 13 << 1,
            MPURegionSize::KBytes32  => 14 << 1,
            MPURegionSize::KBytes64  => 15 << 1,
            MPURegionSize::KBytes128 => 16 << 1,
            MPURegionSize::KBytes256 => 17 << 1,
            MPURegionSize::KBytes512 => 18 << 1,

            MPURegionSize::MBytes1   => 19 << 1,
            MPURegionSize::MBytes2   => 20 << 1,
            MPURegionSize::MBytes4   => 21 << 1,
            MPURegionSize::MBytes8   => 22 << 1,
            MPURegionSize::MBytes16  => 23 << 1,
            MPURegionSize::MBytes32  => 24 << 1,
            MPURegionSize::MBytes64  => 25 << 1,
            MPURegionSize::MBytes128 => 26 << 1,
            MPURegionSize::MBytes256 => 27 << 1,
            MPURegionSize::MBytes512 => 28 << 1,

            MPURegionSize::GBytes1 => 29 << 1,
            MPURegionSize::GBytes2 => 30 << 1,
            MPURegionSize::GBytes4 => 31 << 1,
        }
    }
}
