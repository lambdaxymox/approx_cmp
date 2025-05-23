use core::fmt;

/// Compare two sequences of finite precision floating point numbers using
/// per entry relative difference tolerances.
///
/// Types implement this trait to utilize the [`relative_eq`] and [`relative_ne`]
/// macros.
///
/// More precisely, let `A` be a finite set of values, let `T` be a floating
/// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of finite
/// precision floating point numbers. Let `max_relative :: A -> T` be a sequence
/// of finite precision floating point numbers such that
/// ```text
/// forall a :: A. max_relative[a] >= 0
/// ```
/// We say that `u` is **relative equal** to `v` with tolerance `max_relative`
/// provided that
/// ```text
/// forall a :: A. abs(u[a] - v[a]) <= max(abs(u[a]), abs(v[a])) * max_relative[a]
/// ```
/// The trait implementations for [`f32`] and [`f64`] provided perform an absolute
/// difference comparison before the relative difference comparison. Relative
/// comparisons are not generally meaningful for values near zero.
///
/// # Examples (Floating Point Number Comparisons)
///
/// ```
/// # use relative_cmp::{
/// #     relative_eq,
/// #     relative_ne,
/// #     RelativeEq,
/// # };
/// #
/// let lhs = 11.0_f32;
/// let rhs = 11.000105_f32;
/// let max_abs_diff = 0.0_f32;
/// let max_relative1 = 0.000020_f32;
/// let max_relative2 = 0.000008_f32;
///
/// assert!(lhs.relative_eq(&rhs, &max_abs_diff, &max_relative1));
/// assert!(lhs.relative_ne(&rhs, &max_abs_diff, &max_relative2));
///
/// // Using the [`relative_eq`] macro.
/// assert!(relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative1));
///
/// // Using the [`relative_ne`] macro.
/// assert!(relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative2));
/// ```
///
/// # Examples (Floating Point Number Comparisons Near Zero)
///
/// ```
/// # use relative_cmp::{
/// #     relative_eq,
/// #     relative_ne,
/// #     RelativeEq,
/// # };
/// #
/// let lhs = -0.0000001_f32;
/// let rhs = 0.0000001_f32;
///
/// assert!(relative_eq!(lhs, rhs, abs_diff <= 2e-7_f32, relative <= 1e-6_f32));
///
/// // Relative comparisons are meaningless when `lhs` and `rhs` are near zero.
/// assert!(relative_ne!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 1.0_f32));
/// assert!(relative_eq!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 2.0_f32));
/// ```
///
/// # Examples (Floating Point Sequence Comparisons)
///
/// ```
/// # use relative_cmp::{
/// #     relative_eq,
/// #     relative_ne,
/// #     RelativeEq,
/// # };
/// #
/// let lhs = [1_f32, 2_f32, 3_f32, 4_f32];
/// let rhs = [1.0001195_f32, 2.0002390_f32, 3.0003585, 4.0004780_f32];
/// let max_abs_diff = [0.0_f32; 4];
/// let max_relative1 = [0.0002_f32, 0.0003_f32, 0.0004_f32, 0.0005_f32];
/// let max_relative2 = [0.0001_f32, 0.0002_f32, 0.0003_f32, 0.0004_f32];
///
/// assert!(lhs.relative_eq(&rhs, &max_abs_diff, &max_relative1));
/// assert!(lhs.relative_ne(&rhs, &max_abs_diff, &max_relative2));
///
/// // Using the [`relative_eq`] macro.
/// assert!(relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative1));
///
/// // Using the [`relative_ne`] macro.
/// assert!(relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative2));
/// ```
pub trait RelativeEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The data type representing the maximum allowed relative difference
    /// between two values for them to be considered approximately equal.
    type Tolerance: ?Sized;

    /// Compare two sequences of finite precision floating point numbers for relative
    /// equality.
    ///
    /// Returns a boolean indicating whether two sequences of floating
    /// point numbers are relatively equal with respect to an absolute difference
    /// tolerance `max_abs_diff` for values close to zero, and a relative tolerance
    /// `max_relative` otherwise.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of finite
    /// precision floating point numbers. Let `max_relative :: A -> T` be a sequence
    /// of finite precision floating point numbers such that
    /// ```text
    /// forall a :: A. max_relative[a] >= 0
    /// ```
    /// We say that `u` is **relative equal** to `v` with tolerance `max_relative`
    /// provided that
    /// ```text
    /// forall a :: A. abs(u[a] - v[a]) <= max(abs(u[a]), abs(v[a])) * max_relative[a]
    /// ```
    ///
    /// An implementation of [`RelativeEq::relative_eq`] should be equivalent to
    /// ```
    /// # trait TestRelativeEq {
    /// #     fn relative_eq(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool;
    /// #
    /// #     fn relative_ne(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool {
    /// #         !Self::relative_eq(self, other, max_abs_diff, max_relative)
    /// #     }
    /// # }
    /// #
    /// # impl TestRelativeEq for f32 {
    /// #     fn relative_eq(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool {
    /// self == other
    ///     || { Self::abs(self - other) <= *max_abs_diff }
    ///     || {
    ///         let largest = Self::max(Self::abs(*self), Self::abs(*other));
    ///         Self::abs(self - other) <= largest * (*max_relative)
    ///     }
    /// #     }
    /// # }
    /// ```
    /// where `self == other` handles comparisons of special values, the absolute
    /// difference clause handles values near zero, and the last clause is the
    /// relative difference comparison.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::{
    /// #     relative_eq,
    /// #     RelativeEq,
    /// # };
    /// #
    /// let lhs = 7.9995_f32;
    /// let rhs = 8.0_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_relative = 0.00008_f32;
    ///
    /// assert!(lhs.relative_eq(&rhs, &max_abs_diff, &max_relative));
    ///
    /// assert!(relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative));
    /// ```
    fn relative_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool;

    /// Compare two floating point numbers for relative inequality.
    ///
    /// Returns a boolean indicating whether two sequences of floating
    /// point numbers are relatively unequal with respect to an absolute difference
    /// tolerance `max_abs_diff` for values close to zero, and a relative tolerance
    /// `max_relative` otherwise.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `u :: A -> T` and `v :: A -> T` be sequences of finite
    /// precision floating point numbers. Let `max_relative :: A -> T` be a sequence
    /// of finite precision floating point numbers such that
    /// ```text
    /// forall a :: A. max_relative[a] >= 0
    /// ```
    /// We say that `u` is **relative unequal** to `v` with tolerance `max_relative`
    /// provided that
    /// ```text
    /// forall a :: A. abs(u[a] - v[a]) > max(abs(u[a]), abs(v[a])) * max_relative[a]
    /// ```
    ///
    /// An implementation of [`RelativeEq::relative_ne`] should be equivalent to
    /// ```
    /// # trait TestRelativeEq {
    /// #     fn relative_eq(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool { false }
    /// #
    /// #     fn relative_ne(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool;
    /// # }
    /// #
    /// # impl TestRelativeEq for f32 {
    /// #     fn relative_ne(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool {
    /// !Self::relative_eq(self, other, max_abs_diff, max_relative)
    /// #     }
    /// # }
    /// ```
    /// and should not be implemented directly in general.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::{
    /// #     relative_ne,
    /// #     RelativeEq,
    /// # };
    /// #
    /// let lhs = 7.9995_f32;
    /// let rhs = 8.0_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_relative = 0.00006_f32;
    ///
    /// assert!(lhs.relative_ne(&rhs, &max_abs_diff, &max_relative));
    ///
    /// assert!(relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative));
    /// ```
    fn relative_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        !Self::relative_eq(self, other, max_abs_diff, max_relative)
    }
}

/// Compare two sequences of finite precision floating point numbers for
/// relative equality using a uniform tolerance value.
///
/// Types implement this trait to utilize the [`relative_eq`] and [`relative_ne`]
/// macros using a single relative tolerance value.
///
/// More precisely, let `A` be a finite set of values, let `T` be a floating
/// point data type, and let `u :: A -> T` and `v :: A -> T` be sequences of
/// finite precision floating point numbers. Let `max_relative :: T` be a finite
/// precision floating point number such that `max_relative >= 0`. We say that
/// `u` is **relative equal** to `v` with tolerance `max_relative` provided that
/// ```text
/// forall a :: A. abs(u[a] - v[a]) <= max(abs(u[a]), abs(v[a])) * max_relative
/// ```
///
/// # Examples (Floating Point Number Comparisons)
///
/// ```
/// # use relative_cmp::{
/// #     relative_eq,
/// #     relative_ne,
/// #     RelativeAllEq,
/// # };
/// #
/// let lhs = 11.0_f32;
/// let rhs = 11.000105_f32;
/// let max_abs_diff = 0.0_f32;
/// let max_relative1 = 0.000010_f32;
/// let max_relative2 = 0.000005_f32;
///
/// assert!(lhs.relative_all_eq(&rhs, &max_abs_diff, &max_relative1));
/// assert!(lhs.relative_all_ne(&rhs, &max_abs_diff, &max_relative2));
///
/// // Using the [`relative_eq`] macro with `all` parameters.
/// assert!(relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative1));
///
/// // Using the [`relative_ne`] macro with `all` parameters.
/// assert!(relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative2));
/// ```
///
/// # Examples (Floating Point Number Sequence Comparisons)
///
/// ```
/// # use relative_cmp::{
/// #     relative_eq,
/// #     relative_ne,
/// #     RelativeAllEq,
/// # };
/// #
/// let lhs = [1_f32, 2_f32, 3_f32, 4_f32];
/// let rhs = [1.0001195_f32, 2.0002390_f32, 3.0003585, 4.0004780_f32];
/// let max_abs_diff = 0.0_f32;
/// let max_relative1 = 0.0002_f32;
/// let max_relative2 = 0.0001_f32;
///
/// assert!(lhs.relative_all_eq(&rhs, &max_abs_diff, &max_relative1));
/// assert!(lhs.relative_all_ne(&rhs, &max_abs_diff, &max_relative2));
///
/// // Using the [`relative_eq`] macro with `all` parameters.
/// assert!(relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative1));
///
/// // Using the [`relative_ne`] macro with `all` parameters.
/// assert!(relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative2));
/// ```
pub trait RelativeAllEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The data type representing the uniform maximum allowed relative
    /// difference between every entry of two values to be considered
    /// approximately equal.
    type AllTolerance: ?Sized;

    /// Compare two sequences of floating point numbers for relative equality
    /// using a single uniform tolerance value.
    ///
    /// Returns a boolean indicating whether two sequences of floating
    /// point numbers are relatively equal with respect to a uniform absolute
    /// difference tolerance `max_abs_diff` for values close to zero, and a
    /// uniform relative tolerance `max_relative` for values not close to zero.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, and let `u :: A -> T` and `v :: A -> T` be sequences of
    /// finite precision floating point numbers. Let `max_relative :: T` be a finite
    /// precision floating point number such that `max_relative >= 0`. We say that
    /// `u` is **relative equal** to `v` with tolerance `max_relative` provided that
    /// ```text
    /// forall a :: A. abs(u[a] - v[a]) <= max(abs(u[a]), abs(v[a])) * max_relative
    /// ```
    ///
    /// An implementation of [`RelativeAllEq::relative_all_eq`] must use the same
    /// algorithm as [`RelativeEq::relative_eq`].
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::{
    /// #     relative_eq,
    /// #     RelativeAllEq,
    /// # };
    /// #
    /// let lhs = 1.0_f32;
    /// let rhs = 1.0001115_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_relative = 0.0002_f32;
    ///
    /// assert!(lhs.relative_all_eq(&rhs, &max_abs_diff, &max_relative));
    ///
    /// assert!(relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative));
    /// ```
    fn relative_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool;

    /// Compare two sequences of floating point numbers for relative inequality
    /// using a single uniform tolerance value.
    ///
    /// Returns a boolean indicating whether two sequences of floating
    /// point numbers are relatively unequal with respect to a uniform absolute
    /// difference tolerance `max_abs_diff` for values close to zero, and a
    /// uniform relative tolerance `max_relative` for values not close to zero.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, and let `u :: A -> T` and `v :: A -> T` be sequences of
    /// finite precision floating point numbers. Let `max_relative :: T` be a finite
    /// precision floating point number such that `max_relative >= 0`. We say that
    /// `u` is **relative unequal** to `v` with tolerance `max_relative` provided that
    /// ```text
    /// forall a :: A. abs(u[a] - v[a]) > max(abs(u[a]), abs(v[a])) * max_relative
    /// ```
    ///
    /// An implementation of [`RelativeAllEq::relative_all_ne`] should be
    /// equivalent to
    /// ```
    /// # trait TestRelativeAllEq {
    /// #     fn relative_all_eq(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool { false }
    /// #
    /// #     fn relative_all_ne(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool;
    /// # }
    /// #
    /// # impl TestRelativeAllEq for f32 {
    /// #     fn relative_all_ne(&self, other: &Self, max_abs_diff: &Self, max_relative: &Self) -> bool {
    /// !Self::relative_all_eq(self, other, max_abs_diff, max_relative)
    /// #     }
    /// # }
    /// ```
    /// and should not be implemented directly in general.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::{
    /// #     relative_ne,
    /// #     RelativeAllEq,
    /// # };
    /// #
    /// let lhs = 1.0_f32;
    /// let rhs = 1.0001115_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_relative = 0.0001_f32;
    ///
    /// assert!(lhs.relative_all_ne(&rhs, &max_abs_diff, &max_relative));
    ///
    /// assert!(relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative));
    /// ```
    fn relative_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        !Self::relative_all_eq(self, other, max_abs_diff, max_relative)
    }
}

/// Provide a debugging context for when a relative difference comparison fails.
///
/// Types implement this trait to use the [`assert_relative_eq`] and [`assert_relative_ne`]
/// macros.
pub trait AssertRelativeEq<Rhs = Self>: RelativeEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The absolute difference between two values in a debugging context. This
    /// is used to display results via [`fmt::Debug`].
    type DebugAbsDiff: fmt::Debug + Sized;

    /// The value of the tolerance used for comparing two values in a debugging
    /// context. This is used to display results via [`fmt::Debug`].
    type DebugTolerance: fmt::Debug;

    /// Compute the absolute difference between two values in a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::AssertRelativeEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let expected = (1.001999899999987_f64, 0.99499893_f32);
    /// let result = lhs.debug_abs_diff(&rhs);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    /// Compute the maximum allowed absolute difference between two values for a
    /// debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::AssertRelativeEq;
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

    /// Compute the maximum allowed relative difference between two values for a
    /// debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::AssertRelativeEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let max_relative = (0.004_f64, 0.02_f32);
    /// let expected = (1.2400079996_f64, 1.1798999_f32);
    /// let result = lhs.debug_relative_tolerance(&rhs, &max_relative);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn debug_relative_tolerance(&self, other: &Rhs, max_relative: &Self::Tolerance) -> Self::DebugTolerance;
}

/// Provides a debugging context for when a relative difference comparison using
/// an `all` comparison fails.
///
/// Types implement this trait to use the [`assert_relative_eq`] and [`assert_relative_ne`]
/// macros with `all` parameters.
pub trait AssertRelativeAllEq<Rhs = Self>: RelativeAllEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The data type representing the uniform absolute difference and the
    /// uniform relative difference between two values for them to be considered
    /// approximately equal that can be displayed in a debugging context.
    type AllDebugTolerance: fmt::Debug;

    /// Compute the value of the maximum allowed uniform absolute difference
    /// between two values for a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::AssertRelativeAllEq;
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

    /// Compute the value of the maximum allowed uniform relative difference
    /// between two values for a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use relative_cmp::AssertRelativeAllEq;
    /// #
    /// let lhs = [1.0_f32; 4];
    /// let rhs = [2.0_f32; 4];
    /// let max_relative = 0.0003_f32;
    /// let expected = [0.0006_f32; 4];
    /// let result = lhs.debug_relative_all_tolerance(&rhs, &max_relative);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn debug_relative_all_tolerance(&self, other: &Rhs, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance;
}

#[doc(hidden)]
pub struct RelativeCmp {}

impl RelativeCmp {
    #[must_use]
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_relative: &A::Tolerance) -> bool
    where
        A: RelativeEq<B> + ?Sized,
        B: ?Sized,
    {
        A::relative_eq(lhs, rhs, max_abs_diff, max_relative)
    }

    #[must_use]
    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_relative: &A::Tolerance) -> bool
    where
        A: RelativeEq<B> + ?Sized,
        B: ?Sized,
    {
        A::relative_ne(lhs, rhs, max_abs_diff, max_relative)
    }

    #[must_use]
    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_relative: &A::AllTolerance) -> bool
    where
        A: RelativeAllEq<B> + ?Sized,
        B: ?Sized,
    {
        A::relative_all_eq(lhs, rhs, max_abs_diff, max_relative)
    }

    #[must_use]
    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_relative: &A::AllTolerance) -> bool
    where
        A: RelativeAllEq<B> + ?Sized,
        B: ?Sized,
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
        A: RelativeEq<B> + AssertRelativeEq<B>,
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance
    where
        A: RelativeAllEq<B> + AssertRelativeAllEq<B>,
    {
        A::debug_abs_diff_all_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn relative<A, B>(lhs: &A, rhs: &B, max_relative: &A::Tolerance) -> A::DebugTolerance
    where
        A: RelativeEq<B> + AssertRelativeEq<B>,
    {
        A::debug_relative_tolerance(lhs, rhs, max_relative)
    }

    #[inline]
    pub fn relative_all<A, B>(lhs: &A, rhs: &B, max_relative: &A::AllTolerance) -> A::AllDebugTolerance
    where
        A: RelativeAllEq<B> + AssertRelativeAllEq<B>,
    {
        A::debug_relative_all_tolerance(lhs, rhs, max_relative)
    }
}

/// Compare two finite precision floating point expression for relative
/// difference equality.
///
/// For more details, see the documentation for [`RelativeEq`] and [`RelativeAllEq`].
///
/// # Example
///
/// ```
/// # use relative_cmp::relative_eq;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
///
/// assert!(relative_eq!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 6e-6_f32));
/// assert!(relative_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 6e-6_f32));
/// ```
#[macro_export]
macro_rules! relative_eq {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, relative <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, relative <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, relative_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, relative_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
}

/// Compare two finite precision floating point expression for relative
/// difference inequality.
///
/// For more details, see the documentation for [`RelativeEq`] and [`RelativeAllEq`].
///
/// # Example
///
/// ```
/// # use relative_cmp::relative_ne;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
///
/// assert!(relative_ne!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 2e-6_f32));
/// assert!(relative_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 2e-6_f32));
/// ```
#[macro_export]
macro_rules! relative_ne {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, relative <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, relative <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, relative_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
    ($left:expr, $right:expr, relative_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => $crate::RelativeCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val),
        }
    }};
}

/// Assert that two finite precision floating point expressions are relative
/// difference equal.
///
/// See the documentation for [`RelativeEq`] and [`RelativeAllEq`] for details
/// about relative difference comparisons. See the documentation for
/// [`AssertRelativeEq`] and [`AssertRelativeAllEq`] for details about the
/// debugging context provided when an assertion fails.
///
/// # Example
///
/// ```
/// # use relative_cmp::assert_relative_eq;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
///
/// assert_relative_eq!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 6e-6_f32);
/// assert_relative_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 6e-6_f32);
/// ```
#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `relative_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `relative_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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

/// Assert that two finite precision floating point expressions are relative
/// difference unequal.
///
/// See the documentation for [`RelativeEq`] and [`RelativeAllEq`] for details
/// about relative difference comparisons. See the documentation for
/// [`AssertRelativeEq`] and [`AssertRelativeAllEq`] for details about the
/// debugging context provided when an assertion fails.
///
/// # Example
///
/// ```
/// # use relative_cmp::assert_relative_ne;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
///
/// assert_relative_ne!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 2e-6_f32);
/// assert_relative_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 2e-6_f32);
/// ```
#[macro_export]
macro_rules! assert_relative_ne {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `relative_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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
                    // noticeable slow down. See the documentation for `core::assert_eq`.
                    panic!(concat!(
"assertion failed: `relative_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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

/// Assert that two finite precision floating point expressions are relative
/// difference equal.
///
/// See the documentation for [`RelativeEq`] and [`RelativeAllEq`] for details
/// about relative difference comparisons. See the documentation for
/// [`AssertRelativeEq`] and [`AssertRelativeAllEq`] for details about the
/// debugging context provided when an assertion fails.
///
/// This macro is only enabled in debug builds like [`debug_assert_eq`] in the
/// standard library.
///
/// # Example
///
/// ```
/// # use relative_cmp::debug_assert_relative_eq;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
///
/// debug_assert_relative_eq!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 6e-6_f32);
/// debug_assert_relative_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 6e-6_f32);
/// ```
#[macro_export]
macro_rules! debug_assert_relative_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_relative_eq!($($arg)*); })
}

/// Assert that two finite precision floating point expressions are relative
/// difference equal.
///
/// See the documentation for [`RelativeEq`] and [`RelativeAllEq`] for details
/// about relative difference comparisons. See the documentation for
/// [`AssertRelativeEq`] and [`AssertRelativeAllEq`] for details about the
/// debugging context provided when an assertion fails.
///
/// This macro is only enabled in debug builds like [`debug_assert_ne`] in the
/// standard library.
///
/// # Example
///
/// ```
/// # use relative_cmp::debug_assert_relative_ne;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
///
/// debug_assert_relative_ne!(lhs, rhs, abs_diff <= 0.0_f32, relative <= 2e-6_f32);
/// debug_assert_relative_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, relative_all <= 2e-6_f32);
/// ```
#[macro_export]
macro_rules! debug_assert_relative_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_relative_ne!($($arg)*); })
}
