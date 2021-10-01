//! Common abstraction for FrequencyS.


use core::convert::*;
use core::ops::*;


#[cfg(not(feature = "experimental"))]
pub use self::FrequencyS as Frequency;


#[cfg(feature = "experimental")]
pub use self::FrequencyX as Frequency;


/// Tested implementatino of Frequency abstraction.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrequencyS(u32);

impl FrequencyS {
    const KHZ : u32 = 1000;
    const MHZ : u32 = Self::KHZ * 1000;
    const GHZ : u32 = Self::MHZ * 1000;

    pub const fn zero() -> FrequencyS { FrequencyS(0) }

    pub const fn hz(val: u32)  -> FrequencyS { FrequencyS(val)             }
    pub const fn khz(val: u32) -> FrequencyS { FrequencyS(val * Self::KHZ) }
    pub const fn mhz(val: u32) -> FrequencyS { FrequencyS(val * Self::MHZ) }
    pub const fn ghz(val: u32) -> FrequencyS { FrequencyS(val * Self::GHZ) }

    pub const fn as_hz(&self)  -> u32 { self.0             }
    pub const fn as_khz(&self) -> u32 { self.0 / Self::KHZ }
    pub const fn as_mhz(&self) -> u32 { self.0 / Self::MHZ }
    pub const fn as_ghz(&self) -> u32 { self.0 / Self::GHZ }

    pub const fn is_zero(&self) -> bool { self.0 == 0 }
}

impl Mul<u32> for FrequencyS {
    type Output = FrequencyS;

    fn mul(self, rhs: u32) -> FrequencyS {
        FrequencyS(self.0 * rhs)
    }
}

impl Div<u32> for FrequencyS {
    type Output = FrequencyS;

    fn div(self, rhs: u32) -> FrequencyS {
        FrequencyS(self.0 / rhs)
    }
}

impl core::convert::Into<u32> for FrequencyS {
    fn into(self) -> u32 {
        self.0
    }
}




/// Experimental implementation of Frequency asbtraction.
pub struct FrequencyX<const M: usize>(usize);



pub type GHz = FrequencyX<1_000_000_000>;
pub type MHz = FrequencyX<1_000_000>;
pub type KHz = FrequencyX<1_000>;
pub type Hz  = FrequencyX<1>;



impl<const M: usize> FrequencyX<M> {
    /// Creates a new Frequency abstraction.
    pub const fn new(f: usize) -> Self {
        Self(f)
    }
}



impl<const M: usize> Mul<usize> for FrequencyX<M> {
    type Output = FrequencyX<M>;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<const M: usize> Div<usize> for FrequencyX<M> {
    type Output = FrequencyX<M>;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<const M: usize, const N: usize> Div<FrequencyX<N>> for FrequencyX<M> {
    type Output = usize;

    fn div(self, rhs: FrequencyX<N>) -> Self::Output {
        (self.0 / rhs.0) * (M / N)
    }
}



impl From<GHz> for MHz {
    fn from(input: GHz) -> MHz {
        MHz::new(input.0 * 1_000)
    }
}

impl From<GHz> for KHz {
    fn from(input: GHz) -> KHz {
        KHz::new(input.0 * 1_000_000)
    }
}

impl From<GHz> for Hz {
    fn from(input: GHz) -> Hz {
        Hz::new(input.0 * 1_000_000_000)
    }
}

impl From<MHz> for GHz {
    fn from(input: MHz) -> GHz {
        GHz::new(input.0 / 1_000)
    }
}

impl From<MHz> for KHz {
    fn from(input: MHz) -> KHz {
        KHz::new(input.0 * 1_000)
    }
}

impl From<MHz> for Hz {
    fn from(input: MHz) -> Hz {
        Hz::new(input.0 * 1_000_000)
    }
}

impl From<KHz> for GHz {
    fn from(input: KHz) -> GHz {
        GHz::new(input.0 / 1_000_000)
    }
}

impl From<KHz> for MHz {
    fn from(input: KHz) -> MHz {
        MHz::new(input.0 / 1_000)
    }
}

impl From<KHz> for Hz {
    fn from(input: KHz) -> Hz {
        Hz::new(input.0 * 1_000)
    }
}

impl From<Hz> for GHz {
    fn from(input: Hz) -> GHz {
        GHz::new(input.0 / 1_000_000_000)
    }
}

impl From<Hz> for MHz {
    fn from(input: Hz) -> MHz {
        MHz::new(input.0 / 1_000_000)
    }
}

impl From<Hz> for KHz {
    fn from(input: Hz) -> KHz {
        KHz::new(input.0 / 1_000)
    }
}
