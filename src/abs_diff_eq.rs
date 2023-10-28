use core::cell;
use core::fmt;
use core::mem;

/*
#[inline]
pub fn abs_diff_eq<A, B>(lhs: A, rhs: B, tolerance: &A::Tolerance) -> bool 
where
    A: AbsDiffEq<B>
{
    AbsDiffEq::abs_diff_eq(&lhs, &rhs, tolerance)
}

#[inline]
pub fn abs_diff_ne<A, B>(lhs: A, rhs: B, tolerance: &A::Tolerance) -> bool 
where
    A: AbsDiffEq<B>
{
    AbsDiffEq::abs_diff_ne(&lhs, &rhs, tolerance)
}
*/


pub trait AbsDiffEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type Tolerance: ?Sized;

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
    fn abs_diff_eq(&self, other: &Rhs, tolerance: &Self::Tolerance) -> bool;

    /// Compare two floating point numbers for absolute difference inequality.
    ///
    /// Two floating point numbers are approximately inequal within tolerance
    /// `tolerance` provided that they are not approximately equal within tolerance
    /// `tolerance`.
    ///
    /// - Returns: A boolean indicating whether or not two floating point
    /// numbers are absolute difference inequal with respect to a tolerance
    /// `tolerance`.
    #[inline]
    fn abs_diff_ne(&self, other: &Rhs, tolerance: &Self::Tolerance) -> bool {
        !Self::abs_diff_eq(self, other, tolerance)
    }
}

pub trait AbsDiffEqAll<Rhs = Self>
where
    Rhs: ?Sized
{
    type AllTolerance: ?Sized;

    fn all_abs_diff_eq(&self, other: &Rhs, tolerance: &Self::AllTolerance) -> bool;

    fn all_abs_diff_ne(&self, other: &Rhs, tolerance: &Self::AllTolerance) -> bool {
        !Self::all_abs_diff_eq(self, other, tolerance)
    }
}
pub trait AssertAbsDiffEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized
{
    type DebugAbsDiff: fmt::Debug + Sized;
    type DebugTolerance: fmt::Debug;

    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    fn debug_abs_diff_tolerance(&self, other: &Rhs, tolerance: &Self::Tolerance) -> Self::DebugTolerance;
}

pub trait AssertAbsDiffEqAll<Rhs = Self>: AbsDiffEqAll<Rhs> 
where
    Rhs: ?Sized
{
    type AllDebugTolerance: fmt::Debug;

    fn debug_all_abs_diff_tolerance(&self, other: &Rhs, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance;
}























macro_rules! impl_abs_diff_eq_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffEq for $T {
            type Tolerance = $T;

            #[inline]
            fn abs_diff_eq(&self, other: &$T, tolerance: &Self::Tolerance) -> bool {
                let abs_diff = if self > other { 
                    self - other
                } else {
                    other - self
                };

                abs_diff <= *tolerance
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
            fn abs_diff_eq(&self, other: &$T, tolerance: &Self::Tolerance) -> bool {
                $T::abs(self - other) <= *tolerance
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
    fn abs_diff_eq(&self, other: &&B, tolerance: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEq<&mut B> for &A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&mut B, tolerance: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEq<&B> for &mut A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&B, tolerance: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEq<&mut B> for &mut A
where
    A: AbsDiffEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &&mut B, tolerance: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn abs_diff_eq(&self, other: &[B], tolerance: &Self::Tolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .zip(tolerance.iter())
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
    fn abs_diff_eq(&self, other: &[B; N], tolerance: &Self::Tolerance) -> bool {
        for i in 0..N {
            if !self[i].abs_diff_eq(&other[i], &tolerance[i]) {
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
    fn abs_diff_eq(&self, other: &cell::Cell<B>, tolerance: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&self.get(), &other.get(), tolerance)
    }
}

impl<A, B> AbsDiffEq<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AbsDiffEq<B> + ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &cell::RefCell<B>, tolerance: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&*self.borrow(), &*other.borrow(), tolerance)
    }
}




























macro_rules! impl_abs_diff_eq_all_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffEqAll for $T {
            type AllTolerance = $T;

            #[inline]
            fn all_abs_diff_eq(&self, other: &$T, tolerance: &Self::AllTolerance) -> bool {
                $T::abs_diff_eq(self, other, tolerance)
            }
        }
    )*}
}

impl_abs_diff_eq_all_unsigned!(u8, u16, u32, u64, u128, usize);


macro_rules! impl_abs_diff_eq_all_signed {
    ($($T:ident),* $(,)?) => {$(
        impl AbsDiffEqAll for $T {
            type AllTolerance = $T;

            #[inline]
            fn all_abs_diff_eq(&self, other: &$T, tolerance: &Self::AllTolerance) -> bool {
                $T::abs_diff_eq(self, other, tolerance)
            }
        }
    )*};
}

impl_abs_diff_eq_all_signed!(i8, i16, i32, i64, i128, isize, f32, f64);


impl<A, B> AbsDiffEqAll<&B> for &A
where
    A: AbsDiffEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &&B, tolerance: &Self::AllTolerance) -> bool {
        AbsDiffEqAll::all_abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEqAll<&mut B> for &A
where
    A: AbsDiffEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &&mut B, tolerance: &Self::AllTolerance) -> bool {
        AbsDiffEqAll::all_abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEqAll<&B> for &mut A
where
    A: AbsDiffEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &&B, tolerance: &Self::AllTolerance) -> bool {
        AbsDiffEqAll::all_abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEqAll<&mut B> for &mut A
where
    A: AbsDiffEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &&mut B, tolerance: &Self::AllTolerance) -> bool {
        AbsDiffEqAll::all_abs_diff_eq(*self, *other, tolerance)
    }
}

impl<A, B> AbsDiffEqAll<[B]> for [A]
where
    A: AbsDiffEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &[B], tolerance: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.all_abs_diff_eq(b, tolerance))
    }
}

impl<A, B, const N: usize> AbsDiffEqAll<[B; N]> for [A; N]
where
    A: AbsDiffEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &[B; N], tolerance: &Self::AllTolerance) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.all_abs_diff_eq(b, tolerance))
    }
}

impl<A, B> AbsDiffEqAll<cell::Cell<B>> for cell::Cell<A> 
where
    A: AbsDiffEqAll<B> + Copy,
    B: Copy
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &cell::Cell<B>, tolerance: &Self::AllTolerance) -> bool {
        AbsDiffEqAll::all_abs_diff_eq(&self.get(), &other.get(), tolerance)
    }
}

impl<A, B> AbsDiffEqAll<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AbsDiffEqAll<B> + ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_abs_diff_eq(&self, other: &cell::RefCell<B>, tolerance: &Self::AllTolerance) -> bool {
        AbsDiffEqAll::all_abs_diff_eq(&*self.borrow(), &*other.borrow(), tolerance)
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
            fn debug_abs_diff_tolerance(&self, _other: &$T, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
                *tolerance
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
            fn debug_abs_diff_tolerance(&self, _other: &$T, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
                *tolerance
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
    fn debug_abs_diff_tolerance(&self, other: &&B, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, tolerance)
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
    fn debug_abs_diff_tolerance(&self, other: &&mut B, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, tolerance)
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
    fn debug_abs_diff_tolerance(&self, other: &&B, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, tolerance)
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
    fn debug_abs_diff_tolerance(&self, other: &&mut B, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(*self, *other, tolerance)
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
    fn debug_abs_diff_tolerance(&self, other: &[B], tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == tolerance.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(tolerance.iter())
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
    fn debug_abs_diff_tolerance(&self, other: &[B; N], tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        let mut result: [mem::MaybeUninit<A::DebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_abs_diff_tolerance(&other[i], &tolerance[i]));
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
    fn debug_abs_diff_tolerance(&self, other: &cell::Cell<B>, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&self.get(), &other.get(), tolerance)
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
    fn debug_abs_diff_tolerance(&self, other: &cell::RefCell<B>, tolerance: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&*self.borrow(), &*other.borrow(), tolerance)
    }
}









macro_rules! impl_assert_all_abs_diff_eq_unsigned {
    ($($T:ident),* $(,)?) => {$(
        impl AssertAbsDiffEqAll for $T {
            type AllDebugTolerance = Self::AllTolerance;

            #[inline]
            fn debug_all_abs_diff_tolerance(&self, other: &$T, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_abs_diff_tolerance(other, tolerance)
            }
        }
    )*}
}

impl_assert_all_abs_diff_eq_unsigned!(u8, u16, u32, u64, u128, usize);


macro_rules! impl_assert_all_abs_diff_eq_signed {
    ($($T:ident),* $(,)?) => {$(
        impl AssertAbsDiffEqAll for $T {
            type AllDebugTolerance = Self::AllTolerance;

            #[inline]
            fn debug_all_abs_diff_tolerance(&self, other: &$T, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_abs_diff_tolerance(other, tolerance)
            }
        }
    )*};
}

impl_assert_all_abs_diff_eq_signed!(i8, i16, i32, i64, i128, isize, f32, f64);


impl<A, B> AssertAbsDiffEqAll<&B> for &A
where
    A: AssertAbsDiffEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&B, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(*self, *other, tolerance)
    }
}

impl<A, B> AssertAbsDiffEqAll<&mut B> for &A
where
    A: AssertAbsDiffEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&mut B, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(*self, *other, tolerance)
    }
}

impl<A, B> AssertAbsDiffEqAll<&B> for &mut A
where
    A: AssertAbsDiffEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&B, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(*self, *other, tolerance)
    }
}

impl<A, B> AssertAbsDiffEqAll<&mut B> for &mut A
where
    A: AssertAbsDiffEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&mut B, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(*self, *other, tolerance)
    }
}

impl<A, B> AssertAbsDiffEqAll<[B]> for [A]
where
    A: AssertAbsDiffEqAll<B>,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &[B], tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| a.debug_all_abs_diff_tolerance(b, tolerance))
                .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B, const N: usize> AssertAbsDiffEqAll<[B; N]> for [A; N]
where
    A: AssertAbsDiffEqAll<B>
{
    type AllDebugTolerance = [A::AllDebugTolerance; N];

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &[B; N], tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_all_abs_diff_tolerance(&other[i], &tolerance));
        }

        unsafe { 
            array_assume_init(result) 
        }
    }
}

impl<A, B> AssertAbsDiffEqAll<cell::Cell<B>> for cell::Cell<A> 
where
    A: AssertAbsDiffEqAll<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &cell::Cell<B>, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(&self.get(), &other.get(), tolerance)
    }
}

impl<A, B> AssertAbsDiffEqAll<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AssertAbsDiffEqAll<B> + ?Sized
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &cell::RefCell<B>, tolerance: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(&*self.borrow(), &*other.borrow(), tolerance)
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
        A: AbsDiffEqAll<B> + ?Sized,
        B: ?Sized
    {
        A::all_abs_diff_eq(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> bool 
    where
        A: AbsDiffEqAll<B> + ?Sized,
        B: ?Sized
    {
        A::all_abs_diff_ne(lhs, rhs, max_abs_diff)
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
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_eq!(*left_val, *right_val, abs_diff <= *tol_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(
                        concat!("assertion failed: assert_abs_diff_eq!(left, right, abs_diff <= t)\nleft: `{:?}`,\nright: `{:?}`,\nabs_diff: `{:?}`,\nt: `{:?}`"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEq::debug_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff <= $tol:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_eq!(left_val, right_val, abs_diff <= tol_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(
                        concat!("assertion failed: assert_abs_diff_eq!(left, right, abs_diff <= t)\nleft: `{:?}`,\nright: `{:?}`,\nabs_diff: `{:?}`,\nt: `{:?}`\n{}"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEq::debug_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_eq!(*left_val, *right_val, abs_diff_all <= *tol_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(
                        concat!("assertion failed: assert_abs_diff_eq!(left, right, abs_diff_all <= t)\nleft: `{:?}`,\nright: `{:?}`,\nabs_diff: `{:?}`,\nt: `{:?}`"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_eq!(left_val, right_val, abs_diff_all <= tol_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(
                        concat!("assertion failed: assert_abs_diff_eq!(left, right, abs_diff_all <= t)\nleft: `{:?}`,\nright: `{:?}`,\nabs_diff: `{:?}`,\nt: `{:?}`\n{}"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEqAll::debug_all_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($left:expr, $right:expr, abs_diff <= $tol:expr $(,)?) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_ne!(*left_val, *right_val, abs_diff <= *tol_val) {
                    panic!(
                        concat!("assertion failed: assert_abs_diff_ne!(left, right, abs_diff <= t)\nleft: `{:?}`\nright: `{:?}`\nabs_diff: `{:?}`\nt: `{:?}`"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEq::debug_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff <= $tol:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_ne!(left_val, right_val, abs_diff <= tol_val) {
                    panic!(
                        concat!("assertion failed: assert_abs_diff_ne!(left, right, abs_diff <= t)\nleft: `{:?}`\nright: `{:?}`\nabs_diff: `{:?}`\nt: `{:?}`\n{}"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEq::debug_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr $(,)?) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_ne!(*left_val, *right_val, abs_diff_all <= *tol_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(
                        concat!("assertion failed: assert_abs_diff_ne!(left, right, abs_diff_all <= t)\nleft: `{:?}`,\nright: `{:?}`,\nabs_diff: `{:?}`,\nt: `{:?}`"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if !$crate::abs_diff_ne!(left_val, right_val, abs_diff_all <= tol_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(
                        concat!("assertion failed: assert_abs_diff_ne!(left, right, abs_diff_all <= t)\nleft: `{:?}`,\nright: `{:?}`,\nabs_diff: `{:?}`,\nt: `{:?}`\n{}"),
                        &*left_val,
                        &*right_val,
                        $crate::AssertAbsDiffEqAll::debug_all_abs_diff(&*left_val, &*right_val),
                        $crate::AssertAbsDiffEqAll::debug_all_abs_diff_tolerance(&*left_val, &*right_val, &*tol_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

