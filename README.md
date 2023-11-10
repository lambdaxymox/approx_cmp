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

The library aims to be as platform agnostic as possible. By default, the library
supports any environment that support the full Rust `std` library. However, the
library can operate with either the `core` or `alloc` libraries as well. You can
explicitly support either `core` or `alloc` by adding

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
* a set of macros for making approximate comparisons tidier and more understandable.
The library interfaces are designed to be number system agnostic. Typically one 
would be interested in IEEE 754 floating point number comparisons, but the comparison 
traits can be implemented for other numeric representations too, such as 
[posits](https://posithub.org/).

For more details about the specifics of the comparison algorithms provided by the
library, see the relevant documentation for that comparison algorithm trait.

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
by **approx_cmp** for a single composite data type.

```rust

```