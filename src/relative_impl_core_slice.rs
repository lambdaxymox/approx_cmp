use crate::relative::{
    RelativeAllEq,
    RelativeEq,
};


impl<A, B> RelativeEq<[B]> for [A]
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn relative_eq(&self, other: &[B], max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_relative.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_relative.iter())
                .all(|(((a, b), abs_tol), rel_tol)| a.relative_eq(b, abs_tol, rel_tol))
    }
}

impl<'a, 'b, A, B> RelativeEq<&'b [B]> for &'a [A]
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn relative_eq(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_relative.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_relative.iter())
                .all(|(((a, b), abs_tol), rel_tol)| a.relative_eq(b, abs_tol, rel_tol))
    }
}

impl<A, B> RelativeAllEq<[B]> for [A]
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &[B], max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.relative_all_eq(b, max_abs_diff, max_relative))
    }
}

impl<'a, 'b, A, B> RelativeAllEq<&'b [B]> for &'a [A]
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.relative_all_eq(b, max_abs_diff, max_relative))
    }
}
