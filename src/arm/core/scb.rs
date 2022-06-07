//! System Control Block peripheral.



use crate::reg::{ DefaultRegister, Register };



/// The `SystemControl` trait includes all the necessary methods to interact with the
/// SCB peripheral. This trait must be implemented by an empty struct in the
/// HAL crates. This struct must be protected by a locking mechanism.
pub trait SystemControl {
    /// Requests a system level reset.
    /// UNSAFETY : Calling this method resets the CPU, which may break other
    /// actions being performed in parallel.
    #[inline(always)]
    unsafe fn reset(&mut self) -> ! {
        // Reference to the Application Interrupt and Reset Control Register (AIRCR).
        let aircr = DefaultRegister::at(0xE000ED0C);

        // Request the system reset.
        aircr.write( (0x05FAu32 << 16) | (1u32 << 2) );

        loop { crate::asm::nop() }
    }

    /// Enables / Disables Send Event on Pending bit.
    /// enabling this makes any exception or interrupt generated to wake up the
    /// processor.
    /// UNSAFETY : If disabled, the processor may loose the ability to wake from sleep.
    #[inline(always)]
    unsafe fn sevonpend(&mut self, s: bool) {
        // Reference to the System Control Register (SCR).
        let scr = DefaultRegister::at(0xE000ED10);

        // Enable / Disable SEVONPEND.
        if s { scr.set( 1u32 << 4 ) }
        else { scr.clear( 1u32 << 4) }
    }

    /// Enables Deep Sleep state.
    /// UNSAFETY : The processor may not be configured to wake up from deep sleep.
    #[inline(always)]
    unsafe fn deepsleep(&mut self) {
        // Reference to the System Control Register (SCR).
        let scr = DefaultRegister::at(0xE000ED10);

        // Set Deep Sleep as sleep mode.
        scr.set( 1u32 << 2 );
    }

    /// Enables Normal Sleep state.
    #[inline(always)]
    fn normalsleep(&mut self) {
        // Reference to the System Control Register (SCR).
        let scr = DefaultRegister::at(0xE000ED10);

        // Set Deep Sleep as sleep mode.
        scr.clear( 1u32 << 2 );
    }

    /// Enables / Disables Sleep On Exit from Handler mode.
    /// UNSAFETY : The processor may not be configured to wake up from sleep.
    #[inline(always)]
    unsafe fn sleeponexit(&mut self, s: bool) {
        // Reference to the System Control Register (SCR).
        let scr = DefaultRegister::at(0xE000ED10);

        // Enable / Disable SEVONPEND.
        if s { scr.set( 1u32 << 1 ) }
        else { scr.clear( 1u32 << 1) }
    }

    /// Modifies VTOR to point to the given location.
    #[inline(always)]
    unsafe fn vtor(&mut self, addr: usize) {
        // Reference to the VTOR.
        let vtor = DefaultRegister::at(0xE000ED08);

        // Write the new VTOR.
        vtor.write( addr );

        // Set memory barriers.
        crate::asm::dmb();
        crate::asm::dsb();
        crate::asm::isb();
    }
}
