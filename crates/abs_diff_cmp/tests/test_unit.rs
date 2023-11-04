extern crate abs_diff_cmp;


use abs_diff_cmp::{
    abs_diff_eq,
    assert_abs_diff_eq,
    AbsDiffAllEq,
    AbsDiffEq,
    AssertAbsDiffAllEq,
    AssertAbsDiffEq,
};

#[test]
fn test_eq() {
    assert!(AbsDiffEq::abs_diff_eq(&(), &(), &()));
    assert!(abs_diff_eq!((), (), abs_diff <= ()));
    assert_abs_diff_eq!((), (), abs_diff <= ());
}

#[test]
fn test_all_eq() {
    assert!(AbsDiffAllEq::abs_diff_all_eq(&(), &(), &()));
    assert!(abs_diff_eq!((), (), abs_diff <= ()));
    assert_abs_diff_eq!((), (), abs_diff <= ());
}

#[test]
fn test_debug_abs_diff() {
    assert_eq!(().debug_abs_diff(&()), ());
}

#[test]
fn test_debug_abs_diff_tolerance() {
    assert_eq!(().debug_abs_diff_tolerance(&(), &()), ());
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    assert_eq!(().debug_abs_diff_all_tolerance(&(), &()), ());
}
