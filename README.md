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
approx_cmp = "2.0.0"
```

and then place the crate declaration in either your `lib.rs` or `main.rs` file

```rust
extern crate approx_cmp;
```

The library aims to be as platform-agnostic as possible, including `no_std` 
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
* A set of macros for each comparison algorithm making approximate comparisons 
tidier, and making debugging and logging and more understandable.

The library interfaces are designed to be number system agnostic. Typically one 
would be interested in IEEE 754 floating point number comparisons, but the 
comparison traits can be implemented for other numeric representations too, such 
as [posits](https://posithub.org/).

For more details about the specifics of the comparison algorithms provided by the
library, see the relevant documentation for that comparison algorithm trait.

## Prior Work
The design of the comparison algorithms in this crate is heavily informed by the contents
of the website [Comparing Floating Point Numbers, 2012 Edition](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
as well as the `approx` and `float-eq` crates. See the references section.

## References

- Bruce Dawson. 2012. Comparing Floating Point Numbers, 2012 Edition. (February 2012). Retrieved November 10, 2023
from https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
- Michael Borgwardt. What Every Programmer Should Know About Floating-Point Arithmetic Or Why Don't My Numbers Add Up?
Retrieved from https://floating-point-gui.de/
- David Goldberg. 1991. What every computer scientist should know about floating-point arithmetic.
_ACM Computing Surveys_, 23 (March 1991), 5-48. https://doi.org/10.1145/103162.103163
- Jean-Michel Muller, Nicolas Brunie, Florent de Dinechin, Claude-Pierre Jeannerod, Mioara Joldes, Vincent Lefèvre, 
Guillaume Melquiond, Nathalie Revol, Serge Torres. _Handbook of Floating-Point Arithmetic_ (2nd. ed.). Birkhäuser Cham.
https://doi.org/10.1007/978-3-319-76526-6
