//! Debug extensions.


pub struct Debug;

impl Debug {
    /// Returns `true` if the last debug was caused by an external source.
    #[inline(always)]
    pub fn external(&self) -> bool {
        unsafe { read(0xE000ED30 as *const u32) & (1 << 4) != 0 }
    }

    /// Returns `true` if the last debug was caused by a vector catch.
    #[inline(always)]
    pub fn vectorcatch(&self) -> bool {
        unsafe { read(0xE000ED30 as *const u32) & (1 << 3) != 0 }
    }

    /// Returns `true` if the last debug was caused by the DWT.
    #[inline(always)]
    pub fn dwt(&self) -> bool {
        unsafe { read(0xE000ED30 as *const u32) & (1 << 2) != 0 }
    }

    /// Returns `true` if the last debug was caused by a breakpoint.
    #[inline(always)]
    pub fn breakpoint(&self) -> bool {
        unsafe { read(0xE000ED30 as *const u32) & (1 << 1) != 0 }
    }

    /// Returns `true` if the last debug was caused by a HALT or STEP.
    #[inline(always)]
    pub fn halt(&self) -> bool {
        unsafe { read(0xE000ED30 as *const u32) & (1 << 0) != 0 }
    }

    /// Returns `true` if the processor is locked up by an unrecoverable exception.
    #[inline(always)]
    pub fn lockup(&self) -> bool {
        unsafe { read(0xE000EDF0 as *const u32) & (1 << 19) != 0 }
    }

    /// Returns `true` if the processor is sleeping.
    #[inline(always)]
    pub fn sleeping(&self) -> bool {
        unsafe { read(0xE000EDF0 as *const u32) & (1 << 18) != 0 }
    }

    /// Returns `true` if the processor is halted.
    #[inline(always)]
    pub fn halted(&self) -> bool {
        unsafe { read(0xE000EDF0 as *const u32) & (1 << 17) != 0 }
    }
}