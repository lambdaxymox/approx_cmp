use abs_diff_cmp::{
    assert_abs_diff_eq,
    assert_abs_diff_ne,
    AssertAbsDiffAllEq,
    AssertAbsDiffEq,
};


#[rustfmt::skip]
#[test]
fn test_eq1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 1.0_f32;

    assert_abs_diff_eq!(&a,         &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_eq2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 1.0_f32 * f32::EPSILON;

    assert_abs_diff_eq!(&a,         &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_eq3() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 2.0_f32 * f32::EPSILON;

    assert_abs_diff_eq!(&a,         &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_ne1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.499_f32;

    assert_abs_diff_ne!(&a,         &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_ne2() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.49999_f32;

    assert_abs_diff_ne!(&a,         &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_ne3() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.4999999_f32;

    assert_abs_diff_ne!(&a,         &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_eq1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 1.0_f32;

    assert_abs_diff_eq!(&a,         &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_eq2() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 1.0_f32 * f32::EPSILON;

    assert_abs_diff_eq!(&a,         &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_eq3() {
    let a = &1.0_f32;
    let b = &0.9999999_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &0.9999999_f32;
    let max_abs_diff = 2.0_f32 * f32::EPSILON;

    assert_abs_diff_eq!(&a,         &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_ne1() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.499_f32;

    assert_abs_diff_ne!(&a,         &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_ne2() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.49999_f32;

    assert_abs_diff_ne!(&a,         &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff);
}

#[rustfmt::skip]
#[test]
fn test_all_ne3() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;
    let max_abs_diff = 0.4999999_f32;

    assert_abs_diff_ne!(&a,         &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff);
    assert_abs_diff_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff);
}

#[test]
fn test_debug_abs_diff() {
    let a = &1.0_f32;
    let b = &1.5_f32;
    let mut a_mut = &mut 1.0_f32;
    let mut b_mut = &mut 1.5_f32;

    assert_eq!(AssertAbsDiffEq::debug_abs_diff(&a, &b), 0.5_f32);
    assert_eq!(AssertAbsDiffEq::debug_abs_diff(&a, &mut b_mut), 0.5_f32);
    assert_eq!(AssertAbsDiffEq::debug_abs_diff(&mut a_mut, &b), 0.5_f32);
    assert_eq!(AssertAbsDiffEq::debug_abs_diff(&mut a_mut, &mut b_mut), 0.5_f32);
}

#[test]
fn test_debug_abs_diff_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertAbsDiffEq::debug_abs_diff_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertAbsDiffEq::debug_abs_diff_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertAbsDiffEq::debug_abs_diff_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertAbsDiffEq::debug_abs_diff_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    let a = &1.0_f32;
    let b = &2.0_f32;
    let a_mut = &mut 1.0_f32;
    let b_mut = &mut 2.0_f32;

    assert_eq!(AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&a, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
    assert_eq!(AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
    assert_eq!(AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
}
