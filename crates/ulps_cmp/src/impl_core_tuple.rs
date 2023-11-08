use crate::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};
use core::fmt;


impl UlpsEq for () {
    type Tolerance = ();
    type UlpsTolerance = ();

    #[inline]
    fn ulps_eq(&self, _other: &(), _max_abs_diff: &Self::Tolerance, _max_ulps: &Self::UlpsTolerance) -> bool {
        true
    }
}

impl UlpsAllEq for () {
    type AllTolerance = ();
    type AllUlpsTolerance = ();

    #[inline]
    fn ulps_all_eq(&self, _other: &(), _max_abs_diff: &Self::AllTolerance, _max_ulps: &Self::AllUlpsTolerance) -> bool {
        true
    }
}

impl AssertUlpsEq for () {
    type DebugAbsDiff = ();
    type DebugUlpsDiff = ();
    type DebugTolerance = ();
    type DebugUlpsTolerance = ();

    #[inline]
    fn debug_abs_diff(&self, _other: &()) -> Self::DebugAbsDiff {}

    #[inline]
    fn debug_ulps_diff(&self, _other: &()) -> Self::DebugUlpsDiff {}

    #[inline]
    fn debug_abs_diff_tolerance(&self, _other: &(), _max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {}

    #[inline]
    fn debug_ulps_tolerance(&self, _other: &(), _max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {}
}

impl AssertUlpsAllEq for () {
    type AllDebugTolerance = ();
    type AllDebugUlpsTolerance = ();

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, _other: &(), _max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {}

    #[inline]
    fn debug_ulps_all_tolerance(&self, _other: &(), _max_ulps: &Self::AllTolerance) -> Self::AllDebugTolerance {}
}
