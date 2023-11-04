use crate::traits::{
    AbsDiffAllEq,
    AbsDiffEq,
    AssertAbsDiffAllEq,
    AssertAbsDiffEq,
};
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::sync;


impl<K, VA, VB, S> AbsDiffEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash,
    S: hash::BuildHasher,
    VA: AbsDiffEq<VB>,
    VA::Tolerance: Sized,
{
    type Tolerance = HashMap<K, VA::Tolerance, S>;

    #[inline]
    fn abs_diff_eq(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.iter().all(|(key, a)| {
                if let Some(b) = other.get(key) {
                    if let Some(tol) = max_abs_diff.get(key) {
                        AbsDiffEq::abs_diff_eq(a, b, tol)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }
}


impl<A, B> AbsDiffEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::Tolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            AbsDiffEq::abs_diff_eq(a, b, max_abs_diff)
        } else {
            false
        }
    }
}


impl<K, VA, VB, S> AbsDiffAllEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash,
    S: hash::BuildHasher,
    VA: AbsDiffAllEq<VB>,
{
    type AllTolerance = VA::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, a)| {
                if let Some(b) = other.get(key) {
                    AbsDiffAllEq::abs_diff_all_eq(a, b, max_abs_diff)
                } else {
                    false
                }
            })
    }
}

impl<A, B> AbsDiffAllEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AbsDiffAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            AbsDiffAllEq::abs_diff_all_eq(a, b, max_abs_diff)
        } else {
            false
        }
    }
}


impl<K, VA, VB, S> AssertAbsDiffEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash + Clone + fmt::Debug,
    S: hash::BuildHasher + Clone,
    VA: AssertAbsDiffEq<VB>,
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
}

impl<A, B> AssertAbsDiffEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AssertAbsDiffEq<B>,
    A::Tolerance: Sized,
{
    type DebugAbsDiff = Option<A::DebugAbsDiff>;
    type DebugTolerance = Option<A::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &sync::OnceLock<B>) -> Self::DebugAbsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertAbsDiffEq::debug_abs_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }
}


impl<K, VA, VB, S> AssertAbsDiffAllEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash + Clone + fmt::Debug,
    S: hash::BuildHasher + Clone,
    VA: AssertAbsDiffAllEq<VB>,
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
}

impl<A, B> AssertAbsDiffAllEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AssertAbsDiffAllEq<B>,
    A::AllTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }
}
