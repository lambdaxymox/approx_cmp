use core::fmt;


/// Compare two sequences of finite precision floating point numbers using 
/// per field absolute difference tolerances.
/// 
/// Types implement this trait to utilize the [`abs_diff_eq`] and [`abs_diff_ne`]
/// macros.
/// 
/// More precisely, let `A` be a finite set of values, let `T` be a floating
/// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of finite
/// precision floating point numbers. Let `max_abs_diff :: A -> T` be a sequence
/// of finite precision floating point numbers such that 
/// ```text
/// forall a in A. max_abs_diff[a] >= 0
/// ```
/// We say that `u` is absolute difference equal to `v` with tolerance
/// `max_abs_diff` provided that
/// ```text
/// forall a in A. abs(u[a], v[a]) <= max_abs_diff[a]
/// ```
/// 
/// # Examples (Floating Point Number Comparisons)
/// 
/// ```
/// # use abs_diff_cmp::{
/// #     abs_diff_eq,
/// #     abs_diff_ne,
/// #     AbsDiffEq,
/// # };
/// #
/// let lhs = 11.0_f32;
/// let rhs = 11.000105_f32;
/// let max_abs_diff1 = 0.000105_f32;
/// let max_abs_diff2 = 0.000104904175_f32;
/// let max_abs_diff3 = 0.000104_f32;
/// 
/// assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff1));
/// assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff2));
/// assert!(lhs.abs_diff_ne(&rhs, &max_abs_diff3));
/// 
/// // Using the [`abs_diff_eq`] macro.
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff1));
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff2));
/// 
/// // Using the [`abs_diff_ne`] macro.
/// assert!(abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff3));
/// ```
/// 
/// # Examples (Floating Point Number Sequence Comparisons)
/// 
/// ```
/// # use abs_diff_cmp::{
/// #     abs_diff_eq,
/// #     abs_diff_ne,
/// #     AbsDiffEq,
/// # };
/// #
/// let lhs = [1_f32, 2_f32, 3_f32, 4_f32];
/// let rhs = [1.0001195_f32, 2.0002390_f32, 3.0003585, 4.0004780_f32];
/// let max_abs_diff1 = [0.0002_f32, 0.0003_f32, 0.0004_f32, 0.0005_f32];
/// let max_abs_diff2 = [0.00011944771_f32, 0.00023889542_f32, 0.00035858154_f32, 0.00047779083_f32];
/// let max_abs_diff3 = [0.0001_f32, 0.0002_f32, 0.0003_f32, 0.0004_f32];
/// 
/// assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff1));
/// assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff2));
/// assert!(lhs.abs_diff_ne(&rhs, &max_abs_diff3));
/// 
/// // Using the [`abs_diff_eq`] macro.
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff1));
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff2));
/// 
/// // Using the [`abs_diff_ne`] macro.
/// assert!(abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff3));
/// ```
pub trait AbsDiffEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The data type representing the maximum allowed absolute difference 
    /// between two values for them to be considered approximately equal.
    type Tolerance: ?Sized;

    /// Compare two sequences of finite precision floating point numbers for
    /// absolute difference equality.
    ///
    /// Returns a boolean indicating whether or not two floating point
    /// numbers are absolute difference equal with respect to a tolerance
    /// `max_abs_diff`.
    /// 
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of finite
    /// precision floating point numbers. Let `max_abs_diff :: A -> T` be a sequence
    /// of finite precision floating point numbers such that 
    /// ```text
    /// forall a in A. max_abs_diff[a] >= 0
    /// ```
    /// We say that `u` is absolute difference equal to `v` with tolerance
    /// `max_abs_diff` provided that
    /// ```text
    /// forall a in A. abs(u[a], v[a]) <= max_abs_diff[a]
    /// ```
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::{
    /// #     abs_diff_eq,
    /// #     AbsDiffEq,
    /// # };
    /// #
    /// let lhs = 7.9995_f32;
    /// let rhs = 8.0_f32;
    /// let max_abs_diff = 0.0006_f32;
    /// 
    /// assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff));
    /// 
    /// assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff));
    /// ```
    fn abs_diff_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> bool;

    /// Compare two sequences of finite precision floating point numbers for
    /// absolute difference inequality.
    ///
    /// Returns a boolean indicating whether or not two floating point
    /// numbers are absolute difference unequal with respect to a tolerance
    /// `max_abs_diff`.
    /// 
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of finite
    /// precision floating point numbers. Let `max_abs_diff :: A -> T` be a sequence
    /// of finite precision floating point numbers such that 
    /// ```text
    /// forall a in A. max_abs_diff[a] >= 0
    /// ```
    /// We say that `u` is absolute difference unequal to `v` with tolerance
    /// `max_abs_diff` provided that
    /// ```text
    /// forall a in A. abs(u[a], v[a]) > max_abs_diff[a]
    /// ```
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::{
    /// #     abs_diff_ne,
    /// #     AbsDiffEq,
    /// # };
    /// #
    /// let lhs = 7.9995_f32;
    /// let rhs = 8.0_f32;
    /// let max_abs_diff = 0.0004_f32;
    /// 
    /// assert!(lhs.abs_diff_ne(&rhs, &max_abs_diff));
    /// 
    /// assert!(abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff));
    /// ```
    #[inline]
    fn abs_diff_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> bool {
        !Self::abs_diff_eq(self, other, max_abs_diff)
    }
}

/// Compare two sequences of finite precision floating point numbers for 
/// absolute difference equality using a uniform tolerance value.
/// 
/// Types implement this trait to utilize the [`abs_diff_eq`] and [`abs_diff_ne`]
/// macros using a single absolute difference tolerance value.
/// 
/// More precisely, let `A` be a finite set of values, let `T` be a floating
/// point data type, and let `u :: A -> T` and `v :: A -> T` be sequences of 
/// finite precision floating point numbers. Let `max_abs_diff :: T` be a 
/// finite precision floating point number such that `max_abs_diff >= 0`. We 
/// say that `u` is absolute difference equal to `v` with tolerance 
/// `max_abs_diff` provided that
/// ```text
/// forall a in A. abs(u[a], v[a]) <= max_abs_diff
/// ```
/// 
/// # Examples (Floating Point Number Comparisons)
/// 
/// ```
/// # use abs_diff_cmp::{
/// #     abs_diff_eq,
/// #     abs_diff_ne,
/// #     AbsDiffAllEq,
/// # };
/// #
/// let lhs = 11.0_f32;
/// let rhs = 11.000105_f32;
/// let max_abs_diff1 = 0.000105_f32;
/// let max_abs_diff2 = 0.000104904175_f32;
/// let max_abs_diff3 = 0.000104_f32;
/// 
/// assert!(lhs.abs_diff_all_eq(&rhs, &max_abs_diff1));
/// assert!(lhs.abs_diff_all_eq(&rhs, &max_abs_diff2));
/// assert!(lhs.abs_diff_all_ne(&rhs, &max_abs_diff3));
/// 
/// // Using the [`abs_diff_eq`] macro.
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff1));
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff2));
/// 
/// // Using the [`abs_diff_ne`] macro.
/// assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff3));
/// ```
/// 
/// # Examples (Floating Point Number Sequence Comparisons)
/// 
/// ```
/// # use abs_diff_cmp::{
/// #     abs_diff_eq,
/// #     abs_diff_ne,
/// #     AbsDiffAllEq,
/// # };
/// #
/// let lhs = [1_f32, 2_f32, 3_f32, 4_f32];
/// let rhs = [1.0001195_f32, 2.0002390_f32, 3.0003585, 4.0004780_f32];
/// let max_abs_diff1 = 0.0005_f32;
/// let max_abs_diff2 = 0.0003_f32;
/// 
/// assert!(lhs.abs_diff_all_eq(&rhs, &max_abs_diff1));
/// assert!(lhs.abs_diff_all_ne(&rhs, &max_abs_diff2));
/// 
/// // Using the [`abs_diff_eq`] macro with `all` parameters.
/// assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff1));
/// 
/// // Using the [`abs_diff_ne`] macro with `all` parameters.
/// assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff2));
/// ```
pub trait AbsDiffAllEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The data type representing the uniform maximum allowed absolute 
    /// difference between every field of two values for them to be considered
    /// approximately equal.
    type AllTolerance: ?Sized;

    /// Compare two sequences of floating point numbers for absolute difference
    /// equality using a single uniform tolerance value.
    ///
    /// Returns a boolean indicating whether or not two sequences of floating 
    /// point numbers are absolute difference equal with respect to a tolerance
    /// `max_abs_diff`.
    /// 
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of 
    /// floating point numbers, and `max_abs_diff :: T` be a finite precision
    /// floating point number. Then we say that `u` is absolute difference equal
    /// to `v` with tolerance `max_abs_diff` if
    /// ```text
    /// forall a in A. abs(u[a], v[a]) <= max_abs_diff
    /// ```
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::{
    /// #     abs_diff_eq,
    /// #     AbsDiffAllEq,
    /// # };
    /// #
    /// let lhs = 1.0_f32;
    /// let rhs = 1.0001115_f32;
    /// let max_abs_diff = 0.0002_f32;
    /// 
    /// assert!(lhs.abs_diff_all_eq(&rhs, &max_abs_diff));
    /// 
    /// assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff));
    /// ```
    fn abs_diff_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> bool;

    /// Compare two sequences of floating point numbers for absolute difference
    /// inequality using a single tolerance value.
    ///
    /// Returns a boolean indicating whether or not two sequences of floating 
    /// point numbers are absolute difference **unequal** with respect to a 
    /// tolerance `max_abs_diff`.
    /// 
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of 
    /// floating point numbers, and `max_abs_diff :: T` be a finite precision 
    /// floating point number. Then we say that `u` is absolute difference unequal
    /// to `v` with tolerance `max_abs_diff` if
    /// ```text
    /// forall a in A. abs(u[a], v[a]) > max_abs_diff
    /// ```
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::{
    /// #     abs_diff_ne,
    /// #     AbsDiffAllEq,
    /// # };
    /// #
    /// let lhs = 1.0_f32;
    /// let rhs = 1.0001115_f32;
    /// let max_abs_diff = 0.0001_f32;
    /// 
    /// assert!(lhs.abs_diff_all_ne(&rhs, &max_abs_diff));
    /// 
    /// assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff));
    /// ```
    fn abs_diff_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> bool {
        !Self::abs_diff_all_eq(self, other, max_abs_diff)
    }
}


/// Provides a debugging context for when an absolute difference comparison fails.
/// 
/// Types implement this trait to use the [`assert_abs_diff_eq`] and [`assert_abs_diff_ne`]
/// macros.
pub trait AssertAbsDiffEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The absolute difference between two values in a debugging context. This is used 
    /// to display results via [`fmt::Debug`].
    type DebugAbsDiff: fmt::Debug + Sized;

    /// The value of the tolerance used for comparing two values in a debugging context.
    /// This is used to display results via [`fmt::Debug`].
    type DebugTolerance: fmt::Debug;

    /// Compute the absolute difference between two values for a debugging context.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::AssertAbsDiffEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let expected = (1.001999899999987_f64, 0.99499893_f32);
    /// let result = lhs.debug_abs_diff(&rhs);
    /// 
    /// assert_eq!(result, expected);
    /// ```
    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    /// Compute the debugging value of the maximum allowed absolute difference
    /// between two values for a debugging context.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::AssertAbsDiffEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let max_abs_diff = (0.5_f64, 0.4_f32);
    /// let expected = max_abs_diff;
    /// let result = lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff);
    /// 
    /// assert_eq!(result, expected);
    /// ```
    fn debug_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance;
}

/// Provides a debugging context for when an absolute difference comparison using 
/// an `all` comparison fails.
/// 
/// Types implement this trait to use the [`assert_abs_diff_eq`] and [`assert_abs_diff_ne`]
/// macros with `all` parameters.
pub trait AssertAbsDiffAllEq<Rhs = Self>: AbsDiffAllEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The data type representing the uniform maximum allowed absolute 
    /// difference between every field of two values for them to be considered
    /// approximately equal that can be displayed in a debugging context 
    /// via [`fmt::Debug`].
    type AllDebugTolerance: fmt::Debug;

    /// Compute the value of the maximum allowed uniform absolute difference
    /// between two values for a debugging context.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use abs_diff_cmp::AssertAbsDiffAllEq;
    /// #
    /// let lhs = [1.0_f32; 4];
    /// let rhs = [2.0_f32; 4];
    /// let max_abs_diff = 0.3_f32;
    /// let expected = [max_abs_diff; 4];
    /// let result = lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff);
    /// 
    /// assert_eq!(result, expected);
    /// ```
    fn debug_abs_diff_all_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;
}


#[doc(hidden)]
pub struct AbsDiffCmp {}

impl AbsDiffCmp {
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> bool
    where
        A: AbsDiffEq<B> + ?Sized,
        B: ?Sized,
    {
        A::abs_diff_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> bool
    where
        A: AbsDiffEq<B> + ?Sized,
        B: ?Sized,
    {
        A::abs_diff_ne(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool
    where
        A: AbsDiffAllEq<B> + ?Sized,
        B: ?Sized,
    {
        A::abs_diff_all_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool
    where
        A: AbsDiffAllEq<B> + ?Sized,
        B: ?Sized,
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
        A: AbsDiffEq<B> + AssertAbsDiffEq<B>,
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance
    where
        A: AbsDiffAllEq<B> + AssertAbsDiffAllEq<B>,
    {
        A::debug_abs_diff_all_tolerance(lhs, rhs, max_abs_diff)
    }
}

#[macro_export]
macro_rules! abs_diff_eq {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => $crate::AbsDiffCmp::eq(left_val, right_val, &$tol),
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => $crate::AbsDiffCmp::all_eq(left_val, right_val, &$tol),
        }
    }};
}

#[macro_export]
macro_rules! abs_diff_ne {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => $crate::AbsDiffCmp::ne(left_val, right_val, &$tol),
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => $crate::AbsDiffCmp::all_ne(left_val, right_val, &$tol),
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
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `abs_diff_eq!(left, right, ", stringify!($eq1), " <= t)`", r#"
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
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `abs_diff_eq!(left, right, ", stringify!($eq1), " <= t)`", r#"
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
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `abs_diff_ne!(left, right, ", stringify!($eq1), " <= t)`", r#"
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
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `abs_diff_ne!(left, right, ", stringify!($eq1), " <= t)`", r#"
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
