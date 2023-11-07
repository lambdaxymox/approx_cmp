extern crate ulps_cmp;


use ulps_cmp::{
    assert_ulps_eq,
    assert_ulps_ne,
    ulps_eq,
    ulps_ne,
};


#[rustfmt::skip]
#[test]
fn test_eq() {
    let lhs = [
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ];
    let rhs = [
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ];
    let eps = f32::EPSILON;
    let max_abs_diff = [
        1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];
    let max_ulps = [
        2_u32, 4_u32, 2_u32, 1_u32, 
        1_u32, 1_u32, 1_u32, 1_u32,
    ];

    assert!(ulps_eq!(lhs[..], rhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps));
    assert!(ulps_eq!(rhs[..], lhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps));
    assert_ulps_eq!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&rhs[..], &lhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let lhs = [
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ];
    let rhs = [
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ];
    let eps = f32::EPSILON;
    let max_abs_diff = [eps; 8];
    let max_ulps = [
        1_u32, 3_u32, 1_u32, 1_u32,
        1_u32, 1_u32, 1_u32, 1_u32,
    ];

    assert!(ulps_ne!(lhs[..], rhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps));
    assert!(ulps_ne!(rhs[..], lhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps));
    assert_ulps_ne!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&rhs[..], &lhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let lhs = [
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ];
    let rhs = [
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ];
    let eps = f32::EPSILON;
    let max_abs_diff = [
        1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
    ];
    let max_ulps = [1_u32; 8];

    assert!(ulps_ne!(lhs[..], rhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps));
    assert!(ulps_ne!(rhs[..], lhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps));
    assert_ulps_ne!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&rhs[..], &lhs[..], abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq() {
    let lhs = [
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ];
    let rhs = [
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ];
    let max_abs_diff = 6.0_f32 * f32::EPSILON;
    let max_ulps = 4_u32;

    assert!(ulps_eq!(lhs[..], rhs[..], abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
    assert!(ulps_eq!(rhs[..], lhs[..], abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
    assert_ulps_eq!(&lhs[..], &rhs[..], abs_diff_all <= &max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&rhs[..], &lhs[..], abs_diff_all <= &max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne() {
    let lhs = [
        0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
        4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
    ];
    let rhs = [
        1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
        5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
    ];
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_ulps = 1_u32;

    assert!(ulps_ne!(lhs[..], rhs[..], abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
    assert!(ulps_ne!(rhs[..], lhs[..], abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
    assert_ulps_ne!(&lhs[..], &rhs[..], abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&rhs[..], &lhs[..], abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}
