use crate::relative::{
    RelativeEq,
    RelativeAllEq,
    AssertRelativeEq,
    AssertRelativeAllEq,
};
use std::boxed::{
    Box,
};
use std::rc::{
    Rc,
};
use std::sync::{
    Arc,
};


impl<A, B> RelativeEq<Box<B>> for Box<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<Rc<B>> for Rc<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<Arc<B>> for Arc<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Box<B>> for Box<A> 
where
    A: RelativeAllEq<B> + ?Sized,
    B: ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Rc<B>> for Rc<A> 
where
    A: RelativeAllEq<B> + ?Sized,
    B: ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Arc<B>> for Arc<A> 
where
    A: RelativeAllEq<B> + ?Sized,
    B: ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeEq<Box<B>> for Box<A> 
where
    A: AssertRelativeEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Box<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Box<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<Rc<B>> for Rc<A> 
where
    A: AssertRelativeEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Rc<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Rc<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<Arc<B>> for Arc<A> 
where
    A: AssertRelativeEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Arc<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Arc<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Box<B>> for Box<A> 
where
    A: AssertRelativeAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Box<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Rc<B>> for Rc<A> 
where
    A: AssertRelativeAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Rc<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Arc<B>> for Arc<A> 
where
    A: AssertRelativeAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Arc<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_relative)
    }
}

