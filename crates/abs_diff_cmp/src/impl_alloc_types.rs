use crate::traits::{
    AbsDiffAllEq,
    AbsDiffEq,
    AssertAbsDiffAllEq,
    AssertAbsDiffEq,
};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;

impl<A, B> AbsDiffEq<Box<B>> for Box<A>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<Rc<B>> for Rc<A>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<Arc<B>> for Arc<A>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized,
{
    type Tolerance = A::Tolerance;

    #[inline]
    fn abs_diff_eq(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> bool {
        AbsDiffEq::abs_diff_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffEq<Vec<B>> for Vec<A>
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = Vec<A::Tolerance>;

    #[inline]
    fn abs_diff_eq(&self, other: &Vec<B>, max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff)
                .all(|((a, b), tol)| AbsDiffEq::abs_diff_eq(a, b, tol))
    }
}

impl<A, B> AbsDiffEq<VecDeque<B>> for VecDeque<A>
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = VecDeque<A::Tolerance>;

    #[inline]
    fn abs_diff_eq(&self, other: &VecDeque<B>, max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff)
                .all(|((a, b), tol)| AbsDiffEq::abs_diff_eq(a, b, tol))
    }
}

impl<A, B> AbsDiffEq<LinkedList<B>> for LinkedList<A>
where
    A: AbsDiffEq<B>,
    A::Tolerance: Sized,
{
    type Tolerance = LinkedList<A::Tolerance>;

    #[inline]
    fn abs_diff_eq(&self, other: &LinkedList<B>, max_abs_diff: &Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.len() == max_abs_diff.len()
            && self
                .iter()
                .zip(other.iter())
                .zip(max_abs_diff)
                .all(|((a, b), tol)| AbsDiffEq::abs_diff_eq(a, b, tol))
    }
}

impl<K, VA, VB> AbsDiffEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: AbsDiffEq<VB>,
    VA::Tolerance: Sized,
{
    type Tolerance = BTreeMap<K, VA::Tolerance>;

    #[inline]
    fn abs_diff_eq(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::Tolerance) -> bool {
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

impl<A, B> AbsDiffAllEq<Box<B>> for Box<A>
where
    A: AbsDiffAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Rc<B>> for Rc<A>
where
    A: AbsDiffAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Arc<B>> for Arc<A>
where
    A: AbsDiffAllEq<B> + ?Sized,
    B: ?Sized,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        AbsDiffAllEq::abs_diff_all_eq(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AbsDiffAllEq<Vec<B>> for Vec<A>
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Vec<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| AbsDiffAllEq::abs_diff_all_eq(a, b, max_abs_diff))
    }
}

impl<A, B> AbsDiffAllEq<VecDeque<B>> for VecDeque<A>
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &VecDeque<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| AbsDiffAllEq::abs_diff_all_eq(a, b, max_abs_diff))
    }
}

impl<A, B> AbsDiffAllEq<LinkedList<B>> for LinkedList<A>
where
    A: AbsDiffAllEq<B>,
{
    type AllTolerance = A::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &LinkedList<B>, max_abs_diff: &Self::AllTolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| AbsDiffAllEq::abs_diff_all_eq(a, b, max_abs_diff))
    }
}

impl<K, VA, VB> AbsDiffAllEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord,
    VA: AbsDiffAllEq<VB>,
{
    type AllTolerance = VA::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &BTreeMap<K, VB>, max_abs_diff: &Self::AllTolerance) -> bool {
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

impl<A, B> AssertAbsDiffEq<Box<B>> for Box<A>
where
    A: AssertAbsDiffEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Box<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Rc<B>> for Rc<A>
where
    A: AssertAbsDiffEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Rc<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Arc<B>> for Arc<A>
where
    A: AssertAbsDiffEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type DebugAbsDiff = A::DebugAbsDiff;
    type DebugTolerance = A::DebugTolerance;

    #[inline]
    fn debug_abs_diff(&self, other: &Arc<B>) -> Self::DebugAbsDiff {
        AssertAbsDiffEq::debug_abs_diff(&**self, &**other)
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        AssertAbsDiffEq::debug_abs_diff_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffEq<Vec<B>> for Vec<A>
where
    A: AssertAbsDiffEq<B> + ?Sized + Copy,
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
                    .map(|(a, b)| AssertAbsDiffEq::debug_abs_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Vec<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if self.len() == other.len() && self.len() == max_abs_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertAbsDiffEq<VecDeque<B>> for VecDeque<A>
where
    A: AssertAbsDiffEq<B> + ?Sized + Copy,
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
                    .map(|(a, b)| AssertAbsDiffEq::debug_abs_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &VecDeque<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if self.len() == other.len() && self.len() == max_abs_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertAbsDiffEq<LinkedList<B>> for LinkedList<A>
where
    A: AssertAbsDiffEq<B> + ?Sized + Copy,
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
                    .map(|(a, b)| AssertAbsDiffEq::debug_abs_diff(a, b))
                    .collect(),
            )
        } else {
            None
        }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &LinkedList<B>, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        if self.len() == other.len() && self.len() == max_abs_diff.len() {
            Some(
                self.iter()
                    .zip(other.iter())
                    .zip(max_abs_diff.iter())
                    .map(|((a, b), tol)| AssertAbsDiffEq::debug_abs_diff_tolerance(a, b, tol))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<K, VA, VB> AssertAbsDiffEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertAbsDiffEq<VB>,
    VA::Tolerance: Sized,
    VA::DebugTolerance: Sized,
{
    type DebugAbsDiff = Option<BTreeMap<K, VA::DebugAbsDiff>>;
    type DebugTolerance = Option<BTreeMap<K, VA::DebugTolerance>>;

    #[inline]
    fn debug_abs_diff(&self, other: &BTreeMap<K, VB>) -> Self::DebugAbsDiff {
        if self.len() == other.len() {
            let mut result = BTreeMap::new();
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_diff(other.get(k)?));
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
            for (k, v) in self {
                result.insert(k.clone(), v.debug_abs_diff_tolerance(other.get(k)?, max_abs_diff.get(k)?));
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<A, B> AssertAbsDiffAllEq<Box<B>> for Box<A>
where
    A: AssertAbsDiffAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Box<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Rc<B>> for Rc<A>
where
    A: AssertAbsDiffAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Rc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Arc<B>> for Arc<A>
where
    A: AssertAbsDiffAllEq<B> + ?Sized + Copy,
    B: ?Sized + Copy,
{
    type AllDebugTolerance = A::AllDebugTolerance;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Arc<B>, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&**self, &**other, max_abs_diff)
    }
}

impl<A, B> AssertAbsDiffAllEq<Vec<B>> for Vec<A>
where
    A: AssertAbsDiffAllEq<B> + ?Sized + Copy,
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
                    .map(|(a, b)| AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertAbsDiffAllEq<VecDeque<B>> for VecDeque<A>
where
    A: AssertAbsDiffAllEq<B> + ?Sized + Copy,
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
                    .map(|(a, b)| AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<A, B> AssertAbsDiffAllEq<LinkedList<B>> for LinkedList<A>
where
    A: AssertAbsDiffAllEq<B> + ?Sized + Copy,
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
                    .map(|(a, b)| AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(a, b, max_abs_diff))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<K, VA, VB> AssertAbsDiffAllEq<BTreeMap<K, VB>> for BTreeMap<K, VA>
where
    K: Eq + Ord + Clone + fmt::Debug,
    VA: AssertAbsDiffAllEq<VB>,
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
}
