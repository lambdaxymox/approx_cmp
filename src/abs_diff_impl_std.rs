use crate::abs_diff::{
    AbsDiffEq,
    AbsDiffAllEq,
    AssertAbsDiffEq,
    AssertAbsDiffAllEq,
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


impl<A, B> AbsDiffEq<Box<B>> for Box<A> 
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<Rc<B>> for Rc<A> 
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<Arc<B>> for Arc<A> 
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Box<B>> for Box<A> 
where
    A: AbsDiffAllEq<B> + ?Sized,
    B: ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Rc<B>> for Rc<A> 
where
    A: AbsDiffAllEq<B> + ?Sized,
    B: ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Arc<B>> for Arc<A> 
where
    A: AbsDiffAllEq<B> + ?Sized,
    B: ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Box<B>> for Box<A> 
where
    A: AssertAbsDiffEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Box<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Rc<B>> for Rc<A> 
where
    A: AssertAbsDiffEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Rc<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Arc<B>> for Arc<A> 
where
    A: AssertAbsDiffEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Arc<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Box<B>> for Box<A> 
where
    A: AssertAbsDiffAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Rc<B>> for Rc<A> 
where
    A: AssertAbsDiffAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Arc<B>> for Arc<A> 
where
    A: AssertAbsDiffAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }
}

