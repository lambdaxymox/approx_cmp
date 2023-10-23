use core::cell;


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

    #[inline]
    fn abs_diff_why_ne(&self, other: &Rhs, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        let (result, reason) = Self::abs_diff_why_eq(self, other, tolerance);

        (!result, reason)
    }
}

macro_rules! impl_abs_diff_why_eq_unsigned {
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
        }
    )*}
}

impl_abs_diff_why_eq_unsigned!(
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
}

impl<T> AbsDiffWhyEq for cell::Cell<T> 
where
    T: AbsDiffWhyEq + Copy
{
    type Tolerance = T::Tolerance;
    type Reason = T::Reason;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &cell::Cell<T>, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        T::abs_diff_why_eq(&self.get(), &other.get(), tolerance)
    }
}

impl<T> AbsDiffWhyEq for cell::RefCell<T> 
where
    T: AbsDiffWhyEq + ?Sized
{
    type Tolerance = T::Tolerance;
    type Reason = T::Reason;

    #[inline]
    fn default_tolerance() -> Self::Tolerance {
        T::default_tolerance()
    }

    #[inline]
    fn abs_diff_why_eq(&self, other: &cell::RefCell<T>, tolerance: Self::Tolerance) -> (bool, Self::Reason) {
        T::abs_diff_why_eq(&self.borrow(), &other.borrow(), tolerance)
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
}

