use crate::abs_diff::{
    AssertAbsDiffEq,
    AssertAbsDiffAllEq,
};


impl<A, B> AssertAbsDiffEq<[B]> for [A]
where
    A: AssertAbsDiffEq<B>,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &[B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff(b))
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &[B], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .map(|((a, b), tol)| { AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }
}

impl<'a, 'b, A, B> AssertAbsDiffEq<&'b [B]> for &'a [A]
where
    A: AssertAbsDiffEq<B>,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &&'b [B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff(b))
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .map(|((a, b), tol)| { AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertAbsDiffAllEq<[B]> for [A]
where
    A: AssertAbsDiffAllEq<B>,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff_all_tolerance(b, max_abs_diff))
                .collect(),
            )
        } else {
            None
        }
    }
}

impl<'a, 'b, A, B> AssertAbsDiffAllEq<&'b [B]> for &'a [A]
where
    A: AssertAbsDiffAllEq<B>,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff_all_tolerance(b, max_abs_diff))
                .collect(),
            )
        } else {
            None
        }
    }
}

