use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};
use std::boxed::Box;
use std::rc::Rc;
use std::fmt;


impl<A, B> UlpsEq<Box<B>> for Box<A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<Rc<B>> for Rc<A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}


impl<A, B> UlpsAllEq<Box<B>> for Box<A>
where
    A: UlpsAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<Rc<B>> for Rc<A>
where
    A: UlpsAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}


impl<A, B> AssertUlpsEq<Box<B>> for Box<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Box<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Box<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Box<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<Rc<B>> for Rc<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Rc<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Rc<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Rc<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&**self, &**other, max_ulps)
    }
}


impl<A, B> AssertUlpsAllEq<Box<B>> for Box<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Box<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<Rc<B>> for Rc<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Rc<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&**self, &**other, max_ulps)
    }
}
