use core::fmt;


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
    fn abs_diff_eq(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> bool;

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
    fn abs_diff_ne(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> bool {
        !Self::abs_diff_eq(self, other, max_abs_diff)
    }
}

pub trait AbsDiffAllEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type AllTolerance: ?Sized;

    fn abs_diff_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> bool;

    fn abs_diff_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> bool {
        !Self::abs_diff_all_eq(self, other, max_abs_diff)
    }
}
pub trait AssertAbsDiffEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized
{
    type DebugAbsDiff: fmt::Debug + Sized;
    type DebugTolerance: fmt::Debug;

    fn debug_abs_diff(&self, other: &Rhs) -> Self::DebugAbsDiff;

    fn debug_abs_diff_tolerance(&self, other: &Rhs, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance;
}

pub trait AssertAbsDiffAllEq<Rhs = Self>: AbsDiffAllEq<Rhs> 
where
    Rhs: ?Sized
{
    type AllDebugTolerance: fmt::Debug;

    fn debug_abs_diff_all_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;
}

