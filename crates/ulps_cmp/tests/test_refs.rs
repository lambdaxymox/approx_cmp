extern crate ulps_cmp;


use ulps_cmp::{
    assert_ulps_eq,
    assert_ulps_ne,
    AssertUlpsAllEq,
    AssertUlpsEq,
};


#[rustfmt::skip]
#[test]
fn test_eq1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 1.0_f32;
    let max_ulps = 16_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_eq2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 1.0_f32 * f32::EPSILON;
    let max_ulps = 4_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_eq3() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_ulps = 2_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_eq4() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 4194304_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_eq5() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 2_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.499_f32;
    let max_ulps = 16_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.49999_f32;
    let max_ulps = 4_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne3() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.4999999_f32;
    let max_ulps = 4_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne4() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 4194303_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_ne5() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 1_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, ulps <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, ulps <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 1.0_f32;
    let max_ulps = 16_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 1.0_f32 * f32::EPSILON;
    let max_ulps = 4_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq3() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 2.0_f32 * f32::EPSILON;
    let max_ulps = 2_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq4() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 4194304_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_eq5() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 2_u32;

    assert_ulps_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.499_f32;
    let max_ulps = 16_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne2() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.49999_f32;
    let max_ulps = 4_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne3() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.4999999_f32;
    let max_ulps = 4_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne4() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 4_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[rustfmt::skip]
#[test]
fn test_all_ne5() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.0_f32;
    let max_ulps = 4194303_u32;

    assert_ulps_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    assert_ulps_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
}

#[test]
fn test_debug_abs_diff() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;

    assert_eq!(AssertUlpsEq::debug_abs_diff(&a, &b), 0.5_f32);
    assert_eq!(AssertUlpsEq::debug_abs_diff(&a, &mut b_mut), 0.5_f32);
    assert_eq!(AssertUlpsEq::debug_abs_diff(&mut a_mut, &b), 0.5_f32);
    assert_eq!(AssertUlpsEq::debug_abs_diff(&mut a_mut, &mut b_mut), 0.5_f32);
}

#[test]
fn test_debug_abs_diff_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertUlpsEq::debug_abs_diff_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertUlpsEq::debug_abs_diff_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertUlpsEq::debug_abs_diff_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertUlpsEq::debug_abs_diff_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertUlpsAllEq::debug_abs_diff_all_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertUlpsAllEq::debug_abs_diff_all_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertUlpsAllEq::debug_abs_diff_all_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertUlpsAllEq::debug_abs_diff_all_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}

#[test]
fn test_debug_ulps_diff1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;

    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &b), Some(4194304_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &mut b_mut), Some(4194304_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &b), Some(4194304_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &mut b_mut), Some(4194304_u32));
}

#[test]
fn test_debug_ulps_diff2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;

    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &b), Some(2_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &mut b_mut), Some(2_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &b), Some(2_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &mut b_mut), Some(2_u32));
}

#[test]
fn test_debug_ulps_diff3() {
    let a = &1.0_f32;
    let b = &0.999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.999999_f32;

    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &b), Some(17_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &mut b_mut), Some(17_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &b), Some(17_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &mut b_mut), Some(17_u32));
}

#[test]
fn test_debug_ulps_diff4() {
    let a = &1.0_f32;
    let b = &0.99999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.99999_f32;

    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &b), Some(168_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&a, &mut b_mut), Some(168_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &b), Some(168_u32));
    assert_eq!(AssertUlpsEq::debug_ulps_diff(&mut a_mut, &mut b_mut), Some(168_u32));
}

#[test]
fn test_debug_ulps_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertUlpsEq::debug_ulps_tolerance(&a, &b, &4_u32), 4_u32);
    assert_eq!(AssertUlpsEq::debug_ulps_tolerance(&a, &b_mut, &4_u32), 4_u32);
    assert_eq!(AssertUlpsEq::debug_ulps_tolerance(&a_mut, &b, &4_u32), 4_u32);
    assert_eq!(AssertUlpsEq::debug_ulps_tolerance(&a_mut, &b_mut, &4_u32), 4_u32);
}

#[test]
fn test_debug_ulps_all_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertUlpsAllEq::debug_ulps_all_tolerance(&a, &b, &4_u32), 4_u32);
    assert_eq!(AssertUlpsAllEq::debug_ulps_all_tolerance(&a, &b_mut, &4_u32), 4_u32);
    assert_eq!(AssertUlpsAllEq::debug_ulps_all_tolerance(&a_mut, &b, &4_u32), 4_u32);
    assert_eq!(AssertUlpsAllEq::debug_ulps_all_tolerance(&a_mut, &b_mut, &4_u32), 4_u32);
}
