extern crate abs_diff_cmp;


use abs_diff_cmp::{
    assert_abs_diff_eq,
    assert_abs_diff_ne,
    AssertAbsDiffAllEq,
    AssertAbsDiffEq,
};
use std::sync::Arc;


#[rustfmt::skip]
#[test]
fn test_eq() {
    let lhs = Arc::new([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = Arc::new([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let eps = f32::EPSILON;
    let max_abs_diff = [
        1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];

    assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(rhs, lhs, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let lhs = Arc::new([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = Arc::new([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let eps = f32::EPSILON;
    let max_abs_diff = [
        0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];

    assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(rhs, lhs, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let lhs = Arc::new([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = Arc::new([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let eps = f32::EPSILON;
    let max_abs_diff = [
        1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
    ];

    assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(rhs, lhs, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_eq() {
    let lhs = Arc::new([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = Arc::new([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let max_abs_diff = 4.0_f32 * f32::EPSILON;

    assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(rhs, lhs, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_ne() {
    let lhs = Arc::new([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = Arc::new([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let max_abs_diff = 2.0_f32 * f32::EPSILON;

    assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(rhs, lhs, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff1() {
    let lhs = Arc::new([
        1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
        2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
    ]);
    let abs_diff = [0.0000000_f32; 8];

    assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff2() {
    let lhs = Arc::new([
        1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
        2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
    ]);
    let rhs = Arc::new([
        1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
        2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
    ]);
    let abs_diff = [
        0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
        0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
    ];

    assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff3() {
    let lhs = Arc::new([
        0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
        4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
    ]);
    let abs_diff = [0.00000000000000_f32; 8];

    assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff4() {
    let lhs = Arc::new([
        0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
        4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
    ]);
    let rhs = Arc::new([
        1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
    ]);
    let abs_diff = [
        0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
        0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
    ];

    assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
}

#[test]
fn test_debug_abs_diff_tolerance() {
    let lhs = Arc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = Arc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

    assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    let lhs = Arc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = Arc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_abs_diff_all = 0.20_f32;
    let max_abs_diff = [max_abs_diff_all; 4];

    assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
    assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
}