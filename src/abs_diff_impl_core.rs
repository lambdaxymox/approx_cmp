use crate::abs_diff::{
    AbsDiffEq,
    AbsDiffAllEq,
    AssertAbsDiffEq,
    AssertAbsDiffAllEq,
};

use core::cell;
use core::mem;


macro_rules! impl_abs_diff_eq_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffEq for $T {
            type Tolerance = $T;

            #[inline]
            fn abs_diff_eq(&self, other: &$T, max_abs_diff: &Self::Tolerance) -> bool {
                let abs_diff = if self > other { 
                    self - other
                } else {
                    other - self
                };

                abs_diff <= *max_abs_diff
            }
        }
    )*}
}

impl_abs_diff_eq_unsigned!(u8, u16, u32, u64, u128, usize);


macro_rules! impl_abs_diff_eq_signed {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffEq for $T {
            type Tolerance = $T;

            #[inline]
            fn abs_diff_eq(&self, other: &$T, max_abs_diff: &Self::Tolerance) -> bool {
                (self == other) || ($T::abs(self - other) <= *max_abs_diff)
            }
        }
    )*};
}

impl_abs_diff_eq_signed!(i8, i16, i32, i64, i128, isize, f32, f64);


impl<A, B> AbsDiffEq<&B> for &A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<&mut B> for &A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<&B> for &mut A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<&mut B> for &mut A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn abs_diff_eq(&self, other: &[B], max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .zip(max_abs_diff.iter())
            .all(|((a, b), tol)| a.abs_diff_eq(b, tol))
    }
}

impl<'a, 'b, A, B> AbsDiffEq<&'b [B]> for &'a [A]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn abs_diff_eq(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .zip(max_abs_diff.iter())
            .all(|((a, b), tol)| a.abs_diff_eq(b, tol))
    }
}

impl<A, B, const N: usize> AbsDiffEq<[B; N]> for [A; N]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance; N];

    #[inline]
    fn abs_diff_eq(&self, other: &[B; N], max_abs_diff: &Self::Tolerance) -> bool {
        for i in 0..N {
            if !self[i].abs_diff_eq(&other[i], &max_abs_diff[i]) {
                return false;
            }
        }

        true
    }
}

impl<A, B> AbsDiffEq<cell::Cell<B>> for cell::Cell<A> 
where
    A: AbsDiffEq<B> + Copy,
    B: Copy
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&self.get(), &other.get(), max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AbsDiffEq<B> + ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<Option<B>> for Option<A>
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = Option<A::Tolerance>;

    #[inline]
    fn abs_diff_eq(&self, other: &Option<B>, max_abs_diff: &Self::Tolerance) -> bool {
        if let (Some(a), Some(b), Some(tol)) = (self, other, max_abs_diff) {
            a.abs_diff_eq(b, tol)
        } else {
            false
        }
    }
}


macro_rules! impl_abs_diff_eq_all_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffAllEq for $T {
            type AllTolerance = $T;

            #[inline]
            fn abs_diff_all_eq(&self, other: &$T, max_abs_diff: &Self::AllTolerance) -> bool {
                $T::abs_diff_eq(self, other, max_abs_diff)
            }
        }
    )*}
}

impl_abs_diff_eq_all_unsigned!(u8, u16, u32, u64, u128, usize);


macro_rules! impl_abs_diff_eq_all_signed {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffAllEq for $T {
            type AllTolerance = $T;

            #[inline]
            fn abs_diff_all_eq(&self, other: &$T, max_abs_diff: &Self::AllTolerance) -> bool {
                $T::abs_diff_eq(self, other, max_abs_diff)
            }
        }
    )*};
}

impl_abs_diff_eq_all_signed!(i8, i16, i32, i64, i128, isize, f32, f64);


impl<A, B> AbsDiffAllEq<&B> for &A
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<&mut B> for &A
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<&B> for &mut A
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<&mut B> for &mut A
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(*self, *other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<[B]> for [A]
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.abs_diff_all_eq(b, max_abs_diff))
    }
}

impl<'a, 'b, A, B> AbsDiffAllEq<&'b [B]> for &'a [A]
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.abs_diff_all_eq(b, max_abs_diff))
    }
}

impl<A, B, const N: usize> AbsDiffAllEq<[B; N]> for [A; N]
where
    A: AbsDiffAllEq<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.abs_diff_all_eq(b, max_abs_diff))
    }
}

impl<A, B> AbsDiffAllEq<cell::Cell<B>> for cell::Cell<A> 
where
    A: AbsDiffAllEq<B> + Copy,
    B: Copy
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&self.get(), &other.get(), max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AbsDiffAllEq<B> + ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Option<B>> for Option<A>
where
    A: AbsDiffAllEq<B>,
    A::AllTolerance: Sized
{
    type AllTolerance = Option<A::AllTolerance>;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Option<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        if let (Some(a), Some(b), Some(tol)) = (self, other, max_abs_diff) {
            a.abs_diff_all_eq(b, tol)
        } else {
            false
        }
    }
}


macro_rules! impl_assert_abs_diff_eq_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AssertAbsDiffEq for $T {
            type DebugAbsDiff = $T;
            type DebugTolerance = Self::Tolerance;

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                if self > other { 
                    self - other
                } else {
                    other - self
                }
            }

            #[inline]
            fn debug_abs_diff_tolerance(&self, _other: &$T, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
                *max_abs_diff
            }
        }
    )*}
}

impl_assert_abs_diff_eq_unsigned!(u8, u16, u32, u64, u128, usize);


macro_rules! impl_assert_abs_diff_eq_signed {
    ($($T:ident),* $(,)?) => {$(
        impl AssertAbsDiffEq for $T {
            type DebugAbsDiff = $T;
            type DebugTolerance = Self::Tolerance;

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                $T::abs(self - other)
            }

            #[inline]
            fn debug_abs_diff_tolerance(&self, _other: &$T, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
                *max_abs_diff
            }
        }
    )*};
}

impl_assert_abs_diff_eq_signed!(i8, i16, i32, i64, i128, isize, f32, f64);


impl<A, B> AssertAbsDiffEq<&B> for &A
where
    A: AssertAbsDiffEq<B>
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&B) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<&mut B> for &A
where
    A: AssertAbsDiffEq<B>
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&mut B) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<&B> for &mut A
where
    A: AssertAbsDiffEq<B>
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&B) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<&mut B> for &mut A
where
    A: AssertAbsDiffEq<B>
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &&mut B) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(*self, *other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<[B]> for [A]
where
    A: AssertAbsDiffEq<B>,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &[B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff(b))
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &[B], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .map(|((a, b), tol)| { AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }
}

impl<'a, 'b, A, B> AssertAbsDiffEq<&'b [B]> for &'a [A]
where
    A: AssertAbsDiffEq<B>,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &&'b [B]) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff(b))
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .map(|((a, b), tol)| { AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }
}

#[inline(always)]
fn uninit_array<T, const N: usize>() -> [mem::MaybeUninit<T>; N] {
    unsafe { 
        mem::MaybeUninit::<[mem::MaybeUninit<T>; N]>::uninit().assume_init() 
    }
}

#[inline(always)]
unsafe fn array_assume_init<T, const N: usize>(array: [mem::MaybeUninit<T>; N]) -> [T; N] {
    (&array as *const _ as *const [T; N]).read()
}

impl<A, B, const N: usize> AssertAbsDiffEq<[B; N]> for [A; N]
where
    A: AssertAbsDiffEq<B>,
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
        
        unsafe { 
            array_assume_init(result)
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &[B; N], max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let mut result: [mem::MaybeUninit<A::DebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff_tolerance(&other[i], &max_abs_diff[i]));
        }

        unsafe { 
            array_assume_init(result) 
        }
    }
}

impl<A, B> AssertAbsDiffEq<cell::Cell<B>> for cell::Cell<A> 
where
    A: AssertAbsDiffEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::Cell<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&self.get(), &other.get())
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&self.get(), &other.get(), max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AssertAbsDiffEq<B> + Copy,
    B: Copy
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &cell::RefCell<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&*self.borrow(), &*other.borrow())
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Option<B>> for Option<A>
where
    A: AssertAbsDiffEq<B>,
    A::Tolerance: Sized
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &Option<B>) -> Self::DebugAbsDiff {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;

        Some(AssertAbsDiffEq::debug_abs_diff(ref_self, ref_other))
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Option<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_abs_diff = max_abs_diff.as_ref()?;

        Some(AssertAbsDiffEq::debug_abs_diff_tolerance(ref_self, ref_other, ref_max_abs_diff))
    }
}


macro_rules! impl_assert_all_abs_diff_eq_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AssertAbsDiffAllEq for $T {
            type AllDebugTolerance = Self::AllTolerance;

            #[inline]
            fn debug_abs_diff_all_tolerance(&self, other: &$T, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_abs_diff_tolerance(other, max_abs_diff)
            }
        }
    )*}
}

impl_assert_all_abs_diff_eq_unsigned!(u8, u16, u32, u64, u128, usize);


macro_rules! impl_assert_all_abs_diff_eq_signed {
    ($($T:ident),* $(,)?) => {$(
        impl AssertAbsDiffAllEq for $T {
            type AllDebugTolerance = Self::AllTolerance;

            #[inline]
            fn debug_abs_diff_all_tolerance(&self, other: &$T, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_abs_diff_tolerance(other, max_abs_diff)
            }
        }
    )*};
}

impl_assert_all_abs_diff_eq_signed!(i8, i16, i32, i64, i128, isize, f32, f64);


impl<A, B> AssertAbsDiffAllEq<&B> for &A
where
    A: AssertAbsDiffAllEq<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<&mut B> for &A
where
    A: AssertAbsDiffAllEq<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<&B> for &mut A
where
    A: AssertAbsDiffAllEq<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<&mut B> for &mut A
where
    A: AssertAbsDiffAllEq<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(*self, *other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<[B]> for [A]
where
    A: AssertAbsDiffAllEq<B>,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff_all_tolerance(b, max_abs_diff))
                .collect(),
            )
        } else {
            None
        }
    }
}

impl<'a, 'b, A, B> AssertAbsDiffAllEq<&'b [B]> for &'a [A]
where
    A: AssertAbsDiffAllEq<B>,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_abs_diff_all_tolerance(b, max_abs_diff))
                .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B, const N: usize> AssertAbsDiffAllEq<[B; N]> for [A; N]
where
    A: AssertAbsDiffAllEq<B>
{
    type AllDebugTolerance = [A::AllDebugTolerance; N];

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff_all_tolerance(&other[i], &max_abs_diff));
        }

        unsafe { 
            array_assume_init(result) 
        }
    }
}

impl<A, B> AssertAbsDiffAllEq<cell::Cell<B>> for cell::Cell<A> 
where
    A: AssertAbsDiffAllEq<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&self.get(), &other.get(), max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AssertAbsDiffAllEq<B> + ?Sized
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Option<B>> for Option<A>
where
    A: AssertAbsDiffAllEq<B>,
    A::AllTolerance: Sized
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Option<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let ref_self = self.as_ref()?;
        let ref_other = other.as_ref()?;
        let ref_max_abs_diff = max_abs_diff.as_ref()?;

        Some(AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(ref_self, ref_other, ref_max_abs_diff))
    }
}


#[doc(hidden)]
pub struct AbsDiffCmp {}

impl AbsDiffCmp {
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> bool 
    where
        A: AbsDiffEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> bool 
    where
        A: AbsDiffEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_ne(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool 
    where
        A: AbsDiffAllEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_all_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool 
    where
        A: AbsDiffAllEq<B> + ?Sized,
        B: ?Sized
    {
        A::abs_diff_all_ne(lhs, rhs, max_abs_diff)
    }
}

#[doc(hidden)]
pub struct AbsDiffCmpOpTol {}

impl AbsDiffCmpOpTol {
    #[inline]
    pub fn abs_diff<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> A::DebugTolerance 
    where
        A: AbsDiffEq<B> + AssertAbsDiffEq<B>
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance 
    where
        A: AbsDiffAllEq<B> + AssertAbsDiffAllEq<B>
    {
        A::debug_abs_diff_all_tolerance(lhs, rhs, max_abs_diff)
    }
}

#[macro_export]
macro_rules! abs_diff_eq {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::eq(left_val, right_val, &$tol)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::all_eq(left_val, right_val, &$tol)
            }
        }
    }};
}

#[macro_export]
macro_rules! abs_diff_ne {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::ne(left_val, right_val, &$tol)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                $crate::AbsDiffCmp::all_ne(left_val, right_val, &$tol)
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_eq {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_eq!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_eq!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_eq!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_eq!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_ne!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_ne!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::abs_diff_ne!(*left_val, *right_val, $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_abs_diff_ne!(left, right, ", stringify!($eq1), " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::AbsDiffCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! debug_assert_abs_diff_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_abs_diff_eq!($($arg)*); })
}

#[macro_export]
macro_rules! debug_assert_abs_diff_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_abs_diff_ne!($($arg)*); })
}

