# Approximate Comparison Library

## Introduction

**approx_cmp** is a library that provides a comprehensive feature set for doing
finite precision numeric comparisons, specifically for collections of IEEE 754
floating point numbers, but for other number systems as well.

## Getting Started

To use the library in your project, add the following line to your `Cargo.toml`
file

```toml
[dependencies]
approx_cmp = "0.2.0"
```

and then place the crate declaration in either your `lib.rs` or `main.rs` file

```rust
extern crate approx_cmp;
```

The library aims to be as platform agnostic as possible, including `no_std` 
environments. By default, the library is compatible with any environment that 
supports the full Rust `std` library. However, the library can operate 
with just the `core` or `alloc` libraries as well. You can explicitly support 
either `core` or `alloc` by adding

```toml
[dependencies.approx_cmp]
features = "core"
```

for `core` crate support, and

```toml
[dependencies.approx_cmp]
features = "alloc"
```

for `alloc` crate support. The `core` feature configured the library to
provide implementations of `approx_cmp`'s features for data types found
in the `core` crate, and mutatis mutandis for the `alloc` and `std` crates.
In particular, `alloc` and `std` provide implementations for data structures
found in the respective crates. Data types found in `std` but not `alloc` are
not available using the `alloc` feature, and similarly for `alloc` and `core`.

## Features

The **approx_cmp** crate provides a rich set of features for doing finite
precision arithmetic comparisons using the following comparison algorithms:
* Absolute difference equality comparisons
* Relative difference equality comparisons
* Units In Last Place equality comparisons
* Debugging traits for error reporting in case of failed comparisons
* The capacity to define approximate comparison operations on custom data types.
* A set of macros for each comparison algorithm making approximate comparisons tidier, and
  making debugging and logging and more understandable.
The library interfaces are designed to be number system agnostic. Typically one 
would be interested in IEEE 754 floating point number comparisons, but the comparison 
traits can be implemented for other numeric representations too, such as 
[posits](https://posithub.org/).

For more details about the specifics of the comparison algorithms provided by the
library, see the relevant documentation for that comparison algorithm trait.

## References
Some references going in depth about comparing floating point numbers and their
tricky subtleties.
-[1] [Comparing Floating Point Numbers, 2012 Edition](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
- [The Floating-Point Guide](https://floating-point-gui.de/errors/comparison/)
- [What Every Computer Scientist Should Know About Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)
- [Handbook Of Floating-Point Arithmetic, Second Edition]()

## Comparison Examples

### Absolute Difference Comparisons

Some examples of absolute difference comparisons of scalars.

```rust
use approx_cmp::{
    abs_diff_eq,
    abs_diff_ne,
    assert_abs_diff_eq,
    assert_abs_diff_ne,     
    AbsDiffAllEq,
    AbsDiffEq, 
};

let lhs = 394.995_f32;
let rhs = 395.0_f32;

assert_abs_diff_eq!(lhs, rhs, abs_diff <= 0.006_f32);
assert_abs_diff_ne!(lhs, rhs, abs_diff <= 0.004_f32);

assert!(abs_diff_eq!(lhs, rhs, abs_diff <= 0.006_f32));
assert!(abs_diff_ne!(lhs, rhs, abs_diff <= 0.004_f32));

assert!(AbsDiffEq::abs_diff_eq(&lhs, &rhs, &0.006_f32));
assert!(AbsDiffEq::abs_diff_ne(&lhs, &rhs, &0.004_f32));

// The AbsDiffEq and AbsDiffAllEq traits act the same on scalars.
assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= 0.006_f32);
assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= 0.004_f32);

assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= 0.006_f32));
assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= 0.004_f32));

assert!(AbsDiffAllEq::abs_diff_all_eq(&lhs, &rhs, &0.006_f32));
assert!(AbsDiffAllEq::abs_diff_all_ne(&lhs, &rhs, &0.004_f32));
```

Some examples of absolute difference comparisons of composite types.

```rust
use approx_cmp::{
    abs_diff_eq,
    abs_diff_ne,
    assert_abs_diff_eq,
    assert_abs_diff_ne,     
    AbsDiffAllEq,
    AbsDiffEq, 
};

let lhs = vec![
    302.0_f32,  1867.0_f32, 141.0_f32, 99.0_f32, 
    7434.0_f32, 79.0_f32,   62.0_f32,  9032.0_f32,
];
let rhs = vec![
    302.04081_f32,  1867.08086_f32, 141.01877_f32, 99.02688_f32,
    7434.07746_f32, 79.01996_f32,   62.010079_f32, 9032.07045_f32,
];
let max_abs_diff1 = vec![
    0.05_f32, 0.09_f32, 0.02_f32, 0.03_f32,
    0.08_f32, 0.02_f32, 0.02_f32, 0.08_f32,
];
let max_abs_diff2 = vec![
    0.04_f32, 0.08_f32, 0.01_f32, 0.02_f32,
    0.07_f32, 0.01_f32, 0.01_f32, 0.07_f32,
];

assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff1);
assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff2);

assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff1));
assert!(abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff2));

assert!(AbsDiffEq::abs_diff_eq(&lhs, &rhs, &max_abs_diff1));
assert!(AbsDiffEq::abs_diff_ne(&lhs, &rhs, &max_abs_diff2));

assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= 0.09_f32);
assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= 0.01_f32);

assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= 0.09_f32));
assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= 0.01_f32));

assert!(AbsDiffAllEq::abs_diff_all_eq(&lhs, &rhs, &0.09_f32));
assert!(AbsDiffAllEq::abs_diff_all_ne(&lhs, &rhs, &0.01_f32));
```

### Relative Difference Comparisons

Some examples of relative comparisons of scalars.

```rust
use approx_cmp::{
    relative_eq,
    relative_ne,
    assert_relative_eq,
    assert_relative_ne,     
    RelativeAllEq,
    RelativeEq, 
};

let lhs = 394.995_f32;
let rhs = 395.0_f32;

assert_relative_eq!(lhs, rhs, abs_diff <= 0.006_f32, relative <= 0.000015_f32);
assert_relative_ne!(lhs, rhs, abs_diff <= 0.004_f32, relative <= 0.000010_f32);

assert!(relative_eq!(lhs, rhs, abs_diff <= 0.006_f32, relative <= 0.000015_f32));
assert!(relative_ne!(lhs, rhs, abs_diff <= 0.004_f32, relative <= 0.000010_f32));

assert!(RelativeEq::relative_eq(&lhs, &rhs, &0.006_f32, &0.000015_f32));
assert!(RelativeEq::relative_ne(&lhs, &rhs, &0.004_f32, &0.000010_f32));

// The RelativeEq and RelativeAllEq traits act the same on scalars.
assert_relative_eq!(lhs, rhs, abs_diff_all <= 0.006_f32, relative_all <= 0.000015_f32);
assert_relative_ne!(lhs, rhs, abs_diff_all <= 0.004_f32, relative_all <= 0.000010_f32);

assert!(relative_eq!(lhs, rhs, abs_diff_all <= 0.006_f32, relative_all <= 0.000015_f32));
assert!(relative_ne!(lhs, rhs, abs_diff_all <= 0.004_f32, relative_all <= 0.000010_f32));

assert!(RelativeAllEq::relative_all_eq(&lhs, &rhs, &0.006_f32, &0.000015_f32));
assert!(RelativeAllEq::relative_all_ne(&lhs, &rhs, &0.004_f32, &0.000010_f32));
```

Some examples of relative comparisons of composite types.

```rust
use approx_cmp::{
    relative_eq,
    relative_ne,
    assert_relative_eq,
    assert_relative_ne,     
    RelativeAllEq,
    RelativeEq, 
};

let lhs = vec![
    302.0_f32,  1867.0_f32, 141.0_f32, 99.0_f32, 
    7434.0_f32, 79.0_f32,   62.0_f32,  9032.0_f32,
];
let rhs = vec![
    302.04081_f32,  1867.08086_f32, 141.01877_f32, 99.02688_f32,
    7434.07746_f32, 79.01996_f32,   62.010079_f32, 9032.07045_f32,
];
let max_abs_diff1 = vec![
    0.05_f32, 0.09_f32, 0.02_f32, 0.03_f32,
    0.08_f32, 0.02_f32, 0.02_f32, 0.08_f32,
];
let max_abs_diff2 = vec![
    0.04_f32, 0.08_f32, 0.01_f32, 0.02_f32,
    0.07_f32, 0.01_f32, 0.01_f32, 0.07_f32,
];
let max_relative1 = vec![
    0.0002_f32, 0.0001_f32, 0.0002_f32, 0.0003_f32,
    0.0002_f32, 0.0003_f32, 0.0002_f32, 0.0001_f32,
];
let max_relative2 = vec![
    0.0001_f32, 0.0001_f32, 0.0001_f32, 0.0002_f32,
    0.0001_f32, 0.0002_f32, 0.0001_f32, 0.0001_f32,
];

assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff1, relative <= max_relative1);
assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff2, relative <= max_relative2);

assert!(relative_eq!(lhs, rhs, abs_diff <= max_abs_diff1, relative <= max_relative1));
assert!(relative_ne!(lhs, rhs, abs_diff <= max_abs_diff2, relative <= max_relative2));

assert!(RelativeEq::relative_eq(&lhs, &rhs, &max_abs_diff1, &max_relative1));
assert!(RelativeEq::relative_ne(&lhs, &rhs, &max_abs_diff2, &max_relative2));

assert_relative_eq!(lhs, rhs, abs_diff_all <= 0.08_f32, relative_all <= 0.0003_f32);
assert_relative_ne!(lhs, rhs, abs_diff_all <= 0.01_f32, relative_all <= 0.0001_f32);

assert!(relative_eq!(lhs, rhs, abs_diff_all <= 0.08_f32, relative_all <= 0.0003_f32));
assert!(relative_ne!(lhs, rhs, abs_diff_all <= 0.01_f32, relative_all <= 0.0001_f32));

assert!(RelativeAllEq::relative_all_eq(&lhs, &rhs, &0.08_f32, &0.0003_f32));
assert!(RelativeAllEq::relative_all_ne(&lhs, &rhs, &0.01_f32, &0.0001_f32));
```

### Ulps Difference Comparisons

Some examples of ulps comparisons of scalars.

```rust
use approx_cmp::{
    ulps_eq,
    ulps_ne,
    assert_ulps_eq,
    assert_ulps_ne,     
    UlpsAllEq,
    UlpsEq, 
};

let lhs = 394.995_f32;
let rhs = 395.0_f32;

assert_ulps_eq!(lhs, rhs, abs_diff <= 0.006_f32, ulps <= 170_u32);
assert_ulps_ne!(lhs, rhs, abs_diff <= 0.004_f32, ulps <= 160_u32);

assert!(ulps_eq!(lhs, rhs, abs_diff <= 0.006_f32, ulps <= 170_u32));
assert!(ulps_ne!(lhs, rhs, abs_diff <= 0.004_f32, ulps <= 160_u32));

assert!(UlpsEq::ulps_eq(&lhs, &rhs, &0.006_f32, &170_u32));
assert!(UlpsEq::ulps_ne(&lhs, &rhs, &0.004_f32, &160_u32));

// The UlpsEq and UlpsAllEq traits act the same on scalars.
assert_ulps_eq!(lhs, rhs, abs_diff_all <= 0.006_f32, ulps_all <= 170_u32);
assert_ulps_ne!(lhs, rhs, abs_diff_all <= 0.004_f32, ulps_all <= 160_u32);

assert!(ulps_eq!(lhs, rhs, abs_diff_all <= 0.006_f32, ulps_all <= 170_u32));
assert!(ulps_ne!(lhs, rhs, abs_diff_all <= 0.004_f32, ulps_all <= 160_u32));

assert!(UlpsAllEq::ulps_all_eq(&lhs, &rhs, &0.006_f32, &170_u32));
assert!(UlpsAllEq::ulps_all_ne(&lhs, &rhs, &0.004_f32, &160_u32));
```

Some examples of ulps comparisons of composite types.

```rust
use approx_cmp::{
    ulps_eq,
    ulps_ne,
    assert_ulps_eq,
    assert_ulps_ne,     
    UlpsAllEq,
    UlpsEq, 
};

let lhs = vec![
    302.0_f32,  1867.0_f32, 141.0_f32, 99.0_f32, 
    7434.0_f32, 79.0_f32,   62.0_f32,  9032.0_f32,
];
let rhs = vec![
    302.04081_f32,  1867.08086_f32, 141.01877_f32, 99.02688_f32,
    7434.07746_f32, 79.01996_f32,   62.010079_f32, 9032.07045_f32,
];
let max_abs_diff1 = vec![
    0.05_f32, 0.09_f32, 0.02_f32, 0.03_f32,
    0.08_f32, 0.02_f32, 0.02_f32, 0.08_f32,
];
let max_abs_diff2 = vec![
    0.04_f32, 0.08_f32, 0.01_f32, 0.02_f32,
    0.07_f32, 0.01_f32, 0.01_f32, 0.07_f32,
];
let max_ulps1 = vec![
    1400_u32, 700_u32,  1300_u32, 3600_u32,
    200_u32,  2700_u32, 2700_u32, 90_u32,
];
let max_ulps2 = vec![
    1300_u32, 600_u32,  1200_u32, 3500_u32,
    100_u32,  2600_u32, 2600_u32, 60_u32,
];

assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff1, ulps <= max_ulps1);
assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff2, ulps <= max_ulps2);

assert!(ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff1, ulps <= max_ulps1));
assert!(ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff2, ulps <= max_ulps2));

assert!(UlpsEq::ulps_eq(&lhs, &rhs, &max_abs_diff1, &max_ulps1));
assert!(UlpsEq::ulps_ne(&lhs, &rhs, &max_abs_diff2, &max_ulps2));

assert_ulps_eq!(lhs, rhs, abs_diff_all <= 0.08_f32, ulps_all <= 3600_u32);
assert_ulps_ne!(lhs, rhs, abs_diff_all <= 0.01_f32, ulps_all <= 36_u32);

assert!(ulps_eq!(lhs, rhs, abs_diff_all <= 0.08_f32, ulps_all <= 3600_u32));
assert!(ulps_ne!(lhs, rhs, abs_diff_all <= 0.01_f32, ulps_all <= 36_u32));

assert!(UlpsAllEq::ulps_all_eq(&lhs, &rhs, &0.08_f32, &3600_u32));
assert!(UlpsAllEq::ulps_all_ne(&lhs, &rhs, &0.01_f32, &36_u32));
```

## Implementing Approximate Comparisons On Custom Data Types

In this section we provide an example of implementing all the traits provided
by **approx_cmp** for a single composite data type. Here, we show a possible
implementation of all the library traits for a vector type representing points in
Euclidean space.

```rust
use approx_cmp;
use approx_cmp::{
    assert_abs_diff_eq,
    assert_abs_diff_ne,
    assert_relative_eq,
    assert_relative_ne,
    assert_ulps_eq,
    assert_ulps_ne,
    UlpsEq,
};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> { 
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl approx_cmp::AbsDiffEq for Vector3<f32> {
    type Tolerance = Vector3<f32>;

    fn abs_diff_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> bool {
        self.x.abs_diff_eq(&other.x, &max_abs_diff.x)
            && self.y.abs_diff_eq(&other.y, &max_abs_diff.y)
            && self.z.abs_diff_eq(&other.z, &max_abs_diff.z)
    }
}

impl approx_cmp::AbsDiffAllEq for Vector3<f32> {
    type AllTolerance = f32;

    fn abs_diff_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> bool {
        self.x.abs_diff_all_eq(&other.x, max_abs_diff)
            && self.y.abs_diff_all_eq(&other.y, max_abs_diff)
            && self.z.abs_diff_all_eq(&other.z, max_abs_diff)
    }
}

impl approx_cmp::AssertAbsDiffEq for Vector3<f32> {
    type DebugAbsDiff = Vector3<f32>;
    type DebugTolerance = Vector3<f32>;

    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        Vector3::new(
            self.x.debug_abs_diff(&other.x),
            self.y.debug_abs_diff(&other.y),
            self.z.debug_abs_diff(&other.z),
        )
    }

    fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        *max_abs_diff
    }
}

impl approx_cmp::AssertAbsDiffAllEq for Vector3<f32> {
    type AllDebugTolerance = Vector3<f32>;

    fn debug_abs_diff_all_tolerance(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        Vector3::new(*max_abs_diff, *max_abs_diff, *max_abs_diff)
    }
}

impl approx_cmp::RelativeEq for Vector3<f32> {
    type Tolerance = Vector3<f32>;

    fn relative_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.x.relative_eq(&other.x, &max_abs_diff.x, &max_relative.x)
            && self.y.relative_eq(&other.y, &max_abs_diff.y, &max_relative.y)
            && self.z.relative_eq(&other.z, &max_abs_diff.z, &max_relative.z)
    }
}

impl approx_cmp::RelativeAllEq for Vector3<f32> {
    type AllTolerance = f32;

    fn relative_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.x.relative_all_eq(&other.x, max_abs_diff, max_relative)
            && self.y.relative_all_eq(&other.y, max_abs_diff, max_relative)
            && self.z.relative_all_eq(&other.z, max_abs_diff, max_relative)
    }
}

impl approx_cmp::AssertRelativeEq for Vector3<f32> {
    type DebugAbsDiff = Vector3<f32>;
    type DebugTolerance = Vector3<f32>;

    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        Vector3::new(
            self.x.debug_abs_diff(&other.x),
            self.y.debug_abs_diff(&other.y),
            self.z.debug_abs_diff(&other.z),
        )
    }

    fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        *max_abs_diff
    }

    fn debug_relative_tolerance(&self, other: &Self, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        *max_relative
    }
}

impl approx_cmp::AssertRelativeAllEq for Vector3<f32> {
    type AllDebugTolerance = Vector3<f32>;

    fn debug_abs_diff_all_tolerance(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        Vector3::new(*max_abs_diff, *max_abs_diff, *max_abs_diff)
    }

    fn debug_relative_all_tolerance(&self, other: &Self, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        Vector3::new(*max_relative, *max_relative, *max_relative)
    }
}

impl approx_cmp::UlpsEq for Vector3<f32> {
    type Tolerance = Vector3<f32>;
    type UlpsTolerance = Vector3<u32>;

    fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.x.ulps_eq(&other.x, &max_abs_diff.x, &max_ulps.x)
            && self.y.ulps_eq(&other.y, &max_abs_diff.y, &max_ulps.y)
            && self.z.ulps_eq(&other.z, &max_abs_diff.z, &max_ulps.z)
    }
}

impl approx_cmp::UlpsAllEq for Vector3<f32> {
    type AllTolerance = f32;
    type AllUlpsTolerance = u32;
    
    fn ulps_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.x.ulps_eq(&other.x, max_abs_diff, max_ulps)
            && self.y.ulps_eq(&other.y, max_abs_diff, max_ulps)
            && self.z.ulps_eq(&other.z, max_abs_diff, max_ulps)
    }
}

impl approx_cmp::AssertUlpsEq for Vector3<f32> {
    type DebugAbsDiff = Vector3<f32>;
    type DebugUlpsDiff = Vector3<Option<u32>>;
    type DebugTolerance = Vector3<f32>;
    type DebugUlpsTolerance = Vector3<u32>;

    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        Vector3::new(
            self.x.debug_abs_diff(&other.x),
            self.y.debug_abs_diff(&other.y),
            self.z.debug_abs_diff(&other.z),
        )
    }

    fn debug_ulps_diff(&self, other: &Self) -> Self::DebugUlpsDiff {
        Vector3::new(
            self.x.debug_ulps_diff(&other.x),
            self.y.debug_ulps_diff(&other.y),
            self.z.debug_ulps_diff(&other.z),
        )
    }

    fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        *max_abs_diff
    }

    fn debug_ulps_tolerance(&self, other: &Self, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        *max_ulps
    }
}

impl approx_cmp::AssertUlpsAllEq for Vector3<f32> {
    type AllDebugTolerance = Vector3<f32>;
    type AllDebugUlpsTolerance = Vector3<u32>;

    fn debug_abs_diff_all_tolerance(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        Vector3::new(*max_abs_diff, *max_abs_diff, *max_abs_diff)
    }

    fn debug_ulps_all_tolerance(&self, other: &Self, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        Vector3::new(*max_ulps, *max_ulps, *max_ulps)
    }
}

let lhs = Vector3::new(1.0_f32, 2.0_f32, 3.0_f32);
let rhs = Vector3::new(0.99995_f32, 2.00004_f32, 2.99998_f32);

let zero = Vector3::new(0.0_f32, 0.0_f32, 0.0_f32);
let max_abs_diff1 = Vector3::new(0.00006_f32, 0.00005_f32, 0.00003_f32);
let max_abs_diff2 = Vector3::new(0.00005_f32, 0.00004_f32, 0.00002_f32);
let max_relative1 = Vector3::new(0.00005_32, 0.00003_f32, 0.000007_f32);
let max_relative2 = Vector3::new(0.00004_32, 0.00002_f32, 0.000006_f32);
let max_ulps1 = Vector3::new(850_u32, 170_u32, 90_u32);
let max_ulps2 = Vector3::new(820_u32, 150_u32, 70_u32);

assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff1);
assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff2);

assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff1, relative <= max_relative1);
assert_relative_ne!(lhs, rhs, abs_diff <= zero, relative <= max_relative2);

assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff1, ulps <= max_ulps1);
assert_ulps_ne!(lhs, rhs, abs_diff <= zero, ulps <= max_ulps2);

assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= 0.00006_f32);
assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= 0.00001_f32);

assert_relative_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 0.00008_f32);
assert_relative_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 0.00001_f32);

assert_ulps_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 850_u32);
assert_ulps_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 150_u32);
```