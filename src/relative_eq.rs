use core::cell;
use core::fmt;


#[inline]
pub fn relative_eq<A, B>(lhs: A, rhs: B, tolerance: &A::Tolerance, max_relative: &A::Tolerance) -> bool
where
    A: RelativeEq<B>,
{
    RelativeEq::relative_eq(&lhs, &rhs, tolerance, max_relative)
}

#[inline]
pub fn relative_ne<A, B>(lhs: A, rhs: B, tolerance: &A::Tolerance, max_relative: &A::Tolerance) -> bool 
where
    A: RelativeEq<B>
{
    RelativeEq::relative_ne(&lhs, &rhs, tolerance, max_relative)
}


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
    fn relative_eq(&self, other: &Rhs, tolerance: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool;

    /// Compare two floating point numbers for relative inequality.
    ///
    /// The relative inequality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two floating point numbers
    /// are relatively inequal with respect to a `maxRelative` multiple of the
    /// tolerance `tolerance`.
    fn relative_ne(&self, other: &Rhs, tolerance: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        !Self::relative_eq(self, other, tolerance, max_relative)
    }
}

impl RelativeEq for f32 {
    type Tolerance = f32;

    #[inline]
    fn relative_eq(&self, other: &Self, tolerance: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        // If `self` and `other` are finite and bitwise identical, They are relatively
        // equal. If `self` and `other` are infinite and bitwise identical, they are
        // the same kind of infinity, and therefore also equal.
        if self == other {
            return true;
        }

        // If `self` and `other` are finite, this clause does not apply. If one
        // of `self` and `other` is finite, and the other one is infinite, they
        // are not equal.
        if f32::is_infinite(*self) || f32::is_infinite(*other) {
            return false;
        }
        
        // Now check whether `self` and `other` are really close together.
        let abs_diff = f32::abs(self - other);
        if abs_diff <= *tolerance {
            return true;
        }

        // Finally, if the other cases have failed, we check their relative
        // absolute difference against the largest absolute value of `other` and
        // `self`.
        let abs_self = f32::abs(*self);
        let abs_other = f32::abs(*other);
        let largest = if abs_other > abs_self { 
            abs_other
        } else {
            abs_self
        };

        return abs_diff <= largest * max_relative
    }
}

impl RelativeEq for f64 {
    type Tolerance = f64;

    #[inline]
    fn relative_eq(&self, other: &Self, tolerance: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        // If `self` and `other` are finite and bitwise identical, They are relatively
        // equal. If `self` and `other` are infinite and bitwise identical, they are
        // the same kind of infinity, and therefore also equal.
        if self == other {
            return true;
        }

        // If `self` and `other` are finite, this clause does not apply. If one
        // of `self` and `other` is finite, and the other one is infinite, they
        // are not equal.
        if f64::is_infinite(*self) || f64::is_infinite(*other) {
            return false;
        }
        
        // Now check whether `self` and `other` are really close together.
        let abs_diff = f64::abs(self - other);
        if abs_diff <= *tolerance {
            return true;
        }

        // Finally, if the other cases have failed, we check their relative
        // absolute difference against the largest absolute value of `other` and
        // `self`.
        let abs_self = f64::abs(*self);
        let abs_other = f64::abs(*other);
        let largest = if abs_other > abs_self { 
            abs_other
        } else {
            abs_self
        };

        abs_diff <= largest * max_relative
    }
}

impl<A, B> RelativeEq<&B> for &A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&B, tolerance: &A::Tolerance, max_relative: &A::Tolerance) -> bool {
        A::relative_eq(*self, *other, tolerance, max_relative)
    }
}

impl<A, B> RelativeEq<&mut B> for &A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&mut B, tolerance: &A::Tolerance, max_relative: &A::Tolerance) -> bool {
        A::relative_eq(*self, *other, tolerance, max_relative)
    }
}

impl<A, B> RelativeEq<&B> for &mut A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&B, tolerance: &A::Tolerance, max_relative: &A::Tolerance) -> bool {
        A::relative_eq(*self, *other, tolerance, max_relative)
    }
}

impl<A, B> RelativeEq<&mut B> for &mut A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&mut B, tolerance: &A::Tolerance, max_relative: &A::Tolerance) -> bool {
        A::relative_eq(*self, *other, tolerance, max_relative)
    }
}

impl<A, B> RelativeEq<[B]> for [A]
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &[B], tolerance: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len() &&
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.relative_eq(b, tolerance, max_relative))
    }
}

impl<A, B, const N: usize> RelativeEq<[B; N]> for [A; N]
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &[B; N], tolerance: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.relative_eq(b, tolerance, max_relative))
    }
}

impl<T> RelativeEq for cell::Cell<T>
where
    T: RelativeEq + Copy
{
    type Tolerance = T::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &cell::Cell<T>, tolerance: &T::Tolerance, max_relative: &T::Tolerance) -> bool {
        T::relative_eq(&self.get(), &other.get(), tolerance, max_relative)
    }
}

impl<T> RelativeEq for cell::RefCell<T>
where
    T: RelativeEq + ?Sized
{
    type Tolerance = T::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &cell::RefCell<T>, tolerance: &T::Tolerance, max_relative: &T::Tolerance) -> bool {
        T::relative_eq(&self.borrow(), &other.borrow(), tolerance, max_relative)
    }
}

pub trait AssertRelativeEq<Rhs = Self>: RelativeEq<Rhs>
where
    Rhs: ?Sized
{
    type DebugTolerance: fmt::Debug;

    fn debug_relative_tolerance(&self, other: &Rhs, tolerance: &Self::Tolerance) -> Self::DebugTolerance;
}
/*
#[derive(Clone)]
pub struct Relative<A, B = A> 
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized
{
    pub max_abs_diff: A::Tolerance,
    pub max_relative: A::Tolerance,
}

impl<A, B> Default for Relative<A, B> 
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    #[inline]
    fn default() -> Self {
        Self {
            max_abs_diff: A::Tolerance::default_tolerance(),
            max_relative: A::Tolerance::default_tolerance(),
        }
    }
}

impl<A, B> Relative<A, B> 
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized
{
    #[inline]
    pub fn max_abs_diff(self, max_abs_diff: A::Tolerance) -> Self {
        Self {
            max_abs_diff,
            max_relative: self.max_relative,
        }
    }

    #[inline]
    pub fn max_relative(self, max_relative: A::Tolerance) -> Self {
        Self {
            max_abs_diff: self.max_abs_diff,
            max_relative,
        }
    }

    #[inline]
    pub fn eq(self, lhs: &A, rhs: &B) -> bool {
        A::relative_eq(lhs, rhs, &self.max_abs_diff, &self.max_relative)
    }

    #[inline]
    pub fn ne(self, lhs: &A, rhs: &B) -> bool {
        A::relative_ne(lhs, rhs, &self.max_abs_diff, &self.max_relative)
    }
}

#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative = $crate::Relative::default()$(.$opt($val))*;
                let result = relative.clone().eq(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_eq!({}, {}, {} = {}, {} = {})\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), relative.max_abs_diff,
                    stringify!(max_relative), relative.max_relative,
                    left_val, right_val,
                );
            }
        }
    }};
    ($left:expr, $right:expr, $(, $opt:ident = $val:expr)*, $($arg:tt)+) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative = $crate::Relative::default()$(.$opt($val))*;
                let result = relative.clone().eq(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_eq!({}, {}, {} = {}, {} = {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), relative.max_abs_diff,
                    stringify!(max_relative), relative.max_relative,
                    left_val, right_val,
                    format!($($arg)+),
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_relative_ne {
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative = $crate::Relative::default()$(.$opt($val))*;
                let result = relative.clone().ne(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_ne!({}, {}, {} = {}, {} = {})\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), relative.max_abs_diff,
                    stringify!(max_relative), relative.max_relative,
                    left_val, right_val,
                );
            }
        }
    }};
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)*, $($arg:tt)+) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative = $crate::Relative::default()$(.$opt($val))*;
                let result = relative.clone().ne(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_ne!({}, {}, {} = {}, {} = {})\n{}\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), relative.max_abs_diff,
                    stringify!(max_relative), relative.max_relative,
                    format!($($arg)+),
                    left_val, right_val,
                );
            }
        }
    }};
}
*/