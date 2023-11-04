#![no_std]
#[cfg(feature = "core")]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

extern crate approx_cmp_abs_diff;
extern crate approx_cmp_relative;

pub use approx_cmp_abs_diff::*;
pub use approx_cmp_relative::*;
