use core::cell;

use crate::abs_diff_why_eq::{
    DefaultTolerance,
    AbsDiffWhyEq,
};


#[inline]
pub fn relative_why_eq<A, B>(lhs: A, rhs: B, tolerance: A::Tolerance, max_relative: A::Tolerance) -> (bool, A::Reason)
where
    A: RelativeWhyEq<B>,
{
    RelativeWhyEq::relative_why_eq(&lhs, &rhs, tolerance, max_relative)
}

#[inline]
pub fn relative_why_ne<A, B>(lhs: A, rhs: B, tolerance: A::Tolerance, max_relative: A::Tolerance) -> (bool, A::Reason)
where
    A: RelativeWhyEq<B>
{
    RelativeWhyEq::relative_why_ne(&lhs, &rhs, tolerance, max_relative)
}


#[derive(Clone)]
pub struct RelativeWhy<A, B = A> 
where
    A: RelativeWhyEq<B> + ?Sized,
    B: ?Sized
{
    pub max_abs_diff: A::Tolerance,
    pub max_relative: A::Tolerance,
    _marker: core::marker::PhantomData<B>,
}

impl<A, B> Default for RelativeWhy<A, B> 
where
    A: RelativeWhyEq<B> + ?Sized,
    B: ?Sized,
{
    #[inline]
    fn default() -> Self {
        Self {
            max_abs_diff: A::default_max_abs_diff(),
            max_relative: A::default_max_relative(),
            _marker: core::marker::PhantomData
        }
    }
}

impl<A, B> RelativeWhy<A, B> 
where
    A: RelativeWhyEq<B> + ?Sized,
    B: ?Sized
{
    #[inline]
    pub fn max_abs_diff(self, max_abs_diff: A::Tolerance) -> Self {
        Self {
            max_abs_diff,
            max_relative: self.max_relative,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn max_relative(self, max_relative: A::Tolerance) -> Self {
        Self {
            max_abs_diff: self.max_abs_diff,
            max_relative,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn eq(self, lhs: &A, rhs: &B) -> (bool, A::Reason) {
        A::relative_why_eq(lhs, rhs, self.max_abs_diff, self.max_relative)
    }

    #[inline]
    pub fn ne(self, lhs: &A, rhs: &B) -> (bool, A::Reason) {
        A::relative_why_ne(lhs, rhs, self.max_abs_diff, self.max_relative)
    }
}

#[macro_export]
macro_rules! assert_relative_why_eq {
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative_why = $crate::RelativeWhy::default()$(.$opt($val))*;
                let (result, reason) = relative_why.clone().eq(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_why_eq!({}, {}, {} = {}, {} = {})\nleft = {:?}\nright = {:?}",
                    stringify!($left),
                    stringify!($right),
                    stringify!(max_abs_diff), relative_why.max_abs_diff,
                    stringify!(max_relative), relative_why.max_relative,
                    left_val, right_val,
                );
            }
        }
    }};
    ($left:expr, $right:expr, $(, $opt:ident = $val:expr)*, $($arg:tt)+) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative_why = $crate::RelativeWhy::default()$(.$opt($val))*;
                let result = relative_why.clone().eq(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_why_eq!({}, {}, {} = {}, {} = {})\n{}\nleft = {:?}\nright = {:?}",
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

#[macro_export]
macro_rules! assert_relative_why_ne {
    ($left:expr, $right:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let relative_why = $crate::RelativeWhy::default()$(.$opt($val))*;
                let (result, reason) = relative_why.clone().ne(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_why_ne!({}, {}, {} = {}, {} = {})\nleft = {:?}\nright = {:?}",
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
                let relative_why = $crate::RelativeWhy::default()$(.$opt($val))*;
                let (result, reason) = relative_why.clone().ne(left_val, right_val);
                assert!(
                    result,
                    "assert_relative_why_ne!({}, {}, {} = {}, {} = {})\n{}\nleft = {:?}\nright = {:?}",
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

pub trait RelativeWhyEq<Rhs = Self>: DefaultTolerance
where
    Rhs: ?Sized
{
    type Reason;

    fn default_max_abs_diff() -> Self::Tolerance;

    /// The default maximum relative error multiplier when one is
    /// not specified at the time of comparison.
    fn default_max_relative() -> Self::Tolerance;

    /// Compare two floating point numbers for relative equality.
    ///
    /// The relative equality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two floating point numbers
    /// are relatively equal with respect to a `maxRelative` multiple of the
    /// tolerance `tolerance`.
    fn relative_why_eq(&self, other: &Rhs, tolerance: Self::Tolerance, max_relative: Self::Tolerance) -> (bool, Self::Reason);

    /// Compare two floating point numbers for relative inequality.
    ///
    /// The relative inequality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two floating point numbers
    /// are relatively inequal with respect to a `maxRelative` multiple of the
    /// tolerance `tolerance`.
    fn relative_why_ne(&self, other: &Rhs, tolerance: Self::Tolerance, max_relative: Self::Tolerance) -> (bool, Self::Reason) {
        let (result, reason) = Self::relative_why_eq(self, other, tolerance, max_relative);

        (!result, reason)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RelativeWhyReason {
    BitwiseIdentical,
    OneArgumentIsFiniteAndTheOtherIsInfinite,
    AbsDiffEqual,
    RelativeEqual,
}

impl RelativeWhyEq for f32 {
    type Reason = RelativeWhyReason;

    #[inline]
    fn default_max_abs_diff() -> Self::Tolerance {
        f32::default_tolerance()
    }

    #[inline]
    fn default_max_relative() -> Self::Tolerance {
        f32::default_tolerance()
    }

    #[inline]
    fn relative_why_eq(&self, other: &Self, tolerance: Self::Tolerance, max_relative: Self::Tolerance) -> (bool, Self::Reason) {
        // If `self` and `other` are finite and bitwise identical, They are relatively
        // equal. If `self` and `other` are infinite and bitwise identical, they are
        // the same kind of infinity, and therefore also equal.
        if self == other {
            return (true, RelativeWhyReason::BitwiseIdentical);
        }

        // If `self` and `other` are finite, this clause does not apply. If one
        // of `self` and `other` is finite, and the other one is infinite, they
        // are not equal.
        if f32::is_infinite(*self) || f32::is_infinite(*other) {
            return (false, RelativeWhyReason::OneArgumentIsFiniteAndTheOtherIsInfinite);
        }
        
        // Now check whether `self` and `other` are really close together.
        let abs_diff = f32::abs(self - other);
        if abs_diff <= tolerance {
            return (true, RelativeWhyReason::AbsDiffEqual);
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

        (abs_diff <= largest * max_relative, RelativeWhyReason::RelativeEqual)
    }
}

impl RelativeWhyEq for f64 {
    type Reason = RelativeWhyReason;

    #[inline]
    fn default_max_abs_diff() -> Self::Tolerance {
        f64::default_tolerance()
    }

    #[inline]
    fn default_max_relative() -> Self::Tolerance {
        f64::default_tolerance()
    }

    #[inline]
    fn relative_why_eq(&self, other: &Self, tolerance: Self::Tolerance, max_relative: Self::Tolerance) -> (bool, Self::Reason) {
        // If `self` and `other` are finite and bitwise identical, They are relatively
        // equal. If `self` and `other` are infinite and bitwise identical, they are
        // the same kind of infinity, and therefore also equal.
        if self == other {
            return (true, RelativeWhyReason::BitwiseIdentical);
        }

        // If `self` and `other` are finite, this clause does not apply. If one
        // of `self` and `other` is finite, and the other one is infinite, they
        // are not equal.
        if f64::is_infinite(*self) || f64::is_infinite(*other) {
            return (false, RelativeWhyReason::OneArgumentIsFiniteAndTheOtherIsInfinite);
        }
        
        // Now check whether `self` and `other` are really close together.
        let abs_diff = f64::abs(self - other);
        if abs_diff <= tolerance {
            return (true, RelativeWhyReason::AbsDiffEqual);
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

        (abs_diff <= largest * max_relative, RelativeWhyReason::RelativeEqual)
    }
}

impl<T> RelativeWhyEq for &T 
where
    T: RelativeWhyEq
{
    type Reason = T::Reason;

    #[inline]
    fn default_max_abs_diff() -> T::Tolerance {
        T::default_max_abs_diff()
    }

    #[inline]
    fn default_max_relative() -> T::Tolerance {
        T::default_max_relative()
    }

    #[inline]
    fn relative_why_eq(&self, other: &&T, tolerance: T::Tolerance, max_relative: T::Tolerance) -> (bool, Self::Reason) {
        T::relative_why_eq(*self, *other, tolerance, max_relative)
    }
}

impl<T> RelativeWhyEq for &mut T 
where
    T: RelativeWhyEq
{
    type Reason = T::Reason;

    #[inline]
    fn default_max_abs_diff() -> T::Tolerance {
        T::default_max_abs_diff()
    }

    #[inline]
    fn default_max_relative() -> T::Tolerance {
        T::default_max_relative()
    }

    #[inline]
    fn relative_why_eq(&self, other: &&mut T, tolerance: T::Tolerance, max_relative: T::Tolerance) -> (bool, Self::Reason) {
        T::relative_why_eq(*self, *other, tolerance, max_relative)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SliceReason<A, B> 
where
    A: RelativeWhyEq<B>,
    A::Reason: Clone,
{
    AllSequenceElementsMatch,
    SequenceLengthsNotEqual,
    ElementMismatch { reason: A::Reason },
}

impl<A, B> Clone for SliceReason<A, B> 
where
    A: RelativeWhyEq<B>,
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

impl<A, B> RelativeWhyEq<[B]> for [A]
where
    A: RelativeWhyEq<B>,
    A::Tolerance: Clone,
    A::Reason: Clone
{
    type Reason = SliceReason<A, B>;

    #[inline]
    fn default_max_abs_diff() -> A::Tolerance {
        A::default_max_abs_diff()
    }

    #[inline]
    fn default_max_relative() -> A::Tolerance {
        A:: default_max_relative()
    }

    #[inline]
    fn relative_why_eq(&self, other: &[B], tolerance: Self::Tolerance, max_relative: Self::Tolerance) -> (bool, Self::Reason) {
        if self.len() != other.len() {
            return (false, SliceReason::SequenceLengthsNotEqual);
        }
        
        for (a, b) in self.iter().zip(other.iter()) {
            let (result, reason) = A::relative_why_eq(a, b, tolerance.clone(), max_relative.clone());
            if !result {
                return (false, SliceReason::ElementMismatch { reason });
            }
        }

        return (true, SliceReason::AllSequenceElementsMatch)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArrayReason<A, B> 
where
    A: RelativeWhyEq<B>,
    A::Reason: Clone,
{
    AllSequenceElementsMatch,
    ElementMismatch { reason: A::Reason },
}

impl<A, B> Clone for ArrayReason<A, B> 
where
    A: RelativeWhyEq<B>,
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

impl<A, B, const N: usize> RelativeWhyEq<[B; N]> for [A; N]
where
    A: RelativeWhyEq<B>,
    A::Tolerance: Clone,
    A::Reason: Clone
{
    type Reason = ArrayReason<A, B>;

    #[inline]
    fn default_max_abs_diff() -> Self::Tolerance {
        A::default_max_abs_diff()
    }

    #[inline]
    fn default_max_relative() -> Self::Tolerance {
        A::default_tolerance()
    }

    #[inline]
    fn relative_why_eq(&self, other: &[B; N], tolerance: Self::Tolerance, max_relative: Self::Tolerance) -> (bool, Self::Reason) {
        for (a, b) in self.iter().zip(other.iter()) {
            let (result, reason) = A::relative_why_eq(a, b, tolerance.clone(), max_relative.clone());
            if !result {
                return (false, ArrayReason::ElementMismatch { reason });
            }
        }

        return (true, ArrayReason::AllSequenceElementsMatch)
    }
}

impl<T> RelativeWhyEq for cell::Cell<T>
where
    T: RelativeWhyEq + Copy
{
    type Reason = <T as RelativeWhyEq>::Reason;

    #[inline]
    fn default_max_abs_diff() -> Self::Tolerance {
        T::default_max_abs_diff()
    }

    #[inline]
    fn default_max_relative() -> Self::Tolerance {
        T::default_max_relative()
    }

    #[inline]
    fn relative_why_eq(&self, other: &cell::Cell<T>, tolerance: T::Tolerance, max_relative: T::Tolerance) -> (bool, Self::Reason) {
        T::relative_why_eq(&self.get(), &other.get(), tolerance, max_relative)
    }
}

impl<T> RelativeWhyEq for cell::RefCell<T>
where
    T: RelativeWhyEq + ?Sized
{
    type Reason = <T as RelativeWhyEq>::Reason;

    #[inline]
    fn default_max_abs_diff() -> Self::Tolerance {
        T::default_max_abs_diff()
    }

    #[inline]
    fn default_max_relative() -> T::Tolerance {
        T::default_max_relative()
    }

    #[inline]
    fn relative_why_eq(&self, other: &cell::RefCell<T>, tolerance: T::Tolerance, max_relative: T::Tolerance) -> (bool, Self::Reason) {
        T::relative_why_eq(&self.borrow(), &other.borrow(), tolerance, max_relative)
    }
}

