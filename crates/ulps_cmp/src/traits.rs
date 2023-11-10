use core::fmt;


/// Compare two sequences of finite precision floating point numbers using
/// per entry units in last place (ulps) difference tolerances.
///
/// Types implement this trait to utilize the [`ulps_eq`] and [`ulps_ne`]
/// macros.
///
/// More precisely, let `A` be a finite set of values, let `T` be a floating
/// point data type, let `U` be an integer data type, and let `int :: T -> U` be a
/// monotone bijection between values of type `T` and values of type `U`, i.e.
/// `size_of(T) == `size_of(U)`. Let `u :: A -> T` and `v :: A -> T` be sequences
/// of floating point numbers. Let `max_ulps :: A -> U` be a sequence of integers
/// such that
/// ```text
/// forall a :: A. max_ulps[a] >= 0
/// ```
/// We say that `u` is **ulps equal** to `v` (or `u` is
/// **units in last place equal** to `v`) with tolerance `max_ulps` provided that
/// ```text
/// forall a :: A. abs(int(u[a]) - int(v[a])) <= max_ulps[a]
/// ```
/// Here, a bijection means that every floating point number in `T` corresponds
/// to exactly one integer in `U`, and exactly one integer in `U` corresponds to
/// exactly one floating point number in `T`. Typically this means that we interpret
/// a floating point number as its underlying integer type. Monotone means that
/// ```text
/// forall x :: T. forall y :: T. x <= y ==> int(x) <= int(y)
/// ```
/// so numbers ordered as floating point numbers correspond to their integer
/// representations, meaning that ulps comparisons are well-defined for `T` as the
/// number of representable numbers between two floating point numbers.
///
/// The trait implementations for [`f32`] and [`f64`] provided perform an absolute
/// difference comparison before the ulps difference comparison. Like relative
/// comparisons, ulps comparisons are not generally meaningful for values near zero.
///
/// # Examples (Floating Point Number Comparisons)
///
/// ```
/// # use ulps_cmp::{
/// #     ulps_eq,
/// #     ulps_ne,
/// #     UlpsEq,
/// # };
/// #
/// let lhs = 11.0_f32;
/// let rhs = 11.000105_f32;
/// let max_abs_diff = 0.0_f32;
/// let max_ulps1 = 120_u32;
/// let max_ulps2 = 100_u32;
///
/// assert!(lhs.ulps_eq(&rhs, &max_abs_diff, &max_ulps1));
/// assert!(lhs.ulps_ne(&rhs, &max_abs_diff, &max_ulps2));
///
/// // Using the [`ulps_eq`] macro.
/// assert!(ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps1));
///
/// // Using the [`ulps_ne`] macro.
/// assert!(ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps2));
/// ```
///
/// # Examples (Floating Point Number Comparisons Near Zero)
///
/// ```
/// # use ulps_cmp::{
/// #     ulps_eq,
/// #     ulps_ne,
/// #     UlpsEq,
/// # };
/// #
/// let lhs = 0.00002_f32;
/// let rhs = 0.00001_f32;
///
/// assert!(ulps_eq!(lhs, rhs, abs_diff <= 2e-5_f32, ulps <= 4_u32));
///
/// // Ulps comparisons are meaningless when `lhs` and `rhs` are near zero.
/// assert!(ulps_ne!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 8388607_u32));
/// assert!(ulps_eq!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 8388608_u32));
/// ```
///
/// # Examples (Floating Point Sequence Comparisons)
///
/// ```
/// # use ulps_cmp::{
/// #     ulps_eq,
/// #     ulps_ne,
/// #     UlpsEq,
/// # };
/// #
/// let lhs = [1_f32, 2_f32, 3_f32, 4_f32];
/// let rhs = [1.0001195_f32, 2.0002390_f32, 3.0003585, 4.0004780_f32];
/// let max_abs_diff = [0.0_f32; 4];
/// let max_ulps1 = [1002_u32, 1002_u32, 1504_u32, 1002_u32];
/// let max_ulps2 = [1000_u32, 1000_u32, 1500_u32, 1000_u32];
///
/// assert!(lhs.ulps_eq(&rhs, &max_abs_diff, &max_ulps1));
/// assert!(lhs.ulps_ne(&rhs, &max_abs_diff, &max_ulps2));
///
/// // Using the [`ulps_eq`] macro.
/// assert!(ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps1));
///
/// // Using the [`ulps_ne`] macro.
/// assert!(ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps2));
/// ```
pub trait UlpsEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The data type representing the maximum allowed absolute difference
    /// between two values for them to be considered approximately equal.
    type Tolerance: ?Sized;

    /// The data type representing the maximum allowed ulps difference
    /// between two values for them to be considered approximately equal.
    type UlpsTolerance: ?Sized;

    /// Compare two sequences of finite precision floating point numbers for
    /// units in last place (ulps) equality.
    ///
    /// Returns a boolean indicating whether or not two sequences of floating point
    /// numbers are ulps equal with respect to an absolute difference tolerance
    /// `max_abs_diff` for values close to zero, and an ulps tolerance `max_ulps`
    /// otherwise.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `U` be an integer data type, and let `int :: T -> U` be a
    /// monotone bijection between values of type `T` and values of type `U`, i.e.
    /// `size_of(T) == `size_of(U)`. Let `u :: A -> T` and `v :: A -> T` be sequences
    /// of floating point numbers. Let `max_ulps :: A -> U` be a sequence of integers
    /// such that
    /// ```text
    /// forall a :: A. max_ulps[a] >= 0
    /// ```
    /// We say that `u` is **ulps equal** to `v` (or `u` is
    /// **units in last place equal** to `v`) with tolerance `max_ulps` provided that
    /// ```text
    /// forall a :: A. abs(int(u[a]) - int(v[a])) <= max_ulps[a]
    /// ```
    /// Here, a bijection means that every floating point number in `T` corresponds
    /// to exactly one integer in `U`, and exactly one integer in `U` corresponds to
    /// exactly one floating point number in `T`. Typically this means that we interpret
    /// a floating point number as its underlying integer type. Monotone means that
    /// ```text
    /// forall x :: T. forall y :: T. x <= y ==> int(x) <= int(y)
    /// ```
    /// so numbers ordered as floating point numbers correspond to their integer
    /// representations, meaning that ulps comparisons are well-defined for `T` as the
    /// number of representable numbers between two floating point numbers.
    ///
    /// An implementation of [`UlpsEq::ulps_eq`] should be equivalent to
    /// ```
    /// # trait TestUlpsEq {
    /// #     type Tolerance;
    /// #     type UlpsTolerance;
    /// #
    /// #     fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool;
    /// #
    /// #     fn ulps_ne(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
    /// #         !Self::ulps_eq(self, other, max_abs_diff, max_ulps)
    /// #     }
    /// # }
    /// #
    /// # impl TestUlpsEq for f32 {
    /// #     type Tolerance = f32;
    /// #     type UlpsTolerance = u32;
    /// #
    /// #     fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
    /// self == other
    ///     || { Self::abs(self - other) <= *max_abs_diff }
    ///     || {
    ///         let bits_self = self.to_bits();
    ///         let bits_other = other.to_bits();
    ///         let max = Self::UlpsTolerance::max(bits_self, bits_other);
    ///         let min = Self::UlpsTolerance::min(bits_self, bits_other);
    ///
    ///         (max - min) <= *max_ulps
    ///     }
    /// #     }
    /// # }
    /// ```
    /// where `self == other` handles comparisons of special values, the absolute
    /// difference clause handles values near zero, and the last clause is the
    /// ulps difference comparison.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::{
    /// #     ulps_eq,
    /// #     UlpsEq,
    /// # };
    /// #
    /// let lhs = 7.9995_f32;
    /// let rhs = 8.0_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_ulps = 1100_u32;
    ///
    /// assert!(lhs.ulps_eq(&rhs, &max_abs_diff, &max_ulps));
    ///
    /// assert!(ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps));
    /// ```
    fn ulps_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool;

    /// Compare two sequences of finite precision floating point numbers for
    /// units in last place (ulps) inequality.
    ///
    /// Returns a boolean indicating whether or not two sequences of floating point
    /// numbers are ulps equal with respect to an absolute difference tolerance
    /// `max_abs_diff` for values close to zero, and an ulps tolerance `max_ulps`
    /// otherwise.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `U` be an integer data type, and let `int :: T -> U` be a
    /// monotone bijection between values of type `T` and values of type `U`, i.e.
    /// `size_of(T) == `size_of(U)`. Let `u :: A -> T` and `v :: A -> T` be sequences
    /// of floating point numbers. Let `max_ulps :: A -> U` be a sequence of integers
    /// such that
    /// ```text
    /// forall a :: A. max_ulps[a] >= 0
    /// ```
    /// We say that `u` is **ulps unequal** to `v` (or `u` is
    /// **units in last place unequal** to `v`) with tolerance `max_ulps` provided that
    /// ```text
    /// forall a :: A. abs(int(u[a]) - int(v[a])) > max_ulps[a]
    /// ```
    /// Here, a bijection means that every floating point number in `T` corresponds
    /// to exactly one integer in `U`, and exactly one integer in `U` corresponds to
    /// exactly one floating point number in `T`. Typically this means that we interpret
    /// a floating point number as its underlying integer type. Monotone means that
    /// ```text
    /// forall x :: T. forall y :: T. x <= y ==> int(x) <= int(y)
    /// ```
    /// so numbers ordered as floating point numbers correspond to their integer
    /// representations, meaning that ulps comparisons are well-defined for `T` as the
    /// number of representable numbers between two floating point numbers.
    ///
    /// An implementation of [`UlpsEq::ulps_ne`] should be equivalent to
    /// ```
    /// # trait TestUlpsEq {
    /// #     type Tolerance;
    /// #     type UlpsTolerance;
    /// #
    /// #     fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool { false }
    /// #
    /// #     fn ulps_ne(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool;
    /// # }
    /// #
    /// # impl TestUlpsEq for f32 {
    /// #     type Tolerance = f32;
    /// #     type UlpsTolerance = u32;
    /// #
    /// #     fn ulps_ne(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
    /// !Self::ulps_eq(self, other, max_abs_diff, max_ulps)
    /// #     }
    /// # }
    /// ```
    /// and should not be implemented directly in general.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::{
    /// #     ulps_ne,
    /// #     UlpsEq,
    /// # };
    /// #
    /// let lhs = 7.9995_f32;
    /// let rhs = 8.0_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_ulps = 1000_u32;
    ///
    /// assert!(lhs.ulps_ne(&rhs, &max_abs_diff, &max_ulps));
    ///
    /// assert!(ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps));
    /// ```
    fn ulps_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        !Self::ulps_eq(self, other, max_abs_diff, max_ulps)
    }
}

/// Compare two sequences of finite precision floating point numbers for
/// units in last place (ulps) difference equality using a uniform tolerance
/// value.
///
/// Types implement this trait to utilize the [`ulps_eq`] and [`ulps_ne`]
/// macros using a single ulps tolerance value.
///
/// More precisely, let `A` be a finite set of values, let `T` be a floating
/// point data type, let `U` be an integer data type, and let `int :: T -> U` be a
/// monotone bijection between values of type `T` and values of type `U`, i.e.
/// `size_of(T) == `size_of(U)`. Let `u :: A -> T` and `v :: A -> T` be sequences
/// of floating point numbers. Let `max_ulps :: U` be an integer such that
/// `max_ulps >= 0`. We say that `u` is **ulps equal** to `v` (or `u` is
/// **units in last place equal** to `v`) with tolerance `max_ulps` provided that
/// ```text
/// forall a :: A. abs(int(u[a]) - int(v[a])) <= max_ulps
/// ```
/// Here, a bijection means that every floating point number in `T` corresponds
/// to exactly one integer in `U`, and exactly one integer in `U` corresponds to
/// exactly one floating point number in `T`. Typically this means that we interpret
/// a floating point number as its underlying integer type. Monotone means that
/// ```text
/// forall x :: T. forall y :: T. x <= y ==> int(x) <= int(y)
/// ```
/// so numbers ordered as floating point numbers correspond to their integer
/// representations, meaning that ulps comparisons are well-defined for `T` as the
/// number of representable numbers between two floating point numbers.
///
/// The trait implementations for [`f32`] and [`f64`] provided perform an absolute
/// difference comparison before the ulps difference comparison. Like relative
/// comparisons, ulps comparisons are not generally meaningful for values near zero.
///
/// # Examples (Floating Point Number Comparisons)
///
/// ```
/// # use ulps_cmp::{
/// #     ulps_eq,
/// #     ulps_ne,
/// #     UlpsAllEq,
/// # };
/// #
/// let lhs = 11.0_f32;
/// let rhs = 11.000105_f32;
/// let max_abs_diff = 0.0_f32;
/// let max_ulps1 = 120_u32;
/// let max_ulps2 = 100_u32;
///
/// assert!(lhs.ulps_all_eq(&rhs, &max_abs_diff, &max_ulps1));
/// assert!(lhs.ulps_all_ne(&rhs, &max_abs_diff, &max_ulps2));
///
/// // Using the [`ulps_eq`] macro with `all` parameters.
/// assert!(ulps_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps1));
///
/// // Using the [`ulps_ne`] macro with `all` parameters.
/// assert!(ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps2));
/// ```
///
/// # Examples (Floating Point Number Sequence Comparisons)
///
/// ```
/// # use ulps_cmp::{
/// #     ulps_eq,
/// #     ulps_ne,
/// #     UlpsAllEq,
/// # };
/// #
/// let lhs = [1_f32, 2_f32, 3_f32, 4_f32];
/// let rhs = [1.0001195_f32, 2.0002390_f32, 3.0003585, 4.0004780_f32];
/// let max_abs_diff = 0.0_f32;
/// let max_ulps1 = 1600_u32;
/// let max_ulps2 = 1000_u32;
///
/// assert!(lhs.ulps_all_eq(&rhs, &max_abs_diff, &max_ulps1));
/// assert!(lhs.ulps_all_ne(&rhs, &max_abs_diff, &max_ulps2));
///
/// // Using the [`ulps_eq`] macro with `all` parameters.
/// assert!(ulps_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps1));
///
/// // Using the [`ulps_ne`] macro with `all` parameters.
/// assert!(ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps2));
/// ```
pub trait UlpsAllEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The data type representing the uniform maximum allowed absolute
    /// difference between every entry of two values to be considered
    /// approximately equal.
    type AllTolerance: ?Sized;
    /// The data type representing the uniform maximum allowed ulps
    /// difference between every entry of two values to be considered
    /// approximately equal.
    type AllUlpsTolerance: ?Sized;

    /// Compare two sequences of finite precision floating point numbers for
    /// units in last place (ulps) difference equality using a uniform tolerance
    /// value.
    ///
    /// Returns a boolean indicating whether or not two sequences of floating
    /// point numbers are ulps difference equal with respect to a uniform absolute
    /// difference tolerance `max_abs_diff` for values close to zero, and a
    /// uniform ulps tolerance `max_ulps` for values not close to zero.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `U` be an integer data type, and let `int :: T -> U` be a
    /// monotone bijection between values of type `T` and values of type `U`, i.e.
    /// `size_of(T) == `size_of(U)`. Let `u :: A -> T` and `v :: A -> T` be sequences
    /// of floating point numbers. Let `max_ulps :: U` be an integer such that
    /// `max_ulps >= 0`. We say that `u` is **ulps equal** to `v` (or `u` is
    /// **units in last place equal** to `v`) with tolerance `max_ulps` provided that
    /// ```text
    /// forall a :: A. abs(int(u[a]) - int(v[a])) <= max_ulps
    /// ```
    /// Here, a bijection means that every floating point number in `T` corresponds
    /// to exactly one integer in `U`, and exactly one integer in `U` corresponds to
    /// exactly one floating point number in `T`. Typically this means that we interpret
    /// a floating point number as its underlying integer type. Monotone means that
    /// ```text
    /// forall x :: T. forall y :: T. x <= y ==> int(x) <= int(y)
    /// ```
    /// so numbers ordered as floating point numbers correspond to their integer
    /// representations, meaning that ulps comparisons are well-defined for `T` as the
    /// number of representable numbers between two floating point numbers.
    ///
    /// An implementation of [`UlpsAllEq::ulps_all_eq`] must use the same
    /// algorithm as [`UlpsEq::ulps_eq`].
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::{
    /// #     ulps_eq,
    /// #     UlpsAllEq,
    /// # };
    /// #
    /// let lhs = 1.0_f32;
    /// let rhs = 1.0001115_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_ulps = 1000_u32;
    ///
    /// assert!(lhs.ulps_all_eq(&rhs, &max_abs_diff, &max_ulps));
    ///
    /// assert!(ulps_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
    /// ```
    fn ulps_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool;

    /// Compare two sequences of finite precision floating point numbers for
    /// units in last place (ulps) difference inequality using a uniform tolerance
    /// value.
    ///
    /// Returns a boolean indicating whether or not two sequences of floating
    /// point numbers are ulps difference unequal with respect to a uniform absolute
    /// difference tolerance `max_abs_diff` for values close to zero, and a
    /// uniform ulps tolerance `max_ulps` for values not close to zero.
    ///
    /// More precisely, let `A` be a finite set of values, let `T` be a floating
    /// point data type, let `U` be an integer data type, and let `int :: T -> U` be a
    /// monotone bijection between values of type `T` and values of type `U`, i.e.
    /// `size_of(T) == `size_of(U)`. Let `u :: A -> T` and `v :: A -> T` be sequences
    /// of floating point numbers. Let `max_ulps :: U` be an integer such that
    /// `max_ulps >= 0`. We say that `u` is **ulps unequal** to `v` (or `u` is
    /// **units in last place unequal** to `v`) with tolerance `max_ulps` provided that
    /// ```text
    /// forall a :: A. abs(int(u[a]) - int(v[a])) > max_ulps
    /// ```
    /// Here, a bijection means that every floating point number in `T` corresponds
    /// to exactly one integer in `U`, and exactly one integer in `U` corresponds to
    /// exactly one floating point number in `T`. Typically this means that we interpret
    /// a floating point number as its underlying integer type. Monotone means that
    /// ```text
    /// forall x :: T. forall y :: T. x <= y ==> int(x) <= int(y)
    /// ```
    /// so numbers ordered as floating point numbers correspond to their integer
    /// representations, meaning that ulps comparisons are well-defined for `T` as the
    /// number of representable numbers between two floating point numbers.
    ///
    /// An implementation of [`UlpsAllEq::ulps_all_ne`] should be equivalent to
    /// ```
    /// # trait TestUlpsAllEq {
    /// #     fn ulps_all_eq(&self, other: &Self, max_abs_diff: &Self, max_ulps: &Self) -> bool { false }
    /// #
    /// #     fn ulps_all_ne(&self, other: &Self, max_abs_diff: &Self, max_ulps: &Self) -> bool;
    /// # }
    /// #
    /// # impl TestUlpsAllEq for f32 {
    /// #     fn ulps_all_ne(&self, other: &Self, max_abs_diff: &Self, max_ulps: &Self) -> bool {
    /// !Self::ulps_all_eq(self, other, max_abs_diff, max_ulps)
    /// #     }
    /// # }
    /// ```
    /// and should not be implemented directly in general.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::{
    /// #     ulps_ne,
    /// #     UlpsAllEq,
    /// # };
    /// #
    /// let lhs = 1.0_f32;
    /// let rhs = 1.0001115_f32;
    /// let max_abs_diff = 0.0_f32;
    /// let max_ulps = 900_u32;
    ///
    /// assert!(lhs.ulps_all_ne(&rhs, &max_abs_diff, &max_ulps));
    ///
    /// assert!(ulps_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
    /// ```
    fn ulps_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        !Self::ulps_all_eq(self, other, max_abs_diff, max_ulps)
    }
}

/// Procide a debugging context for when an ulps difference comparison fails.
///
/// Types implement this trait to use the [`assert_ulps_eq`] and [`assert_ulps_ne`]
/// macros.
pub trait AssertUlpsEq<Rhs = Self>: UlpsEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The absolute difference between two values in a debugging context. This
    /// is used to display results via [`fmt::Debug`].
    type DebugAbsDiff: fmt::Debug + Sized;

    /// The ulps difference between two values in a debugging context. This
    /// is used to display results via [`fmt::Debug`].
    type DebugUlpsDiff: fmt::Debug + Sized;

    /// The value of the absolute difference tolerance used for comparing two
    /// values in a debugging context. This is used to display results via
    /// [`fmt::Debug`].
    type DebugTolerance: fmt::Debug;

    /// The value of the ulps difference tolerance used for comparing two
    /// values in a debugging context. This is used to display results via
    /// [`fmt::Debug`].
    type DebugUlpsTolerance: fmt::Debug;

    /// Compute the absolute difference between two values in a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::AssertUlpsEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let expected = (1.001999899999987_f64, 0.99499893_f32);
    /// let result = lhs.debug_abs_diff(&rhs);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    /// Compute the ulps difference between two values in a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::AssertUlpsEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let expected = (17627368657286_u64, 260833_u32);
    /// let result = lhs.debug_ulps_diff(&rhs);
    /// ```
    fn debug_ulps_diff(&self, other: &Rhs) -> Self::DebugUlpsDiff;

    /// Compute the maximum allowed absolute difference between two values for a
    /// debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::AssertUlpsEq;
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

    /// Compute the maximum allowed ulps difference between two values for a
    /// debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::AssertUlpsEq;
    /// #
    /// let lhs = (309.0_f64, 58.0_f32);
    /// let rhs = (310.0019999_f64, 58.995_f32);
    /// let max_ulps = (500_u64, 5_u32);
    /// let expected = max_ulps;
    /// let result = lhs.debug_ulps_tolerance(&rhs, &max_ulps);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn debug_ulps_tolerance(&self, other: &Rhs, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance;
}

/// Provide a debugging context for when an ulps difference comparison fails.
///
/// Types implement this trait to use the [`assert_ulps_eq`] and [`assert_ulps_ne`]
/// macros with `all` parameters.
pub trait AssertUlpsAllEq<Rhs = Self>: UlpsAllEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The data type representing the uniform absolute difference between two
    /// values for them to be considered approximately equal that can be
    /// displayed in a debugging context.
    type AllDebugTolerance: fmt::Debug;

    /// The data type representing the uniform ulps difference between two
    /// values for them to be considered approximately equal that can be
    /// displayed in a debugging context.
    type AllDebugUlpsTolerance: fmt::Debug;

    /// Compute the value of the maximum allowed uniform absolute difference
    /// between two values for a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::AssertUlpsAllEq;
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

    /// Compute the value of the maximum allowed uniform ulps difference
    /// between two values for a debugging context.
    ///
    /// # Example
    ///
    /// ```
    /// # use ulps_cmp::AssertUlpsAllEq;
    /// #
    /// let lhs = [1.0_f32; 4];
    /// let rhs = [2.0_f32; 4];
    /// let max_ulps = 50_u32;
    /// let expected = [max_ulps; 4];
    /// let result = lhs.debug_ulps_all_tolerance(&rhs, &max_ulps);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn debug_ulps_all_tolerance(&self, other: &Rhs, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance;
}


#[doc(hidden)]
pub struct UlpsCmp {}

impl UlpsCmp {
    #[must_use]
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_ulps: &A::UlpsTolerance) -> bool
    where
        A: UlpsEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_eq(lhs, rhs, max_abs_diff, max_ulps)
    }

    #[must_use]
    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_ulps: &A::UlpsTolerance) -> bool
    where
        A: UlpsEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_ne(lhs, rhs, max_abs_diff, max_ulps)
    }

    #[must_use]
    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_ulps: &A::AllUlpsTolerance) -> bool
    where
        A: UlpsAllEq<B> + ?Sized,
        B: ?Sized,
    {
        A::ulps_all_eq(lhs, rhs, max_abs_diff, max_ulps)
    }

    #[must_use]
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

/// Compare two finite precision floating point expression for ulps
/// difference equality.
/// 
/// For more details, see the documentation for [`UlpsEq`] and [`UlpsAllEq`].
/// 
/// # Example
/// 
/// ```
/// # use ulps_cmp::ulps_eq;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
/// 
/// assert!(ulps_eq!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 60_u32));
/// assert!(ulps_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 60_u32));
/// ```
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

/// Compare two finite precision floating point expression for ulps
/// difference inequality.
/// 
/// For more details, see the documentation for [`UlpsEq`] and [`UlpsAllEq`].
/// 
/// # Example
/// 
/// ```
/// # use ulps_cmp::ulps_ne;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
/// 
/// assert!(ulps_ne!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 40_u32));
/// assert!(ulps_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 40_u32));
/// ```
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

/// Assert that two finite precision floating point expressions are ulps
/// difference equal.
/// 
/// See the documentation for [`UlpsEq`] and [`UlpsAllEq`] for details about
/// ulps difference comparisons. See the documentation for [`AssertUlpsEq`] 
/// and [`AssertUlpsAllEq`] for details about the debugging context provided
/// when an assertion fails.
/// 
/// # Example
/// 
/// ```
/// # use ulps_cmp::assert_ulps_eq;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
/// 
/// assert_ulps_eq!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 60_u32);
/// assert_ulps_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 60_u32);
/// ```
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
"assertion failed: `ulps_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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
"assertion failed: `ulps_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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

/// Assert that two finite precision floating point expressions are ulps
/// difference inequal.
/// 
/// See the documentation for [`UlpsEq`] and [`UlpsAllEq`] for details about
/// ulps difference comparisons. See the documentation for [`AssertUlpsEq`] 
/// and [`AssertUlpsAllEq`] for details about the debugging context provided
/// when an assertion fails.
/// 
/// # Example
/// 
/// ```
/// # use ulps_cmp::assert_ulps_ne;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
/// 
/// assert_ulps_ne!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 40_u32);
/// assert_ulps_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 40_u32);
/// ```
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
"assertion failed: `ulps_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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
"assertion failed: `ulps_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
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

/// Assert that two finite precision floating point expressions are ulps
/// difference equal.
/// 
/// See the documentation for [`UlpsEq`] and [`UlpsAllEq`] for details about
/// ulps difference comparisons. See the documentation for [`AssertUlpsEq`] 
/// and [`AssertUlpsAllEq`] for details about the debugging context provided
/// when an assertion fails.
/// 
/// This macro is only enable in debug builds like [`debug_assert_eq`] in the
/// standard library.
/// 
/// # Example
/// 
/// ```
/// # use ulps_cmp::debug_assert_ulps_eq;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
/// 
/// debug_assert_ulps_eq!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 60_u32);
/// debug_assert_ulps_eq!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 60_u32);
/// ```
#[macro_export]
macro_rules! debug_assert_ulps_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ulps_eq!($($arg)*); })
}

/// Assert that two finite precision floating point expressions are ulps
/// difference equal.
/// 
/// See the documentation for [`UlpsEq`] and [`UlpsAllEq`] for details about
/// ulps difference comparisons. See the documentation for [`AssertUlpsEq`] 
/// and [`AssertUlpsAllEq`] for details about the debugging context provided
/// when an assertion fails.
/// 
/// This macro is only enable in debug builds like [`debug_assert_ne`] in the
/// standard library.
/// 
/// # Example
/// 
/// ```
/// # use ulps_cmp::debug_assert_ulps_ne;
/// #
/// let lhs = 98.0005_f32;
/// let rhs = 98.0001_f32;
/// 
/// debug_assert_ulps_ne!(lhs, rhs, abs_diff <= 0.0_f32, ulps <= 40_u32);
/// debug_assert_ulps_ne!(lhs, rhs, abs_diff_all <= 0.0_f32, ulps_all <= 40_u32);
/// ```
#[macro_export]
macro_rules! debug_assert_ulps_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ulps_ne!($($arg)*); })
}
