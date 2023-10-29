use core::cell;
use core::fmt;
use core::mem;


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
    fn relative_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool;

    /// Compare two floating point numbers for relative inequality.
    ///
    /// The relative inequality comparison for floating point numbers is based on
    /// the contents of the article [Comparing Floating Point Numbers, 2012 Edition]
    /// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    ///
    /// - Returns: A boolean indicating whether or not two floating point numbers
    /// are relatively inequal with respect to a `maxRelative` multiple of the
    /// tolerance `tolerance`.
    fn relative_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        !Self::relative_eq(self, other, max_abs_diff, max_relative)
    }
}

pub trait RelativeEqAll<Rhs = Self>
where
    Rhs: ?Sized
{
    type AllTolerance: ?Sized;

    fn all_relative_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool;

    fn all_relative_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        !Self::all_relative_eq(self, other, max_abs_diff, max_relative)
    }
}

pub trait AssertRelativeEq<Rhs = Self>: RelativeEq<Rhs>
where
    Rhs: ?Sized
{
    type DebugAbsDiff: fmt::Debug + Sized;
    type DebugTolerance: fmt::Debug;

    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    fn debug_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance;

    fn debug_relative_tolerance(&self, other: &Rhs, max_relative: &Self::Tolerance) -> Self::DebugTolerance;
}

pub trait AssertRelativeEqAll<Rhs = Self>: RelativeEqAll<Rhs> 
where
    Rhs: ?Sized
{
    type AllDebugTolerance: fmt::Debug;

    fn debug_all_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;

    fn debug_all_relative_tolerance(&self, other: &Rhs, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance;
}

#[macro_export]
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
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<&mut B> for &A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<&B> for &mut A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<&mut B> for &mut A
where
    A: RelativeEq<B>
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &&mut B, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<[B]> for [A]
where
    A: RelativeEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn relative_eq(&self, other: &[B], max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len() &&
        self.len() == max_abs_diff.len() &&
        self.len() == max_relative.len() &&
        self.iter()
            .zip(other.iter())
            .zip(max_abs_diff.iter())
            .zip(max_relative.iter())
            .all(|(((a, b), abs_diff_tol), relative_tol)| a.relative_eq(b, abs_diff_tol, relative_tol))
    }
}

impl<'a, 'b, A, B> RelativeEq<&'b [B]> for &'a [A]
where
    A: RelativeEq<B>,
    A::Tolerance: Sized
{
    type Tolerance = [A::Tolerance];

    #[inline]
    fn relative_eq(&self, other: &&'b [B], max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len() &&
        self.len() == max_abs_diff.len() &&
        self.len() == max_relative.len() &&
        self.iter()
            .zip(other.iter())
            .zip(max_abs_diff.iter())
            .zip(max_relative.iter())
            .all(|(((a, b), abs_diff_tol), relative_tol)| a.relative_eq(b, abs_diff_tol, relative_tol))
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
    B: Copy
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
    B: ?Sized
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&*self.borrow(), &*other.borrow(), max_abs_diff, max_relative)
    }
}


















macro_rules! impl_relative_eq_all_float {
    ($($T:ident),* $(,)?) => {$(
        impl RelativeEqAll for $T {
            type AllTolerance = $T;

            #[inline]
            fn all_relative_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
                self.relative_eq(other, max_abs_diff, max_relative)
            }
        }
    )*}
}

impl_relative_eq_all_float!(f32, f64);


impl<A, B> RelativeEqAll<&B> for &A
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeEqAll::all_relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEqAll<&mut B> for &A
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeEqAll::all_relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEqAll<&B> for &mut A
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &&B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeEqAll::all_relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEqAll<&mut B> for &mut A
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeEqAll::all_relative_eq(*self, *other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEqAll<[B]> for [A]
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &[B], max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.all_relative_eq(b, max_abs_diff, max_relative))
    }
}

impl<'a, 'b, A, B> RelativeEqAll<&'b [B]> for &'a [A]
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len() && 
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.all_relative_eq(b, max_abs_diff, max_relative))
    }
}

impl<A, B, const N: usize> RelativeEqAll<[B; N]> for [A; N]
where
    A: RelativeEqAll<B>
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.all_relative_eq(b, max_abs_diff, max_relative))
    }
}

impl<A, B> RelativeEqAll<cell::Cell<B>> for cell::Cell<A> 
where
    A: RelativeEqAll<B> + Copy,
    B: Copy
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeEqAll::all_relative_eq(&self.get(), &other.get(), max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEqAll<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: RelativeEqAll<B> + ?Sized
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn all_relative_eq(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeEqAll::all_relative_eq(&*self.borrow(), &*other.borrow(), max_abs_diff, max_relative)
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
    A: AssertRelativeEq<B>
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
    A: AssertRelativeEq<B>
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
    A: AssertRelativeEq<B>
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
    A: AssertRelativeEq<B>
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

impl<A, B> AssertRelativeEq<[B]> for [A]
where
    A: AssertRelativeEq<B>,
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
                .map(|((a, b), tol)| { AssertRelativeEq::debug_abs_diff_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &[B], max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(max_relative.iter())
                .map(|((a, b), tol)| { AssertRelativeEq::debug_relative_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }
}

impl<'a, 'b, A, B> AssertRelativeEq<&'b [B]> for &'a [A]
where
    A: AssertRelativeEq<B>,
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
                .map(|((a, b), tol)| { AssertRelativeEq::debug_abs_diff_tolerance(a, b, tol) })
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &&'b [B], max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            Some(self.iter()
                .zip(other.iter())
                .zip(max_relative.iter())
                .map(|((a, b), tol)| { AssertRelativeEq::debug_relative_tolerance(a, b, tol) })
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

    #[inline]
    fn debug_relative_tolerance(&self, other: &[B; N], max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        let mut result: [mem::MaybeUninit<A::DebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_relative_tolerance(&other[i], &max_relative[i]));
        }

        unsafe { 
            array_assume_init(result) 
        }
    }
}

impl<A, B> AssertRelativeEq<cell::Cell<B>> for cell::Cell<A> 
where
    A: AssertRelativeEq<B> + Copy,
    B: Copy
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
    B: Copy
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




 










macro_rules! impl_assert_relative_eq_all_float {
    ($($T:ident),* $(,)?) => {$(
        impl AssertRelativeEqAll for $T {
            type AllDebugTolerance = Self::AllTolerance;

            #[inline]
            fn debug_all_abs_diff_tolerance(&self, other: &$T, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_abs_diff_tolerance(other, max_abs_diff)
            }

            #[inline]
            fn debug_all_relative_tolerance(&self, other: &$T, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
                self.debug_relative_tolerance(other, max_relative)
            }
        }
    )*};
}

impl_assert_relative_eq_all_float!(f32, f64);


impl<A, B> AssertRelativeEqAll<&B> for &A
where
    A: AssertRelativeEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &&B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEqAll<&mut B> for &A
where
    A: AssertRelativeEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &&mut B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEqAll<&B> for &mut A
where
    A: AssertRelativeEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &&B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEqAll<&mut B> for &mut A
where
    A: AssertRelativeEqAll<B>
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&mut B, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_abs_diff_tolerance(*self, *other, max_abs_diff)
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &&mut B, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(*self, *other, max_relative)
    }
}

impl<A, B> AssertRelativeEqAll<[B]> for [A]
where
    A: AssertRelativeEqAll<B>,
    A::AllTolerance: Sized,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &[B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len()  {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| { AssertRelativeEqAll::debug_all_abs_diff_tolerance(a, b, max_abs_diff) })
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &[B], max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len()  {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| { AssertRelativeEqAll::debug_all_relative_tolerance(a, b, max_relative) })
                .collect()
            )
        } else {
            None
        }
    }
}

impl<'a, 'b, A, B> AssertRelativeEqAll<&'b [B]> for &'a [A]
where
    A: AssertRelativeEqAll<B>,
    A::AllTolerance: Sized,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &&'b [B], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len()  {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| { AssertRelativeEqAll::debug_all_abs_diff_tolerance(a, b, max_abs_diff) })
                .collect()
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &&'b [B], max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len()  {
            Some(self.iter()
                .zip(other.iter())
                .map(|(a, b)| { AssertRelativeEqAll::debug_all_relative_tolerance(a, b, max_relative) })
                .collect()
            )
        } else {
            None
        }
    }
}

impl<A, B, const N: usize> AssertRelativeEqAll<[B; N]> for [A; N]
where
    A: AssertRelativeEqAll<B>,
{
    type AllDebugTolerance = [A::AllDebugTolerance; N];

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &[B; N], max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_all_abs_diff_tolerance(&other[i], max_abs_diff));
        }

        unsafe { 
            array_assume_init(result) 
        }
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &[B; N], max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let mut result: [mem::MaybeUninit<A::AllDebugTolerance>; N] = uninit_array();
        for i in 0..N {
            result[i] = mem::MaybeUninit::new(self[i].debug_all_relative_tolerance(&other[i], max_relative));
        }

        unsafe { 
            array_assume_init(result) 
        }
    }
}

impl<A, B> AssertRelativeEqAll<cell::Cell<B>> for cell::Cell<A> 
where
    A: AssertRelativeEqAll<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &cell::Cell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_abs_diff_tolerance(&self.get(), &other.get(), max_abs_diff)
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &cell::Cell<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(&self.get(), &other.get(), max_relative)
    }
}

impl<A, B> AssertRelativeEqAll<cell::RefCell<B>> for cell::RefCell<A> 
where
    A: AssertRelativeEqAll<B> + Copy,
    B: Copy
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_all_abs_diff_tolerance(&self, other: &cell::RefCell<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(&*self.borrow(), &*other.borrow(), max_abs_diff)
    }

    #[inline]
    fn debug_all_relative_tolerance(&self, other: &cell::RefCell<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeEqAll::debug_all_relative_tolerance(&*self.borrow(), &*other.borrow(), max_relative)
    }
}


#[doc(hidden)]
pub struct RelativeCmp {}

impl RelativeCmp {
    #[inline]
    pub fn eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_relative: &A::Tolerance) -> bool 
    where
        A: RelativeEq<B> + ?Sized,
        B: ?Sized
    {
        A::relative_eq(lhs, rhs, max_abs_diff, max_relative)
    }

    #[inline]
    pub fn ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance, max_relative: &A::Tolerance) -> bool 
    where
        A: RelativeEq<B> + ?Sized,
        B: ?Sized
    {
        A::relative_ne(lhs, rhs, max_abs_diff, max_relative)
    }

    #[inline]
    pub fn all_eq<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_relative: &A::AllTolerance) -> bool 
    where
        A: RelativeEqAll<B> + ?Sized,
        B: ?Sized
    {
        A::all_relative_eq(lhs, rhs, max_abs_diff, max_relative)
    }

    #[inline]
    pub fn all_ne<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance, max_relative: &A::AllTolerance) -> bool 
    where
        A: RelativeEqAll<B> + ?Sized,
        B: ?Sized
    {
        A::all_relative_ne(lhs, rhs, max_abs_diff, max_relative)
    }
}


#[doc(hidden)]
pub struct RelativeCmpOpTol {}

impl RelativeCmpOpTol {
    #[inline]
    pub fn abs_diff<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::Tolerance) -> A::DebugTolerance 
    where
        A: RelativeEq<B> + AssertRelativeEq<B>
    {
        A::debug_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn abs_diff_all<A, B>(lhs: &A, rhs: &B, max_abs_diff: &A::AllTolerance) -> A::AllDebugTolerance 
    where
        A: RelativeEqAll<B> + AssertRelativeEqAll<B>
    {
        A::debug_all_abs_diff_tolerance(lhs, rhs, max_abs_diff)
    }

    #[inline]
    pub fn relative<A, B>(lhs: &A, rhs: &B, max_relative: &A::Tolerance) -> A::DebugTolerance
    where
        A: RelativeEq<B> + AssertRelativeEq<B>
    {
        A::debug_relative_tolerance(lhs,rhs, max_relative)
    }

    #[inline]
    pub fn relative_all<A, B>(lhs: &A, rhs: &B, max_relative: &A::AllTolerance) -> A::AllDebugTolerance
    where
        A: RelativeEqAll<B> + AssertRelativeEqAll<B>
    {
        A::debug_all_relative_tolerance(lhs,rhs, max_relative)
    }
}


#[macro_export]
macro_rules! relative_eq {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, relative <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, relative_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_eq(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
}

#[macro_export]
macro_rules! relative_ne {
    ($left:expr, $right:expr, abs_diff <= $tol_1:expr, relative <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative <= $tol_2:expr, abs_diff <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, abs_diff_all <= $tol_1:expr, relative_all <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
    ($left:expr, $right:expr, relative_all <= $tol_2:expr, abs_diff_all <= $tol_1:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                $crate::RelativeCmp::all_ne(left_val, right_val, tol_1_val, tol_2_val)
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_eq!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_relative_ne {
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr $(,)?) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => {{
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::relative_ne!(*left_val, *right_val, $eq1 <= *tol_1_val, $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down. See the documentation for `core::approx_eq`.
                    panic!(concat!(
"assertion failed: `assert_relative_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2),  " <= t)`", r#"
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertRelativeEq::debug_abs_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::RelativeCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::RelativeCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+),
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! debug_assert_relative_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_relative_eq!($($arg)*); })
}

#[macro_export]
macro_rules! debug_assert_relative_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_relative_ne!($($arg)*); })
}

