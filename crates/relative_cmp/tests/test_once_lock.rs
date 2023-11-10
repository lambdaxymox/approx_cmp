extern crate relative_cmp;


use relative_cmp::{
    assert_relative_eq,
    assert_relative_ne,
    AssertRelativeAllEq,
    AssertRelativeEq,
};
use std::sync;


#[rustfmt::skip]
#[test]
fn test_eq() {
    let lhs = sync::OnceLock::from([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let eps = f32::EPSILON;
    let max_abs_diff = [
        1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];
    let max_relative = [
        1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
    ];

    assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let lhs = sync::OnceLock::from([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let eps = f32::EPSILON;
    let max_abs_diff = [
        0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];
    let max_relative = [
        0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];

    assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let lhs = sync::OnceLock::from([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let eps = f32::EPSILON;
    let max_abs_diff = [
        1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
    ];
    let max_relative = [eps; 8];

    assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_eq() {
    let lhs = sync::OnceLock::from([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let max_abs_diff = 4.0_f32 * f32::EPSILON;
    let max_relative = 4.0_f32 * f32::EPSILON;

    assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_ne() {
    let lhs = sync::OnceLock::from([
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ]);
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_relative = 1.0_f32 * f32::EPSILON;

    assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff1() {
    let lhs = sync::OnceLock::from([
        1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
        2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
    ]);
    let abs_diff = [0.0000000_f32; 8];

    assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff2() {
    let lhs = sync::OnceLock::from([
        1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
        2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
        2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
    ]);
    let abs_diff = [
        0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
        0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
    ];

    assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff));
    assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff3() {
    let lhs = sync::OnceLock::from([
        0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
        4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
    ]);
    let abs_diff = [0.00000000000000_f32; 8];

    assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff4() {
    let lhs = sync::OnceLock::from([
        0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
        4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
    ]);
    let abs_diff = [
        0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
        0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
    ];

    assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff));
    assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff));
}

#[test]
fn test_debug_abs_diff_tolerance() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];
    let tolerance = max_abs_diff;

    assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(tolerance));
    assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), Some(tolerance));
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_abs_diff = 0.20_f32;
    let tolerance = [max_abs_diff; 4];

    assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), Some(tolerance));
    assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff), Some(tolerance));
}

#[test]
fn test_debug_relative_tolerance() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];
    let tolerance = [0.25_f32, 0.65000004_f32, 1.35_f32, 2.40_f32];

    assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(tolerance));
    assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), Some(tolerance));
}

#[test]
fn test_debug_relative_all_tolerance() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_relative = 0.20_f32;
    let tolerance = [0.50_f32, 0.65000004_f32, 0.90000004_f32, 1.20_f32];

    assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative), Some(tolerance));
    assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative), Some(tolerance));
}

#[test]
fn test_eq_none() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let rhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let max_abs_diff = f32::EPSILON;
    let max_relative = f32::EPSILON;

    assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[test]
fn test_ne_none1() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let rhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let max_abs_diff = f32::EPSILON;
    let max_relative = f32::EPSILON;

    assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[test]
fn test_ne_none2() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let rhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let max_abs_diff = f32::EPSILON;
    let max_relative = f32::EPSILON;

    assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[test]
fn test_ne_none3() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let rhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let max_abs_diff = f32::EPSILON;
    let max_relative = f32::EPSILON;

    assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[test]
fn test_debug_abs_diff_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];
    let tolerance = max_abs_diff.clone();

    assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(tolerance));
    assert_eq!(lhs.debug_abs_diff_tolerance(&empty_cell, &max_abs_diff), None);
    assert_eq!(empty_cell.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
}

#[test]
fn test_debug_abs_diff_all_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_abs_diff = 0.20_f32;
    let tolerance = [max_abs_diff; 4];

    assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), Some(tolerance));
    assert_eq!(lhs.debug_abs_diff_all_tolerance(&empty_cell, &max_abs_diff), None);
    assert_eq!(empty_cell.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), None);
}

#[test]
fn test_debug_relative_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];
    let tolerance = [0.25_f32, 0.65000004_f32, 1.35_f32, 2.40_f32];

    assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(tolerance));
    assert_eq!(lhs.debug_relative_tolerance(&empty_cell, &max_relative), None);
    assert_eq!(empty_cell.debug_relative_tolerance(&rhs, &max_relative), None);
}

#[test]
fn test_debug_relative_all_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_relative = 0.20_f32;
    let tolerance = [0.50_f32, 0.65000004_f32, 0.90000004_f32, 1.20_f32];

    assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative), Some(tolerance));
    assert_eq!(lhs.debug_relative_all_tolerance(&empty_cell, &max_relative), None);
    assert_eq!(empty_cell.debug_relative_all_tolerance(&rhs, &max_relative), None);
}
