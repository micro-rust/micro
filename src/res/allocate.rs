//! Preallocation macro.
//! Creates a static allocation of data.


#[macro_export]
macro_rules! preallocate {
    ($alloc:ty) => {{
        fn resolve<'a>() -> &'a mut $alloc {
            #[link_section = ".uninit.PREALLOCATE"]
            #[used]
            pub static mut PREALLOCATION: [u8; core::mem::size_of::<$alloc>()] = [0u8; core::mem::size_of::<$alloc>()];

            unsafe { &mut *(&mut PREALLOCATION as *mut _ as *mut $alloc) }
        }

        resolve()
    }};
}

#[macro_export]
macro_rules! reserve {
    ($alloc:ty) => { preallocate!($alloc) };
}
