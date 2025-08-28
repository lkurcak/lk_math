use std::{
    iter::Sum,
    ops::{Add, Sub},
};

use crate::interval::UniversalInterval;

use super::interval::{ExclusiveMax, InclusiveMin, Interval};

/// Disjoint set of intervals.
///
/// `T` must implement `Copy` and `Ord`.
///
/// Because of the `Ord` constraint, floating point types are not supported.
/// This can be worked around by creating a wrapper type that implements `Ord`.
/// Wrappers `OrdF32` and `OrdF64` are provided in the `ord_float` module.
#[derive(Debug, PartialEq, Eq)]
pub struct IntervalSet<T> {
    pub intervals: Vec<std::ops::Range<T>>,
}

impl<T: Copy + Ord> IntervalSet<T> {
    pub fn new() -> Self {
        Self { intervals: vec![] }
    }

    pub fn intersect(&mut self, interval: std::ops::Range<T>) {
        self.intervals = self
            .intervals
            .iter()
            .filter_map(|x| x.intersection(&interval))
            .collect();
    }

    /// Remove all intervals that do not intersect with the given interval.
    pub fn retain_intersecting(&mut self, interval: std::ops::Range<T>) {
        self.intervals = self
            .intervals
            .iter()
            .filter(|x| x.intersection(&interval).is_some())
            .cloned()
            .collect();
    }

    pub fn union(&mut self, interval: std::ops::Range<T>) {
        if *interval.inclusive_min() >= *interval.exclusive_max() {
            return;
        }

        if self.intervals.is_empty() {
            self.intervals.push(interval);
            return;
        }

        let index0 = match self
            .intervals
            .binary_search_by(|x| x.inclusive_min().cmp(interval.inclusive_min()))
        {
            Ok(value) => value,
            Err(value) => value,
        };
        let index1 = match self
            .intervals
            .binary_search_by(|x| x.exclusive_max().cmp(interval.exclusive_max()))
        {
            Ok(value) => value,
            Err(value) => value,
        };

        if index0 > index1 {
            // NOTE(lubo): Already included
            return;
        }

        if index0 < index1 {
            // NOTE(lubo): We can definitely remove n = (index1 - index0) segments.
            // Segments to definitely remove:
            //  1. index0
            //  2. index0 + 1
            //  ...
            //  n. index0 + n - 1
            self.intervals.drain(index0..index1);
        }

        // NOTE(lubo): Either
        // 1. add new segment (+1 total)
        // 2. join left segment
        // 3. join right segment
        // 4. join both (-1 total)
        let index = index0;

        if index > 0 {
            let pre = self.intervals[index - 1].union(&interval);
            if let Some(mut interval) = pre {
                if index < self.intervals.len() {
                    let all_three = self.intervals[index].union(&interval);
                    if let Some(all_three) = all_three {
                        interval = all_three;
                        self.intervals.remove(index);
                    }
                }

                self.intervals[index - 1] = interval;
                return;
            }
        }

        if index < self.intervals.len() {
            let post = self.intervals[index].union(&interval);
            if let Some(post) = post {
                self.intervals[index] = post;
                return;
            }
        }

        self.intervals.insert(index, interval);
    }
}

impl<T: Copy + Ord> Default for IntervalSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Sum> IntervalSet<T> {
    pub fn measure(&self) -> T {
        self.intervals
            .iter()
            .map(|x| *x.exclusive_max() - *x.inclusive_min())
            .sum()
    }

    pub fn bounds(&self) -> Option<std::ops::Range<T>> {
        let count = self.intervals.len();
        if count > 0 {
            Some(*self.intervals[0].inclusive_min()..*self.intervals[count - 1].exclusive_max())
        } else {
            None
        }
    }

    /// Negation of the set of intervals.
    ///
    /// The negation of an empty set is the entire domain (the "universal interval").
    /// This requires the notion of "most extreme values" for the type `T`.
    /// For example, the most extreme values for `i32` are `i32::MIN` and `i32::MAX`.
    /// For `f32`, the most extreme values would be `f32::NEG_INFINITY` and `f32::INFINITY`.
    /// (Although `f32` cannot be used since it does not implement `Ord`. See [`crate::ord_float::OrdF32`].)
    /// These bounds are defined in the [`UniversalInterval`] trait which is required for
    /// this function.
    ///
    /// See [`negation_within_bounds`] for a version that does not require universal bounds.
    pub fn negation(&self) -> Self
    where
        T: UniversalInterval,
    {
        let count = self.intervals.len();

        if count > 0 {
            let mut negated = vec![];

            if !self.intervals[0].inclusive_min().is_infinum() {
                negated.push(T::INFINUM..*self.intervals[0].inclusive_min());
            }

            for i in 0..count - 1 {
                negated.push(
                    *self.intervals[i].exclusive_max()..*self.intervals[i + 1].inclusive_min(),
                )
            }

            if !self.intervals[count - 1].exclusive_max().is_supremum() {
                negated.push(*self.intervals[count - 1].exclusive_max()..T::SUPREMUM);
            }

            Self { intervals: negated }
        } else {
            Self {
                intervals: vec![T::universal_interval()],
            }
        }
    }

    pub fn negation_within_bounds(&self) -> Self {
        let count = self.intervals.len();

        if count > 0 {
            let mut negated = vec![];

            for i in 0..count - 1 {
                negated.push(
                    *self.intervals[i].exclusive_max()..*self.intervals[i + 1].inclusive_min(),
                )
            }

            Self { intervals: negated }
        } else {
            Self { intervals: vec![] }
        }
    }
}

impl<T: Copy + Ord> IntervalSet<T> {
    pub fn containing_interval(&self, value: &T) -> Option<std::ops::Range<T>> {
        let index0 = match self
            .intervals
            .binary_search_by(|probe| probe.exclusive_max().cmp(value))
        {
            Ok(value) => value,
            Err(value) => value,
        };
        if let Some(a) = self.intervals.get(index0) {
            if a.contains(value) {
                Some(a.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.containing_interval(value).is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        interval_set::IntervalSet,
        ord_float::{OrdF32, OrdF64},
    };

    #[test]
    fn empty() {
        let set = IntervalSet::<i32>::new();
        assert_eq!(set.measure(), 0);
        assert_eq!(set.bounds(), None);
        assert!(set.negation_within_bounds().intervals.is_empty());

        assert_eq!(set.negation().intervals, vec![-2147483648..2147483647]);
        assert_eq!(set.negation().negation(), set);

        assert!(!set.contains(&i32::MIN));
        assert!(!set.contains(&-1));
        assert!(!set.contains(&0));
        assert!(!set.contains(&1));
        assert!(!set.contains(&i32::MAX));
    }

    #[test]
    fn i32() {
        let a = 0..2;
        let b = 1..3;
        let mut set = IntervalSet::new();
        set.union(a);
        set.union(b);
        assert_eq!(set.measure(), 3);

        assert_eq!(
            set.negation().intervals,
            vec![-2147483648..0, 3..2147483647]
        );
        assert_eq!(set.negation().negation(), set);

        assert!(!set.contains(&i32::MIN));
        assert!(!set.contains(&-1));
        assert!(set.contains(&0));
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
        assert!(!set.contains(&i32::MAX));
    }

    #[test]
    fn f32() {
        let a = OrdF32(0.0)..OrdF32(2.0);
        let b = OrdF32(1.0)..OrdF32(3.0);
        let mut set = IntervalSet::new();
        set.union(a);
        set.union(b);
        assert_eq!(*set.measure(), 3.0);

        assert_eq!(
            set.negation().intervals,
            vec![
                OrdF32(f32::NEG_INFINITY)..OrdF32(0.0),
                OrdF32(3.0)..OrdF32(f32::INFINITY)
            ]
        );
        assert_eq!(set.negation().negation(), set);

        assert!(!set.contains(&OrdF32(f32::NEG_INFINITY)));
        assert!(!set.contains(&OrdF32(f32::MIN)));
        assert!(!set.contains(&OrdF32(-1.0)));
        assert!(!set.contains(&OrdF32(-f32::EPSILON)));
        assert!(set.contains(&OrdF32(0.0)));
        assert!(set.contains(&OrdF32(1.0)));
        assert!(set.contains(&OrdF32(2.0)));
        assert!(set.contains(&OrdF32(2.999)));
        assert!(!set.contains(&OrdF32(3.0)));
        assert!(!set.contains(&OrdF32(f32::MAX)));
        assert!(!set.contains(&OrdF32(f32::INFINITY)));
    }

    #[test]
    fn f64() {
        let a = OrdF64(0.0)..OrdF64(2.0);
        let b = OrdF64(1.0)..OrdF64(3.0);
        let mut set = IntervalSet::new();
        set.union(a);
        set.union(b);
        assert_eq!(*set.measure(), 3.0);

        assert_eq!(
            set.negation().intervals,
            vec![
                OrdF64(f64::NEG_INFINITY)..OrdF64(0.0),
                OrdF64(3.0)..OrdF64(f64::INFINITY)
            ]
        );
        assert_eq!(set.negation().negation(), set);

        assert!(!set.contains(&OrdF64(f64::NEG_INFINITY)));
        assert!(!set.contains(&OrdF64(f64::MIN)));
        assert!(!set.contains(&OrdF64(-1.0)));
        assert!(!set.contains(&OrdF64(-f64::EPSILON)));
        assert!(set.contains(&OrdF64(0.0)));
        assert!(set.contains(&OrdF64(1.0)));
        assert!(set.contains(&OrdF64(2.0)));
        assert!(set.contains(&OrdF64(2.999)));
        assert!(!set.contains(&OrdF64(3.0)));
        assert!(!set.contains(&OrdF64(f64::MAX)));
        assert!(!set.contains(&OrdF64(f64::INFINITY)));
    }
}
