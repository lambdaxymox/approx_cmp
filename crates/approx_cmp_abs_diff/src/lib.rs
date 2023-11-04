#![no_std]
#[cfg(feature = "core")]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;


mod abs_diff;
mod abs_diff_impl_core;
mod abs_diff_impl_core_slice;
mod abs_diff_impl_core_tuple;

#[cfg(feature = "std")]
mod abs_diff_impl_std_slice;

#[cfg(feature = "std")]
mod abs_diff_impl_std;

pub use abs_diff::*;
pub use abs_diff_impl_core::*;
pub use abs_diff_impl_core_slice::*;
pub use abs_diff_impl_core_tuple::*;

#[cfg(feature = "std")]
pub use abs_diff_impl_std_slice::*;

#[cfg(feature = "std")]
pub use abs_diff_impl_std::*;
