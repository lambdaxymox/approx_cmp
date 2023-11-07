use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
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


macro_rules! impl_ulps_eq_float {
    ($T:ident, $U:ident) => {
        impl UlpsEq for $T {
            type Tolerance = $T;
            type UlpsTolerance = $U;

            #[inline]
            fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
                // Check whether the two numbers `self` and `other` are NaN. The sign test
                // fails on NaNs.
                if self.is_nan() || other.is_nan() {
                    return false;
                }

                // First check whether the two numbers `self` and `other` are really close
                // together.
                let abs_diff = $T::abs(self - other);
                if abs_diff <= *max_abs_diff {
                    return true;
                }

                // If they do not fall inside the absolute difference equality closeness
                // threshold, compare their signs. Sign comparison is a cheap check
                // before comparing bit patterns.
                if self.signum() != other.signum() {
                    return false;
                }

                // Compare the two numbers by their bit patterns.
                let bits_self: $U = self.to_bits();
                let bits_other: $U = other.to_bits();
                let ulps_distance = if bits_self <= bits_other {
                    bits_other - bits_self
                } else {
                    bits_self - bits_other
                };

                ulps_distance <= *max_ulps
            }
        }
    };
}

impl_ulps_eq_float!(f32, u32);
impl_ulps_eq_float!(f64, u64);


impl<A, B> UlpsEq<&B> for &A
where
    A: UlpsEq<B>,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<&mut B> for &A
where
    A: UlpsEq<B>,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<&B> for &mut A
where
    A: UlpsEq<B>,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<&mut B> for &mut A
where
    A: UlpsEq<B>,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B, const N: usize> UlpsEq<[B; N]> for [A; N]
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = [A::Tolerance; N];
    type UlpsTolerance = [A::UlpsTolerance; N];

    #[inline]
    fn ulps_eq(&self, other: &[B; N], max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        for i in 0..N {
            if !self[i].ulps_eq(&other[i], &max_abs_diff[i], &max_ulps[i]) {
                return false;
            }
        }

        true
    }
}

impl<A, B> UlpsEq<cell::Cell<B>> for cell::Cell<A>
where
    A: UlpsEq<B> + Copy,
    B: Copy,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&self.get(), &other.get(), max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&*self.borrow(), &*other.borrow(), max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<Option<B>> for Option<A>
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = Option<A::Tolerance>;
    type UlpsTolerance = Option<A::UlpsTolerance>;

    #[inline]
    fn ulps_eq(&self, other: &Option<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        if let (Some(a), Some(b), Some(abs_tol), Some(ulps_tol)) = (self, other, max_abs_diff, max_ulps) {
            UlpsEq::ulps_eq(a, b, abs_tol, ulps_tol)
        } else {
            false
        }
    }
}

impl<A, B> UlpsEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            UlpsEq::ulps_eq(a, b, max_abs_diff, max_ulps)
        } else {
            false
        }
    }
}


macro_rules! impl_ulps_all_eq_float {
    ($T:ident, $U:ident) => {
        impl UlpsAllEq for $T {
            type AllTolerance = $T;
            type AllUlpsTolerance = $U;

            #[inline]
            fn ulps_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
                self.ulps_eq(other, max_abs_diff, max_ulps)
            }
        }
    };
}

impl_ulps_all_eq_float!(f32, u32);
impl_ulps_all_eq_float!(f64, u64);


impl<A, B> UlpsAllEq<&B> for &A
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<&mut B> for &A
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<&B> for &mut A
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<&mut B> for &mut A
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(*self, *other, max_abs_diff, max_ulps)
    }
}

impl<A, B, const N: usize> UlpsAllEq<[B; N]> for [A; N]
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[rustfmt::skip]
    #[inline]
    fn ulps_all_eq(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.ulps_all_eq(b, max_abs_diff, max_ulps))
    }
}

impl<A, B> UlpsAllEq<cell::Cell<B>> for cell::Cell<A>
where
    A: UlpsAllEq<B> + Copy,
    B: Copy,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&self.get(), &other.get(), max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: UlpsAllEq<B> + Copy,
    B: Copy,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&*self.borrow(), &*other.borrow(), max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<Option<B>> for Option<A>
where
    A: UlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
{
    type AllTolerance = Option<A::AllTolerance>;
    type AllUlpsTolerance = Option<A::AllUlpsTolerance>;

    #[inline]
    fn ulps_all_eq(&self, other: &Option<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        if let (Some(a), Some(b), Some(abs_tol), Some(ulps_tol)) = (self, other, max_abs_diff, max_ulps) {
            a.ulps_all_eq(b, abs_tol, ulps_tol)
        } else {
            false
        }
    }
}

impl<A, B> UlpsAllEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: UlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            UlpsAllEq::ulps_all_eq(a, b, max_abs_diff, max_ulps)
        } else {
            false
        }
    }
}


macro_rules! impl_assert_ulps_eq_float {
    ($T:ident, $U:ident) => {
        impl AssertUlpsEq for $T {
            type DebugAbsDiff = $T;
            type DebugUlpsDiff = Option<$U>;
            type DebugTolerance = $T;
            type DebugUlpsTolerance = $U;

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                $T::abs(self - other)
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &Self) -> Self::DebugUlpsDiff {
                if self == other {
                    Some(0)
                } else if self.is_nan() || other.is_nan() {
                    None
                } else if self.signum() != other.signum() {
                    None
                } else {
                    let bits_self = self.to_bits();
                    let bits_other = other.to_bits();
                    let bits_max = $U::max(bits_self, bits_other);
                    let bits_min = $U::min(bits_self, bits_other);

                    Some(bits_max - bits_min)
                }
            }

            #[inline]
            fn debug_abs_diff_tolerance(&self, _other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
                *max_abs_diff
            }

            #[inline]
            fn debug_ulps_tolerance(&self, _other: &Self, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
                *max_ulps
            }
        }
    };
}

impl_assert_ulps_eq_float!(f32, u32);
impl_assert_ulps_eq_float!(f64, u64);


impl<A, B> AssertUlpsEq<&B> for &A
where
    A: AssertUlpsEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    fn debug_abs_diff(&self, other: &&B) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(*self, *other)
    }

    fn debug_ulps_diff(&self, other: &&B) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(*self, *other)
    }

    fn debug_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_tolerance(&self, other: &&B, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<&mut B> for &A
where
    A: AssertUlpsEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    fn debug_abs_diff(&self, other: &&mut B) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(*self, *other)
    }

    fn debug_ulps_diff(&self, other: &&mut B) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(*self, *other)
    }

    fn debug_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_tolerance(&self, other: &&mut B, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<&B> for &mut A
where
    A: AssertUlpsEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    fn debug_abs_diff(&self, other: &&B) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(*self, *other)
    }

    fn debug_ulps_diff(&self, other: &&B) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(*self, *other)
    }

    fn debug_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_tolerance(&self, other: &&B, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<&mut B> for &mut A
where
    A: AssertUlpsEq<B>,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    fn debug_abs_diff(&self, other: &&mut B) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(*self, *other)
    }

    fn debug_ulps_diff(&self, other: &&mut B) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(*self, *other)
    }

    fn debug_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_tolerance(&self, other: &&mut B, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B, const N: usize> AssertUlpsEq<[B; N]> for [A; N]
where
    A: AssertUlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
    A::DebugTolerance: Sized,
    A::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = [A::DebugAbsDiff; N];
    type DebugUlpsDiff = [A::DebugUlpsDiff; N];
    type DebugTolerance = [A::DebugTolerance; N];
    type DebugUlpsTolerance = [A::DebugUlpsTolerance; N];

    #[inline]
    fn debug_abs_diff(&self, other: &[B; N]) -> Self::DebugAbsDiff {
        let mut result: [mem::MaybeUninit<A::DebugAbsDiff>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff(&other[i]));
        }

        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &[B; N]) -> Self::DebugUlpsDiff {
        let mut result: [mem::MaybeUninit<A::DebugUlpsDiff>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_ulps_diff(&other[i]));
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
    fn debug_ulps_tolerance(&self, other: &[B; N], max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        let mut result: [mem::MaybeUninit<A::DebugUlpsTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_ulps_tolerance(&other[i], &max_ulps[i]));
        }

        unsafe { array_assume_init(result) }
    }
}

impl<A, B> AssertUlpsEq<cell::Cell<B>> for cell::Cell<A>
where
    A: AssertUlpsEq<B> + Copy,
    B: Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::Cell<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &cell::Cell<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&self.get(), &other.get(), max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &cell::Cell<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&self.get(), &other.get(), max_ulps)
    }
}

impl<A, B> AssertUlpsEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::RefCell<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &cell::RefCell<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &cell::RefCell<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&*self.borrow(), &*other.borrow(), max_ulps)
    }
}

impl<A, B> AssertUlpsEq<Option<B>> for Option<A>
where
    A: AssertUlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugUlpsDiff = Option<A::DebugUlpsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;
    type DebugUlpsTolerance = Option<A::DebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &Option<B>) -> Self::DebugAbsDiff {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;

        Some(AssertUlpsEq::debug_abs_diff(ref_self, ref_other))
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Option<B>) -> Self::DebugUlpsDiff {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        
        Some(AssertUlpsEq::debug_ulps_diff(ref_self, ref_other))
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Option<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_abs_diff = max_abs_diff.as_ref()?;
        
        Some(AssertUlpsEq::debug_abs_diff_tolerance(ref_self, ref_other, ref_max_abs_diff))
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Option<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_ulps = max_ulps.as_ref()?;

        Some(AssertUlpsEq::debug_ulps_tolerance(ref_self, ref_other, ref_max_ulps))
    }
}

impl<A, B> AssertUlpsEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: AssertUlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugUlpsDiff = Option<A::DebugUlpsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;
    type DebugUlpsTolerance = Option<A::DebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::OnceCell<B>) -> Self::DebugAbsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_abs_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &cell::OnceCell<B>) -> Self::DebugUlpsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_ulps_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_abs_diff_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &cell::OnceCell<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_ulps_tolerance(a, b, max_ulps))
        } else {
            None
        }
    }
}


macro_rules! impl_assert_ulps_all_eq_float {
    ($T:ident, $U:ident) => {
        impl AssertUlpsAllEq for $T {
            type AllDebugTolerance = $T;
            type AllDebugUlpsTolerance = $U;

            fn debug_abs_diff_all_tolerance(&self, _other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
                *max_abs_diff
            }

            fn debug_ulps_all_tolerance(&self, _other: &Self, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
                *max_ulps
            }
        }
    };
}

impl_assert_ulps_all_eq_float!(f32, u32);
impl_assert_ulps_all_eq_float!(f64, u64);


impl<A, B> AssertUlpsAllEq<&B> for &A
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    fn debug_abs_diff_all_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_all_tolerance(&self, other: &&B, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<&mut B> for &A
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    fn debug_abs_diff_all_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_all_tolerance(&self, other: &&mut B, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<&B> for &mut A
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    fn debug_abs_diff_all_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_all_tolerance(&self, other: &&B, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<&mut B> for &mut A
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    fn debug_abs_diff_all_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }

    fn debug_ulps_all_tolerance(&self, other: &&mut B, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(*self, *other, max_ulps)
    }
}

impl<A, B, const N: usize> AssertUlpsAllEq<[B; N]> for [A; N]
where
    A: AssertUlpsAllEq<B>,
{
    type AllDebugTolerance = [A::AllDebugTolerance; N];
    type AllDebugUlpsTolerance = [A::AllDebugUlpsTolerance; N];

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff_all_tolerance(&other[i], max_abs_diff));
        }

        unsafe { array_assume_init(result) }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &[B; N], max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugUlpsTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_ulps_all_tolerance(&other[i], max_ulps));
        }

        unsafe { array_assume_init(result) }
    }
}

impl<A, B> AssertUlpsAllEq<cell::Cell<B>> for cell::Cell<A>
where
    A: AssertUlpsAllEq<B> + Copy,
    B: Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&self.get(), &other.get(), max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &cell::Cell<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&self.get(), &other.get(), max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<cell::RefCell<B>> for cell::RefCell<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &cell::RefCell<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&*self.borrow(), &*other.borrow(), max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<Option<B>> for Option<A>
where
    A: AssertUlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;
    type AllDebugUlpsTolerance = Option<A::AllDebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Option<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_abs_diff = max_abs_diff.as_ref()?;

        Some(AssertUlpsAllEq::debug_abs_diff_all_tolerance(
            ref_self,
            ref_other,
            ref_max_abs_diff,
        ))
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Option<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_ulps = max_ulps.as_ref()?;

        Some(AssertUlpsAllEq::debug_ulps_all_tolerance(
            ref_self,
            ref_other,
            ref_max_ulps,
        ))
    }
}

impl<A, B> AssertUlpsAllEq<cell::OnceCell<B>> for cell::OnceCell<A>
where
    A: AssertUlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;
    type AllDebugUlpsTolerance = Option<A::AllDebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::OnceCell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &cell::OnceCell<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsAllEq::debug_ulps_all_tolerance(a, b, max_ulps))
        } else {
            None
        }
    }
}
