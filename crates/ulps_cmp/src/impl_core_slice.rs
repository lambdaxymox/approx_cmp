use crate::traits::{
    UlpsAllEq,
    UlpsEq,
};


impl<A, B> UlpsEq<[B]> for [A]
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = [A::Tolerance];
    type UlpsTolerance = [A::UlpsTolerance];

    #[inline]
    fn ulps_eq(&self, other: &[B], max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_ulps.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_ulps.iter())
                .all(|(((a, b), abs_tol), ulps_tol)| a.ulps_eq(b, abs_tol, ulps_tol))
    }
}

impl<'a, 'b, A, B> UlpsEq<&'b [B]> for &'a [A]
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = [A::Tolerance];
    type UlpsTolerance = [A::UlpsTolerance];

    #[inline]
    fn ulps_eq(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_ulps.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_ulps.iter())
                .all(|(((a, b), abs_tol), ulps_tol)| a.ulps_eq(b, abs_tol, ulps_tol))
    }
}

impl<A, B> UlpsAllEq<[B]> for [A]
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &[B], max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.ulps_all_eq(b, max_abs_diff, max_ulps))
    }
}

impl<'a, 'b, A, B> UlpsAllEq<&'b [B]> for &'a [A]
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.ulps_all_eq(b, max_abs_diff, max_ulps))
    }
}
