use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};

use core::cell;
use core::mem;


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
    }
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
    }
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
    }
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
