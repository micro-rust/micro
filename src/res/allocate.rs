//! Preallocation macro.
//! Creates a static allocation of data.


#[macro_export]
macro_rules! preallocate {
    ($alloc:ty) => {{
        fn resolve<'a>() -> &'a mut micro::res::allocate::PreAllocation<$alloc> {
            #[link_section = ".bss.PREALLOCATE"]
            #[used]
            pub static mut PREALLOCATION: micro::res::allocate::PreAllocation<$alloc> = micro::res::allocate::PreAllocation::empty();

            unsafe { &mut PREALLOCATION }
        }

        resolve()
    }};
}

#[macro_export]
macro_rules! reserve {
    ($alloc:ty) => { preallocate!($alloc) };
}



/// Preallocation of a struct.
#[repr(C)]
pub struct PreAllocation<T: Sized> {
    /// Wrapped value.
    inner: T,

    /// Has it been initialized already?.
    init: u32,
}

impl<T: Sized> PreAllocation<T> {
    /// Static initializer.
    pub fn empty() -> Self {
        Self {
            inner: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            init: 0,
        }
    }

    /// Initialization of a PreAllocation.
    /// The only way to use a preallocation is to initialize it and turn it into an Initialized allocation.
    pub fn init(&mut self, inner: T) -> Option<&'static mut Allocation<T>> {
        if self.init != 0xA110CA7E {
            self.inner = inner;
            self.init = 0xA110CA7E;

            return unsafe { Some( &mut *(&mut self.inner as *mut T as *mut Allocation<T>) ) }
        }

        None
    }
}

/// Allocated and initialized value.
#[repr(transparent)]
pub struct Allocation<T: Sized> {
    /// Wrapped value.
    inner: T,
}

impl<T: Sized> Allocation<T> {
    /// Makes a static reference non-static.
    pub fn nonstatic<'a>(x: &'static Self) -> &'a Self {
        x
    }

    /// Makes a static mut reference non-static.
    pub fn nonstatic_mut<'a>(x: &'static mut Self) -> &'a mut Self {
        x
    }
}

impl<T: Sized> core::borrow::Borrow<T> for Allocation<T> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T: Sized> core::borrow::BorrowMut<T> for Allocation<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Sized> core::convert::AsMut<T> for Allocation<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Sized> core::convert::AsRef<T> for Allocation<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: Sized> core::ops::Deref for Allocation<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Sized> core::ops::DerefMut for Allocation<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
