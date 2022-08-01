//! Drivers module.
//! Contains the drivers / traits / abstractions over the different types
//! of peripherals in a MCU.




pub mod comm;

//pub mod dma;

//pub mod usb;

/*

/// Common trait for data types that can be operated on.
pub trait Data: Copy + Sized {
    /// Size in bytes of the data type.
    const SIZE: usize = core::mem::size_of::<Self>();
}


impl Data for i8  {}
impl Data for u8  {}
impl Data for i16 {}
impl Data for u16 {}
impl Data for i32 {}
impl Data for u32 {}
impl Data for i64 {}
impl Data for u64 {}


/// Basic async/blocking handle type to be wrapped by specialized handles.
/// Bits 8 through 31 are available for each wrapper's specific errors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Handle(pub u32);

impl Handle {
    /// Flag set when this handle is launched.
    pub const LAUNCH: u32 = 0x1;

    /// Flag set when this handle copmletes.
    pub const DONE: u32 = 0x2;

    /// Flag set when this handle errors.
    pub const ERROR: u32 = 0x4;

    /// Static initializer.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Returns `true` if the `LAUNCH` flag is set.
    #[inline(always)]
    pub fn is_launched(&self) -> bool {
        (self.0 & Self::LAUNCH) != 0
    }

    /// Returns `true` if the `DONE` flag is set.
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        (self.0 & Self::DONE) != 0
    }

    /// Returns `true` if the `ERROR` flag is set.
    #[inline(always)]
    pub fn is_errored(&self) -> bool {
        (self.0 & Self::ERROR) != 0
    }

    /// Sets the `LAUNCH` flag.
    #[inline(always)]
    pub fn launch(&mut self) {
        self.0 |= Self::LAUNCH
    }

    /// Sets the `DONE` flag.
    #[inline(always)]
    pub fn done(&mut self) {
        self.0 |= Self::DONE
    }

    /// Sets the `ERROR` flag.
    #[inline(always)]
    pub fn errored(&mut self) {
        self.0 |= Self::ERROR
    }
}

*/