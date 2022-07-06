//! Assembly instructions module.

mod ints;
mod misc;
mod sync;

pub use self::ints::*;

pub use self::misc::*;

pub use self::sync::*;


#[inline(always)]
pub fn critical<T>(f: FnOnce() -> T) -> T {
	cpsid_i();
	let r = f();
	cpsie_i();
	r
}
