extern crate approx_cmp;


use approx_cmp::{
    assert_relative_eq,
    assert_relative_ne,
    relative_eq,
    relative_ne,
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
    let max_relative = [
        1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
    ];

    assert!(relative_eq!(lhs[..], rhs[..], abs_diff <= max_abs_diff, relative <= max_relative));
    assert!(relative_eq!(rhs[..], lhs[..], abs_diff <= max_abs_diff, relative <= max_relative));
    assert_relative_eq!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&rhs[..], &lhs[..], abs_diff <= max_abs_diff, relative <= max_relative);
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
    let max_relative = [eps; 8];

    assert!(relative_ne!(lhs[..], rhs[..], abs_diff <= max_abs_diff, relative <= max_relative));
    assert!(relative_ne!(rhs[..], lhs[..], abs_diff <= max_abs_diff, relative <= max_relative));
    assert_relative_ne!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&rhs[..], &lhs[..], abs_diff <= max_abs_diff, relative <= max_relative);
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
        1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, eps / 2.0_f32, 4.0_f32 * eps,
    ];
    let max_relative = [
        1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        1.0_f32 * eps, 1.0_f32 * eps, eps / 2.0_f32, 4.0_f32 * eps,
    ];

    assert!(relative_ne!(lhs[..], rhs[..], abs_diff <= max_abs_diff, relative <= max_relative));
    assert!(relative_ne!(rhs[..], lhs[..], abs_diff <= max_abs_diff, relative <= max_relative));
    assert_relative_ne!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&rhs[..], &lhs[..], abs_diff <= max_abs_diff, relative <= max_relative);
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
    let max_relative = 6.0_f32 * f32::EPSILON;

    assert!(relative_eq!(lhs[..], rhs[..], abs_diff_all <= max_abs_diff, relative_all <= max_relative));
    assert!(relative_eq!(rhs[..], lhs[..], abs_diff_all <= max_abs_diff, relative_all <= max_relative));
    assert_relative_eq!(&lhs[..], &rhs[..], abs_diff_all <= &max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&rhs[..], &lhs[..], abs_diff_all <= &max_abs_diff, relative_all <= max_relative);
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
    let max_relative = 1.0_f32 * f32::EPSILON;

    assert!(relative_ne!(lhs[..], rhs[..], abs_diff_all <= max_abs_diff, relative_all <= max_relative));
    assert!(relative_ne!(rhs[..], lhs[..], abs_diff_all <= max_abs_diff, relative_all <= max_relative));
    assert_relative_ne!(&lhs[..], &rhs[..], abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&rhs[..], &lhs[..], abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}
