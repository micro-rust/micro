//! Exception Request Table.


#[repr(C)]
pub struct EXCTable {
    table: [fn(); 16],
}

impl EXCTable {
    /// Creates a EXCTable at the given address and returns a reference.
    #[inline(always)]
    pub fn at(addr: u32) -> &'static mut EXCTable {
        unsafe { &mut *(addr as *mut _) }
    }

    /// Initializes the table.
    /// Ignores the SP and Reset vectors.
    pub fn init(&mut self) {
        for i in 2..=16 {
            self.table[i] = default;
        }
    }

    /// Sets the NMI exception handler.
    #[inline(always)]
    pub fn nmi(&mut self, f: fn()) {
        self.table[2] = f;
    }

    /// Sets the Hardfault exception handler.
    #[inline(always)]
    pub fn hardfault(&mut self, f: fn()) {
        self.table[3] = f;
    }

    #[cfg(not(target_arch="thumbv6"))]
    #[doc = "Sets the Memfault exception handler"]
    #[inline(always)]
    pub fn memfault(&mut self, f: fn()) {
        self.table[4] = f;
    }

    #[cfg(not(target_arch="thumbv6"))]
    #[doc = "Sets the Busfault exception handler"]
    #[inline(always)]
    pub fn busfault(&mut self, f: fn()) {
        self.table[5] = f;
    }

    #[cfg(not(target_arch="thumbv6"))]
    #[doc = "Sets the Usagefault exception handler"]
    #[inline(always)]
    pub fn usagefault(&mut self, f: fn()) {
        self.table[6] = f;
    }


    /// Sets the SVCall exception handler.
    #[inline(always)]
    pub fn svcall(&mut self, f: fn()) {
        self.table[11] = f;
    }

    #[cfg(not(target_arch="thumbv6"))]
    #[doc = "Sets the Debug monitor exception handler"]
    #[inline(always)]
    pub fn debug(&mut self, f: fn()) {
        self.table[12] = f;
    }

    /// Sets the PendSV exception handler.
    #[inline(always)]
    pub fn pendsv(&mut self, f: fn()) {
        self.table[14] = f;
    }

    /// Sets the Systick exception handler.
    #[inline(always)]
    pub fn systick(&mut self, f: fn()) {
        self.table[15] = f;
    }
}


/// Default IRQ is just to return to execution.
/// Adds some cycles of latency.
fn default() {
    return;
}
