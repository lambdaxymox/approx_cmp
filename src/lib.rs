#![no_std]

#[cfg(feature = "core")]
extern crate core as std;

#[cfg(feature = "std")]
extern crate std;


mod abs_diff;
mod abs_diff_impl_core;
mod abs_diff_impl_core_slice;
mod relative;
mod relative_impl_core;
mod relative_impl_core_slice;

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
pub use relative::*;
pub use relative_impl_core::*;
pub use relative_impl_core_slice::*;

#[cfg(feature = "std")]
pub use abs_diff_impl_std_slice::*;

#[cfg(feature = "std")]
pub use abs_diff_impl_std::*;

#[cfg(feature = "std")]
pub use relative_impl_std_slice::*;

#[cfg(feature = "std")]
pub use relative_impl_std::*;
