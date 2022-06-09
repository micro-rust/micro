//! Controller of the exceptions within a system.



use super::super::asm::*;



#[repr(C)]
pub struct ExceptionControl {
    #[doc(hidden)]
    _sp: u32,



    #[doc(hidden)]
    _reset: u32,



    /// Non Maskable Interrupt.
    /// Available in all versions of Cortex-M.
    nmi: fn(),



    /// Hard Fault.
    /// Available in all versions of Cortex-M.
    hf: fn(),



    #[cfg(any(armv6m, armv8m_base))]
    #[doc(hidden)]
    _r0: u32,

    #[cfg(any(armv7m, armv8m_main))]
    /// Memory Management.
    /// Available in ARMv7 and ARMv8.main.
    mm: fn(),



    #[cfg(any(armv6m, armv8m_base))]
    #[doc(hidden)]
    _r1: u32,

    #[cfg(any(armv7m, armv8m_main))]
    /// Bus Fault.
    /// Available in ARMv7 and ARMv8.main.
    bf: fn(),



    #[cfg(any(armv6m, armv8m_base))]
    #[doc(hidden)]
    _r2: u32,

    #[cfg(any(armv7m, armv8m_main))]
    /// Usage Fault.
    /// Available in ARMv7 and ARMv8.main.
    uf: fn(),



    #[cfg(any(armv6m, armv7m, armv8m_base))]
    #[doc(hidden)]
    _r3: u32,

    #[cfg(any(armv8m_main))]
    /// Secure Fault.
    /// Available in ARMv8 with Main extension.
    sf: fn(),



    #[doc(hidden)]
    _r4: u32,



    #[doc(hidden)]
    _r5: u32,



    #[doc(hidden)]
    _r6: u32,

    /// SuperVisor Call.
    /// Available in all versions of Cortex-M.
    svc: fn(),



    #[cfg(any(armv6m, armv8m_base))]
    #[doc(hidden)]
    _r7: u32,

    #[cfg(any(armv7m, armv8m_main))]
    /// Debug Monitor.
    /// Available in ARMv7 and ARMv8.
    dm: fn(),



    #[doc(hidden)]
    _r8: u32,



    /// Pend SuperVisor.
    /// Available in all versions of Cortex-M.
    psv: fn(),



    /// Systick.
    /// Available in all versions of Cortex-M.
    stk: fn(),
}

impl ExceptionControl {
    /// Reads the VTOR and returns a reference to the ExceptionControl.
    pub fn get<'a>() -> &'a mut ExceptionControl {
        // Read VTOR.
        let vtor = unsafe { core::ptr::read_volatile( 0xE000ED08 as *const u32 ) };

        // Transform into a reference.
        unsafe { &mut *(vtor as *mut Self) }
    }

    /// Relocates the Vector table and returns the reference to the new one.
    /// If this function returns `None` it means the address given was not aligned.
    pub fn relocate<'a>(&mut self, address: u32) -> Option<&'a mut ExceptionControl> {
        // Validate bits 0-6.
        if (address & !0x3F) != address { return None }

        // Get the source and destination pointers.
        let src: *const u32 = self as *mut Self as *mut u32 as *const u32;
        let dest: *mut u32 = address as *mut u32;

        // Relocate the vector.
        for _ in 0..16 {
            unsafe {
                core::ptr::write_volatile( dest, core::ptr::read_volatile( src ) );
                let src = src.offset(1);
                let dest = dest.offset(1);
            }
        }

        // Disable interrupts.
        cpsid_i();

        // Change VTOR.
        unsafe { core::ptr::write_volatile( 0xE000ED08 as *mut u32, address ) }

        // Memory and instruction barriers.
        dsb();
        dmb();
        isb();

        // Restore interrupts.
        cpsie_i();

        unsafe { Some( &mut *(address as *mut Self) ) }
    }

    /// Sets the given Exception handler.
    pub fn handler(&mut self, e: Exception, sys: fn()) {
        // Disable interrupts.
        cpsid_i();

        match e {
            Exception::NMI => self.nmi = sys,
            Exception::HardFault => self.hf = sys,
            Exception::SVCall => self.svc = sys,
            Exception::PendSV => self.psv = sys,
            Exception::Systick => self.stk = sys,

            #[cfg(any(armv7m, armv8m_main))]
            Exception::MemManage => self.mm = sys,

            #[cfg(any(armv7m, armv8m_main))]
            Exception::BusFault => self.bf = sys,

            #[cfg(any(armv7m, armv8m_main))]
            Exception::UsageFault => self.uf = sys,

            #[cfg(any(armv7m, armv8m_main))]
            Exception::DebugMonitor => self.dm = sys,

            #[cfg(armv8m_main)]
            Exception::SecureFault => self.sf = sys,
        }

        // Memory and instruction barriers.
        dsb();
        dmb();
        isb();

        // Restore interrupts.
        cpsie_i();
    }

    /// Clears the given exception handler.
    /// This function sets the handler of the given exception to 'bx lr' (return).
    pub fn clear(&mut self, e: Exception) {
        self.handler(e, default)
    }

    /// Sets the priority of the given exception.
    pub fn priority(&mut self, e: Exception, prio: u8) {
        // Disable interrupts.
        cpsid_i();

        // Get SHPR register and offset.
        let (r, o) = match e {
            Exception::NMI | Exception::HardFault => return,

            Exception::SVCall => (2, 24),
            Exception::PendSV => (3, 16),
            Exception::Systick => (3, 24),

            #[cfg(any(armv7m, armv8m_main))]
            Exception::MemManage => (1, 0),

            #[cfg(any(armv7m, armv8m_main))]
            Exception::BusFault => (1, 8),

            #[cfg(any(armv7m, armv8m_main))]
            Exception::UsageFault => (1, 16),

            #[cfg(any(armv7m, armv8m_main))]
            Exception::DebugMonitor => (3, 0),

            #[cfg(armv8m_main)]
            Exception::SecureFault => (1, 24),
        };

        // Read the corresponding SHPR register.
        let mut r = unsafe { core::ptr::read_volatile( (0xE000ED14 + r) as *const u32 ) };

        // Clear the previous priority.
        r &= !(0xFF << o);

        // Set the new priority.
        r |= (prio as u32) << o;

        // Write back the SHPR register.
        unsafe { core::ptr::write_volatile( (0xED000ED14 + r) as *mut u32, r ) };

        // Memory and instruction barriers.
        dsb();
        dmb();
        isb();

        // Restore interrupts.
        cpsie_i();
    }
}



/// Exceptions of the Cortex-M architectures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exception {
    /// Non Maskable Interrupt.
    NMI,

    /// Hard Fault.
    HardFault,

    /// Supervisor Call.
    SVCall,

    /// Pend Supervisor Call.
    PendSV,

    /// System tick.
    Systick,

    #[cfg(any(armv7m, armv8m_main))]
    /// Memory management.
    MemManage,

    #[cfg(any(armv7m, armv8m_main))]
    /// Bus Fault.
    BusFault,

    #[cfg(any(armv7m, armv8m_main))]
    /// Usage Fault.
    UsageFault,

    #[cfg(any(armv7m, armv8m_main))]
    /// Debug Monitor.
    DebugMonitor,

    #[cfg(armv8m_main)]
    /// Secure Fault.
    SecureFault,
}



fn default() {
    return;
}
