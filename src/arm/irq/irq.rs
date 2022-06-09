//! Interrupt Request Table.


#[repr(C)]
pub struct IRQTable<const LEN: usize> {
    table: [fn(); LEN],
}

impl<const LEN: usize> IRQTable<LEN> {
    /// Creates a IRQTable at the given address and returns a reference.
    #[inline(always)]
    pub fn at(addr: u32) -> &'static mut Self {
        unsafe { &mut *(addr as *mut _) }
    }

    /// Initializes the table.
    pub fn init(&mut self) {
        self.table = [default; LEN];
    }

    /// Sets the given IRQ handler.
    #[inline(always)]
    pub fn set<const IRQ: usize>(&mut self, f: fn()) {
        self.table[IRQ] = f;
    }

    /// Sets the given IRQ handler.
    #[inline(always)]
    pub fn setn(&mut self, irq: u8, f: fn()) {
        self.table[irq as usize] = f;
    }
}


/// Default IRQ is just to return to execution.
/// Adds some cycles of latency.
fn default() {
    return;
}
