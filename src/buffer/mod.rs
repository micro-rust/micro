



#[repr(C)]
pub struct Buffer<T: 'static> {
    /// Wrapped buffer.
    buffer: &'static mut [T],

    /// Expected amount of items.
    pub expected: usize,

    /// Amount of items actually transfered.
    pub actual: usize,
}

impl<T: 'static> Buffer<T> {
    /// Creates a new `RXBuffer`.
    pub fn new(buffer: &'static mut [T]) -> Self {
        Self { buffer, expected: 0, actual: 0, }
    }

    /// Resets the actual and expected counts.
    pub fn reset(&mut self) {
        self.expected = 0;
        self.actual = 0;
    }

    /// Sets the expected amoun of items.
    pub fn expected(&mut self, expected: usize) {
        self.expected = expected;
    }

    /// Consumes the buffer and creates a buffer writer.
    pub fn writer(self) -> BufferWriter<T> {
        BufferWriter {
            buffer: self.buffer,
            expected: self.expected,
            actual: self.actual,
        }
    }
}



/// TX Buffer Writer. Writes the data that will be sent and updates the expected count.
#[repr(C)]
pub struct BufferWriter<T: 'static> {
    /// Wrapped buffer.
    buffer: &'static mut [T],

    /// Expected amount of items.
    pub expected: usize,

    /// Amount of items actually transfered.
    pub actual: usize,
}

impl<T: 'static> core::ops::Index<usize> for BufferWriter<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T: 'static> core::ops::IndexMut<usize> for BufferWriter<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}
