use core::fmt;


pub trait UlpsEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    type Tolerance: ?Sized;
    type UlpsTolerance: ?Sized;

    /// Compare two floating point numbers for units in last place (ULPS)
    /// equality.
    ///
    /// The ulps equality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two are floating point
    /// numbers are equal with respect to a maximum number `maxUlps` of units
    /// in last place.
    fn ulps_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool;

    /// Compare two floating point numbers for units in last place (ULPS)
    /// inequality.
    ///
    /// The ulps inequality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two are floating point
    /// numbers are inequal with respect to a maximum number `maxUlps` of units
    /// in last place.
    fn ulps_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        !Self::ulps_eq(self, other, max_abs_diff, max_ulps)
    }
}

pub trait UlpsAllEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    type AllTolerance: ?Sized;
    type AllUlpsTolerance: ?Sized;

    fn ulps_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool;

    fn ulps_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        !Self::ulps_all_eq(self, other, max_abs_diff, max_ulps)
    }
}

pub trait AssertUlpsEq<Rhs = Self>: UlpsEq<Rhs>
where
    Rhs: ?Sized,
{
    type DebugAbsDiff: fmt::Debug + Sized;
    type DebugUlpsDiff: fmt::Debug + Sized;
    type DebugTolerance: fmt::Debug;
    type DebugUlpsTolerance: fmt::Debug;

    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    fn debug_ulps_diff(&self, other: &Rhs) -> Self::DebugUlpsDiff;

    fn debug_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance;

    fn debug_ulps_tolerance(&self, other: &Rhs, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance;
}

pub trait AssertUlpsAllEq<Rhs = Self>: UlpsAllEq<Rhs>
where
    Rhs: ?Sized,
{
    type AllDebugTolerance: fmt::Debug;
    type AllDebugUlpsTolerance: fmt::Debug;

    fn debug_abs_diff_all_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;

    fn debug_ulps_all_tolerance(&self, other: &Rhs, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance;
}


#[doc(hidden)]
pub struct UlpsCmp {}

impl UlpsCmp {
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_ulps: &A::UlpsTolerance) -> bool
    where
        A: UlpsEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_eq(lhs, rhs, max_abs_diff, max_ulps)
    }

    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_ulps: &A::UlpsTolerance) -> bool
    where
        A: UlpsEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_ne(lhs, rhs, max_abs_diff, max_ulps)
    }

    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_ulps: &A::AllUlpsTolerance) -> bool
    where
        A: UlpsAllEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_all_eq(lhs, rhs, max_abs_diff, max_ulps)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_ulps: &A::AllUlpsTolerance) -> bool
    where
        A: UlpsAllEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_all_ne(lhs, rhs, max_abs_diff, max_ulps)
    }
}


#[doc(hidden)]
pub struct UlpsCmpOpTol {}

impl UlpsCmpOpTol {
    #[inline]
    pub fn abs_diff<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> A::DebugTolerance
    where
        A: UlpsEq<B> + AssertUlpsEq<B>,
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance
    where
        A: UlpsAllEq<B> + AssertUlpsAllEq<B>,
    {
        A::debug_abs_diff_all_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn ulps<A, B>(lhs: &A, rhs: &B, max_ulps: &A::UlpsTolerance) -> A::DebugUlpsTolerance
    where
        A: UlpsEq<B> + AssertUlpsEq<B>,
    {
        A::debug_ulps_tolerance(lhs, rhs, max_ulps)
    }

    #[inline]
    pub fn ulps_all<A, B>(lhs: &A, rhs: &B, max_ulps: &A::AllUlpsTolerance) -> A::AllDebugUlpsTolerance
    where
        A: UlpsAllEq<B> + AssertUlpsAllEq<B>,
    {
        A::debug_ulps_all_tolerance(lhs, rhs, max_ulps)
    }
}

#[macro_export]
macro_rules! ulps_eq {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, ulps <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, ulps <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, ulps_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, ulps_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
}

#[macro_export]
macro_rules! ulps_ne {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, ulps <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, ulps <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, ulps_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, ulps_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::UlpsCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
}

#[macro_export]
macro_rules! assert_ulps_eq {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::ulps_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `assert_ulps_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertUlpsEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertUlpsEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::UlpsCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::UlpsCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::ulps_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `assert_ulps_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertUlpsEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertUlpsEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::UlpsCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::UlpsCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_ulps_ne {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::ulps_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `assert_ulps_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertUlpsEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertUlpsEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::UlpsCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::UlpsCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::ulps_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `assert_ulps_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertUlpsEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertUlpsEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::UlpsCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::UlpsCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! debug_assert_ulps_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ulps_eq!($($arg)*); })
}

#[macro_export]
macro_rules! debug_assert_ulps_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ulps_ne!($($arg)*); })
}
