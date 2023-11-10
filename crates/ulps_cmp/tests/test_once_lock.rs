extern crate ulps_cmp;


use std::sync;
use ulps_cmp::{
    assert_ulps_eq,
    assert_ulps_ne,
    AssertUlpsAllEq,
    AssertUlpsEq,
};


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
    let max_ulps = [
        2_u32, 4_u32, 2_u32, 1_u32, 
        1_u32, 1_u32, 1_u32, 1_u32,
    ];

    assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
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
    let max_ulps = [
        1_u32, 3_u32, 1_u32, 1_u32,
        1_u32, 1_u32, 1_u32, 1_u32,
    ];

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
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
    let max_ulps = [1_u32; 8];

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
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
    let max_ulps = 4_u32;

    assert_ulps_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
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
    let max_ulps = 1_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
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

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff1() {
    let lhs = sync::OnceLock::from([
        1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
        2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
    ]);
    let ulps_diff = [Some(0_u32); 8];

    assert_eq!(lhs.debug_ulps_diff(&lhs), Some(ulps_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff2() {
    let lhs = sync::OnceLock::from([
        1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
        2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
        2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
    ]);
    let ulps_diff = [
        Some(838861_u32),  Some(838861_u32),  Some(1677722_u32), Some(1677722_u32),
        Some(1258291_u32), Some(1258291_u32), Some(838861_u32),  Some(838861_u32),
    ];

    assert_eq!(lhs.debug_ulps_diff(&rhs), Some(ulps_diff));
    assert_eq!(rhs.debug_ulps_diff(&lhs), Some(ulps_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff3() {
    let lhs = sync::OnceLock::from([
        0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
        4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
    ]);
    let ulps_diff = [Some(0_u32); 8];

    assert_eq!(lhs.debug_ulps_diff(&lhs), Some(ulps_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff4() {
    let lhs = sync::OnceLock::from([
        0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
        4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
    ]);
    let rhs = sync::OnceLock::from([
        1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
    ]);
    let ulps_diff = [
        Some(839_u32), Some(419_u32), Some(210_u32), Some(1_u32),
        Some(105_u32), Some(0_u32),   Some(105_u32), Some(0_u32),
    ];

    assert_eq!(lhs.debug_ulps_diff(&rhs), Some(ulps_diff));
    assert_eq!(rhs.debug_ulps_diff(&lhs), Some(ulps_diff));
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
fn test_debug_ulps_tolerance() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_ulps = [2_u32, 4_u32, 8_u32, 16_u32];
    let tolerance = max_ulps;

    assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), Some(tolerance));
    assert_eq!(rhs.debug_ulps_tolerance(&lhs, &max_ulps), Some(tolerance));
}

#[test]
fn test_debug_ulps_all_tolerance() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let max_ulps = 8_u32;
    let tolerance = [max_ulps; 4];

    assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), Some(tolerance));
    assert_eq!(rhs.debug_ulps_all_tolerance(&lhs, &max_ulps), Some(tolerance));
}

#[test]
fn test_eq_none() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let rhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let max_abs_diff = f32::EPSILON;
    let max_ulps = 1_u32;

    assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[test]
fn test_ne_none1() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let rhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let max_abs_diff = f32::EPSILON;
    let max_ulps = 8_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[test]
fn test_ne_none2() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let rhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let max_abs_diff = f32::EPSILON;
    let max_ulps = 8_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[test]
fn test_ne_none3() {
    let lhs: sync::OnceLock<f32> = sync::OnceLock::from(1.0_f32);
    let rhs: sync::OnceLock<f32> = sync::OnceLock::new();
    let max_abs_diff = f32::EPSILON;
    let max_ulps = 8_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[test]
fn test_debug_abs_diff_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];
    let tolerance = max_abs_diff;

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
fn test_debug_ulps_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_ulps = [2_u32, 4_u32, 8_u32, 16_u32];
    let tolerance = max_ulps;

    assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), Some(tolerance));
    assert_eq!(lhs.debug_ulps_tolerance(&empty_cell, &max_ulps), None);
    assert_eq!(empty_cell.debug_ulps_tolerance(&rhs, &max_ulps), None);
}

#[test]
fn test_debug_ulps_all_tolerance_none() {
    let lhs = sync::OnceLock::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
    let rhs = sync::OnceLock::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
    let empty_cell = sync::OnceLock::new();
    let max_ulps = 4_u32;
    let tolerance = [max_ulps; 4];

    assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), Some(tolerance));
    assert_eq!(lhs.debug_ulps_all_tolerance(&empty_cell, &max_ulps), None);
    assert_eq!(empty_cell.debug_ulps_all_tolerance(&rhs, &max_ulps), None);
}
