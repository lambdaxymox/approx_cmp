extern crate approx_cmp;


use approx_cmp::{
    abs_diff_eq,
    abs_diff_ne,
    assert_abs_diff_eq,
    assert_abs_diff_ne,
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
        1.0 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
        1.0 * eps, 1.0 * eps, 4.0 * eps, 4.0 * eps,
    ];

    assert!(abs_diff_eq!(lhs[..], rhs[..], abs_diff <= max_abs_diff));
    assert_abs_diff_eq!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff);
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

    assert!(abs_diff_ne!(lhs[..], rhs[..], abs_diff <= max_abs_diff));
    assert_abs_diff_ne!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff);
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
        1.0 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
        1.0 * eps, 1.0 * eps, eps / 2.0, 4.0 * eps,
    ];

    assert!(abs_diff_ne!(lhs[..], rhs[..], abs_diff <= max_abs_diff));
    assert_abs_diff_ne!(&lhs[..], &rhs[..], abs_diff <= max_abs_diff);
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

    assert!(abs_diff_eq!(lhs[..], rhs[..], abs_diff_all <= max_abs_diff));
    assert_abs_diff_eq!(&lhs[..], &rhs[..], abs_diff_all <= &max_abs_diff);
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

    assert!(abs_diff_ne!(lhs[..], rhs[..], abs_diff_all <= max_abs_diff));
    assert_abs_diff_ne!(&lhs[..], &rhs[..], abs_diff_all <= max_abs_diff);
}
