use crate::relative::{
    AssertRelativeAllEq,
    AssertRelativeEq,
    RelativeAllEq,
    RelativeEq,
};
use std::boxed::Box;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;
use std::hash;
use std::fmt;


impl<A, B> RelativeEq<Box<B>> for Box<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<Rc<B>> for Rc<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<Arc<B>> for Arc<A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn relative_eq(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        RelativeEq::relative_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeEq<Vec<B>> for Vec<A>
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = Vec<A::Tolerance>;

    #[inline]
    fn relative_eq(&self, other: &Vec<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_relative.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_relative.iter())
                .all(|(((a, b), abs_tol), rel_tol)| RelativeEq::relative_eq(a, b, abs_tol, rel_tol))
    }
}

impl<A, B> RelativeEq<VecDeque<B>> for VecDeque<A>
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = VecDeque<A::Tolerance>;

    #[inline]
    fn relative_eq(&self, other: &VecDeque<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_relative.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_relative.iter())
                .all(|(((a, b), abs_tol), rel_tol)| RelativeEq::relative_eq(a, b, abs_tol, rel_tol))
    }
}

impl<A, B> RelativeEq<LinkedList<B>> for LinkedList<A>
where
    A: RelativeEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = LinkedList<A::Tolerance>;

    #[inline]
    fn relative_eq(&self, other: &LinkedList<B>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self.len() == max_relative.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff.iter())
                .zip(max_relative.iter())
                .all(|(((a, b), abs_tol), rel_tol)| RelativeEq::relative_eq(a, b, abs_tol, rel_tol))
    }
}

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

impl<K, VA, VB> RelativeEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: RelativeEq<VB>,
    VA::Tolerance: Sized,
{
    type Tolerance = BTreeMap<K, VA::Tolerance>;

    #[inline]
    fn relative_eq(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
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


impl<A, B> RelativeAllEq<Box<B>> for Box<A>
where
    A: RelativeAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Rc<B>> for Rc<A>
where
    A: RelativeAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Arc<B>> for Arc<A>
where
    A: RelativeAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        RelativeAllEq::relative_all_eq(&**self, &**other, max_abs_diff, max_relative)
    }
}

impl<A, B> RelativeAllEq<Vec<B>> for Vec<A>
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Vec<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| RelativeAllEq::relative_all_eq(a, b, max_abs_diff, max_relative))
    }
}

impl<A, B> RelativeAllEq<VecDeque<B>> for VecDeque<A>
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &VecDeque<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| RelativeAllEq::relative_all_eq(a, b, max_abs_diff, max_relative))
    }
}

impl<A, B> RelativeAllEq<LinkedList<B>> for LinkedList<A>
where
    A: RelativeAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &LinkedList<B>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| RelativeAllEq::relative_all_eq(a, b, max_abs_diff, max_relative))
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

impl<K, VA, VB> RelativeAllEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: RelativeAllEq<VB>,
{
    type AllTolerance = VA::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
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


impl<A, B> AssertRelativeEq<Box<B>> for Box<A>
where
    A: AssertRelativeEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Box<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Box<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<Rc<B>> for Rc<A>
where
    A: AssertRelativeEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Rc<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Rc<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<Arc<B>> for Arc<A>
where
    A: AssertRelativeEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Arc<B>) -> Self::DebugAbsDiff {
        AssertRelativeEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Arc<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        AssertRelativeEq::debug_relative_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeEq<Vec<B>> for Vec<A>
where
    A: AssertRelativeEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<Vec<A::DebugAbsDiff>>;
    type DebugTolerance = Option<Vec<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &Vec<B>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeEq::debug_abs_diff(a, b))
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
                    .map(|((a, b), tol)| AssertRelativeEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Vec<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_relative.iter())
                    .map(|((a, b), tol)| AssertRelativeEq::debug_relative_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeEq<VecDeque<B>> for VecDeque<A>
where
    A: AssertRelativeEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<VecDeque<A::DebugAbsDiff>>;
    type DebugTolerance = Option<VecDeque<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &VecDeque<B>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeEq::debug_abs_diff(a, b))
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
                    .map(|((a, b), tol)| AssertRelativeEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &VecDeque<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_relative.iter())
                    .map(|((a, b), tol)| AssertRelativeEq::debug_relative_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeEq<LinkedList<B>> for LinkedList<A>
where
    A: AssertRelativeEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::Tolerance: Sized,
    A::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<LinkedList<A::DebugAbsDiff>>;
    type DebugTolerance = Option<LinkedList<A::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &LinkedList<B>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeEq::debug_abs_diff(a, b))
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
                    .map(|((a, b), tol)| AssertRelativeEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &LinkedList<B>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if (self.len() == other.len()) && (self.len() == max_relative.len()) {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_relative.iter())
                    .map(|((a, b), tol)| AssertRelativeEq::debug_relative_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
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

impl<K, VA, VB> AssertRelativeEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertRelativeEq<VB>,
    VA::Tolerance: Sized,
    VA::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<BTreeMap<K, VA::DebugAbsDiff>>;
    type DebugTolerance = Option<BTreeMap<K, VA::DebugTolerance>>;

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
    fn debug_relative_tolerance(&self, other: &BTreeMap<K, VB>, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        if self.len() == other.len() && self.len() == max_relative.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_relative_tolerance(other.get(key)?, max_relative.get(key)?));
            }
            Some(result)
        } else {
            None
        }
    }
}


impl<A, B> AssertRelativeAllEq<Box<B>> for Box<A>
where
    A: AssertRelativeAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Box<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Rc<B>> for Rc<A>
where
    A: AssertRelativeAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Rc<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Arc<B>> for Arc<A>
where
    A: AssertRelativeAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_abs_diff)
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Arc<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertRelativeAllEq::debug_relative_all_tolerance(&**self, &**other, max_relative)
    }
}

impl<A, B> AssertRelativeAllEq<Vec<B>> for Vec<A>
where
    A: AssertRelativeAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<Vec<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Vec<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Vec<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_relative_all_tolerance(a, b, max_relative))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeAllEq<VecDeque<B>> for VecDeque<A>
where
    A: AssertRelativeAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<VecDeque<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &VecDeque<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &VecDeque<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_relative_all_tolerance(a, b, max_relative))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertRelativeAllEq<LinkedList<B>> for LinkedList<A>
where
    A: AssertRelativeAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
    A::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<LinkedList<A::AllDebugTolerance>>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &LinkedList<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &LinkedList<B>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| AssertRelativeAllEq::debug_relative_all_tolerance(a, b, max_relative))
                    .collect(),
            )
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

impl<K, VA, VB> AssertRelativeAllEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertRelativeAllEq<VB>,
    VA::AllDebugTolerance: Sized,
{
    type AllDebugTolerance = Option<BTreeMap<K, VA::AllDebugTolerance>>;

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
    fn debug_relative_all_tolerance(&self, other: &BTreeMap<K, VB>, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (key, v) in self {
                result.insert(key.clone(), v.debug_relative_all_tolerance(other.get(key)?, max_relative));
            }
            Some(result)
        } else {
            None
        }
    }
}

