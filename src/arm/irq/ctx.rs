//! IRQ Context module.
//! Grants ISR scoped access to IRQ configuration and related data structures.



/// A `Context` contains an IRQ configuration and IRQ associated data.
pub struct Context<T, const N: u32> {
    /// `cfg` allows for IRQ reconfiguration.
    pub cfg: IRQConfig<N>,

    /// `data` grants access to ISR specific data.
    pub data: T,
}

impl<T: Sized, const N: u32> Context<T, N> {
    /// Creates a new `Context`.
    /// UNSAFETY: Does not check for uniqueness or context correctness.
    pub unsafe fn new(data: T) -> Self {
        Self { cfg: IRQConfig::new(), data }
    }
}



/// An `IRQConfig` struct is used to enable, disable or reconfigure an IRQ
/// from within it's own ISR.
pub struct IRQConfig<const N: u32>;

impl<const N: u32> IRQConfig<N> {

    /// Interrupt Clear Enable register.
    const ICER: u32 = 0xE000E180 + (N / 32);

    /// Interrupt Set Enable register.
    const ISER: u32 = 0xE000E100 + (N / 32);

    /// Interrupt Clear Pending register.
    const ICPR: u32 = 0xE000E280 + (N / 32);

    /// Interrupt Set Pending register.
    const ISPR: u32 = 0xE000E200 + (N / 32);


    /// Related bit for this interrupt.
    const BIT: u32 = 1 << (N % 32);


    /// Interrupt Priority register.
    const IPR: u32 = 0xE000E400 + (N / 4);

    /// Priority offset.
    const POFF: u32 = 8 * (N % 4);


    /// Creates a new IRQ Configuration struct.
    /// UNSAFETY: Does not check for uniqueness or contextual correctness.
    pub const unsafe fn new() -> Self {
        Self
    }

    /// Disables the IRQ.
    #[inline]
    pub fn disable(&mut self) {
        unsafe { core::ptr::write_volatile(Self::ICER as *mut u32, Self::BIT) }
    }

    /// Enables the IRQ.
    #[inline]
    pub fn enable(&mut self) {
        unsafe { core::ptr::write_volatile(Self::ISER as *mut u32, Self::BIT) }
    }

    /// Clears the pending status of the IRQ.
    #[inline]
    pub fn clearpend(&mut self) {
        unsafe { core::ptr::write_volatile(Self::ICPR as *mut u32, Self::BIT) }
    }

    /// Sets the pending status of the IRQ.
    #[inline]
    pub fn setpend(&mut self) {
        unsafe { core::ptr::write_volatile(Self::ISPR as *mut u32, Self::BIT) }
    }

    /// Sets the priority of the IRQ.
    #[inline]
    pub fn priority(&mut self, p: u8) {
        unsafe {
            let mut r = core::ptr::read_volatile(Self::IPR as *const u32);
            r &= !(0xFF << Self::POFF);
            r |= (p as u32) << Self::POFF;

            core::ptr::write_volatile(Self::IPR as *mut u32, r);
        }
    }
}
