//! Preallocation macro.
//! Creates a static allocation of data.


#[macro_export]
macro_rules! preallocate {
    ($alloc:ty) => {{
        #[link_section = ".uninit.PREALLOCATE"]
        #[used]
        pub static mut PREALLOCATION: $alloc = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        unsafe { &mut PREALLOCATION }
    }};
}

#[macro_export]
macro_rules! reserve {
    ($alloc:ty) => { preallocate!($alloc) };
}
