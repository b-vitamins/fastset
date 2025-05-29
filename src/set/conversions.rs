use super::core::Set;
use std::collections::HashSet;

/// Converts a `Vec<usize>` into a `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let vec = vec![1, 2, 3, 4, 5];
/// let set = Set::from(vec);
///
/// assert!(set.contains(&3));
/// ```
impl From<Vec<usize>> for Set {
    fn from(vec: Vec<usize>) -> Self {
        let mut set = Set::with_max(vec.iter().max().cloned().unwrap_or(0));
        vec.iter().for_each(|&item| {
            set.insert(item);
        });
        set
    }
}

/// Converts a slice of `usize` into a `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let items = &[1, 2, 3, 4, 5];
/// let set = Set::from(items);
///
/// assert!(set.contains(&3));
/// ```
impl<'a> From<&'a [usize]> for Set {
    fn from(slice: &'a [usize]) -> Self {
        let max_element = slice.iter().max().cloned().unwrap_or_default();
        let mut set = Set::with_max(max_element);
        slice.iter().for_each(|&item| {
            set.insert(item);
        });
        set
    }
}

/// Converts an array of `usize` into a `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let items = &[1, 2, 3, 4, 5];
/// let set = Set::from(items);
///
/// assert!(set.contains(&3));
/// ```
impl<const N: usize> From<&[usize; N]> for Set {
    fn from(array: &[usize; N]) -> Self {
        let max_element = *array.iter().max().unwrap_or(&0);
        let mut set = Set::with_max(max_element);
        array.iter().for_each(|&item| {
            set.insert(item);
        });
        set
    }
}

/// Converts a `HashSet<usize>` into a `Set`.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use fastset::Set;
///
/// let mut hash_set = HashSet::new();
/// hash_set.insert(1);
/// hash_set.insert(2);
/// hash_set.insert(3);
///
/// let set = Set::from(hash_set);
///
/// assert!(set.contains(&3));
/// ```
impl From<HashSet<usize>> for Set {
    fn from(hashset: HashSet<usize>) -> Self {
        let mut set = Set::with_max(*hashset.iter().max().unwrap_or(&0));
        hashset.iter().for_each(|&item| {
            set.insert(item);
        });
        set
    }
}

/// Converts a reference to `HashSet<usize>` into a `Set`.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use fastset::Set;
///
/// let mut hash_set = HashSet::new();
/// hash_set.insert(1);
/// hash_set.insert(2);
/// hash_set.insert(3);
///
/// let set = Set::from(&hash_set);
///
/// assert!(set.contains(&3));
/// ```
impl<'a> From<&'a HashSet<usize>> for Set {
    fn from(hashset: &'a HashSet<usize>) -> Self {
        let mut set = Set::with_max(*hashset.iter().max().unwrap_or(&0) + 1);
        hashset.iter().for_each(|&item| {
            set.insert(item);
        });
        set
    }
}

/// Extends the `Set` with elements from an iterator over `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let mut set = Set::with_max(0);
/// set.extend(vec![1, 2, 3]);
///
/// assert!(set.contains(&2));
/// ```
impl Extend<usize> for Set {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        iter.into_iter().for_each(|elem| {
            self.insert(elem);
        });
    }
}

/// Extends the `Set` with elements from an iterator over references to `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let mut set = Set::with_max(0);
/// let values = vec![1, 2, 3];
/// set.extend(values.iter());
///
/// assert!(set.contains(&2));
/// ```
impl<'a> Extend<&'a usize> for Set {
    fn extend<I: IntoIterator<Item = &'a usize>>(&mut self, iter: I) {
        iter.into_iter().for_each(|&elem| {
            self.insert(elem);
        });
    }
}

/// Converts an iterator over `usize` values into a `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let set: Set = (1..=5).collect();
///
/// assert!(set.contains(&3));
/// ```
impl FromIterator<usize> for Set {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let collected: Vec<usize> = iter.into_iter().collect();
        let max_element = collected.iter().max().cloned().unwrap_or(0);
        let mut set = Set::with_max(max_element);
        collected.into_iter().for_each(|i| {
            set.insert(i);
        });
        set
    }
}

/// Converts an iterator over references to `usize` values into a `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let values = vec![1, 2, 3];
/// let set: Set = values.iter().collect();
///
/// assert!(set.contains(&2));
/// ```
impl<'a> FromIterator<&'a usize> for Set {
    fn from_iter<I: IntoIterator<Item = &'a usize>>(iter: I) -> Self {
        let collected: Vec<usize> = iter.into_iter().cloned().collect();
        let max_element = collected.iter().max().cloned().unwrap_or(0);
        let mut set = Set::with_max(max_element);
        collected.into_iter().for_each(|i| {
            set.insert(i);
        });
        set
    }
}
