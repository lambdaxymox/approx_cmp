extern crate relative_cmp;


use relative_cmp::{
    assert_relative_eq,
    relative_eq,
    AssertRelativeAllEq,
    AssertRelativeEq,
    RelativeAllEq,
    RelativeEq,
};


#[test]
fn test_eq() {
    assert!(RelativeEq::relative_eq(&(), &(), &(), &()));
    assert!(relative_eq!((), (), abs_diff <= (), relative <= ()));
    assert_relative_eq!((), (), abs_diff <= (), relative <= ());
}

#[test]
fn test_all_eq() {
    assert!(RelativeAllEq::relative_all_eq(&(), &(), &(), &()));
    assert!(relative_eq!((), (), abs_diff_all <= (), relative_all <= ()));
    assert_relative_eq!((), (), abs_diff_all <= (), relative_all <= ());
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
fn test_debug_relative_tolerance() {
    assert_eq!(().debug_relative_tolerance(&(), &()), ());
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    assert_eq!(().debug_abs_diff_all_tolerance(&(), &()), ());
}

#[test]
fn test_debug_relative_all_tolerance() {
    assert_eq!(().debug_relative_all_tolerance(&(), &()), ());
}
