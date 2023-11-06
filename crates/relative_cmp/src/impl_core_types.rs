use crate::traits::{
    AssertRelativeAllEq,
    AssertRelativeEq,
    RelativeAllEq,
    RelativeEq,
};

use core::cell;
use core::mem;


#[inline(always)]
fn uninit_array<T, const N: usize>() -> [mem::MaybeUninit<T>; N] {
    unsafe { mem::MaybeUninit::<[mem::MaybeUninit<T>; N]>::uninit().assume_init() }
}

#[inline(always)]
unsafe fn array_assume_init<T, const N: usize>(array: [mem::MaybeUninit<T>; N]) -> [T; N] {
    (&array as *const _ as *const [T; N]).read()
}

macro_rules! impl_relative_eq_float {
    ($($T:ident),* $(,)?) => {$(
        impl RelativeEq for $T {
            type Tolerance = $T;

            #[inline]
            fn relative_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
                // If `self` and `other` are finite and bitwise identical, They are relatively
                // equal. If `self` and `other` are infinite and bitwise identical, they are
                // the same kind of infinity, and therefore also equal.
                if self == other {
                    return true;
                }

                // If `self` and `other` are finite, this clause does not apply. If one
                // of `self` and `other` is finite, and the other one is infinite, they
                // are not equal.
                if $T::is_infinite(*self) || $T::is_infinite(*other) {
                    return false;
                }

                // Now check whether `self` and `other` are really close together.
                // This is necessary when `self` and `other` are near zero.
                let abs_diff = $T::abs(self - other);
                if abs_diff <= *max_abs_diff {
                    return true;
                }

                // Finally, if the other cases have failed, we check their relative
                // absolute difference against the largest absolute value of `other` and
                // `self`.
                let abs_self = $T::abs(*self);
                let abs_other = $T::abs(*other);
                let largest = if abs_other > abs_self {
                    abs_other
                } else {
                    abs_self
                };

                return abs_diff <= largest * max_relative
            }
        }
    )*};
}

impl_relative_eq_float!(f32, f64);


impl<A, B> RelativeEq<&B> for &A
where
    A: RelativeEq<B>,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<&mut B> for &A
where
    A: RelativeEq<B>,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<&B> for &mut A
where
    A: RelativeEq<B>,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<&mut B> for &mut A
where
    A: RelativeEq<B>,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B, const N: usize> RelativeEq<[B; N]> for [A; N]
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = [A::Tolerance; N];

    #[inline]
    fn relative_eq(&self, other: &[B; N], max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        for i in 0..N {
            if !self[i].relative_eq(&other[i], &max_abs_diff[i], &max_relative[i]) {
                return false;
            }
        }

        true
    }
}

impl<A, B> RelativeEq<cell::Cell<B>> for cell::Cell<A>
where
    A: RelativeEq<B> + Copy,
    B: Copy,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&self.get(), &other.get(), max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&*self.borrow(), &*other.borrow(), max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<Option<B>> for Option<A>
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = Option<A::Tolerance>;

    #[inline]
    fn relative_eq(&self, other: &Option<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        if let (Some(a), Some(b), Some(abs_tol), Some(rel_tol)) = (self, other, max_abs_diff, max_relative) {
            RelativeEq::relative_eq(a, b, abs_tol, rel_tol)
        } else {
            false
        }
    }
}

impl<A, B> RelativeEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            RelativeEq::relative_eq(a, b, max_abs_diff, max_relative)
        } else {
            false
        }
    }
}


macro_rules! impl_relative_all_eq_float {
    ($($T:ident),* $(,)?) => {$(
        impl RelativeAllEq for $T {
            type AllTolerance = $T;

            #[inline]
            fn relative_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
                self.relative_eq(other, max_abs_diff, max_relative)
            }
        }
    )*}
}

impl_relative_all_eq_float!(f32, f64);


impl<A, B> RelativeAllEq<&B> for &A
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<&mut B> for &A
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<&B> for &mut A
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<&mut B> for &mut A
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B, const N: usize> RelativeAllEq<[B; N]> for [A; N]
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.relative_all_eq(b, max_abs_diff, max_relative))
    }
}

impl<A, B> RelativeAllEq<cell::Cell<B>> for cell::Cell<A>
where
    A: RelativeAllEq<B> + Copy,
    B: Copy,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&self.get(), &other.get(), max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: RelativeAllEq<B> + ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&*self.borrow(), &*other.borrow(), max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Option<B>> for Option<A>
where
    A: RelativeAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllTolerance = Option<A::AllTolerance>;

    #[inline]
    fn relative_all_eq(&self, other: &Option<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        if let (Some(a), Some(b), Some(abs_tol), Some(rel_tol)) = (self, other, max_abs_diff, max_relative) {
            a.relative_all_eq(b, abs_tol, rel_tol)
        } else {
            false
        }
    }
}

impl<A, B> RelativeAllEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: RelativeAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            a.relative_all_eq(b, max_abs_diff, max_relative)
        } else {
            false
        }
    }
}


macro_rules! impl_assert_relative_eq_float {
    ($($T:ident),* $(,)?) => {$(
        impl AssertRelativeEq for $T {
            type DebugAbsDiff = $T;
            type DebugTolerance = Self::Tolerance;

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                $T::abs(self - other)
            }

            #[inline]
            fn debug_abs_diff_tolerance(&self, _other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
                *max_abs_diff
            }

            #[inline]
            fn debug_relative_tolerance(&self, _other: &Self, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
                *max_relative
            }
        }
    )*};
}

impl_assert_relative_eq_float!(f32, f64);


impl<A, B> AssertRelativeEq<&B> for &A
where
    A: AssertRelativeEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&B) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &&B, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<&mut B> for &A
where
    A: AssertRelativeEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&mut B) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &&mut B, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<&B> for &mut A
where
    A: AssertRelativeEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&B) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &&B, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<&mut B> for &mut A
where
    A: AssertRelativeEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&mut B) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &&mut B, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B, const N: usize> AssertRelativeEq<[B; N]> for [A; N]
where
    A: AssertRelativeEq<B>,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = [A::DebugAbsDiff; N];
    type DebugTolerance = [A::DebugTolerance; N];

    #[inline]
    fn debug_abs_diff(&self, other: &[B; N]) -> Self::DebugAbsDiff {
        let mut result: [mem::MaybeUninit<A::DebugAbsDiff>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff(&other[i]));
        }

        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &[B; N], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let mut result: [mem::MaybeUninit<A::DebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff_tolerance(&other[i], &max_abs_diff[i]));
        }

        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &[B; N], max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        let mut result: [mem::MaybeUninit<A::DebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_relative_tolerance(&other[i], &max_relative[i]));
        }

        unsafe { array_assume_init(result) }
    }
}

impl<A, B> AssertRelativeEq<cell::Cell<B>> for cell::Cell<A>
where
    A: AssertRelativeEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::Cell<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&self.get(), &other.get(), max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &cell::Cell<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&self.get(), &other.get(), max_relative)
    }
}

impl<A, B> AssertRelativeEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: AssertRelativeEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::RefCell<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &cell::RefCell<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&*self.borrow(), &*other.borrow(), max_relative)
    }
}

impl<A, B> AssertRelativeEq<Option<B>> for Option<A>
where
    A: AssertRelativeEq<B>,
    A::Tolerance: Sized,
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &Option<B>) -> Self::DebugAbsDiff {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;

        Some(AssertRelativeEq::debug_abs_diff(ref_self, ref_other))
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Option<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_abs_diff = max_abs_diff.as_ref()?;

        Some(AssertRelativeEq::debug_abs_diff_tolerance(ref_self, ref_other, ref_max_abs_diff))
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Option<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_relative = max_relative.as_ref()?;

        Some(AssertRelativeEq::debug_relative_tolerance(ref_self, ref_other, ref_max_relative))
    }
}

impl<A, B> AssertRelativeEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: AssertRelativeEq<B>,
    A::Tolerance: Sized,
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::OnceCell<B>) -> Self::DebugAbsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeEq::debug_abs_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeEq::debug_abs_diff_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &cell::OnceCell<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeEq::debug_relative_tolerance(a, b, max_relative))
        } else {
            None
        }
    }
}


macro_rules! impl_assert_relative_all_eq_float {
    ($($T:ident),* $(,)?) => {$(
        impl AssertRelativeAllEq for $T {
            type AllDebugTolerance = Self::AllTolerance;

            #[inline]
            fn debug_abs_diff_all_tolerance(&self, other: &$T, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_abs_diff_tolerance(other, max_abs_diff)
            }

            #[inline]
            fn debug_relative_all_tolerance(&self, other: &$T, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_relative_tolerance(other, max_relative)
            }
        }
    )*};
}

impl_assert_relative_all_eq_float!(f32, f64);


impl<A, B> AssertRelativeAllEq<&B> for &A
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &&B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<&mut B> for &A
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &&mut B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<&B> for &mut A
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &&B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<&mut B> for &mut A
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &&mut B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(*self, *other, max_relative)
    }
}

impl<A, B, const N: usize> AssertRelativeAllEq<[B; N]> for [A; N]
where
    A: AssertRelativeAllEq<B>,
{
    type AllDebugTolerance = [A::AllDebugTolerance; N];

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff_all_tolerance(&other[i], max_abs_diff));
        }

        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &[B; N], max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_relative_all_tolerance(&other[i], max_relative));
        }

        unsafe { array_assume_init(result) }
    }
}

impl<A, B> AssertRelativeAllEq<cell::Cell<B>> for cell::Cell<A>
where
    A: AssertRelativeAllEq<B> + Copy,
    B: Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(&self.get(), &other.get(), max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &cell::Cell<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&self.get(), &other.get(), max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: AssertRelativeAllEq<B> + Copy,
    B: Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &cell::RefCell<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&*self.borrow(), &*other.borrow(), max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Option<B>> for Option<A>
where
    A: AssertRelativeAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Option<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_abs_diff = max_abs_diff.as_ref()?;

        Some(AssertRelativeAllEq::debug_abs_diff_all_tolerance(
            ref_self,
            ref_other,
            ref_max_abs_diff,
        ))
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Option<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_relative = max_relative.as_ref()?;

        Some(AssertRelativeAllEq::debug_relative_all_tolerance(
            ref_self,
            ref_other,
            ref_max_relative,
        ))
    }
}

impl<A, B> AssertRelativeAllEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: AssertRelativeAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &cell::OnceCell<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeAllEq::debug_relative_all_tolerance(a, b, max_relative))
        } else {
            None
        }
    }
}
