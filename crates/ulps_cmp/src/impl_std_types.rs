use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::sync;


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

impl<A, B> UlpsEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            UlpsEq::ulps_eq(a, b, max_abs_diff, max_ulps)
        } else {
            false
        }
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

impl<A, B> UlpsAllEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: UlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            UlpsAllEq::ulps_all_eq(a, b, max_abs_diff, max_ulps)
        } else {
            false
        }
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

impl<A, B> AssertUlpsEq<sync::OnceLock<B>> for sync::OnceLock<A>
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
    fn debug_abs_diff(&self, other: &sync::OnceLock<B>) -> Self::DebugAbsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_abs_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &sync::OnceLock<B>) -> Self::DebugUlpsDiff {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_ulps_diff(a, b))
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_abs_diff_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &sync::OnceLock<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsEq::debug_ulps_tolerance(a, b, max_ulps))
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

impl<A, B> AssertUlpsAllEq<sync::OnceLock<B>> for sync::OnceLock<A>
where
    A: AssertUlpsAllEq<B>,
    A::AllTolerance: Sized,
    A::AllUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<A::AllDebugTolerance>;
    type AllDebugUlpsTolerance = Option<A::AllDebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &sync::OnceLock<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &sync::OnceLock<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if let (Some(a), Some(b)) = (self.get(), other.get()) {
            Some(AssertUlpsAllEq::debug_ulps_all_tolerance(a, b, max_ulps))
        } else {
            None
        }
    }
}
