//! Preallocation macro.
//! Creates a static allocation of data.


#[macro_export]
macro_rules! preallocate {
    ($alloc:ty) => {{
        fn resolve() -> &mut $alloc {
            #[link_section = ".uninit.PREALLOCATE"]
            #[used]
            pub static mut PREALLOCATION: $alloc = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

            unsafe { &mut PREALLOCATION }
        }

        resolve()
    }};
}

#[macro_export]
macro_rules! reserve {
    ($alloc:ty) => { preallocate!($alloc) };
}
