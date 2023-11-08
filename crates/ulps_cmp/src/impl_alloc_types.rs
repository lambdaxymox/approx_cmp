use crate::traits::{
    AssertUlpsAllEq,
    AssertUlpsEq,
    UlpsAllEq,
    UlpsEq,
};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;


impl<A, B> UlpsEq<Box<B>> for Box<A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<Rc<B>> for Rc<A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<Arc<B>> for Arc<A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;
    type UlpsTolerance = A::UlpsTolerance;

    #[inline]
    fn ulps_eq(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        UlpsEq::ulps_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsEq<Vec<B>> for Vec<A>
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = Vec<A::Tolerance>;
    type UlpsTolerance = Vec<A::UlpsTolerance>;

    #[inline]
    fn ulps_eq(&self, other: &Vec<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_ulps.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_ulps.iter())
                .all(|(((a, b), abs_tol), ulps_tol)| UlpsEq::ulps_eq(a, b, abs_tol, ulps_tol))
    }
}

impl<A, B> UlpsEq<VecDeque<B>> for VecDeque<A>
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = VecDeque<A::Tolerance>;
    type UlpsTolerance = VecDeque<A::UlpsTolerance>;

    #[inline]
    fn ulps_eq(&self, other: &VecDeque<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_ulps.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_ulps.iter())
                .all(|(((a, b), abs_tol), ulps_tol)| UlpsEq::ulps_eq(a, b, abs_tol, ulps_tol))
    }
}

impl<A, B> UlpsEq<LinkedList<B>> for LinkedList<A>
where
    A: UlpsEq<B>,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
{
    type Tolerance = LinkedList<A::Tolerance>;
    type UlpsTolerance = LinkedList<A::UlpsTolerance>;

    #[inline]
    fn ulps_eq(&self, other: &LinkedList<B>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_ulps.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_ulps.iter())
                .all(|(((a, b), abs_tol), ulps_tol)| UlpsEq::ulps_eq(a, b, abs_tol, ulps_tol))
    }
}

impl<K, VA, VB> UlpsEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: UlpsEq<VB>,
    VA::Tolerance: Sized,
    VA::UlpsTolerance: Sized,
{
    type Tolerance = BTreeMap<K, VA::Tolerance>;
    type UlpsTolerance = BTreeMap<K, VA::UlpsTolerance>;

    #[inline]
    fn ulps_eq(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
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


impl<A, B> UlpsAllEq<Box<B>> for Box<A>
where
    A: UlpsAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<Rc<B>> for Rc<A>
where
    A: UlpsAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<Arc<B>> for Arc<A>
where
    A: UlpsAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        UlpsAllEq::ulps_all_eq(&**self, &**other, max_abs_diff, max_ulps)
    }
}

impl<A, B> UlpsAllEq<Vec<B>> for Vec<A>
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Vec<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| UlpsAllEq::ulps_all_eq(a, b, max_abs_diff, max_ulps))
    }
}

impl<A, B> UlpsAllEq<VecDeque<B>> for VecDeque<A>
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &VecDeque<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| UlpsAllEq::ulps_all_eq(a, b, max_abs_diff, max_ulps))
    }
}

impl<A, B> UlpsAllEq<LinkedList<B>> for LinkedList<A>
where
    A: UlpsAllEq<B>,
{
    type AllTolerance = A::AllTolerance;
    type AllUlpsTolerance = A::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &LinkedList<B>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| UlpsAllEq::ulps_all_eq(a, b, max_abs_diff, max_ulps))
    }
}

impl<K, VA, VB> UlpsAllEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: UlpsAllEq<VB>,
{
    type AllTolerance = VA::AllTolerance;
    type AllUlpsTolerance = VA::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
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


impl<A, B> AssertUlpsEq<Box<B>> for Box<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Box<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Box<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Box<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<Rc<B>> for Rc<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Rc<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Rc<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Rc<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<Arc<B>> for Arc<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugUlpsDiff = A::DebugUlpsDiff;
    type DebugTolerance = A::DebugTolerance;
    type DebugUlpsTolerance = A::DebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Arc<B>) -> Self::DebugAbsDiff {
        AssertUlpsEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Arc<B>) -> Self::DebugUlpsDiff {
        AssertUlpsEq::debug_ulps_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertUlpsEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Arc<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        AssertUlpsEq::debug_ulps_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsEq<Vec<B>> for Vec<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
    A::DebugTolerance: Sized,
    A::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugUlpsDiff = Option<Vec<A::DebugUlpsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;
    type DebugUlpsTolerance = Option<Vec<A::DebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &Vec<B>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsEq::debug_abs_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Vec<B>) -> Self::DebugUlpsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsEq::debug_ulps_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Vec<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Vec<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if (self.len() == other.len()) && (self.len() == max_ulps.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_ulps.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_ulps_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertUlpsEq<VecDeque<B>> for VecDeque<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
    A::DebugTolerance: Sized,
    A::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<VecDeque<A::DebugAbsDiff>>;
    type DebugUlpsDiff = Option<VecDeque<A::DebugUlpsDiff>>;
    type DebugTolerance = Option<VecDeque<A::DebugTolerance>>;
    type DebugUlpsTolerance = Option<VecDeque<A::DebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &VecDeque<B>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsEq::debug_abs_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &VecDeque<B>) -> Self::DebugUlpsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsEq::debug_ulps_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &VecDeque<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &VecDeque<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if (self.len() == other.len()) && (self.len() == max_ulps.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_ulps.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_ulps_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertUlpsEq<LinkedList<B>> for LinkedList<A>
where
    A: AssertUlpsEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::Tolerance: Sized,
    A::UlpsTolerance: Sized,
    A::DebugTolerance: Sized,
    A::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<LinkedList<A::DebugAbsDiff>>;
    type DebugUlpsDiff = Option<LinkedList<A::DebugUlpsDiff>>;
    type DebugTolerance = Option<LinkedList<A::DebugTolerance>>;
    type DebugUlpsTolerance = Option<LinkedList<A::DebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &LinkedList<B>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsEq::debug_abs_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &LinkedList<B>) -> Self::DebugUlpsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsEq::debug_ulps_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &LinkedList<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_abs_diff.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &LinkedList<B>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if (self.len() == other.len()) && (self.len() == max_ulps.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_ulps.iter())
                    .map(|((a, b), tol)| AssertUlpsEq::debug_ulps_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<K, VA, VB> AssertUlpsEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertUlpsEq<VB>,
    VA::Tolerance: Sized,
    VA::UlpsTolerance: Sized,
    VA::DebugTolerance: Sized,
    VA::DebugUlpsTolerance: Sized,
{
    type DebugAbsDiff = Option<BTreeMap<K, VA::DebugAbsDiff>>;
    type DebugUlpsDiff = Option<BTreeMap<K, VA::DebugUlpsDiff>>;
    type DebugTolerance = Option<BTreeMap<K, VA::DebugTolerance>>;
    type DebugUlpsTolerance = Option<BTreeMap<K, VA::DebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &BTreeMap<K, VB>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_abs_diff(other.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &BTreeMap<K, VB>) -> Self::DebugUlpsDiff {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_ulps_diff(other.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if self.len() == other.len() && self.len() == max_abs_diff.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_abs_diff_tolerance(other.get(key)?, max_abs_diff.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &BTreeMap<K, VB>, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        if self.len() == other.len() && self.len() == max_ulps.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_ulps_tolerance(other.get(key)?, max_ulps.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }
}


impl<A, B> AssertUlpsAllEq<Box<B>> for Box<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Box<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<Rc<B>> for Rc<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Rc<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<Arc<B>> for Arc<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;
    type AllDebugUlpsTolerance = A::AllDebugUlpsTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertUlpsAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Arc<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        AssertUlpsAllEq::debug_ulps_all_tolerance(&**self, &**other, max_ulps)
    }
}

impl<A, B> AssertUlpsAllEq<Vec<B>> for Vec<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::AllDebugTolerance: Sized,
    A::AllDebugUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;
    type AllDebugUlpsTolerance = Option<Vec<A::AllDebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Vec<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Vec<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_ulps_all_tolerance(a, b, max_ulps))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertUlpsAllEq<VecDeque<B>> for VecDeque<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::AllDebugTolerance: Sized,
    A::AllDebugUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<VecDeque<A::AllDebugTolerance>>;
    type AllDebugUlpsTolerance = Option<VecDeque<A::AllDebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &VecDeque<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &VecDeque<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_ulps_all_tolerance(a, b, max_ulps))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertUlpsAllEq<LinkedList<B>> for LinkedList<A>
where
    A: AssertUlpsAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::AllDebugTolerance: Sized,
    A::AllDebugUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<LinkedList<A::AllDebugTolerance>>;
    type AllDebugUlpsTolerance = Option<LinkedList<A::AllDebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &LinkedList<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &LinkedList<B>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertUlpsAllEq::debug_ulps_all_tolerance(a, b, max_ulps))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<K, VA, VB> AssertUlpsAllEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertUlpsAllEq<VB>,
    VA::AllDebugTolerance: Sized,
    VA::AllDebugUlpsTolerance: Sized,
{
    type AllDebugTolerance = Option<BTreeMap<K, VA::AllDebugTolerance>>;
    type AllDebugUlpsTolerance = Option<BTreeMap<K, VA::AllDebugUlpsTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_abs_diff_all_tolerance(other.get(key)?, max_abs_diff));
            }
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &BTreeMap<K, VB>, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_ulps_all_tolerance(other.get(key)?, max_ulps));
            }
            Some(result)
        } else {
            None
        }
    }
}
