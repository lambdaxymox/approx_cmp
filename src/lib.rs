#![deny(unsafe_op_in_unsafe_fn)]
#![doc = include_str!("../README.md")]
#![doc = include_str!("../EXAMPLES.md")]
#![no_std]
#[cfg(feature = "core")]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

extern crate abs_diff_cmp;
extern crate relative_cmp;
extern crate ulps_cmp;

pub use abs_diff_cmp::*;
pub use relative_cmp::*;
pub use ulps_cmp::*;
