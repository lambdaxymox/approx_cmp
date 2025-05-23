use crate::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};
use core::fmt;

impl UlpsEq for () {
    type Tolerance = ();
    type UlpsTolerance = ();

    #[inline]
    fn ulps_eq(&self, _other: &(), _max_abs_diff: &Self::Tolerance, _max_ulps: &Self::UlpsTolerance) -> bool {
        true
    }
}

impl UlpsAllEq for () {
    type AllTolerance = ();
    type AllUlpsTolerance = ();

    #[inline]
    fn ulps_all_eq(&self, _other: &(), _max_abs_diff: &Self::AllTolerance, _max_ulps: &Self::AllUlpsTolerance) -> bool {
        true
    }
}

impl AssertUlpsEq for () {
    type DebugAbsDiff = ();
    type DebugUlpsDiff = ();
    type DebugTolerance = ();
    type DebugUlpsTolerance = ();

    #[inline]
    fn debug_abs_diff(&self, _other: &()) -> Self::DebugAbsDiff {}

    #[inline]
    fn debug_ulps_diff(&self, _other: &()) -> Self::DebugUlpsDiff {}

    #[inline]
    fn debug_abs_diff_tolerance(&self, _other: &(), _max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {}

    #[inline]
    fn debug_ulps_tolerance(&self, _other: &(), _max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {}
}

impl AssertUlpsAllEq for () {
    type AllDebugTolerance = ();
    type AllDebugUlpsTolerance = ();

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, _other: &(), _max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {}

    #[inline]
    fn debug_ulps_all_tolerance(&self, _other: &(), _max_ulps: &Self::AllTolerance) -> Self::AllDebugTolerance {}
}

macro_rules! impl_ulps_tuple {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {$(
        impl<$($T:UlpsEq),+> UlpsEq for ($($T,)+)
        where
            last_type!($($T,)+): ?Sized,
            $($T::Tolerance: Sized, $T::UlpsTolerance: Sized,)+
        {
            type Tolerance = ($($T::Tolerance,)+);
            type UlpsTolerance = ($($T::UlpsTolerance,)+);

            #[inline]
            fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
                $(self.$idx.ulps_eq(&other.$idx, &max_abs_diff.$idx, &max_ulps.$idx))&&+
            }
        }

        impl<$($T:AssertUlpsEq + fmt::Debug),+> AssertUlpsEq for ($($T,)+)
        where
            last_type!($($T,)+): ?Sized,
            $($T::Tolerance: Sized,)+
            $($T::UlpsTolerance: Sized,)+
            $($T::DebugTolerance: Sized,)+
            $($T::DebugUlpsTolerance: Sized,)+
        {
            type DebugAbsDiff = ($($T::DebugAbsDiff,)+);
            type DebugUlpsDiff = ($($T::DebugUlpsDiff,)+);
            type DebugTolerance = ($($T::DebugTolerance,)+);
            type DebugUlpsTolerance = ($($T::DebugUlpsTolerance,)+);

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                ($(self.$idx.debug_abs_diff(&other.$idx),)+)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &Self) -> Self::DebugUlpsDiff {
                ($(self.$idx.debug_ulps_diff(&other.$idx),)+)
            }

            #[inline]
            fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
                ($(self.$idx.debug_abs_diff_tolerance(&other.$idx, &max_abs_diff.$idx),)+)
            }

            #[inline]
            fn debug_ulps_tolerance(&self, other: &Self, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
                ($(self.$idx.debug_ulps_tolerance(&other.$idx, &max_ulps.$idx),)+)
            }
        }
    )+};
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

impl_ulps_tuple! {
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

impl<A, B> UlpsAllEq<(B,)> for (A,)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B,), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B,)> for (A,)
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance,);
    type AllDebugUlpsTolerance = (A::AllDebugUlpsTolerance,);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B,), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &(B,), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (self.0.debug_ulps_all_tolerance(&other.0, max_ulps),)
    }
}

impl<A, B> UlpsAllEq<(B, B)> for (A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps) && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B)> for (A, A)
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance, A::AllDebugTolerance);
    type AllDebugUlpsTolerance = (A::AllDebugUlpsTolerance, A::AllDebugUlpsTolerance);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
        )
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &(B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B)> for (A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B)> for (A, A, A)
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = (A::AllDebugTolerance, A::AllDebugTolerance, A::AllDebugTolerance);
    type AllDebugUlpsTolerance = (A::AllDebugUlpsTolerance, A::AllDebugUlpsTolerance, A::AllDebugUlpsTolerance);

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &(B, B, B), max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        (
            self.0.debug_abs_diff_all_tolerance(&other.0, max_abs_diff),
            self.1.debug_abs_diff_all_tolerance(&other.1, max_abs_diff),
            self.2.debug_abs_diff_all_tolerance(&other.2, max_abs_diff),
        )
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &(B, B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B)> for (A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B, B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B)> for (A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(&self, other: &(B, B, B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B)> for (A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B)> for (A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(&self, other: &(B, B, B, B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B)> for (A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B)> for (A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = (
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
        A::AllDebugTolerance,
    );
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(&self, other: &(B, B, B, B, B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
            && self.6.ulps_all_eq(&other.6, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
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
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(&self, other: &(B, B, B, B, B, B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
            self.6.debug_ulps_all_tolerance(&other.6, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &(B, B, B, B, B, B, B, B), max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
            && self.6.ulps_all_eq(&other.6, max_abs_diff, max_ulps)
            && self.7.ulps_all_eq(&other.7, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
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
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(&self, other: &(B, B, B, B, B, B, B, B), max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
            self.6.debug_ulps_all_tolerance(&other.6, max_ulps),
            self.7.debug_ulps_all_tolerance(&other.7, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_ulps: &Self::AllUlpsTolerance,
    ) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
            && self.6.ulps_all_eq(&other.6, max_abs_diff, max_ulps)
            && self.7.ulps_all_eq(&other.7, max_abs_diff, max_ulps)
            && self.8.ulps_all_eq(&other.8, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
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
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B),
        max_ulps: &Self::AllUlpsTolerance,
    ) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
            self.6.debug_ulps_all_tolerance(&other.6, max_ulps),
            self.7.debug_ulps_all_tolerance(&other.7, max_ulps),
            self.8.debug_ulps_all_tolerance(&other.8, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_ulps: &Self::AllUlpsTolerance,
    ) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
            && self.6.ulps_all_eq(&other.6, max_abs_diff, max_ulps)
            && self.7.ulps_all_eq(&other.7, max_abs_diff, max_ulps)
            && self.8.ulps_all_eq(&other.8, max_abs_diff, max_ulps)
            && self.9.ulps_all_eq(&other.9, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
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
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B),
        max_ulps: &Self::AllUlpsTolerance,
    ) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
            self.6.debug_ulps_all_tolerance(&other.6, max_ulps),
            self.7.debug_ulps_all_tolerance(&other.7, max_ulps),
            self.8.debug_ulps_all_tolerance(&other.8, max_ulps),
            self.9.debug_ulps_all_tolerance(&other.9, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_ulps: &Self::AllUlpsTolerance,
    ) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
            && self.6.ulps_all_eq(&other.6, max_abs_diff, max_ulps)
            && self.7.ulps_all_eq(&other.7, max_abs_diff, max_ulps)
            && self.8.ulps_all_eq(&other.8, max_abs_diff, max_ulps)
            && self.9.ulps_all_eq(&other.9, max_abs_diff, max_ulps)
            && self.10.ulps_all_eq(&other.10, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
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
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B),
        max_ulps: &Self::AllUlpsTolerance,
    ) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
            self.6.debug_ulps_all_tolerance(&other.6, max_ulps),
            self.7.debug_ulps_all_tolerance(&other.7, max_ulps),
            self.8.debug_ulps_all_tolerance(&other.8, max_ulps),
            self.9.debug_ulps_all_tolerance(&other.9, max_ulps),
            self.10.debug_ulps_all_tolerance(&other.10, max_ulps),
        )
    }
}

impl<A, B> UlpsAllEq<(B, B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A, A)
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B, B),
        max_abs_diff: &Self::AllTolerance,
        max_ulps: &Self::AllUlpsTolerance,
    ) -> bool {
        self.0.ulps_all_eq(&other.0, max_abs_diff, max_ulps)
            && self.1.ulps_all_eq(&other.1, max_abs_diff, max_ulps)
            && self.2.ulps_all_eq(&other.2, max_abs_diff, max_ulps)
            && self.3.ulps_all_eq(&other.3, max_abs_diff, max_ulps)
            && self.4.ulps_all_eq(&other.4, max_abs_diff, max_ulps)
            && self.5.ulps_all_eq(&other.5, max_abs_diff, max_ulps)
            && self.6.ulps_all_eq(&other.6, max_abs_diff, max_ulps)
            && self.7.ulps_all_eq(&other.7, max_abs_diff, max_ulps)
            && self.8.ulps_all_eq(&other.8, max_abs_diff, max_ulps)
            && self.9.ulps_all_eq(&other.9, max_abs_diff, max_ulps)
            && self.10.ulps_all_eq(&other.10, max_abs_diff, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<(B, B, B, B, B, B, B, B, B, B, B, B)> for (A, A, A, A, A, A, A, A, A, A, A, A)
where
    A: AssertUlpsAllEq<B>,
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
    type AllDebugUlpsTolerance = (
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
        A::AllDebugUlpsTolerance,
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
    fn debug_ulps_all_tolerance(
        &self,
        other: &(B, B, B, B, B, B, B, B, B, B, B, B),
        max_ulps: &Self::AllUlpsTolerance,
    ) -> Self::AllDebugUlpsTolerance {
        (
            self.0.debug_ulps_all_tolerance(&other.0, max_ulps),
            self.1.debug_ulps_all_tolerance(&other.1, max_ulps),
            self.2.debug_ulps_all_tolerance(&other.2, max_ulps),
            self.3.debug_ulps_all_tolerance(&other.3, max_ulps),
            self.4.debug_ulps_all_tolerance(&other.4, max_ulps),
            self.5.debug_ulps_all_tolerance(&other.5, max_ulps),
            self.6.debug_ulps_all_tolerance(&other.6, max_ulps),
            self.7.debug_ulps_all_tolerance(&other.7, max_ulps),
            self.8.debug_ulps_all_tolerance(&other.8, max_ulps),
            self.9.debug_ulps_all_tolerance(&other.9, max_ulps),
            self.10.debug_ulps_all_tolerance(&other.10, max_ulps),
            self.11.debug_ulps_all_tolerance(&other.11, max_ulps),
        )
    }
}
