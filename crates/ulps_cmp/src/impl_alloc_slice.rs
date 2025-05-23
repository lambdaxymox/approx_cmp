use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
};
use std::vec::Vec;

impl<A, B> AssertUlpsEq<[B]> for [A]
where
    A: AssertUlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
    A::DebugTolerance: Sized,
    A::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugUlpsDiff = Option<Vec<A::DebugUlpsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;
    type DebugUlpsTolerance = Option<Vec<A::DebugUlpsTolerance>>;

    #[rustfmt::skip]
    #[inline]
    fn debug_abs_diff(&self, other: &[B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_abs_diff(b))
                    .collect(),
                )
        } else {
            None
        }
    }

    #[rustfmt::skip]
    #[inline]
    fn debug_ulps_diff(&self, other: &[B]) -> Self::DebugUlpsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| a.debug_ulps_diff(b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &[B], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &[B], max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if (self.len() == other.len()) && (self.len() == max_ulps.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_ulps.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_ulps_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertUlpsAllEq<[B]> for [A]
where
    A: AssertUlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
    A::AllDebugTolerance: Sized,
    A::AllDebugUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;
    type AllDebugUlpsTolerance = Option<Vec<A::AllDebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &[B], max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_ulps_all_tolerance(a, b, max_ulps))
                    .collect(),
            )
        } else {
            None
        }
    }
}
