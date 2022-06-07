//! Common trait for all registers.



use core::ops::*;



/// Default implementation of a hardware register.
#[repr(transparent)]
pub struct DefaultRegister<T: RegisterData>(T);

impl<T: RegisterData> Register<T> for DefaultRegister<T> {}



/// Common trait for all possible register inner types.
pub trait RegisterData: Sized + BitAnd<Output=Self> + BitOr<Output=Self> + BitXor<Output=Self> + Not<Output=Self> {}

impl RegisterData for i8  {}
impl RegisterData for u8  {}
impl RegisterData for i16 {}
impl RegisterData for u16 {}
impl RegisterData for i32 {}
impl RegisterData for u32 {}
impl RegisterData for i64 {}
impl RegisterData for u64 {}

impl RegisterData for isize {}
impl RegisterData for usize {}



/// Common trait for all possible register types.
/// Each hardware may have a different implementation (e.g. hardware atomics, bit banding, etc...).
pub trait Register<T: RegisterData>: Sized {
    /// Reference creator.
    fn at<'a>(addr: usize) -> &'a mut Self {
        unsafe { &mut *(addr as *mut Self) }
    }

    /// Array reference creator.
    fn array<'a, const N: usize>(addr: usize) -> &'a mut [Self; N] {
        unsafe { &mut *(addr as *mut [Self; N]) }
    }

    /// Reads the register from memory.
    fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(self as *const Self as *const T) }
    }

    /// Writes the given value to the register.
    fn write(&mut self, val: T) {
        unsafe { core::ptr::write_volatile(self as *mut Self as *mut T, val) }
    }

    /// Clears the given bit mask in the register.
    fn clear(&mut self, mask: T) {
        self.write( self.read() & !mask );
    }

    /// Sets the given bit mask in the register.
    fn set(&mut self, mask: T) {
        self.write( self.read() | mask );
    }

    /// Toggles the given bit mask in the register.
    fn toggle(&mut self, mask: T) {
        self.write( self.read() ^ mask )
    }
}

impl<D: RegisterData> Not for &'_ DefaultRegister<D> {
    type Output = D;

    fn not(self) -> Self::Output {
        !self.read()
    }
}

impl<D: RegisterData> BitAnd<D> for &'_ DefaultRegister<D> {
    type Output = D;

    fn bitand(self, rhs: D) -> Self::Output {
        self.read() & rhs
    }
}

impl<D: RegisterData> BitOr<D> for &'_ DefaultRegister<D> {
    type Output = D;

    fn bitor(self, rhs: D) -> Self::Output {
        self.read() | rhs
    }
}

impl<D: RegisterData> BitXor<D> for &'_ DefaultRegister<D> {
    type Output = D;

    fn bitxor(self, rhs: D) -> Self::Output {
        self.read() ^ rhs
    }
}

impl<D: RegisterData> BitAndAssign<D> for &'_ mut DefaultRegister<D> {
    fn bitand_assign(&mut self, rhs: D) {
        self.write( self.read() & rhs )
    }
}

impl<D: RegisterData> BitOrAssign<D> for &'_ mut DefaultRegister<D> {
    fn bitor_assign(&mut self, rhs: D) {
        self.write( self.read() | rhs )
    }
}

impl<D: RegisterData> BitXorAssign<D> for &'_ mut DefaultRegister<D> {
    fn bitxor_assign(&mut self, rhs: D) {
        self.write( self.read() ^ rhs )
    }
}
