use crate::traits::{
    AssertRelativeAllEq,
    AssertRelativeEq,
    RelativeAllEq,
    RelativeEq,
};
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::sync;

impl<K, VA, VB, S> RelativeEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash,
    S: hash::BuildHasher,
    VA: RelativeEq<VB>,
    VA::Tolerance: Sized,
{
    type Tolerance = HashMap<K, VA::Tolerance, S>;

    #[inline]
    fn relative_eq(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_relative.len()
            && self.iter().all(|(key, a)| {
                if let Some(b) = other.get(key) {
                    if let (Some(abs_tol), Some(rel_tol)) = (max_abs_diff.get(key), max_relative.get(key)) {
                        RelativeEq::relative_eq(a, b, abs_tol, rel_tol)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }
}

impl<A, B> RelativeEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            RelativeEq::relative_eq(a, b, max_abs_diff, max_relative)
        } else {
            false
        }
    }
}

impl<K, VA, VB, S> RelativeAllEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash,
    S: hash::BuildHasher,
    VA: RelativeAllEq<VB>,
{
    type AllTolerance = VA::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, a)| {
                if let Some(b) = other.get(key) {
                    RelativeAllEq::relative_all_eq(a, b, max_abs_diff, max_relative)
                } else {
                    false
                }
            })
    }
}

impl<A, B> RelativeAllEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: RelativeAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            a.relative_all_eq(b, max_abs_diff, max_relative)
        } else {
            false
        }
    }
}

impl<K, VA, VB, S> AssertRelativeEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash + Clone + fmt::Debug,
    S: hash::BuildHasher + Clone,
    VA: AssertRelativeEq<VB>,
    VA::Tolerance: Sized,
    VA::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<HashMap<K, VA::DebugAbsDiff, S>>;
    type DebugTolerance = Option<HashMap<K, VA::DebugTolerance, S>>;

    #[inline]
    fn debug_abs_diff(&self, other: &HashMap<K, VB, S>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, val) in self {
                result.insert(key.clone(), val.debug_abs_diff(other.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, val) in self {
                result.insert(key.clone(), val.debug_abs_diff_tolerance(other.get(key)?, max_abs_diff.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &HashMap<K, VB, S>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, val) in self {
                result.insert(key.clone(), val.debug_relative_tolerance(other.get(key)?, max_relative.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AssertRelativeEq<B>,
    A::Tolerance: Sized,
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &sync::OnceLock<B>) -> Self::DebugAbsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeEq::debug_abs_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeEq::debug_abs_diff_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &sync::OnceLock<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeEq::debug_relative_tolerance(a, b, max_relative))
        } else {
            None
        }
    }
}

impl<K, VA, VB, S> AssertRelativeAllEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash + Clone + fmt::Debug,
    S: hash::BuildHasher + Clone,
    VA: AssertRelativeAllEq<VB>,
    VA::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<HashMap<K, VA::AllDebugTolerance, S>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, v) in self {
                result.insert(key.clone(), v.debug_abs_diff_all_tolerance(other.get(key)?, max_abs_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &HashMap<K, VB, S>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, v) in self {
                result.insert(key.clone(), v.debug_relative_all_tolerance(other.get(key)?, max_relative));
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeAllEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AssertRelativeAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &sync::OnceLock<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertRelativeAllEq::debug_relative_all_tolerance(a, b, max_relative))
        } else {
            None
        }
    }
}
