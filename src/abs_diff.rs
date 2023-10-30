use core::fmt;


pub trait AbsDiffEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type Tolerance: ?Sized;

    /// Compare two floating point numbers for absolute difference equality.
    ///
    /// Two floating point numbers are equal relative to some error if the
    /// following condition holds.
    ///
    /// Given floating point numbers `lhs` and `rhs`, and an error `tolerance`,
    /// we say that `lhs` and `rhs` are approximately equal within tolerance
    /// `tolerance` provided that
    /// ```text
    /// abs(lhs - rhs) <= tolerance
    /// ```
    ///
    /// - Returns: A boolean indicating whether or not two floating point
    /// numbers are absolute difference equal with respect to a tolerance
    /// `tolerance`.
    fn abs_diff_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> bool;

    /// Compare two floating point numbers for absolute difference inequality.
    ///
    /// Two floating point numbers are approximately inequal within tolerance
    /// `tolerance` provided that they are not approximately equal within tolerance
    /// `tolerance`.
    ///
    /// - Returns: A boolean indicating whether or not two floating point
    /// numbers are absolute difference inequal with respect to a tolerance
    /// `tolerance`.
    #[inline]
    fn abs_diff_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> bool {
        !Self::abs_diff_eq(self, other, max_abs_diff)
    }
}

pub trait AbsDiffAllEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type AllTolerance: ?Sized;

    fn abs_diff_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> bool;

    fn abs_diff_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> bool {
        !Self::abs_diff_all_eq(self, other, max_abs_diff)
    }
}
pub trait AssertAbsDiffEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized
{
    type DebugAbsDiff: fmt::Debug + Sized;
    type DebugTolerance: fmt::Debug;

    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    fn debug_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance;
}

pub trait AssertAbsDiffAllEq<Rhs = Self>: AbsDiffAllEq<Rhs> 
where
    Rhs: ?Sized
{
    type AllDebugTolerance: fmt::Debug;

    fn debug_abs_diff_all_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;
}


#[doc(hidden)]
pub struct AbsDiffCmp {}

impl AbsDiffCmp {
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> bool 
    where
        A: AbsDiffEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> bool 
    where
        A: AbsDiffEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_ne(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool 
    where
        A: AbsDiffAllEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_all_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool 
    where
        A: AbsDiffAllEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_all_ne(lhs, rhs, max_abs_diff)
    }
}

#[doc(hidden)]
pub struct AbsDiffCmpOpTol {}

impl AbsDiffCmpOpTol {
    #[inline]
    pub fn abs_diff<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> A::DebugTolerance 
    where
        A: AbsDiffEq<B> + AssertAbsDiffEq<B>
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance 
    where
        A: AbsDiffAllEq<B> + AssertAbsDiffAllEq<B>
    {
        A::debug_abs_diff_all_tolerance(lhs, rhs, max_abs_diff)
    }
}

#[macro_export]
macro_rules! abs_diff_eq {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::eq(left_val, right_val, &$tol)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::all_eq(left_val, right_val, &$tol)
            }
        }
    }};
}

#[macro_export]
macro_rules! abs_diff_ne {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::ne(left_val, right_val, &$tol)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::all_ne(left_val, right_val, &$tol)
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_eq {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_eq!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_eq!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_eq!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_eq!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_ne!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_ne!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_ne!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_ne!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! debug_assert_abs_diff_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_abs_diff_eq!($($arg)*); })
}

#[macro_export]
macro_rules! debug_assert_abs_diff_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_abs_diff_ne!($($arg)*); })
}

