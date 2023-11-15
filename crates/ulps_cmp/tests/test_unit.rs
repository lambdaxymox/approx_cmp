use ulps_cmp::{
    assert_ulps_eq,
    ulps_eq,
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};


#[test]
fn test_eq() {
    assert!(UlpsEq::ulps_eq(&(), &(), &(), &()));
    assert!(ulps_eq!((), (), abs_diff <= (), ulps <= ()));
    assert_ulps_eq!((), (), abs_diff <= (), ulps <= ());
}

#[test]
fn test_all_eq() {
    assert!(UlpsAllEq::ulps_all_eq(&(), &(), &(), &()));
    assert!(ulps_eq!((), (), abs_diff_all <= (), ulps_all <= ()));
    assert_ulps_eq!((), (), abs_diff_all <= (), ulps_all <= ());
}

#[test]
fn test_debug_abs_diff() {
    assert_eq!(().debug_abs_diff(&()), ());
}

#[test]
fn test_debub_ulps_diff() {
    assert_eq!(().debug_ulps_diff(&()), ());
}

#[test]
fn test_debug_abs_diff_tolerance() {
    assert_eq!(().debug_abs_diff_tolerance(&(), &()), ());
}

#[test]
fn test_debug_ulps_tolerance() {
    assert_eq!(().debug_ulps_tolerance(&(), &()), ());
}

#[test]
fn test_debug_abs_diff_all_tolerance() {
    assert_eq!(().debug_abs_diff_all_tolerance(&(), &()), ());
}

#[test]
fn test_debug_ulps_all_tolerance() {
    assert_eq!(().debug_ulps_all_tolerance(&(), &()), ());
}
