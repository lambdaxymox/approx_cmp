use crate::abs_diff::{
    AbsDiffAllEq,
    AbsDiffEq,
    AssertAbsDiffEq,
    AssertAbsDiffAllEq,
};
use core::fmt;


impl AbsDiffEq for () {
    type Tolerance = ();

    #[inline]
    fn abs_diff_eq(&self, _other: &(), _max_abs_diff: &Self::Tolerance) -> bool {
        true
    }
}

impl AbsDiffAllEq for () {
    type AllTolerance = ();

    #[inline]
    fn abs_diff_all_eq(&self, _other: &(), _max_abs_diff: &Self::AllTolerance) -> bool {
        true
    }
}

impl AssertAbsDiffEq for () {
    type DebugAbsDiff = ();
    type DebugTolerance = ();

    #[inline]
    fn debug_abs_diff(&self, _other: &()) -> Self::DebugAbsDiff {}

    #[inline]
    fn debug_abs_diff_tolerance(&self, _other: &(), _max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {}
}

impl AssertAbsDiffAllEq for () {
    type AllDebugTolerance = ();

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, _other: &(), _max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {}
}


macro_rules! impl_abs_diff_tuple {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {$(
        impl<$($T:AbsDiffEq),+> AbsDiffEq for ($($T,)+)
        where
            last_type!($($T,)+): ?Sized,
            $($T::Tolerance: Sized,)+
        {
            type Tolerance = ($($T::Tolerance,)+);

            #[inline]
            fn abs_diff_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> bool {
                $(self.$idx.abs_diff_eq(&other.$idx, &max_abs_diff.$idx))&&+
            }
        }

        impl<$($T:AssertAbsDiffEq + fmt::Debug),+> AssertAbsDiffEq for ($($T,)+)
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
        }
    )+};
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

impl_abs_diff_tuple! {
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


impl<A, B> AbsDiffAllEq<(B,)> for (A,)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B,), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B,)> for (A,)
where
    A: AssertAbsDiffAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance,);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B,), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),)
    }
}

impl<A, B> AbsDiffAllEq<(B, B)> for (A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B)> for (A, A)
where
    A: AssertAbsDiffAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance, A::AllDebugTolerance);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
        )
    }
}

impl<A, B> AbsDiffAllEq<(B, B, B)> for (A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B)> for (A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B)> for (A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B)> for (A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B)> for (A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B)> for (A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B)> for (A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B)> for (A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff) &&
        self.6.abs_diff_all_eq(&other.6, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff) &&
        self.6.abs_diff_all_eq(&other.6, max_abs_diff) &&
        self.7.abs_diff_all_eq(&other.7, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff) &&
        self.6.abs_diff_all_eq(&other.6, max_abs_diff) &&
        self.7.abs_diff_all_eq(&other.7, max_abs_diff) &&
        self.8.abs_diff_all_eq(&other.8, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff) &&
        self.6.abs_diff_all_eq(&other.6, max_abs_diff) &&
        self.7.abs_diff_all_eq(&other.7, max_abs_diff) &&
        self.8.abs_diff_all_eq(&other.8, max_abs_diff) &&
        self.9.abs_diff_all_eq(&other.9, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff) &&
        self.6.abs_diff_all_eq(&other.6, max_abs_diff) &&
        self.7.abs_diff_all_eq(&other.7, max_abs_diff) &&
        self.8.abs_diff_all_eq(&other.8, max_abs_diff) &&
        self.9.abs_diff_all_eq(&other.9, max_abs_diff) &&
        self.10.abs_diff_all_eq(&other.10, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
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
}

impl<A, B> AbsDiffAllEq<(B, B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A, A)
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &(B, B, B, B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> bool {
        self.0.abs_diff_all_eq(&other.0, max_abs_diff) &&
        self.1.abs_diff_all_eq(&other.1, max_abs_diff) &&
        self.2.abs_diff_all_eq(&other.2, max_abs_diff) &&
        self.3.abs_diff_all_eq(&other.3, max_abs_diff) &&
        self.4.abs_diff_all_eq(&other.4, max_abs_diff) &&
        self.5.abs_diff_all_eq(&other.5, max_abs_diff) &&
        self.6.abs_diff_all_eq(&other.6, max_abs_diff) &&
        self.7.abs_diff_all_eq(&other.7, max_abs_diff) &&
        self.8.abs_diff_all_eq(&other.8, max_abs_diff) &&
        self.9.abs_diff_all_eq(&other.9, max_abs_diff) &&
        self.10.abs_diff_all_eq(&other.10, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<(B, B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A, A)
where
    A: AssertAbsDiffAllEq<B>,
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
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
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
}

