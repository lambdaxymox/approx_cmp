use core::fmt;


pub trait RelativeEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type Tolerance: ?Sized;

    /// Compare two floating point numbers for relative equality.
    ///
    /// The relative equality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two floating point numbers
    /// are relatively equal with respect to a `maxRelative` multiple of the
    /// tolerance `tolerance`.
    fn relative_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool;

    /// Compare two floating point numbers for relative inequality.
    ///
    /// The relative inequality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two floating point numbers
    /// are relatively inequal with respect to a `maxRelative` multiple of the
    /// tolerance `tolerance`.
    fn relative_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        !Self::relative_eq(self, other, max_abs_diff, max_relative)
    }
}

pub trait RelativeAllEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type AllTolerance: ?Sized;

    fn relative_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool;

    fn relative_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        !Self::relative_all_eq(self, other, max_abs_diff, max_relative)
    }
}

pub trait AssertRelativeEq<Rhs = Self>: RelativeEq<Rhs>
where
    Rhs: ?Sized
{
    type DebugAbsDiff: fmt::Debug + Sized;
    type DebugTolerance: fmt::Debug;

    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    fn debug_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance;

    fn debug_relative_tolerance(&self, other: &Rhs, max_relative: &Self::Tolerance) -> Self::DebugTolerance;
}

pub trait AssertRelativeAllEq<Rhs = Self>: RelativeAllEq<Rhs> 
where
    Rhs: ?Sized
{
    type AllDebugTolerance: fmt::Debug;

    fn debug_abs_diff_all_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;

    fn debug_relative_all_tolerance(&self, other: &Rhs, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance;
}



#[doc(hidden)]
pub struct RelativeCmp {}

impl RelativeCmp {
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_relative: &A::Tolerance) -> bool 
    where
        A: RelativeEq<B> + ?Sized,
        B: ?Sized
    {
        A::relative_eq(lhs, rhs, max_abs_diff, max_relative)
    }

    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_relative: &A::Tolerance) -> bool 
    where
        A: RelativeEq<B> + ?Sized,
        B: ?Sized
    {
        A::relative_ne(lhs, rhs, max_abs_diff, max_relative)
    }

    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_relative: &A::AllTolerance) -> bool 
    where
        A: RelativeAllEq<B> + ?Sized,
        B: ?Sized
    {
        A::relative_all_eq(lhs, rhs, max_abs_diff, max_relative)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_relative: &A::AllTolerance) -> bool 
    where
        A: RelativeAllEq<B> + ?Sized,
        B: ?Sized
    {
        A::relative_all_ne(lhs, rhs, max_abs_diff, max_relative)
    }
}


#[doc(hidden)]
pub struct RelativeCmpOpTol {}

impl RelativeCmpOpTol {
    #[inline]
    pub fn abs_diff<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> A::DebugTolerance 
    where
        A: RelativeEq<B> + AssertRelativeEq<B>
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance 
    where
        A: RelativeAllEq<B> + AssertRelativeAllEq<B>
    {
        A::debug_abs_diff_all_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn relative<A, B>(lhs: &A, rhs: &B, max_relative: &A::Tolerance) -> A::DebugTolerance
    where
        A: RelativeEq<B> + AssertRelativeEq<B>
    {
        A::debug_relative_tolerance(lhs,rhs, max_relative)
    }

    #[inline]
    pub fn relative_all<A, B>(lhs: &A, rhs: &B, max_relative: &A::AllTolerance) -> A::AllDebugTolerance
    where
        A: RelativeAllEq<B> + AssertRelativeAllEq<B>
    {
        A::debug_relative_all_tolerance(lhs,rhs, max_relative)
    }
}

#[macro_export]
macro_rules! relative_eq {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, relative <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, relative_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
}

#[macro_export]
macro_rules! relative_ne {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, relative <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, relative_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_relative_ne {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! debug_assert_relative_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_relative_eq!($($arg)*); })
}

#[macro_export]
macro_rules! debug_assert_relative_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_relative_ne!($($arg)*); })
}

