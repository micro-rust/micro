//! System Timer peripheral.



use crate::reg::{ DefaultRegister, Register };



/// The `Systick` trait includes all the necessary methods to interact with the
/// Systick peripheral. This trait must be implemented by an empty struct in
/// the HAL crates. This struct must be protected by a locking mechanism.
pub trait Systick {
    /// Enables the Systick.
    #[inline(always)]
    fn enable(&mut self) {
        // Set the Interrupt Enable bit in Control and Status Register (CSR).
        DefaultRegister::at(0xE000E010).set(1u32);
    }

    /// Disable the Systick.
    #[inline(always)]
    fn disable(&mut self) {
        // Clear the Enable bit in Control and Status Register (CSR).
        DefaultRegister::at(0xE000E010).clear(1u32);
    }

    /// Enables the Systick Exception generation.
    #[inline(always)]
    fn intenable(&mut self) {
        // Set the Interrupt Enable bit in Control and Status Register (CSR).
        DefaultRegister::at(0xE000E010).set(1u32 << 1);
    }

    /// Disable the Systick Exception generation.
    #[inline(always)]
    fn intdisable(&mut self) {
        // Clear the Interrupt Enable bit in Control and Status Register (CSR).
        DefaultRegister::at(0xE000E010).clear(1u32 << 1);
    }

    /// Selects external clock as the clock source.
    #[inline(always)]
    fn external(&mut self) {
        // Clear the Clock Source bit in the Control and Status Register (CSR).
        DefaultRegister::at(0xE000E010).set(1u32 << 2);
    }

    /// Selects processor clock as the clock source.
    #[inline(always)]
    fn processor(&mut self) {
        // Set the Clock Source bit in the Control and Status Register (CSR).
        DefaultRegister::at(0xE000E010).set(1u32 << 2);
    }

    /// Sets the next value to be reloaded into the counter.
    #[inline(always)]
    fn reload(&mut self, v: u32) {
        // Set the Reload Value Register.
        DefaultRegister::at(0xE000E014).write(v);

        // Set the Current Value Register.
        DefaultRegister::at(0xE000E018).write(v);
    }

    /// Reads the current counter value.
    #[inline(always)]
    fn current(&mut self) -> u32 {
        // Read the Current Value Register.
        DefaultRegister::at(0xE000E018).read()
    }
}
