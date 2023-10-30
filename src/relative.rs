use core::fmt;


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

pub trait RelativeAllEq<Rhs = Self>
where
    Rhs: ?Sized
{
    type AllTolerance: ?Sized;

    fn relative_all_eq(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool;

    fn relative_all_ne(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        !Self::relative_all_eq(self, other, max_abs_diff, max_relative)
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

pub trait AssertRelativeAllEq<Rhs = Self>: RelativeAllEq<Rhs> 
where
    Rhs: ?Sized
{
    type AllDebugTolerance: fmt::Debug;

    fn debug_abs_diff_all_tolerance(&self, other: &Rhs, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance;

    fn debug_relative_all_tolerance(&self, other: &Rhs, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance;
}

