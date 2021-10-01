//! Common trait for all registers.

use core::fmt::Debug;
use core::ops::*;


pub trait Register<X: XType> :
    BitAnd<X> + BitAndAssign<X> +
    BitXor<X> + BitXorAssign<X> +
    BitOr<X>  + BitOrAssign<X>  +
    Sized
{
    /// Returns the `const` pointer to the inner data.
    fn ptr(&self) -> *const X {
        self as *const Self as *const X
    }

    /// Returns the `const` pointer to the inner data.
    fn ptr_mut(&mut self) -> *mut X {
        self as *mut Self as *mut X
    }

    /// Reads the register from memory.
    fn read(&self) -> X;

    /// Write the value to the register.
    fn write(&mut self, val: X);

    /// Clears the given mask in the register.
    fn clear(&mut self, mask: X);

    /// Sets the given mask in the register.
    fn set(&mut self, mask: X);
}


pub trait XType : 
    BitAnd<Self> + BitAndAssign<Self> + BitAnd<Output=Self> +
    BitXor<Self> + BitXorAssign<Self> + BitXor<Output=Self> +
    BitOr<Self>  + BitOrAssign<Self>  + BitOr<Output=Self>  +
    Not<Output=Self> +
    Debug + Clone + Copy +
    PartialEq + Eq + PartialOrd + Ord +
    Sized {}


impl XType for  i8 {}
impl XType for  u8 {}

impl XType for i16 {}
impl XType for u16 {}

impl XType for i32 {}
impl XType for u32 {}

impl XType for i64 {}
impl XType for u64 {}
