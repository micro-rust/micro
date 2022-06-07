//! System Timer peripheral.



use crate::reg::{ DefaultRegister, Register };



/// The `Systick` trait includes all the necessary methods to interact with the
/// Systick peripheral. This trait must be implemented by an empty struct in
/// the HAL crates. This struct must be protected by a locking mechanism.
pub trait Systick {
    /// Enables the Systick.
    #[inline(always)]
    fn enable(&mut self) {
        // Reference to the SYST Control and Status Register (CSR).
        let csr = DefaultRegister::at(0xE000E010);

        // Set the ENABLE bit.
        csr.set(1u32);
    }

    /// Disable the Systick.
    #[inline(always)]
    fn disable(&mut self) {
        // Reference to the SYST Control and Status Register (CSR).
        let csr = DefaultRegister::at(0xE000E010);

        // Set the ENABLE bit.
        csr.clear(1u32);
    }

    /// Enables the Systick Exception generation.
    #[inline(always)]
    fn intenable(&mut self) {
        // Reference to the SYST Control and Status Register (CSR).
        let csr = DefaultRegister::at(0xE000E010);

        // Set the ENABLE bit.
        csr.set(1u32 << 1);
    }

    /// Disable the Systick Exception generation.
    #[inline(always)]
    fn intdisable(&mut self) {
        // Reference to the SYST Control and Status Register (CSR).
        let csr = DefaultRegister::at(0xE000E010);

        // Set the ENABLE bit.
        csr.clear(1u32 << 1);
    }

    /// Selects external clock as the clock source.
    #[inline(always)]
    fn external(&mut self) {
        // Reference to the SYST Control and Status Register (CSR).
        let csr = DefaultRegister::at(0xE000E010);

        // Set the ENABLE bit.
        csr.clear(1u32 << 2);
    }

    /// Selects processor clock as the clock source.
    #[inline(always)]
    fn processor(&mut self) {
        // Reference to the SYST Control and Status Register (CSR).
        let csr = DefaultRegister::at(0xE000E010);

        // Set the ENABLE bit.
        csr.set(1u32 << 2);
    }

    /// Sets the next value to be reloaded into the counter.
    #[inline(always)]
    fn reload(&mut self, v: u32) {
        // Reference to the SYST Reload Value Register (RVR).
        let rvr = DefaultRegister::at(0xE000E014);

        // Write the reload value.
        rvr.write(v);
    }

    /// Reads the current counter value.
    #[inline(always)]
    fn current(&mut self) -> u32 {
        // Reference to the SYST Current Value Register (CVR).
        let cvr = DefaultRegister::at(0xE000E018);

        // Read the counter value.
        cvr.read()
    }
}
