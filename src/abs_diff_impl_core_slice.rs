use crate::abs_diff::{
    AbsDiffEq,
    AbsDiffAllEq,
    AssertAbsDiffEq,
    AssertAbsDiffAllEq,
};


impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn abs_diff_eq(&self, other: &[B], max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .zip(max_abs_diff.iter())
            .all(|((a, b), tol)| a.abs_diff_eq(b, tol))
    }
}

impl<'a, 'b, A, B> AbsDiffEq<&'b [B]> for &'a [A]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn abs_diff_eq(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .zip(max_abs_diff.iter())
            .all(|((a, b), tol)| a.abs_diff_eq(b, tol))
    }
}

impl<A, B> AbsDiffAllEq<[B]> for [A]
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.abs_diff_all_eq(b, max_abs_diff))
    }
}

impl<'a, 'b, A, B> AbsDiffAllEq<&'b [B]> for &'a [A]
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.abs_diff_all_eq(b, max_abs_diff))
    }
}

