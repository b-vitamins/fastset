use super::core::Set;
use std::collections::HashSet;

/// Provides operations common to sets, such as containment check, iteration, and finding the maximum value.
pub trait SetOps {
    /// Checks whether the set contains the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for containment.
    ///
    /// # Returns
    ///
    /// `true` if the set contains the value, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::{Set, SetOps, set};
    ///
    /// let set = set![42];
    /// assert!(set.contains(&42));
    /// assert!(!set.contains(&100));
    /// ```
    fn contains(&self, value: &usize) -> bool;

    /// Returns an iterator over the elements of the set.
    ///
    /// # Returns
    ///
    /// A boxed iterator yielding references to the elements of the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::{Set, SetOps, set};
    ///
    /// let set = set![1, 2, 3, 4, 5];
    /// for item in set.iter() {
    ///     println!("{}", *item);
    /// }
    /// ```
    fn iter(&self) -> Box<dyn Iterator<Item = &usize> + '_>;

    /// Returns the maximum value in the set, if any.
    ///
    /// # Returns
    ///
    /// The maximum value in the set, or `None` if the set is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::{Set, insert};
    ///
    /// let mut set = Set::with_max(2);
    /// insert!(set, 2, 42);
    /// assert_eq!(set.max(), Some(42));
    /// ```
    fn max(&self) -> Option<usize>;
}

impl SetOps for Set {
    /// Checks whether the set contains the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for containment.
    ///
    /// # Returns
    ///
    /// `true` if the set contains the value, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    /// use fastset::SetOps;
    ///
    /// let mut set = Set::with_max(1);
    /// set.insert(42);
    /// assert!(set.contains(&42));
    /// assert!(!set.contains(&100));
    /// ```
    fn contains(&self, value: &usize) -> bool {
        self.contains(value)
    }

    /// Returns an iterator over the elements of the set.
    ///
    /// # Returns
    ///
    /// A boxed iterator yielding references to the elements of the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    /// use fastset::SetOps;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(42);
    /// set.insert(100);
    ///
    /// let mut iter = set.iter();
    /// assert_eq!(iter.next(), Some(&42));
    /// assert_eq!(iter.next(), Some(&100));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter(&self) -> Box<dyn Iterator<Item = &usize> + '_> {
        Box::new(self.elements.iter())
    }

    /// Returns the maximum value in the set, if any.
    ///
    /// # Returns
    ///
    /// The maximum value in the set, or `None` if the set is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::{Set, SetOps};
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(42);
    /// set.insert(100);
    ///
    /// assert_eq!(set.max(), Some(100));
    /// ```
    fn max(&self) -> Option<usize> {
        self.current_max
    }
}

impl SetOps for HashSet<usize> {
    /// Checks whether the set contains the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for containment.
    ///
    /// # Returns
    ///
    /// `true` if the set contains the value, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use fastset::SetOps;
    ///
    /// let mut set = HashSet::new();
    /// set.insert(42);
    /// assert!(set.contains(&42));
    /// assert!(!set.contains(&100));
    /// ```
    fn contains(&self, value: &usize) -> bool {
        HashSet::contains(self, value)
    }

    /// Returns an iterator over the elements of the set.
    ///
    /// # Returns
    ///
    /// A boxed iterator yielding references to the elements of the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use fastset::SetOps;
    ///
    /// let mut set = HashSet::<usize>::new();
    /// set.insert(42);
    /// set.insert(100);
    /// let mut results = Vec::new();
    /// set.iter().for_each(|&elem| results.push(elem));
    /// let expected = HashSet::from([42, 100]);
    /// let results_set: HashSet<usize> = results.into_iter().collect();
    /// assert_eq!(results_set, expected);
    /// ```
    fn iter(&self) -> Box<dyn Iterator<Item = &usize> + '_> {
        Box::new(self.iter())
    }

    /// Returns the maximum value in the set, if any.
    ///
    /// # Returns
    ///
    /// The maximum value in the set, or `None` if the set is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use fastset::SetOps;
    ///
    /// let mut set = HashSet::new();
    /// set.insert(42);
    /// set.insert(100);
    ///
    /// assert_eq!(set.max(), Some(100));
    /// ```
    fn max(&self) -> Option<usize> {
        self.iter().max().copied()
    }
}

impl Set {
    /// Checks if the set is a subset of another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns `true` if all elements of the set are contained within the other set, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let set1 = Set::from_iter(1..=5);
    /// let set2 = Set::from_iter(1..=10);
    ///
    /// assert!(set1.is_subset(&set2));
    /// ```
    #[inline(always)]
    pub fn is_subset<T: SetOps>(&self, other: &T) -> bool {
        self.elements.iter().all(|&value| other.contains(&value))
    }

    /// Checks if the set is a superset of another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns `true` if all elements of the other set are contained within this set, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let set1 = Set::from_iter(1..=10);
    /// let set2 = Set::from_iter(1..=5);
    ///
    /// assert!(set1.is_superset(&set2));
    /// ```
    #[inline(always)]
    pub fn is_superset<T: SetOps>(&self, other: &T) -> bool {
        other.iter().all(|value| self.contains(value))
    }

    /// Checks if the set has no elements in common with another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns `true` if the two sets have no elements in common, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let set1 = Set::from_iter(1..=5);
    /// let set2 = Set::from_iter(6..=10);
    ///
    /// assert!(set1.is_disjoint(&set2));
    /// ```
    #[inline(always)]
    pub fn is_disjoint<T: SetOps>(&self, other: &T) -> bool {
        !self.iter().any(|&value| other.contains(&value))
    }

    /// Returns the union of the set with another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns a new `Set` containing all elements present in either set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let set1 = Set::from_iter(1..=5);
    /// let set2 = Set::from_iter(4..=8);
    ///
    /// let union = set1.union(&set2);
    ///
    /// assert_eq!(union.len(), 8);
    /// ```
    #[inline(always)]
    pub fn union<T: SetOps>(&self, other: &T) -> Self {
        let max_other = other.max().unwrap_or(0);
        let mut result = Set::with_max(std::cmp::max(self.max, max_other));
        self.iter().chain(other.iter()).for_each(|&value| {
            result.insert(value);
        });
        result
    }

    /// Returns the intersection of the set with another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns a new `Set` containing only elements present in both sets.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let set1 = Set::from_iter(1..=5);
    /// let set2 = Set::from_iter(4..=8);
    ///
    /// let intersection = set1.intersection(&set2);
    ///
    /// assert_eq!(intersection.len(), 2);
    /// ```
    #[inline(always)]
    pub fn intersection<T: SetOps>(&self, other: &T) -> Self {
        let max_other = other.max().unwrap_or(0);
        let mut result = Set::with_max(std::cmp::max(self.max, max_other));
        self.elements
            .iter()
            .filter(|&&value| other.contains(&value))
            .for_each(|&value| {
                result.insert(value);
            });
        result
    }

    /// Returns the difference of the set with another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns a new `Set` containing elements present in the first set but not in the second set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let set1 = Set::from_iter(1..=5);
    /// let set2 = Set::from_iter(4..=8);
    ///
    /// let difference = set1.difference(&set2);
    ///
    /// assert_eq!(difference.len(), 3);
    /// ```
    #[inline(always)]
    pub fn difference<T: SetOps>(&self, other: &T) -> Self {
        let max_other = other.max().unwrap_or(0);
        let mut result = Set::with_max(std::cmp::max(self.max, max_other));
        self.iter()
            .filter(|&&value| !other.contains(&value))
            .for_each(|&value| {
                result.insert(value);
            });
        result
    }

    /// Returns the symmetric difference of the set with another set.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another data structure implementing `Contains` and `Into<Set>`.
    ///
    /// # Returns
    ///
    /// Returns a new `Set` containing elements present in either set but not in both.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::{Set, SetOps};
    ///
    /// let set1 = Set::from_iter(1..=5);
    /// let set2 = Set::from_iter(4..=8);
    ///
    /// let symmetric_difference = set1.symmetric_difference(&set2);
    ///
    /// assert_eq!(symmetric_difference.len(), 6);
    /// ```
    #[inline(always)]
    pub fn symmetric_difference<T: SetOps>(&self, other: &T) -> Self {
        let max_other = other.max().unwrap_or(0);
        let mut result = Set::with_max(std::cmp::max(self.max, max_other));
        self.iter()
            .filter(|&&value| !other.contains(&value))
            .chain(other.iter().filter(|&value| !self.contains(value)))
            .for_each(|&value| {
                result.insert(value);
            });
        result
    }
}
