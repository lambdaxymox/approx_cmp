use std::collections::BTreeMap;
use ulps_cmp::{
    assert_ulps_eq,
    assert_ulps_ne,
    AssertUlpsAllEq,
    AssertUlpsEq,
};


#[rustfmt::skip]
#[test]
fn test_eq() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 2.0_f32 * f32::EPSILON),
        ("1", 1.0_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 1.0_f32 * f32::EPSILON),
        ("6", 1.0_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 878100373_u32),
        ("1", 2_u32),
        ("2", 4_u32),
        ("3", 2_u32),
        ("4", 1_u32),
        ("5", 1_u32),
        ("6", 1_u32),
        ("7", 1_u32),
        ("8", 1_u32),
    ]);

    assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 1.0_f32 * f32::EPSILON),
        ("1", 0.5_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 0.5_f32 * f32::EPSILON),
        ("6", 0.5_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 878100373_u32),
        ("1", 1_u32),
        ("2", 3_u32),
        ("3", 1_u32),
        ("4", 1_u32),
        ("5", 1_u32),
        ("6", 1_u32),
        ("7", 1_u32),
        ("8", 1_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 2.0_f32 * f32::EPSILON),
        ("1", 1.0_f32 * f32::EPSILON),
        ("2", 2.0_f32 * f32::EPSILON),
        ("3", 2.0_f32 * f32::EPSILON),
        ("4", 2.0_f32 * f32::EPSILON),
        ("5", 1.0_f32 * f32::EPSILON),
        ("6", 1.0_f32 * f32::EPSILON),
        ("7", 2.0_f32 * f32::EPSILON),
        ("8", 2.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 878100372_u32),
        ("1", 1_u32),
        ("2", 1_u32),
        ("3", 1_u32),
        ("4", 1_u32),
        ("5", 1_u32),
        ("6", 1_u32),
        ("7", 1_u32),
        ("8", 1_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = 4.0_f32 * f32::EPSILON;
    let max_ulps = 4_u32;

    assert_ulps_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_ulps = 1_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff1() {
    let lhs = BTreeMap::from([
        ("0", 0.00_f32),
        ("1", 1.00_f32),
        ("2", 1.25_f32),
        ("3", 1.50_f32),
        ("4", 2.00_f32),
        ("5", 2.50_f32),
        ("6", 3.00_f32),
        ("7", 4.00_f32),
        ("8", 5.00_f32),
    ]);
    let abs_diff = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.0000000_f32),
        ("2", 0.0000000_f32),
        ("3", 0.0000000_f32),
        ("4", 0.0000000_f32),
        ("5", 0.0000000_f32),
        ("6", 0.0000000_f32),
        ("7", 0.0000000_f32),
        ("8", 0.0000000_f32),
    ]);

    assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff2() {
    let lhs = BTreeMap::from([
        ("0", 0.00_f32),
        ("1", 1.00_f32),
        ("2", 1.25_f32),
        ("3", 1.50_f32),
        ("4", 2.00_f32),
        ("5", 2.50_f32),
        ("6", 3.00_f32),
        ("7", 4.00_f32),
        ("8", 5.00_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.10_f32),
        ("1", 1.10_f32),
        ("2", 1.15_f32),
        ("3", 1.70_f32),
        ("4", 1.80_f32),
        ("5", 2.80_f32),
        ("6", 2.70_f32),
        ("7", 4.40_f32),
        ("8", 4.60_f32),
    ]);
    let abs_diff = BTreeMap::from([
        ("0", 0.100000000_f32),
        ("1", 0.100000024_f32),
        ("2", 0.100000024_f32),
        ("3", 0.200000050_f32),
        ("4", 0.200000050_f32),
        ("5", 0.299999950_f32),
        ("6", 0.299999950_f32),
        ("7", 0.400000100_f32),
        ("8", 0.400000100_f32),
    ]);

    assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
    assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff3() {
    let lhs = BTreeMap::from([
        ("0", 0.0000500_f32),
        ("1", 0.9999500_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999500_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999500_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999500_f32),
        ("8", 8.0000000_f32),
    ]);
    let abs_diff = BTreeMap::from([
        ("0", 0.00000000000000_f32),
        ("1", 0.00000000000000_f32),
        ("2", 0.00000000000000_f32),
        ("3", 0.00000000000000_f32),
        ("4", 0.00000000000000_f32),
        ("5", 0.00000000000000_f32),
        ("6", 0.00000000000000_f32),
        ("7", 0.00000000000000_f32),
        ("8", 0.00000000000000_f32),
    ]);

    assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff4() {
    let lhs = BTreeMap::from([
        ("0", 0.0000500_f32),
        ("1", 0.9999500_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999500_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999500_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999500_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999500_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000000_f32),
        ("7", 7.0000000_f32),
        ("8", 8.0000000_f32),
    ]);
    let abs_diff = BTreeMap::from([
        ("0", 0.00005000000000_f32),
        ("1", 0.00005000829700_f32),
        ("2", 0.00004994869200_f32),
        ("3", 0.00005006790000_f32),
        ("4", 0.00000047683716_f32),
        ("5", 0.00005006790000_f32),
        ("6", 0.00000000000000_f32),
        ("7", 0.00005006790000_f32),
        ("8", 0.00000000000000_f32),
    ]);

    assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
    assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff1() {
    let lhs = BTreeMap::from([
        ("0", 0.00_f32),
        ("1", 1.00_f32),
        ("2", 1.25_f32),
        ("3", 1.50_f32),
        ("4", 2.00_f32),
        ("5", 2.50_f32),
        ("6", 3.00_f32),
        ("7", 4.00_f32),
        ("8", 5.00_f32),
    ]);
    let ulps_diff = BTreeMap::from([
        ("0", Some(0_u32)),
        ("1", Some(0_u32)),
        ("2", Some(0_u32)),
        ("3", Some(0_u32)),
        ("4", Some(0_u32)),
        ("5", Some(0_u32)),
        ("6", Some(0_u32)),
        ("7", Some(0_u32)),
        ("8", Some(0_u32)),
    ]);

    assert_eq!(lhs.debug_ulps_diff(&lhs), Some(ulps_diff));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff2() {
    let lhs = BTreeMap::from([
        ("0", 0.00_f32),
        ("1", 1.00_f32),
        ("2", 1.25_f32),
        ("3", 1.50_f32),
        ("4", 2.00_f32),
        ("5", 2.50_f32),
        ("6", 3.00_f32),
        ("7", 4.00_f32),
        ("8", 5.00_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.10_f32),
        ("1", 1.10_f32),
        ("2", 1.15_f32),
        ("3", 1.70_f32),
        ("4", 1.80_f32),
        ("5", 2.80_f32),
        ("6", 2.70_f32),
        ("7", 4.40_f32),
        ("8", 4.60_f32),
    ]);
    let ulps_diff = BTreeMap::from([
        ("0", Some(1036831949_u32)),
        ("1", Some(838861_u32)),
        ("2", Some(838861_u32)),
        ("3", Some(1677722_u32)),
        ("4", Some(1677722_u32)),
        ("5", Some(1258291_u32)),
        ("6", Some(1258291_u32)),
        ("7", Some(838861_u32)),
        ("8", Some(838861_u32)),
    ]);

    assert_eq!(lhs.debug_ulps_diff(&rhs), Some(ulps_diff.clone()));
    assert_eq!(rhs.debug_ulps_diff(&lhs), Some(ulps_diff.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff3() {
    let lhs = BTreeMap::from([
        ("0", 0.0000500_f32),
        ("1", 0.9999500_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999500_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999500_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999500_f32),
        ("8", 8.0000000_f32),
    ]);
    let ulps_diff = BTreeMap::from([
        ("0", Some(0_u32)),
        ("1", Some(0_u32)),
        ("2", Some(0_u32)),
        ("3", Some(0_u32)),
        ("4", Some(0_u32)),
        ("5", Some(0_u32)),
        ("6", Some(0_u32)),
        ("7", Some(0_u32)),
        ("8", Some(0_u32)),
    ]);

    assert_eq!(lhs.debug_ulps_diff(&lhs), Some(ulps_diff.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_diff4() {
    let lhs = BTreeMap::from([
        ("0", 0.0000500_f32),
        ("1", 0.9999500_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999500_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999500_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999500_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999500_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000000_f32),
        ("7", 7.0000000_f32),
        ("8", 8.0000000_f32),
    ]);
    let ulps_diff = BTreeMap::from([
        ("0", Some(944879383_u32)),
        ("1", Some(839_u32)),
        ("2", Some(419_u32)),
        ("3", Some(210_u32)),
        ("4", Some(1_u32)),
        ("5", Some(105_u32)),
        ("6", Some(0_u32)),
        ("7", Some(105_u32)),
        ("8", Some(0_u32)),
    ]);

    assert_eq!(lhs.debug_ulps_diff(&rhs), Some(ulps_diff.clone()));
    assert_eq!(rhs.debug_ulps_diff(&lhs), Some(ulps_diff.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff_tolerance() {
    let lhs = BTreeMap::from([
        ("0", 2.00_f32),
        ("1", 3.25_f32),
        ("2", 4.50_f32),
        ("3", 5.75_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 2.50_f32),
        ("1", 3.00_f32),
        ("2", 4.00_f32),
        ("3", 6.00_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 0.10_f32),
        ("1", 0.20_f32),
        ("2", 0.30_f32),
        ("3", 0.40_f32),
    ]);
    let tolerance = max_abs_diff.clone();

    assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(tolerance.clone()));
    assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), Some(tolerance.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff_all_tolerance() {
    let lhs = BTreeMap::from([
        ("0", 2.00_f32),
        ("1", 3.25_f32),
        ("2", 4.50_f32),
        ("3", 5.75_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 2.50_f32),
        ("1", 3.00_f32),
        ("2", 4.00_f32),
        ("3", 6.00_f32),
    ]);
    let max_abs_diff = 0.20_f32;
    let tolerance = BTreeMap::from([
        ("0", max_abs_diff),
        ("1", max_abs_diff),
        ("2", max_abs_diff),
        ("3", max_abs_diff),
    ]);

    assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), Some(tolerance.clone()));
    assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff), Some(tolerance.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_tolerance() {
    let lhs = BTreeMap::from([
        ("0", 2.00_f32),
        ("1", 3.25_f32),
        ("2", 4.50_f32),
        ("3", 5.75_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 2.50_f32),
        ("1", 3.00_f32),
        ("2", 4.00_f32),
        ("3", 6.00_f32),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 4_u32),
        ("2", 8_u32),
        ("3", 16_u32),
    ]);
    let tolerance = max_ulps.clone();

    assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), Some(tolerance.clone()));
    assert_eq!(rhs.debug_ulps_tolerance(&lhs, &max_ulps), Some(tolerance.clone()));
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_all_tolerance() {
    let lhs = BTreeMap::from([
        ("0", 2.00_f32),
        ("1", 3.25_f32),
        ("2", 4.50_f32),
        ("3", 5.75_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 2.50_f32),
        ("1", 3.00_f32),
        ("2", 4.00_f32),
        ("3", 6.00_f32),
    ]);
    let max_ulps = 8_u32;
    let tolerance = BTreeMap::from([
        ("0", max_ulps),
        ("1", max_ulps),
        ("2", max_ulps),
        ("3", max_ulps),
    ]);

    assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), Some(tolerance.clone()));
    assert_eq!(rhs.debug_ulps_all_tolerance(&lhs, &max_ulps), Some(tolerance.clone()));
}

#[test]
fn test_eq_empty1() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs: BTreeMap<&str, f32> = BTreeMap::new();
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::new();

    assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty1() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs: BTreeMap<&str, f32> = BTreeMap::new();
    let max_abs_diff = BTreeMap::from([
        ("0", 1.0_f32 * f32::EPSILON),
        ("1", 0.5_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 0.5_f32 * f32::EPSILON),
        ("6", 0.5_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 1_u32),
        ("2", 8_u32),
        ("3", 8_u32),
        ("4", 8_u32),
        ("5", 1_u32),
        ("6", 1_u32),
        ("7", 8_u32),
        ("8", 8_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty2() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs: BTreeMap<&str, f32> = BTreeMap::new();
    let max_abs_diff = BTreeMap::from([
        ("0", 1.0_f32 * f32::EPSILON),
        ("1", 0.5_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 0.5_f32 * f32::EPSILON),
        ("6", 0.5_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty3() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs: BTreeMap<&str, f32> = BTreeMap::new();
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::from([
        ("0", 4_u32),
        ("1", 2_u32),
        ("2", 4_u32),
        ("3", 4_u32),
        ("4", 4_u32),
        ("5", 2_u32),
        ("6", 2_u32),
        ("7", 4_u32),
        ("8", 4_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty4() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_abs_diff = BTreeMap::from([
        ("0", 1.0_f32 * f32::EPSILON),
        ("1", 0.5_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 0.5_f32 * f32::EPSILON),
        ("6", 0.5_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 1_u32),
        ("2", 8_u32),
        ("3", 8_u32),
        ("4", 8_u32),
        ("5", 1_u32),
        ("6", 1_u32),
        ("7", 8_u32),
        ("8", 8_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty5() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_abs_diff = BTreeMap::from([
        ("0", 1.0_f32 * f32::EPSILON),
        ("1", 0.5_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 0.5_f32 * f32::EPSILON),
        ("6", 0.5_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty6() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 1_u32),
        ("2", 8_u32),
        ("3", 8_u32),
        ("4", 8_u32),
        ("5", 1_u32),
        ("6", 1_u32),
        ("7", 8_u32),
        ("8", 8_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty7() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty8() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 2.0_f32 * f32::EPSILON),
        ("1", 1.0_f32 * f32::EPSILON),
        ("2", 2.0_f32 * f32::EPSILON),
        ("3", 2.0_f32 * f32::EPSILON),
        ("4", 2.0_f32 * f32::EPSILON),
        ("5", 1.0_f32 * f32::EPSILON),
        ("6", 1.0_f32 * f32::EPSILON),
        ("7", 2.0_f32 * f32::EPSILON),
        ("8", 2.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 2_u32),
        ("2", 2_u32),
        ("3", 2_u32),
        ("4", 2_u32),
        ("5", 2_u32),
        ("6", 2_u32),
        ("7", 2_u32),
        ("8", 2_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty9() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 2.0_f32 * f32::EPSILON),
        ("1", 1.0_f32 * f32::EPSILON),
        ("2", 2.0_f32 * f32::EPSILON),
        ("3", 2.0_f32 * f32::EPSILON),
        ("4", 2.0_f32 * f32::EPSILON),
        ("5", 1.0_f32 * f32::EPSILON),
        ("6", 1.0_f32 * f32::EPSILON),
        ("7", 2.0_f32 * f32::EPSILON),
        ("8", 2.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty10() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 2_u32),
        ("2", 2_u32),
        ("3", 2_u32),
        ("4", 2_u32),
        ("5", 2_u32),
        ("6", 2_u32),
        ("7", 2_u32),
        ("8", 2_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty11() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty12() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty13() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::from([
        ("0", 2.0_f32 * f32::EPSILON),
        ("1", 1.0_f32 * f32::EPSILON),
        ("2", 4.0_f32 * f32::EPSILON),
        ("3", 4.0_f32 * f32::EPSILON),
        ("4", 4.0_f32 * f32::EPSILON),
        ("5", 1.0_f32 * f32::EPSILON),
        ("6", 1.0_f32 * f32::EPSILON),
        ("7", 4.0_f32 * f32::EPSILON),
        ("8", 4.0_f32 * f32::EPSILON),
    ]);
    let max_ulps = BTreeMap::new();

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne_empty14() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = BTreeMap::new();
    let max_ulps = BTreeMap::from([
        ("0", 4_u32),
        ("1", 2_u32),
        ("2", 4_u32),
        ("3", 4_u32),
        ("4", 4_u32),
        ("5", 2_u32),
        ("6", 2_u32),
        ("7", 4_u32),
        ("8", 4_u32),
    ]);

    assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[test]
fn test_all_eq_empty() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs: BTreeMap<&str, f32> = BTreeMap::new();
    let max_abs_diff = 4.0_f32 * f32::EPSILON;
    let max_ulps = 32_u32;

    assert_ulps_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne_empty1() {
    let lhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 0.9999999_f32),
        ("2", 2.0000000_f32),
        ("3", 2.9999995_f32),
        ("4", 4.0000000_f32),
        ("5", 4.9999999_f32),
        ("6", 6.0000000_f32),
        ("7", 6.9999995_f32),
        ("8", 8.0000000_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_ulps = 64_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne_empty2() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 0.0000002_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999995_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000001_f32),
        ("7", 7.0000000_f32),
        ("8", 7.9999995_f32),
    ]);
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_ulps = 64_u32;

    assert_ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff_empty1() {
    let lhs = BTreeMap::from([
        ("0", 0.00_f32),
        ("1", 1.00_f32),
        ("2", 1.25_f32),
        ("3", 1.50_f32),
        ("4", 2.00_f32),
        ("5", 2.50_f32),
        ("6", 3.00_f32),
        ("7", 4.00_f32),
        ("8", 5.00_f32),
    ]);
    let rhs = BTreeMap::new();

    assert_eq!(lhs.debug_abs_diff(&rhs), None);
    assert_eq!(rhs.debug_abs_diff(&lhs), None);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff_empty2() {
    let lhs = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 0.0000000_f32),
        ("1", 1.0000000_f32),
        ("2", 1.9999500_f32),
        ("3", 3.0000000_f32),
        ("4", 4.0000005_f32),
        ("5", 5.0000000_f32),
        ("6", 6.0000000_f32),
        ("7", 7.0000000_f32),
        ("8", 8.0000000_f32),
    ]);

    assert_eq!(lhs.debug_abs_diff(&rhs), None);
    assert_eq!(rhs.debug_abs_diff(&lhs), None);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff_tolerance_empty() {
    let lhs = BTreeMap::from([
        ("0", 2.00_f32),
        ("1", 3.25_f32),
        ("2", 4.50_f32),
        ("3", 5.75_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_abs_diff = BTreeMap::from([
        ("0", 0.10_f32),
        ("1", 0.20_f32),
        ("2", 0.30_f32),
        ("3", 0.40_f32),
    ]);

    assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
    assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), None);
}

#[rustfmt::skip]
#[test]
fn test_debug_abs_diff_all_tolerance_empty() {
    let lhs: BTreeMap<&str, f32> = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 2.50_f32),
        ("1", 3.00_f32),
        ("2", 4.00_f32),
        ("3", 6.00_f32),
    ]);
    let max_abs_diff = 0.20_f32;

    assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), None);
    assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff), None);
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_tolerance_empty() {
    let lhs = BTreeMap::new();
    let rhs = BTreeMap::from([
        ("0", 2.50_f32),
        ("1", 3.00_f32),
        ("2", 4.00_f32),
        ("3", 6.00_f32),
    ]);
    let max_ulps = BTreeMap::from([
        ("0", 2_u32),
        ("1", 4_u32),
        ("2", 8_u32),
        ("3", 16_u32),
    ]);

    assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), None);
    assert_eq!(rhs.debug_ulps_tolerance(&lhs, &max_ulps), None);
}

#[rustfmt::skip]
#[test]
fn test_debug_ulps_all_tolerance_empty() {
    let lhs = BTreeMap::from([
        ("0", 2.00_f32),
        ("1", 3.25_f32),
        ("2", 4.50_f32),
        ("3", 5.75_f32),
    ]);
    let rhs = BTreeMap::new();
    let max_ulps = 8_u32;

    assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), None);
    assert_eq!(rhs.debug_ulps_all_tolerance(&lhs, &max_ulps), None);
}
