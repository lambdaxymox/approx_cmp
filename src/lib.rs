use core::cell;


#[inline]
pub fn abs_diff_eq<A, B>(lhs: A, rhs: B, tolerance: A::Tolerance) -> bool 
where
    A: AbsDiffWhyEq<B>
{
    AbsDiffWhyEq::abs_diff_eq(&lhs, &rhs, tolerance)
}

#[inline]
pub fn abs_diff_ne<A, B>(lhs: A, rhs: B, tolerance: A::Tolerance) -> bool 
where
    A: AbsDiffWhyEq<B>
{
    AbsDiffWhyEq::abs_diff_ne(&lhs, &rhs, tolerance)
}

#[inline]
pub fn abs_diff_eq_default<A, B>(lhs: A, rhs: B) -> bool
where
    A: AbsDiffWhyEq<B>
{
    AbsDiffWhyEq::abs_diff_eq(&lhs, &rhs, A::default_tolerance())
}

#[inline]
pub fn abs_diff_ne_default<A, B>(lhs: A, rhs: B) -> bool
where
    A: AbsDiffWhyEq<B>
{
    AbsDiffWhyEq::abs_diff_ne(&lhs, &rhs, A::default_tolerance())
}


#[macro_export]
macro_rules! assert_abs_diff_eq {
    ($result:expr, $expected:expr, max_abs_diff = $max_abs_diff:expr $(,)?) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_eq(result, expected, $max_abs_diff),
                    "assert_abs_diff_eq!({}, {}, {})\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    stringify!($max_abs_diff = $max_abs_diff),
                    result, expected,
                );
            }
        }
    }};
    ($result:expr, $expected:expr, max_abs_diff = $max_abs_diff:expr, $($arg:tt)+) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_eq(result, expected, $max_abs_diff),
                    "assert_abs_diff_eq!({}, {}, {})\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    stringify!($max_abs_diff = $max_abs_diff),
                    stringify!($($arg)+),
                    result, expected,
                );
            }
        }
    }};
    ($result:expr, $expected:expr $(,)?) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_eq_default(result, expected),
                    "assert_abs_diff_eq!({}, {})\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    result, expected,
                );
            }
        }
    }};
    ($result:expr, $expected:expr, $($arg:tt)+) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_eq_default(result, expected),
                    "assert_abs_diff_eq!({}, {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    stringify!($($arg)+),
                    result, expected,
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($result:expr, $expected:expr, max_abs_diff = $max_abs_diff:expr $(,)?) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_ne(result, expected, $max_abs_diff),
                    "assert_abs_diff_ne!({}, {}, {})\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    stringify!($max_abs_diff = $max_abs_diff),
                    result, expected,
                );
            }
        }
    }};
    ($result:expr, $expected:expr, max_abs_diff = $max_abs_diff:expr, $($arg:tt)+) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_ne(result, expected, $max_abs_diff),
                    "assert_abs_diff_ne!({}, {}, {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    stringify!($max_abs_diff = $max_abs_diff),
                    stringify!($($arg)+),
                    result, expected,
                );
            }
        }
    }};
    ($result:expr, $expected:expr $(,)?) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_ne_default(result, expected),
                    "assert_abs_diff_ne!({}, {})\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    result, expected,
                );
            }
        }
    }};
    ($result:expr, $expected:expr, $($arg:tt)+) => {{
        match (&($result), &($expected)) {
            (result, expected) => {
                assert!(
                    $crate::abs_diff_ne_default(result, expected),
                    "assert_abs_diff_ne!({}, {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($result),
                    stringify!($expected),
                    stringify!($($arg)+),
                    result, expected,
                );
            }
        }
    }};
}


pub trait AbsDiffWhyEq<Rhs = Self>: PartialEq<Rhs> 
where
    Rhs: ?Sized
{
    type Tolerance;
    type Reason: Sized + Clone;

    /// The default tolerance for absolute difference comparisons when a
    /// tolerance is not specified at the time of comparison.
    fn default_tolerance() -> Self::Tolerance;

    fn abs_diff_why_eq(&self, other: &Rhs, tolerance: Self::Tolerance) -> (bool, Self::Reason);

    fn abs_diff_why_ne(&self, other: &Rhs, tolerance: Self::Tolerance) -> (bool, Self::Reason);

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
    fn abs_diff_eq(&self, other: &Rhs, tolerance: Self::Tolerance) -> bool {
        Self::abs_diff_why_eq(self, other, tolerance).0
    }

    /// Compare two floating point numbers for absolute difference inequality.
    ///
    /// Two floating point numbers are approximately inequal within tolerance
    /// `tolerance` provided that they are not approximately equal within tolerance
    /// `tolerance`.
    ///
    /// - Returns: A boolean indicating whether or not two floating point
    /// numbers are absolute difference inequal with respect to a tolerance
    /// `tolerance`.
    fn abs_diff_ne(&self, other: &Rhs, tolerance: Self::Tolerance) -> bool {
        !Self::abs_diff_eq(self, other, tolerance)
    }
}

macro_rules! impl_abs_diff_eq_unsigned {
    ($(($T:ident, $ReasonType:ty, $default_tolerance:expr)),* $(,)?) => {$(
        impl AbsDiffWhyEq for $T {
            type Tolerance = $T;
            type Reason = $ReasonType;

            #[inline]
            fn default_tolerance() -> Self::Tolerance {
                $default_tolerance
            }

            #[inline]
            fn abs_diff_why_eq(&self, other: &$T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
                let abs_diff = if self > other { 
                    self - other
                } else {
                    other - self
                };

                (abs_diff <= tolerance, ())
            }

            #[inline]
            fn abs_diff_why_ne(&self, other: &$T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
                let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

                (!result, reason)
            }
        }
    )*}
}

impl_abs_diff_eq_unsigned!(
    (u8, (), 0),
    (u16, (), 0),
    (u32, (), 0),
    (u64, (), 0),
    (u128, (), 0),
    (usize, (), 0),
);

macro_rules! impl_abs_diff_eq_signed {
    ($(($T:ident, $ReasonType:ty, $default_tolerance:expr)),* $(,)?) => {$(
        impl AbsDiffWhyEq for $T {
            type Tolerance = $T;
            type Reason = $ReasonType;

            #[inline]
            fn default_tolerance() -> Self::Tolerance {
                $default_tolerance
            }

            #[inline]
            fn abs_diff_why_eq(&self, other: &$T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
                ($T::abs(self - other) <= tolerance, ())
            }

            #[inline]
            fn abs_diff_why_ne(&self, other: &$T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
                let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

                (!result, reason)
            }
        }
    )*};
}

impl_abs_diff_eq_signed!(
    (i8, (), 0),
    (i16, (), 0),
    (i32, (), 0),
    (i64, (), 0),
    (i128, (), 0),
    (isize, (), 0),
);

impl AbsDiffWhyEq for f32 {
    type Tolerance = f32;
    type Reason = ();

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &f32, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        (f32::abs(self - other) <= tolerance, ())
    }

    #[inline]
    fn abs_diff_why_ne(&self, other: &f32, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

        (!result, reason)
    }
}

impl AbsDiffWhyEq for f64 {
    type Tolerance = f64;
    type Reason = ();

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        f64::EPSILON
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &f64, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        (f64::abs(self - other) <= tolerance, ())
    }

    #[inline]
    fn abs_diff_why_ne(&self, other: &f64, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

        (!result, reason)
    }
}

impl<T> AbsDiffWhyEq for &T
where
    T: AbsDiffWhyEq
{
    type Tolerance = T::Tolerance;
    type Reason = T::Reason;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &&T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        T::abs_diff_why_eq(self, other, tolerance)
    }

    #[inline]
    fn abs_diff_why_ne(&self, other: &&T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        T::abs_diff_why_ne(self, other, tolerance)
    }
}

impl<T> AbsDiffWhyEq for &mut T
where
    T: AbsDiffWhyEq
{
    type Tolerance = T::Tolerance;
    type Reason = T::Reason;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &&mut T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        T::abs_diff_why_eq(self, other, tolerance)
    }

    #[inline]
    fn abs_diff_why_ne(&self, other: &&mut T, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        T::abs_diff_why_ne(self, other, tolerance)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum SliceReason<A, B> 
where
    A: AbsDiffWhyEq<B>,
    A::Reason: Clone,
{
    AllSequenceElementsMatch,
    SequenceLengthsNotEqual,
    ElementMismatch { reason: A::Reason },
}

impl<A, B> Clone for SliceReason<A, B> 
where
    A: AbsDiffWhyEq<B>,
    A::Reason: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::AllSequenceElementsMatch => Self::AllSequenceElementsMatch,
            Self::SequenceLengthsNotEqual => Self::SequenceLengthsNotEqual,
            Self::ElementMismatch { reason } => {
                Self::ElementMismatch { reason: reason.clone() }
            }
        }
    }
}

impl<A, B> AbsDiffWhyEq<[B]> for [A]
where
    A: AbsDiffWhyEq<B>,
    A::Tolerance: Clone,
    A::Reason: Clone,
{
    type Tolerance = A::Tolerance;
    type Reason = SliceReason<A, B>;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        A::default_tolerance()
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &[B], tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        if self.len() != other.len() {
            return (false, SliceReason::SequenceLengthsNotEqual);
        }
        
        for (a, b) in self.iter().zip(other.iter()) {
            let (result, reason) = A::abs_diff_why_eq(a, b, tolerance.clone());
            if !result {
                return (false, SliceReason::ElementMismatch { reason });
            }
        }

        return (true, SliceReason::AllSequenceElementsMatch)
    }

    #[inline]
    fn abs_diff_why_ne(&self, other: &[B], tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

        (!result, reason)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArrayReason<A, B> 
where
    A: AbsDiffWhyEq<B>,
    A::Reason: Clone,
{
    AllSequenceElementsMatch,
    ElementMismatch { reason: A::Reason },
}

impl<A, B> Clone for ArrayReason<A, B> 
where
    A: AbsDiffWhyEq<B>,
    A::Reason: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::AllSequenceElementsMatch => Self::AllSequenceElementsMatch,
            Self::ElementMismatch { reason } => {
                Self::ElementMismatch { reason: reason.clone() }
            }
        }
    }
}


impl<A, B, const N: usize> AbsDiffWhyEq<[B; N]> for [A; N]
where
    A: AbsDiffWhyEq<B>,
    A::Tolerance: Clone,
    A::Reason: Clone,
{
    type Tolerance = A::Tolerance;
    type Reason = ArrayReason<A, B>;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        A::default_tolerance()
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &[B; N], tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        for (a, b) in self.iter().zip(other.iter()) {
            let (result, reason) = A::abs_diff_why_eq(a, b, tolerance.clone());
            if !result {
                return (false, ArrayReason::ElementMismatch { reason });
            }
        }

        return (true, ArrayReason::AllSequenceElementsMatch)
    }

    #[inline]
    fn abs_diff_why_ne(&self, other: &[B; N], tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

        (!result, reason)
    }
}

