use core::cell;


#[inline]
pub fn abs_diff_eq<A, B>(lhs: A, rhs: B, tolerance: A::Tolerance) -> bool 
where
    A: AbsDiffEq<B>
{
    AbsDiffEq::abs_diff_eq(&lhs, &rhs, tolerance)
}

#[inline]
pub fn abs_diff_ne<A, B>(lhs: A, rhs: B, tolerance: A::Tolerance) -> bool 
where
    A: AbsDiffEq<B>
{
    AbsDiffEq::abs_diff_ne(&lhs, &rhs, tolerance)
}


#[derive(Clone)]
pub struct AbsDiff<A, B = A>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized
{
    /// The tolerance to use when testing values that are close together.
    pub max_abs_diff: A::Tolerance,
}

impl<A, B> Default for AbsDiff<A, B>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized
{
    #[inline]
    fn default() -> AbsDiff<A, B> {
        AbsDiff {
            max_abs_diff: A::default_tolerance(),
        }
    }
}

impl<A, B> AbsDiff<A, B>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized
{
    #[inline]
    pub fn max_abs_diff(self, max_abs_diff: A::Tolerance) -> AbsDiff<A, B> {
        AbsDiff { max_abs_diff }
    }

    #[inline]
    pub fn eq(self, lhs: &A, rhs: &B) -> bool {
        A::abs_diff_eq(lhs, rhs, self.max_abs_diff)
    }

    #[inline]
    pub fn ne(self, lhs: &A, rhs: &B) -> bool {
        A::abs_diff_ne(lhs, rhs, self.max_abs_diff)
    }
}

#[macro_export]
macro_rules! assert_abs_diff_eq {
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let abs_diff = $crate::AbsDiff::default()$(.$opt($val))*;
                let result = abs_diff.clone().eq(left_val, right_val);
                assert!(
                    result,
                    "assert_abs_diff_eq!({}, {}, {} = {})\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), abs_diff.max_abs_diff,
                    left_val, right_val,
                );
            }
        }
    }};
    ($left:expr, $right:expr, $(, $opt:ident = $val:expr)*, $($arg:tt)+) => {{
        match (&($left), &($right)) {
            (result, expected) => {
                assert!(
                    let abs_diff = $crate::AbsDiff::default()$(.$opt($val))*;
                    let result = abs_diff.clone().eq(left_val, right_val);
                    "assert_abs_diff_eq!({}, {}, {} = {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!($max_abs_diff), abs_diff.max_abs_diff,
                    format!($($arg)+),
                    left_val, right_val,
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let abs_diff = $crate::AbsDiff::default()$(.$opt($val))*;
                let result = abs_diff.clone().ne(left_val, right_val);
                assert!(
                    result,
                    "assert_abs_diff_ne!({}, {}, {} = {})\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), abs_diff.max_abs_diff,
                    left_val, right_val,
                );
            }
        }
    }};
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let abs_diff = $crate::AbsDiff::default()$(.$opt($val))*;
                let result = abs_diff.clone().ne(left_val, right_val);
                assert!(
                    result,
                    "assert_abs_diff_ne!({}, {}, {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), abs_diff.max_abs_diff,
                    stringify!($($arg)+),
                    left_val, right_val,
                );
            }
        }
    }};
}

pub trait AbsDiffEq<Rhs = Self>: PartialEq<Rhs> 
where
    Rhs: ?Sized
{
    type Tolerance: Clone;

    /// The default tolerance for absolute difference comparisons when a
    /// tolerance is not specified at the time of comparison.
    fn default_tolerance() -> Self::Tolerance;

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
    fn abs_diff_eq(&self, other: &Rhs, tolerance: Self::Tolerance) -> bool;

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
        impl AbsDiffEq for $T {
            type Tolerance = $T;

            #[inline]
            fn default_tolerance() -> Self::Tolerance {
                $default_tolerance
            }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, tolerance: Self::Tolerance) -> bool {
                let abs_diff = if self > other { 
                    self - other
                } else {
                    other - self
                };

                abs_diff <= tolerance
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
        impl AbsDiffEq for $T {
            type Tolerance = $T;

            #[inline]
            fn default_tolerance() -> Self::Tolerance {
                $default_tolerance
            }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, tolerance: Self::Tolerance) -> bool {
                $T::abs(self - other) <= tolerance
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
    (f32, (), f32::EPSILON),
    (f64, (), f64::EPSILON),
);


impl<T> AbsDiffEq for &T
where
    T: AbsDiffEq
{
    type Tolerance = T::Tolerance;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&T, tolerance: Self::Tolerance) -> bool {
        T::abs_diff_eq(self, other, tolerance)
    }
}

impl<T> AbsDiffEq for &mut T
where
    T: AbsDiffEq
{
    type Tolerance = T::Tolerance;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&mut T, tolerance: Self::Tolerance) -> bool {
        T::abs_diff_eq(self, other, tolerance)
    }
}

impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Clone,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        A::default_tolerance()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B], tolerance: Self::Tolerance) -> bool {
        if self.len() != other.len() {
            return false;
        }
        
        for (a, b) in self.iter().zip(other.iter()) {
            if !A::abs_diff_eq(a, b, tolerance.clone()) {
                return false;
            }
        }

        true
    }
}

impl<A, B, const N: usize> AbsDiffEq<[B; N]> for [A; N]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Clone,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        A::default_tolerance()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B; N], tolerance: Self::Tolerance) -> bool {
        for (a, b) in self.iter().zip(other.iter()) {
            if !A::abs_diff_eq(a, b, tolerance.clone()) {
                return false;
            }
        }

        true
    }
}

impl<T> AbsDiffEq for cell::Cell<T> 
where
    T: AbsDiffEq + Copy
{
    type Tolerance = T::Tolerance;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::Cell<T>, tolerance: Self::Tolerance) -> bool {
        T::abs_diff_eq(&self.get(), &other.get(), tolerance)
    }
}

impl<T> AbsDiffEq for cell::RefCell<T> 
where
    T: AbsDiffEq + ?Sized
{
    type Tolerance = T::Tolerance;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::RefCell<T>, tolerance: Self::Tolerance) -> bool {
        T::abs_diff_eq(&self.borrow(), &other.borrow(), tolerance)
    }
}

