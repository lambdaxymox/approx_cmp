extern crate approx_cmp_relative;


use approx_cmp_relative::{
    assert_relative_eq,
    assert_relative_ne,
    AssertRelativeAllEq,
    AssertRelativeEq,
};


#[rustfmt::skip]
#[test]
fn test_eq1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 1.0_f32;
    let max_relative = 1.0_f32;

    assert_relative_eq!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_eq2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 1.0_f32 * f32::EPSILON;
    let max_relative = 1.0_f32 * f32::EPSILON;

    assert_relative_eq!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_eq3() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_relative = 2.0_f32 * f32::EPSILON;

    assert_relative_eq!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.499_f32;
    let max_relative = 0.0499_f32;

    assert_relative_ne!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.49999_f32;
    let max_relative = 0.049999_f32;

    assert_relative_ne!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_ne3() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.4999999_f32;
    let max_relative = 0.04999999_f32;

    assert_relative_ne!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
    assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_eq1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 1.0_f32;
    let max_relative = 1.0_f32;

    assert_relative_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_eq2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 1.0_f32 * f32::EPSILON;
    let max_relative = 1.0_f32 * f32::EPSILON;

    assert_relative_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_eq3() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_relative = 2.0_f32 * f32::EPSILON;

    assert_relative_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_ne1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.499_f32;
    let max_relative = 0.0499_f32;

    assert_relative_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_ne2() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.49999_f32;
    let max_relative = 0.049999_f32;

    assert_relative_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[rustfmt::skip]
#[test]
fn test_all_ne3() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.4999999_f32;
    let max_relative = 0.04999999_f32;

    assert_relative_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
}

#[test]
fn test_debug_abs_diff() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;

    assert_eq!(AssertRelativeEq::debug_abs_diff(&a, &b), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_abs_diff(&a, &mut b_mut), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_abs_diff(&mut a_mut, &b), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_abs_diff(&mut a_mut, &mut b_mut), 0.5_f32);
}

#[test]
fn test_debug_abs_diff_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}

#[test]
fn test_debug_relative_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}

#[test]
fn test_debug_relative_all_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}
