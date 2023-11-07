#![no_std]
#[cfg(feature = "core")]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;


mod impl_core_slice;
// mod impl_core_tuple;
mod impl_core_types;
mod traits;
/*
#[cfg(any(feature = "alloc", feature = "std"))]
mod impl_alloc_types;
*/
#[cfg(any(feature = "alloc", feature = "std"))]
mod impl_alloc_slice;
/*
#[cfg(feature = "std")]
mod impl_std_types;
*/

pub use impl_core_slice::*;
// pub use impl_core_tuple::*;
pub use impl_core_types::*;
pub use traits::*;
/*
#[cfg(any(feature = "alloc", feature = "std"))]
pub use impl_alloc_types::*;
*/
#[cfg(any(feature = "alloc", feature = "std"))]
pub use impl_alloc_slice::*;
/*
#[cfg(feature = "std")]
pub use impl_std_types::*;
*/
