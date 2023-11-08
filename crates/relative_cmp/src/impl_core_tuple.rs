use crate::{
    AssertRelativeAllEq,
    AssertRelativeEq,
    RelativeAllEq,
    RelativeEq,
};
use core::fmt;


impl RelativeEq for () {
    type Tolerance = ();

    #[inline]
    fn relative_eq(&self, _other: &(), _max_abs_diff: &Self::Tolerance, _max_relative: &Self::Tolerance) -> bool {
        true
    }
}

impl RelativeAllEq for () {
    type AllTolerance = ();

    #[inline]
    fn relative_all_eq(&self, _other: &(), _max_abs_diff: &Self::AllTolerance, _max_relative: &Self::AllTolerance) -> bool {
        true
    }
}

impl AssertRelativeEq for () {
    type DebugAbsDiff = ();
    type DebugTolerance = ();

    #[inline]
    fn debug_abs_diff(&self, _other: &()) -> Self::DebugAbsDiff {}

    #[inline]
    fn debug_abs_diff_tolerance(&self, _other: &(), _max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {}

    #[inline]
    fn debug_relative_tolerance(&self, _other: &(), _max_relative: &Self::Tolerance) -> Self::DebugTolerance {}
}

impl AssertRelativeAllEq for () {
    type AllDebugTolerance = ();

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, _other: &(), _max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {}

    #[inline]
    fn debug_relative_all_tolerance(&self, _other: &(), _max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {}
}


macro_rules! impl_relative_tuple {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {$(
        impl<$($T:RelativeEq),+> RelativeEq for ($($T,)+)
        where
            last_type!($($T,)+): ?Sized,
            $($T::Tolerance: Sized,)+
        {
            type Tolerance = ($($T::Tolerance,)+);

            #[inline]
            fn relative_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
                $(self.$idx.relative_eq(&other.$idx, &max_abs_diff.$idx, &max_relative.$idx))&&+
            }
        }

        impl<$($T:AssertRelativeEq + fmt::Debug),+> AssertRelativeEq for ($($T,)+)
        where
            last_type!($($T,)+): ?Sized,
            $($T::Tolerance: Sized,)+
            $($T::DebugTolerance: Sized,)+
        {
            type DebugAbsDiff = ($($T::DebugAbsDiff,)+);
            type DebugTolerance = ($($T::DebugTolerance,)+);

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                ($(self.$idx.debug_abs_diff(&other.$idx),)+)
            }

            #[inline]
            fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
                ($(self.$idx.debug_abs_diff_tolerance(&other.$idx, &max_abs_diff.$idx),)+)
            }

            #[inline]
            fn debug_relative_tolerance(&self, other: &Self, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
                ($(self.$idx.debug_relative_tolerance(&other.$idx, &max_relative.$idx),)+)
            }
        }
    )+};
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

impl_relative_tuple! {
    Tuple1 {
        (0) -> A0
    }
    Tuple2 {
        (0) -> A0
        (1) -> A1
    }
    Tuple3 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
    }
    Tuple4 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
    }
    Tuple5 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
    }
    Tuple6 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
    }
    Tuple7 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
        (6) -> A6
    }
    Tuple8 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
        (6) -> A6
        (7) -> A7
    }
    Tuple9 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
        (6) -> A6
        (7) -> A7
        (8) -> A8
    }
    Tuple10 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
        (6) -> A6
        (7) -> A7
        (8) -> A8
        (9) -> A9
    }
    Tuple11 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
        (6) -> A6
        (7) -> A7
        (8) -> A8
        (9) -> A9
        (10) -> A10
    }
    Tuple12 {
        (0) -> A0
        (1) -> A1
        (2) -> A2
        (3) -> A3
        (4) -> A4
        (5) -> A5
        (6) -> A6
        (7) -> A7
        (8) -> A8
        (9) -> A9
        (10) -> A10
        (11) -> A11
    }
}

impl<A, B> RelativeAllEq<(B,)> for (A,)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B,), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B,)> for (A,)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance,);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B,), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B,), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (self.0.debug_relative_all_tolerance(&other.0, max_relative),)
    }
}

impl<A, B> RelativeAllEq<(B, B)> for (A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B, B), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative) && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B)> for (A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance, A::AllDebugTolerance);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B)> for (A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B, B, B), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B)> for (A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance, A::AllDebugTolerance, A::AllDebugTolerance);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B)> for (A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B, B, B, B), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B)> for (A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B, B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B)> for (A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B)> for (A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B, B, B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B)> for (A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B)> for (A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B, B, B, B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &(B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
            && self.6.relative_all_eq(&other.6, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
            self.6.debug_abs_diff_all_tolerance(&other.6, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B, B, B, B, B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
            self.6.debug_relative_all_tolerance(&other.6, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_relative: &Self::AllTolerance,
    ) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
            && self.6.relative_all_eq(&other.6, max_abs_diff, max_relative)
            && self.7.relative_all_eq(&other.7, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
            self.6.debug_abs_diff_all_tolerance(&other.6, max_abs_diff),
            self.7.debug_abs_diff_all_tolerance(&other.7, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B), max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
            self.6.debug_relative_all_tolerance(&other.6, max_relative),
            self.7.debug_relative_all_tolerance(&other.7, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_relative: &Self::AllTolerance,
    ) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
            && self.6.relative_all_eq(&other.6, max_abs_diff, max_relative)
            && self.7.relative_all_eq(&other.7, max_abs_diff, max_relative)
            && self.8.relative_all_eq(&other.8, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
            self.6.debug_abs_diff_all_tolerance(&other.6, max_abs_diff),
            self.7.debug_abs_diff_all_tolerance(&other.7, max_abs_diff),
            self.8.debug_abs_diff_all_tolerance(&other.8, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B),
        max_relative: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
            self.6.debug_relative_all_tolerance(&other.6, max_relative),
            self.7.debug_relative_all_tolerance(&other.7, max_relative),
            self.8.debug_relative_all_tolerance(&other.8, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_relative: &Self::AllTolerance,
    ) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
            && self.6.relative_all_eq(&other.6, max_abs_diff, max_relative)
            && self.7.relative_all_eq(&other.7, max_abs_diff, max_relative)
            && self.8.relative_all_eq(&other.8, max_abs_diff, max_relative)
            && self.9.relative_all_eq(&other.9, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
            self.6.debug_abs_diff_all_tolerance(&other.6, max_abs_diff),
            self.7.debug_abs_diff_all_tolerance(&other.7, max_abs_diff),
            self.8.debug_abs_diff_all_tolerance(&other.8, max_abs_diff),
            self.9.debug_abs_diff_all_tolerance(&other.9, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B),
        max_relative: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
            self.6.debug_relative_all_tolerance(&other.6, max_relative),
            self.7.debug_relative_all_tolerance(&other.7, max_relative),
            self.8.debug_relative_all_tolerance(&other.8, max_relative),
            self.9.debug_relative_all_tolerance(&other.9, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_relative: &Self::AllTolerance,
    ) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
            && self.6.relative_all_eq(&other.6, max_abs_diff, max_relative)
            && self.7.relative_all_eq(&other.7, max_abs_diff, max_relative)
            && self.8.relative_all_eq(&other.8, max_abs_diff, max_relative)
            && self.9.relative_all_eq(&other.9, max_abs_diff, max_relative)
            && self.10.relative_all_eq(&other.10, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
            self.6.debug_abs_diff_all_tolerance(&other.6, max_abs_diff),
            self.7.debug_abs_diff_all_tolerance(&other.7, max_abs_diff),
            self.8.debug_abs_diff_all_tolerance(&other.8, max_abs_diff),
            self.9.debug_abs_diff_all_tolerance(&other.9, max_abs_diff),
            self.10.debug_abs_diff_all_tolerance(&other.10, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B),
        max_relative: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
            self.6.debug_relative_all_tolerance(&other.6, max_relative),
            self.7.debug_relative_all_tolerance(&other.7, max_relative),
            self.8.debug_relative_all_tolerance(&other.8, max_relative),
            self.9.debug_relative_all_tolerance(&other.9, max_relative),
            self.10.debug_relative_all_tolerance(&other.10, max_relative),
        )
    }
}

impl<A, B> RelativeAllEq<(B, B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A, A)
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_relative: &Self::AllTolerance,
    ) -> bool {
        self.0.relative_all_eq(&other.0, max_abs_diff, max_relative)
            && self.1.relative_all_eq(&other.1, max_abs_diff, max_relative)
            && self.2.relative_all_eq(&other.2, max_abs_diff, max_relative)
            && self.3.relative_all_eq(&other.3, max_abs_diff, max_relative)
            && self.4.relative_all_eq(&other.4, max_abs_diff, max_relative)
            && self.5.relative_all_eq(&other.5, max_abs_diff, max_relative)
            && self.6.relative_all_eq(&other.6, max_abs_diff, max_relative)
            && self.7.relative_all_eq(&other.7, max_abs_diff, max_relative)
            && self.8.relative_all_eq(&other.8, max_abs_diff, max_relative)
            && self.9.relative_all_eq(&other.9, max_abs_diff, max_relative)
            && self.10.relative_all_eq(&other.10, max_abs_diff, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<(B, B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A, A)
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );

    #[inline]
    fn debug_abs_diff_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
            self.3.debug_abs_diff_all_tolerance(&other.3, max_abs_diff),
            self.4.debug_abs_diff_all_tolerance(&other.4, max_abs_diff),
            self.5.debug_abs_diff_all_tolerance(&other.5, max_abs_diff),
            self.6.debug_abs_diff_all_tolerance(&other.6, max_abs_diff),
            self.7.debug_abs_diff_all_tolerance(&other.7, max_abs_diff),
            self.8.debug_abs_diff_all_tolerance(&other.8, max_abs_diff),
            self.9.debug_abs_diff_all_tolerance(&other.9, max_abs_diff),
            self.10.debug_abs_diff_all_tolerance(&other.10, max_abs_diff),
            self.11.debug_abs_diff_all_tolerance(&other.11, max_abs_diff),
        )
    }

    #[inline]
    fn debug_relative_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B, B),
        max_relative: &Self::AllTolerance,
    ) -> Self::AllDebugTolerance {
        (
            self.0.debug_relative_all_tolerance(&other.0, max_relative),
            self.1.debug_relative_all_tolerance(&other.1, max_relative),
            self.2.debug_relative_all_tolerance(&other.2, max_relative),
            self.3.debug_relative_all_tolerance(&other.3, max_relative),
            self.4.debug_relative_all_tolerance(&other.4, max_relative),
            self.5.debug_relative_all_tolerance(&other.5, max_relative),
            self.6.debug_relative_all_tolerance(&other.6, max_relative),
            self.7.debug_relative_all_tolerance(&other.7, max_relative),
            self.8.debug_relative_all_tolerance(&other.8, max_relative),
            self.9.debug_relative_all_tolerance(&other.9, max_relative),
            self.10.debug_relative_all_tolerance(&other.10, max_relative),
            self.11.debug_relative_all_tolerance(&other.11, max_relative),
        )
    }
}
