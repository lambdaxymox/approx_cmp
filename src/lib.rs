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
/*
mod abs_diff;
mod abs_diff_impl_core;
mod abs_diff_impl_core_slice;
mod abs_diff_impl_core_tuple;
mod relative;
mod relative_impl_core;
mod relative_impl_core_slice;
mod relative_impl_core_tuple;

#[cfg(feature = "std")]
mod abs_diff_impl_std_slice;

#[cfg(feature = "std")]
mod abs_diff_impl_std;

#[cfg(feature = "std")]
mod relative_impl_std_slice;

#[cfg(feature = "std")]
mod relative_impl_std;


pub use abs_diff::*;
pub use abs_diff_impl_core::*;
pub use abs_diff_impl_core_slice::*;
pub use abs_diff_impl_core_tuple::*;
pub use relative::*;
pub use relative_impl_core::*;
pub use relative_impl_core_slice::*;
pub use relative_impl_core_tuple::*;

#[cfg(feature = "std")]
pub use abs_diff_impl_std_slice::*;

#[cfg(feature = "std")]
pub use abs_diff_impl_std::*;

#[cfg(feature = "std")]
pub use relative_impl_std_slice::*;

#[cfg(feature = "std")]
pub use relative_impl_std::*;
*/
