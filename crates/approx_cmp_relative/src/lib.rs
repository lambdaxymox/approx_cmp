#![no_std]
#[cfg(feature = "core")]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;


mod relative;
mod relative_impl_core;
mod relative_impl_core_slice;
mod relative_impl_core_tuple;

#[cfg(feature = "std")]
mod relative_impl_std_slice;

#[cfg(feature = "std")]
mod relative_impl_std;

pub use relative::*;
pub use relative_impl_core::*;
pub use relative_impl_core_slice::*;
pub use relative_impl_core_tuple::*;

#[cfg(feature = "std")]
pub use relative_impl_std_slice::*;

#[cfg(feature = "std")]
pub use relative_impl_std::*;
