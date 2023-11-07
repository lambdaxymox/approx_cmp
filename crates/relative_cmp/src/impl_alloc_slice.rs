use crate::traits::{
    AssertRelativeAllEq,
    AssertRelativeEq,
};
use std::vec::Vec;


impl<A, B> AssertRelativeEq<[B]> for [A]
where
    A: AssertRelativeEq<B>,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &[B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(self.iter().zip(other.iter()).map(|(a, b)| a.debug_abs_diff(b)).collect())
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
                    .map(|((a, b), tol)| AssertRelativeEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &[B], max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_relative.iter())
                    .map(|((a, b), tol)| AssertRelativeEq::debug_relative_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeAllEq<[B]> for [A]
where
    A: AssertRelativeAllEq<B>,
    A::AllTolerance: Sized,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &[B], max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_relative_all_tolerance(a, b, max_relative))
                    .collect(),
            )
        } else {
            None
        }
    }
}
