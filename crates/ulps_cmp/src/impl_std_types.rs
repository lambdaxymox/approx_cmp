use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};
use std::fmt;
use std::hash;
use std::collections::HashMap;


impl<K, VA, VB, S> UlpsEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash,
    S: hash::BuildHasher,
    VA: UlpsEq<VB>,
    VA::Tolerance: Sized,
    VA::UlpsTolerance: Sized,
{
    type Tolerance = HashMap<K, VA::Tolerance, S>;
    type UlpsTolerance = HashMap<K, VA::UlpsTolerance, S>;

    #[inline]
    fn ulps_eq(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_ulps.len()
            && self.iter().all(|(key, a)| {
                if let Some(b) = other.get(key) {
                    if let (Some(abs_tol), Some(ulps_tol)) = (max_abs_diff.get(key), max_ulps.get(key)) {
                        UlpsEq::ulps_eq(a, b, abs_tol, ulps_tol)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }
}

impl<K, VA, VB, S> UlpsAllEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash,
    S: hash::BuildHasher,
    VA: UlpsAllEq<VB>,
{
    type AllTolerance = VA::AllTolerance;
    type AllUlpsTolerance = VA::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &HashMap<K, VB, S>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, a)| {
                if let Some(b) = other.get(key) {
                    UlpsAllEq::ulps_all_eq(a, b, max_abs_diff, max_ulps)
                } else {
                    false
                }
            })
    }
}

impl<K, VA, VB, S> AssertUlpsEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash + Clone + fmt::Debug,
    S: hash::BuildHasher + Clone,
    VA: AssertUlpsEq<VB>,
    VA::Tolerance: Sized,
    VA::UlpsTolerance: Sized,
    VA::DebugTolerance: Sized,
    VA::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<HashMap<K, VA::DebugAbsDiff, S>>;
    type DebugUlpsDiff = Option<HashMap<K, VA::DebugUlpsDiff, S>>;
    type DebugTolerance = Option<HashMap<K, VA::DebugTolerance, S>>;
    type DebugUlpsTolerance = Option<HashMap<K, VA::DebugUlpsTolerance, S>>;

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
    fn debug_ulps_diff(&self, other: &HashMap<K, VB, S>) -> Self::DebugUlpsDiff {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, val) in self {
                result.insert(key.clone(), val.debug_ulps_diff(other.get(key)?));
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
    fn debug_ulps_tolerance(&self, other: &HashMap<K, VB, S>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if (self.len() == other.len()) && (self.len() == max_ulps.len()) {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, val) in self {
                result.insert(key.clone(), val.debug_ulps_tolerance(other.get(key)?, max_ulps.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<K, VA, VB, S> AssertUlpsAllEq<HashMap<K, VB, S>> for HashMap<K, VA, S>
where
    K: Eq + hash::Hash + Clone + fmt::Debug,
    S: hash::BuildHasher + Clone,
    VA: AssertUlpsAllEq<VB>,
    VA::AllDebugTolerance: Sized,
    VA::AllDebugUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<HashMap<K, VA::AllDebugTolerance, S>>;
    type AllDebugUlpsTolerance = Option<HashMap<K, VA::AllDebugUlpsTolerance, S>>;

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
    fn debug_ulps_all_tolerance(&self, other: &HashMap<K, VB, S>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if self.len() == other.len() {
            let mut result = HashMap::with_hasher(self.hasher().clone());
            for (key, v) in self {
                result.insert(key.clone(), v.debug_ulps_all_tolerance(other.get(key)?, max_ulps));
            }
            Some(result)
        } else {
            None
        }
    }
}
