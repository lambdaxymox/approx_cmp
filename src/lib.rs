#![no_std]
#[cfg(feature = "core")]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

extern crate abs_diff_cmp;
extern crate relative_cmp;

pub use abs_diff_cmp::*;
pub use relative_cmp::*;
