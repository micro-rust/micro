//! System timer peripheral.


use crate::Peripheral;
use crate::Register;


use core::marker::PhantomData;


pub struct Systick<R: Register<u32>>(PhantomData<R>);


impl<R: Register<u32>> Systick<R> {
    /// Creates an empty Systick abstraction.
    pub fn empty() -> Self {
        Self(PhantomData)
    }

    /// Sets the clock source as the external clock.
    #[inline(always)]
    pub fn external(&mut self) {
        let mut systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        systick[0].clear(1 << 2)
    }

    /// Sets the clock source as the processor's clock.
    #[inline(always)]
    pub fn processor(&mut self) {
        let mut systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        systick[0].set(1 << 2)
    }

    /// Enables / Disables the Systick interrupt.
    #[inline(always)]
    pub fn interrupt(&mut self, s: bool) {
        let mut systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        if s { systick[0].set(1 << 1) }
        else { systick[0].clear(1 << 1) }
    }

    /// Enables the system timer.
    #[inline(always)]
    pub fn enable(&mut self) {
        let mut systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        systick[0].set(1)
    }

    /// Disables the system timer.
    #[inline(always)]
    pub fn disable(&mut self) {
        let mut systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        systick[0].clear(1)
    }

    /// Reloads the system timer with this countdown value.
    #[inline(always)]
    pub fn reload(&mut self, rvr: u32) {
        let mut systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        systick[1].write(rvr & 0xFFFFFF)
    }

    /// Returns the current count.
    #[inline(always)]
    pub fn current() -> u32 {
        let systick: Peripheral<u32, R, 4, 0xE000E010> = Peripheral::get();

        systick[2].read()
    }
}
