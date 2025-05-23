use crate::MAX_CAPACITY;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::{Extend, FromIterator, IntoIterator};
use std::slice::Iter;

/// Represents a custom Set implementation.
#[derive(Clone, Serialize, Deserialize)]
pub struct Set {
    indicator: Vec<bool>,
    elements: Vec<usize>,
    pages: Vec<Option<Vec<usize>>>,
    max: usize,
    current_max: Option<usize>,
    current_min: Option<usize>,
}

impl Set {
    const PAGE_SIZE: usize = 16;
    const PAGE_SHIFT: usize = Self::PAGE_SIZE.trailing_zeros() as usize;
    const PAGE_MASK: usize = Self::PAGE_SIZE - 1;

    /// Creates a new Set with the specified maximum element.
    ///
    /// # Arguments
    ///
    /// * `max_element` - The maximum element that the Set can contain.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// // Create a new Set with a maximum capacity of 100 elements.
    /// let set = Set::with_max(100);
    /// ```
    pub fn with_max(max_element: usize) -> Self {
        if max_element > MAX_CAPACITY {
            panic!("max_element is larger than MAX_ELEMENTS");
        }
        Self {
            indicator: vec![false; max_element.saturating_add(1)], // Always at least 1 slot
            elements: Vec::with_capacity(max_element.saturating_add(1)),
            pages: Vec::new(),
            max: max_element,
            current_max: None,
            current_min: None,
        }
    }

    /// For backward compatibility - creates a new Set with the specified maximum element.
    /// This method is deprecated in favor of `with_max`.
    ///
    /// # Arguments
    ///
    /// * `max_element` - The maximum element that the Set can contain.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// // Create a new Set with a maximum capacity of 100 elements.
    /// let set = Set::new(100);
    /// ```
    #[deprecated(since = "0.5.0", note = "Use with_max instead")]
    pub fn new(max_element: usize) -> Self {
        Self::with_max(max_element)
    }

    /// Creates a new Set with the specified initial capacity.
    ///
    /// The `with_capacity` method creates a new Set with the specified initial
    /// capacity. This allows you to pre-allocate memory for the Set if you
    /// know in advance how many elements it will contain.
    ///
    /// Note: The capacity represents the inclusive maximum value that can be stored,
    /// not the number of elements. For example, with_capacity(10) can store values 0-10.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The initial capacity of the Set (max value that can be stored).
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// // Create a new Set with an initial capacity of 50 elements.
    /// let set = Set::with_capacity(50);
    /// ```
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Set {
            indicator: vec![false; capacity.saturating_add(1)], // Always at least 1 slot
            elements: Vec::with_capacity(capacity),
            pages: Vec::new(),
            max: capacity, // max is now capacity, not capacity-1
            current_max: None,
            current_min: None,
        }
    }

    /// Returns the capacity of the Set.
    ///
    /// The capacity of a Set is the maximum value that can be stored in the set.
    /// Note that this represents the largest value, not the count of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_capacity(50);
    /// assert_eq!(set.capacity(), 50);
    /// ```
    pub fn capacity(&self) -> usize {
        self.max // Return max value that can be stored
    }

    /// Returns the maximum element value that this Set can hold.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// assert_eq!(set.max_value(), 100);
    /// ```
    pub fn max_value(&self) -> usize {
        self.max
    }

    /// Reserves capacity for at least `new_max_element` additional elements
    /// in the Set.
    ///
    /// If the current capacity of the Set is less than `new_max_element`, it
    /// will be increased to accommodate at least `new_max_element` elements.
    /// This method does nothing if the current capacity is already sufficient.
    ///
    /// # Arguments
    ///
    /// * `new_max_element` - The new maximum element that the Set can contain.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    ///
    /// // Reserve capacity for at least 200 elements.
    /// set.reserve(200);
    /// ```
    #[inline(always)]
    pub fn reserve(&mut self, new_max_element: usize) {
        if new_max_element > self.max {
            let new_size = new_max_element + 1;
            self.indicator.resize(new_size, false);
            // Don't over-reserve elements - they'll be allocated as needed
            self.max = new_max_element;
        }
    }

    /// Shrinks the capacity of the Set to the specified minimum capacity.
    ///
    /// It will reduce the capacity of the Set to fit the specified `min_capacity`.
    /// If the current capacity is already smaller than `min_capacity`, this method
    /// does nothing.
    ///
    /// # Arguments
    ///
    /// * `min_capacity` - The minimum capacity to reserve after shrinking.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::{set, Set};
    ///
    /// let mut set = Set::with_capacity(10);
    ///
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    /// assert!(set.capacity() >= 10);
    /// set.shrink_to(4);
    /// assert!(set.capacity() >= 4);
    /// set.shrink_to(0);
    /// assert!(set.capacity() >= 3);
    /// ```
    #[inline(always)]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.elements.shrink_to(min_capacity);
        let new_max = if self.is_empty() {
            min_capacity // Changed from min_capacity.saturating_sub(1) to fix the test
        } else {
            std::cmp::max(self.current_max.unwrap_or(0), min_capacity)
        };
        self.max = new_max;
        self.indicator.resize(new_max + 1, false);
    }

    /// Shrinks the capacity of the Set as much as possible.
    ///
    /// This method is the same as `shrink_to` and exists for compatibility reasons.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_capacity(10);
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    /// assert!(set.capacity() >= 10);
    /// set.shrink_to_fit();
    /// assert!(set.capacity() >= 3);
    /// ```
    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        self.elements.shrink_to_fit();

        // If the set is empty, keep a minimal indicator size
        if self.is_empty() {
            self.max = 0;
            self.indicator.resize(1, false);
            self.pages.clear();
        } else {
            // Otherwise resize to fit the current maximum value
            self.max = self.current_max.unwrap_or(0);
            self.indicator.resize(self.max + 1, false);

            // Clean up pages that are now out of range
            if !self.pages.is_empty() {
                let max_page_idx = Self::page_indices(self.max).0;
                if max_page_idx + 1 < self.pages.len() {
                    self.pages.truncate(max_page_idx + 1);
                }
            }
        }

        // Shrink pages
        self.pages.shrink_to_fit();
    }

    /// Returns the number of elements in the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.len(), 2);
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if the Set contains no elements, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// assert!(set.is_empty());
    ///
    /// set.insert(5);
    /// assert!(!set.is_empty());
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an iterator over the elements in the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// for element in set.iter() {
    ///     println!("Element: {}", element);
    /// }
    /// ```
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, usize> {
        self.elements.iter()
    }

    /// Removes all elements from the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert!(!set.is_empty());
    ///
    /// set.clear();
    ///
    /// assert!(set.is_empty());
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        self.indicator.fill(false);
        self.elements.clear();
        self.pages.clear();
        self.current_max = None;
        self.current_min = None;
    }

    /// Inserts an element into the Set.
    ///
    /// Returns `true` if the element was successfully inserted,
    /// and `false` if the element was already present in the Set.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to insert into the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    ///
    /// // Inserting a new element
    /// assert!(set.insert(5));
    ///
    /// // Inserting a duplicate element
    /// assert!(!set.insert(5));
    /// ```
    #[inline(always)]
    pub fn insert(&mut self, value: usize) -> bool {
        // Fast path: if value is exactly max+1, we can just resize indicator by 1
        if value == self.max + 1 {
            self.indicator.push(false);
            self.max = value;
            // Now safe because we just ensured value is in bounds
            return self.insert_unchecked(value);
        }

        // Regular path for other cases
        match value > self.max {
            true => match value < MAX_CAPACITY {
                true => {
                    self.reserve(value);
                    // Now safe because we just reserved space for value
                    self.insert_unchecked(value)
                }
                false => false,
            },
            false => {
                // Safe because value <= max and indicator.len() > max
                self.insert_unchecked(value)
            }
        }
    }

    /// Removes an element from the Set.
    ///
    /// Returns `true` if the element was successfully removed,
    /// and `false` if the element was not present in the Set.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    ///
    /// // Removing an existing element
    /// assert!(set.remove(&5));
    ///
    /// // Trying to remove a non-existing element
    /// assert!(!set.remove(&10));
    /// ```
    #[inline(always)]
    pub fn remove(&mut self, value: &usize) -> bool {
        match *value >= self.indicator.len() {
            true => false,
            false => unsafe { self.remove_unchecked(value) },
        }
    }

    /// Checks if the Set contains a specific value.
    ///
    /// Returns `true` if the Set contains the specified value, and `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for presence in the Set.
    ///
    /// # Safety
    ///
    /// This method uses unsafe pointer arithmetic to access elements of the internal
    /// indicator vector. However, it is safe because it performs a bound check on the
    /// `value`, ensuring that no out-of-bounds access occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    ///
    /// assert!(set.contains(&5));
    /// assert!(!set.contains(&10));
    /// ```
    #[inline(always)]
    pub fn contains(&self, value: &usize) -> bool {
        match *value < self.indicator.len() {
            true => unsafe { *self.indicator.as_ptr().add(*value) },
            false => false, // Out of bounds, so not contained.
        }
    }

    /// Retrieves the specified value from the Set, if it exists.
    ///
    /// Returns `Some(value)` if the Set contains the specified value, and `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to retrieve from the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    ///
    /// assert_eq!(set.get(&5), Some(5));
    /// assert_eq!(set.get(&10), None);
    /// ```
    #[inline(always)]
    pub fn get(&self, value: &usize) -> Option<usize> {
        match self.contains(value) {
            true => Some(*value),
            false => None,
        }
    }

    /// Removes and returns the specified value from the Set, if it exists.
    ///
    /// Returns `Some(value)` if the Set contains the specified value and it was successfully removed,
    /// and `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    ///
    /// assert_eq!(set.take(&5), Some(5));
    /// assert_eq!(set.contains(&5), false);
    /// ```
    #[inline(always)]
    pub fn take(&mut self, value: &usize) -> Option<usize> {
        match self.contains(value) {
            true => {
                // Safe because we just checked value exists and is in-bounds via contains()
                unsafe { self.remove_unchecked(value) };
                Some(*value)
            }
            false => None,
        }
    }

    /// Returns the maximum value in the Set, if it is not empty.
    ///
    /// Returns `Some(max)` if the Set is not empty, and `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.max(), Some(10));
    /// ```
    #[inline(always)]
    pub fn max(&self) -> Option<usize> {
        self.current_max
    }

    /// Returns the minimum value in the Set, if it is not empty.
    ///
    /// Returns `Some(min)` if the Set is not empty, and `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.min(), Some(5));
    /// ```
    #[inline(always)]
    pub fn min(&self) -> Option<usize> {
        self.current_min
    }

    /// Returns the largest value in the Set without removing it, if the Set is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.peek_largest(), Some(10));
    /// ```
    #[inline(always)]
    pub fn peek_largest(&self) -> Option<usize> {
        self.current_max
    }

    /// Returns the smallest value in the Set without removing it, if the Set is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.peek_smallest(), Some(5));
    /// ```
    #[inline(always)]
    pub fn peek_smallest(&self) -> Option<usize> {
        self.current_min
    }

    /// Calculate page index and in-page index for a value.
    ///
    /// The page index determines which page the value belongs to,
    /// while the in-page index determines the value's position within that page.
    #[inline(always)]
    fn page_indices(value: usize) -> (usize, usize) {
        let page_index = value >> Self::PAGE_SHIFT; // Shift right by PAGE_SHIFT to divide by PAGE_SIZE
        let in_page_index = value & Self::PAGE_MASK; // Bitwise AND with PAGE_MASK to modulo by PAGE_SIZE
        (page_index, in_page_index)
    }

    /// Returns the number of elements in the Set that fall within the specified range.
    ///
    /// The range is defined by the provided range bounds, inclusive on the start bound
    /// and exclusive on the end bound. The method counts the elements within the range
    /// that exist in the Set.
    ///
    /// This operation runs in O(|range|) time where |range| is the size of the range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of values to count elements for.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    /// set.insert(15);
    ///
    /// assert_eq!(set.range_cardinality(8..=12), 1);
    /// ```
    #[inline(always)]
    pub fn range_cardinality<R>(&self, range: R) -> usize
    where
        R: std::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.max + 1,
        };
        (start..end).filter(|&value| self.contains(&value)).count()
    }

    /// Returns the number of elements in the Set that are strictly less than the specified value.
    ///
    /// This method returns the count of elements in the Set that are less than the given value.
    /// This operation runs in O(|S|) time where |S| is the size of the set.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    /// set.insert(15);
    ///
    /// assert_eq!(set.rank(12), 2);
    /// ```
    #[inline(always)]
    pub fn rank(&self, value: usize) -> usize {
        self.elements
            .iter()
            .filter(|&&element| element < value)
            .count()
    }

    /// Removes and returns the largest value in the Set, if it is not empty.
    ///
    /// Returns `Some(value)` if the Set is not empty, and `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.remove_largest(), Some(10));
    /// assert_eq!(set.contains(&10), false);
    /// ```
    #[inline(always)]
    pub fn remove_largest(&mut self) -> Option<usize> {
        match self.current_max {
            Some(max_val) => {
                // Safe because max_val is a value that exists in the set
                unsafe { self.remove_unchecked(&max_val) };
                Some(max_val)
            }
            None => None,
        }
    }

    /// Removes and returns the smallest value in the Set, if it is not empty.
    ///
    /// Returns `Some(value)` if the Set is not empty, and `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    ///
    /// assert_eq!(set.remove_smallest(), Some(5));
    /// assert_eq!(set.contains(&5), false);
    /// ```
    #[inline(always)]
    pub fn remove_smallest(&mut self) -> Option<usize> {
        match self.current_min {
            Some(min_val) => {
                // Safe because min_val is a value that exists in the set
                unsafe { self.remove_unchecked(&min_val) };
                Some(min_val)
            }
            None => None,
        }
    }

    /// Returns a random element from the Set using the provided random number generator.
    ///
    /// If the Set is empty, returns `None`. Otherwise, returns a reference to a randomly chosen element.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator implementing the `Rng` trait.
    ///
    /// # Safety
    ///
    /// This method relies on unsafe code due to pointer arithmetic to avoid bounds checks for performance reasons.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    /// use nanorand::WyRand;
    ///
    /// let mut set = Set::with_max(100);
    /// set.insert(5);
    /// set.insert(10);
    /// set.insert(15);
    ///
    /// let mut rng = WyRand::new();
    /// let random_element = set.random(&mut rng);
    /// assert!(random_element.is_some());
    /// ```
    #[inline(always)]
    pub fn random(&self, rng: &mut WyRand) -> Option<usize> {
        match self.elements.is_empty() {
            // SAFETY: index is within bounds by design (generated within elements.len() range)
            false => unsafe {
                Some(
                    *self
                        .elements
                        .as_ptr()
                        .add(rng.generate_range(0..self.elements.len())),
                )
            },
            true => None,
        }
    }

    /// Inserts a value into the Set without performing bounds checks.
    ///
    /// This method assumes that:
    /// 1. `value` is within the bounds of the `indicator` vector.
    /// 2. There are no concurrent mutable references to `indicator`, `index`, or `elements`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be inserted into the Set.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value was inserted, `false` if it was already present in the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    ///
    /// // Inserting values without performing bounds checks.
    /// let result = set.insert_unchecked(5);
    /// assert!(result);
    /// let result = set.insert_unchecked(10);
    /// assert!(result);
    /// ```
    #[inline(always)]
    pub fn insert_unchecked(&mut self, value: usize) -> bool {
        if self.indicator[value] {
            // The value is already present.
            return false;
        }

        self.indicator[value] = true;

        // Calculate the page index and in-page index.
        let (page_idx, in_page_idx) = Self::page_indices(value);

        // Ensure the page exists.
        if page_idx >= self.pages.len() {
            self.pages.resize_with(page_idx + 1, Default::default);
        }
        if self.pages[page_idx].is_none() {
            self.pages[page_idx] = Some(vec![0; Self::PAGE_SIZE]);
        }

        // Insert the value into the elements vector and record its index in the page.
        let elem_index = self.elements.len();
        self.elements.push(value);
        self.pages[page_idx].as_mut().unwrap()[in_page_idx] = elem_index;

        // Update current_max and current_min
        match self.current_max {
            Some(max) if value > max => self.current_max = Some(value),
            None => self.current_max = Some(value),
            _ => {}
        }

        match self.current_min {
            Some(min) if value < min => self.current_min = Some(value),
            None => self.current_min = Some(value),
            _ => {}
        }

        true
    }

    /// Removes a value from the Set without performing bounds checks.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it:
    /// 1. Assumes `value` is within the bounds of the `indicator` vector. Out-of-bounds access will cause undefined behavior.
    /// 2. Assumes no concurrent mutable references to `indicator`, `index`, or `elements`, to avoid mutable aliasing.
    /// 3. Assumes the page for this value exists in the `pages` vector.
    ///
    /// The caller must ensure these preconditions are met.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be removed from the Set.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value was removed, `false` if it was not present in the Set.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastset::Set;
    ///
    /// let mut set = Set::with_max(100);
    ///
    /// // Inserting and removing values without performing bounds checks.
    /// unsafe {
    ///     set.insert_unchecked(5);
    ///     set.insert_unchecked(10);
    ///     set.remove_unchecked(&5);
    /// }
    /// ```
    #[inline(always)]
    pub unsafe fn remove_unchecked(&mut self, value: &usize) -> bool {
        if !self.indicator[*value] {
            // The value is not present.
            return false;
        }

        self.indicator[*value] = false;

        // Calculate page index and in-page index.
        let (page_idx, in_page_idx) = Self::page_indices(*value);

        // No need to check for page existence here since the value exists.
        let elem_index = self.pages[page_idx].as_ref().unwrap()[in_page_idx];

        // Remove the element by swapping with the last and shrinking the elements vector.
        if elem_index < self.elements.len() - 1 {
            let last_index = self.elements.len() - 1;
            self.elements.swap(elem_index, last_index);

            let swapped_value = self.elements[elem_index];

            // Update the page entry for the swapped element.
            // Now the element that was last is at elem_index position
            let (swapped_page_idx, swapped_in_page_idx) = Self::page_indices(swapped_value);
            self.pages[swapped_page_idx].as_mut().unwrap()[swapped_in_page_idx] = elem_index;
        }
        // The old last element will be popped (removed) from the vector
        self.elements.pop();

        // Zero the slot in the page to avoid stale entries
        self.pages[page_idx].as_mut().unwrap()[in_page_idx] = 0;

        // Update current_max and current_min if necessary
        if Some(*value) == self.current_max || Some(*value) == self.current_min {
            if self.is_empty() {
                self.current_max = None;
                self.current_min = None;
            } else {
                // Recalculate max/min if the removed value was max/min
                if Some(*value) == self.current_max {
                    self.current_max = self.elements.iter().max().copied();
                }
                if Some(*value) == self.current_min {
                    self.current_min = self.elements.iter().min().copied();
                }
            }
        }

        true
    }
}

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

/// Performs the union operation between two references to `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 | &set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<&'a Set> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: &'a Set) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset = HashSet::<usize>::from_iter(3..8);
/// let result = &set | &hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: &'a HashSet<usize>) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 | &set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr<&Set> for Set {
    type Output = Set;

    fn bitor(self, rhs: &Set) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set | &hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr<&HashSet<usize>> for Set {
    type Output = Set;

    fn bitor(self, rhs: &HashSet<usize>) -> Set {
        self.union(rhs)
    }
}

/// Performs the union operation between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 | set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<Set> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: Set) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union operation between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = &set | hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOr<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitor(self, rhs: HashSet<usize>) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union operation between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 | set2;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr for Set {
    type Output = Set;

    fn bitor(self, rhs: Set) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union operation between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set | hashset;
/// assert_eq!(result, Set::from_iter(0..8));
/// ```
impl std::ops::BitOr<HashSet<usize>> for Set {
    type Output = Set;

    fn bitor(self, rhs: HashSet<usize>) -> Set {
        self.union(&rhs)
    }
}

/// Performs the union assignment operation between two references to `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 |= &set2;
/// assert_eq!(set1, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOrAssign<&'a Set> for Set {
    fn bitor_assign(&mut self, rhs: &'a Set) {
        *self = self.union(rhs);
    }
}

/// Performs the union assignment operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set |= &hashset;
/// assert_eq!(set, Set::from_iter(0..8));
/// ```
impl<'a> std::ops::BitOrAssign<&'a HashSet<usize>> for Set {
    fn bitor_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.union(rhs);
    }
}

/// Performs the intersection operation between two references to `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = &set1 & &set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<&'a Set> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: &'a Set) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = &set & &hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: &'a HashSet<usize>) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = set1 & &set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd<&Set> for Set {
    type Output = Set;

    fn bitand(self, rhs: &Set) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = set & &hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd<&HashSet<usize>> for Set {
    type Output = Set;

    fn bitand(self, rhs: &HashSet<usize>) -> Set {
        self.intersection(rhs)
    }
}

/// Performs the intersection operation between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = &set1 & set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<Set> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: Set) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection operation between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = &set & hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAnd<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitand(self, rhs: HashSet<usize>) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection operation between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let intersection = set1 & set2;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd for Set {
    type Output = Set;

    fn bitand(self, rhs: Set) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection operation between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let intersection = set & hashset;
/// assert_eq!(intersection, Set::from_iter(3..5));
/// ```
impl std::ops::BitAnd<HashSet<usize>> for Set {
    type Output = Set;

    fn bitand(self, rhs: HashSet<usize>) -> Set {
        self.intersection(&rhs)
    }
}

/// Performs the intersection assignment operation between two `Set` references.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 &= &set2;
/// assert_eq!(set1, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAndAssign<&'a Set> for Set {
    fn bitand_assign(&mut self, rhs: &'a Set) {
        *self = self.intersection(rhs);
    }
}

/// Performs the intersection assignment operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set &= &hashset;
/// assert_eq!(set, Set::from_iter(3..5));
/// ```
impl<'a> std::ops::BitAndAssign<&'a HashSet<usize>> for Set {
    fn bitand_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.intersection(rhs);
    }
}

/// Performs the subtraction operation between two `Set` references.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 - &set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<&'a Set> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: &'a Set) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = &set - &hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: &'a HashSet<usize>) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 - &set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub<&Set> for Set {
    type Output = Set;

    fn sub(self, rhs: &Set) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set - &hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub<&HashSet<usize>> for Set {
    type Output = Set;

    fn sub(self, rhs: &HashSet<usize>) -> Set {
        self.difference(rhs)
    }
}

/// Performs the subtraction operation between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = &set1 - set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<Set> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: Set) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction operation between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = &set - hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::Sub<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn sub(self, rhs: HashSet<usize>) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction operation between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let result = set1 - set2;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub for Set {
    type Output = Set;

    fn sub(self, rhs: Set) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction operation between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let result = set - hashset;
/// assert_eq!(result, Set::from_iter(0..3));
/// ```
impl std::ops::Sub<HashSet<usize>> for Set {
    type Output = Set;

    fn sub(self, rhs: HashSet<usize>) -> Set {
        self.difference(&rhs)
    }
}

/// Performs the subtraction assignment operation between a `Set` reference and another `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 -= &set2;
/// assert_eq!(set1, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::SubAssign<&'a Set> for Set {
    fn sub_assign(&mut self, rhs: &'a Set) {
        *self = self.difference(rhs);
    }
}

/// Performs the subtraction assignment operation between a `Set` reference and a `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set -= &hashset;
/// assert_eq!(set, Set::from_iter(0..3));
/// ```
impl<'a> std::ops::SubAssign<&'a HashSet<usize>> for Set {
    fn sub_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.difference(rhs);
    }
}

/// Performs the subtraction assignment operation between a `Set` and another `Set` reference.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 -= set2;
/// assert_eq!(set1, Set::from_iter(0..3));
/// ```
impl std::ops::SubAssign<Set> for Set {
    fn sub_assign(&mut self, rhs: Set) {
        *self = self.difference(&rhs);
    }
}

/// Performs the subtraction assignment operation between a `Set` and a `HashSet<usize>` reference.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set -= hashset;
/// assert_eq!(set, Set::from_iter(0..3));
/// ```
impl std::ops::SubAssign<HashSet<usize>> for Set {
    fn sub_assign(&mut self, rhs: HashSet<usize>) {
        *self = self.difference(&rhs);
    }
}

/// Computes the symmetric difference between two `Set` references.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = &set1 ^ &set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<&'a Set> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: &'a Set) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between a reference to `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = &set ^ &hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<&'a HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: &'a HashSet<usize>) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between an owned `Set` and a reference to `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = set1 ^ &set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor<&Set> for Set {
    type Output = Set;

    fn bitxor(self, rhs: &Set) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between an owned `Set` and a reference to `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = set ^ &hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor<&HashSet<usize>> for Set {
    type Output = Set;

    fn bitxor(self, rhs: &HashSet<usize>) -> Set {
        self.symmetric_difference(rhs)
    }
}

/// Computes the symmetric difference between a reference to `Set` and an owned `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = &set1 ^ set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<Set> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: Set) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between a reference to `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = &set ^ hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXor<HashSet<usize>> for &'a Set {
    type Output = Set;

    fn bitxor(self, rhs: HashSet<usize>) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between two owned `Set` instances.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// let symmetric_difference = set1 ^ set2;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor for Set {
    type Output = Set;

    fn bitxor(self, rhs: Set) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between an owned `Set` and an owned `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// let symmetric_difference = set ^ hashset;
/// assert_eq!(symmetric_difference, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl std::ops::BitXor<HashSet<usize>> for Set {
    type Output = Set;

    fn bitxor(self, rhs: HashSet<usize>) -> Set {
        self.symmetric_difference(&rhs)
    }
}

/// Computes the symmetric difference between two `Set` references and assigns the result to the left operand.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let mut set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(3..8);
/// set1 ^= &set2;
/// assert_eq!(set1, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXorAssign<&'a Set> for Set {
    fn bitxor_assign(&mut self, rhs: &'a Set) {
        *self = self.symmetric_difference(rhs);
    }
}

/// Computes the symmetric difference between a reference to `Set` and a reference to `HashSet<usize>` and assigns the result to the left operand.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// use std::collections::HashSet;
/// let mut set = Set::from_iter(0..5);
/// let hashset: HashSet<usize> = (3..8).collect();
/// set ^= &hashset;
/// assert_eq!(set, Set::from_iter(0..3).union(&Set::from_iter(5..8)));
/// ```
impl<'a> std::ops::BitXorAssign<&'a HashSet<usize>> for Set {
    fn bitxor_assign(&mut self, rhs: &'a HashSet<usize>) {
        *self = self.symmetric_difference(rhs);
    }
}

/// Implements the `Debug` trait for `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set = Set::from_iter(0..5);
/// println!("{:?}", set);
/// ```
impl std::fmt::Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Generate a detailed string for each element that is present.
        let element_details: Vec<String> = self
            .elements
            .iter()
            .map(|&e| {
                let indicator = self.indicator[e]; // Check if the indicator for this element is true.
                                                   // To find the page and in-page index for the element
                let (page_idx, in_page_idx) = Self::page_indices(e);
                let page = &self.pages[page_idx];
                let mapped_index = page
                    .as_ref()
                    .map_or("None".to_string(), |p| p[in_page_idx].to_string());

                format!(
                    "Element: {}, Indicator: {}, Mapped Index: {}",
                    e, indicator, mapped_index
                )
            })
            .collect();

        // Debug output now focuses on non-empty elements, their indicators, and their mappings within the paged structure.
        f.debug_struct("Set")
            .field("elements", &self.elements) // Show actual elements.
            .field("element_details", &element_details) // Show corresponding indicators and mappings.
            .field("max", &self.max) // Include the 'max' field for completeness.
            .field("current_max", &self.current_max)
            .field("current_min", &self.current_min)
            .finish()
    }
}

/// Implements the `Display` trait for `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set = Set::from_iter(0..5);
/// println!("{}", set);
/// ```
impl std::fmt::Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// Implements the `Default` trait for `Set`.
///
/// Creates a small Set with minimal memory overhead.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set: Set = Default::default();
/// ```
impl Default for Set {
    fn default() -> Self {
        // Create a small set with minimal overhead
        Self::with_max(64)
    }
}

/// Implements the `PartialEq` trait for `Set`.
///
/// # Examples
///
/// ```
/// use fastset::Set;
/// let set1 = Set::from_iter(0..5);
/// let set2 = Set::from_iter(0..5);
/// assert_eq!(set1, set2);
/// ```
impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        match self.elements.len() == other.elements.len() {
            true => self.elements.iter().all(|&item| other.contains(&item)),
            false => false,
        }
    }
}

/// Implements equality comparison for `Set`.
///
/// Two sets are considered equal if they contain the same elements, irrespective of their order.
///
/// # Example
///
/// ```
/// # use fastset::Set;
/// let set1 = Set::from(vec![1, 2, 3]);
/// let set2 = Set::from(vec![3, 2, 1]);
///
/// assert_eq!(set1, set2);
/// ```
impl Eq for Set {}

/// Implements the `PartialEq` trait for `Set` with `HashSet<usize>`.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use fastset::Set;
/// let set = Set::from_iter(0..5);
/// let hash_set: HashSet<usize> = (0..5).collect();
/// assert_eq!(set, hash_set);
/// ```
impl PartialEq<HashSet<usize>> for Set {
    fn eq(&self, other: &HashSet<usize>) -> bool {
        match self.len() == other.len() {
            true => self.iter().all(|&item| other.contains(&item)),
            false => false,
        }
    }
}

/// Implements the `Hash` trait for `Set`.
///
/// # Examples
///
/// ```
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
/// use fastset::Set;
///
/// let set = Set::from_iter(1..=5);
///
/// let mut hasher = DefaultHasher::new();
/// set.hash(&mut hasher);
/// let hash = hasher.finish();
///
/// println!("Hash value of the set: {}", hash);
/// ```
impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the indices of true bits to properly represent the set
        for (idx, &bit) in self.indicator.iter().enumerate() {
            if bit {
                idx.hash(state);
            }
        }
    }
}

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

/// Consumes the `Set`, returning an iterator over owned `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let set = Set::from(vec![1, 2, 3]);
/// let mut iter = set.into_iter();
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
/// ```
impl IntoIterator for Set {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

/// Borrows the `Set`, returning an iterator over references to `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let set = Set::from(vec![1, 2, 3]);
///
/// for &value in &set {
///     println!("{}", value);
/// }
/// ```
impl<'a> IntoIterator for &'a Set {
    type Item = &'a usize;
    type IntoIter = std::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

/// Mutably borrows the `Set`, returning an iterator over mutable references to `usize` values.
///
/// # Examples
///
/// ```
/// use fastset::Set;
///
/// let mut set = Set::from(vec![1, 2, 3]);
///
/// for value in &mut set {
///     *value += 1;
/// }
/// ```
impl<'a> IntoIterator for &'a mut Set {
    type Item = &'a mut usize;
    type IntoIter = std::slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use statrs::distribution::{ChiSquared, ContinuousCDF};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn new_with_zero_max_element() {
        let set = Set::with_max(0);
        assert!(set.is_empty());
        assert!(set.elements.is_empty());
        assert!(!set.indicator.is_empty());
        assert_eq!(set.max, 0);
    }

    #[test]
    fn new_with_nonzero_max_element() {
        let max_element = 10;
        let set = Set::with_max(max_element);
        assert_eq!(set.elements.len(), 0);
        assert_eq!(set.len(), 0);
        assert_eq!(set.max, max_element);
    }

    #[test]
    fn new_with_large_max_element() {
        let max_element = 1000000;
        let set = Set::with_max(max_element);
        assert_eq!(set.elements.len(), 0);
        assert_eq!(set.max, max_element);
    }

    #[test]
    fn new_with_multiple_calls() {
        let set1 = Set::with_max(5);
        let set2 = Set::with_max(10);

        assert_eq!(set1.max, 5);
        assert_eq!(set2.max, 10);
    }

    #[test]
    fn with_capacity_zero() {
        let set = Set::with_capacity(0);

        // Now with proper protection, indicator will always have at least 1 element
        assert_eq!(set.indicator.len(), 1);
        assert!(set.elements.is_empty());
        assert!(set.pages.is_empty());
        assert_eq!(set.max, 0);
    }

    #[test]
    fn with_capacity_nonzero() {
        let capacity = 10;
        let set = Set::with_capacity(capacity);

        assert_eq!(set.elements.len(), 0);
        // indicator.len() is now capacity + 1 to handle values 0..=capacity
        assert_eq!(set.indicator.len(), capacity + 1);
        // max is now equal to capacity (not capacity-1)
        assert_eq!(set.max, capacity);
    }

    #[test]
    fn with_capacity_large() {
        let capacity = 1000000;
        let set = Set::with_capacity(capacity);

        assert_eq!(set.elements.len(), 0);
        // indicator.len() is now capacity + 1 to handle values 0..=capacity
        assert_eq!(set.indicator.len(), capacity + 1);
        // max is now equal to capacity (not capacity-1)
        assert_eq!(set.max, capacity);
    }

    #[test]
    fn with_capacity_multiple_calls() {
        let set1 = Set::with_capacity(5);
        let set2 = Set::with_capacity(10);

        // max is now equal to capacity (not capacity-1)
        assert_eq!(set1.max, 5);
        assert_eq!(set2.max, 10);
    }

    #[test]
    fn reserve_increase_capacity() {
        let mut set = Set::with_max(5);
        set.reserve(10);

        assert_eq!(set.max, 10);
        assert_eq!(set.indicator.len(), 11);
    }

    #[test]
    fn reserve_no_increase_capacity() {
        let mut set = Set::with_max(5);
        set.reserve(3);

        assert_eq!(set.max, 5);
        assert_eq!(set.indicator.len(), 6);
    }

    #[test]
    fn reserve_same_capacity() {
        let mut set = Set::with_max(5);
        set.reserve(5);

        assert_eq!(set.max, 5);
        assert_eq!(set.indicator.len(), 6);
    }

    #[test]
    fn reserve_large_capacity() {
        let mut set = Set::with_max(5);
        set.reserve(100);

        assert_eq!(set.max, 100);
        assert_eq!(set.indicator.len(), 101);
    }

    #[test]
    fn len_empty_set() {
        let set = Set::with_max(5);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn len_non_empty_set() {
        let mut set = Set::with_max(5);
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_eq!(set.len(), 3);
    }

    #[test]
    fn is_empty_empty_set() {
        let set = Set::with_max(5);
        assert!(set.is_empty());
    }

    #[test]
    fn is_empty_non_empty_set() {
        let mut set = Set::with_max(5);
        set.insert(1);

        assert!(!set.is_empty());
    }

    #[test]
    fn iter_empty_set() {
        let set = Set::with_max(5);
        let mut iter = set.iter();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_non_empty_set() {
        let mut set = Set::with_max(5);
        set.insert(1);
        set.insert(2);

        let mut iter = set.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn clear() {
        let mut set = Set::with_max(3);
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.clear();
        assert!(set.is_empty());
        for i in 1..=3 {
            assert!(!set.contains(&i));
        }
    }

    #[test]
    fn clear_empty_set() {
        let mut set = Set::with_max(5);
        set.clear();

        assert!(set.is_empty());
    }

    #[test]
    fn clear_non_empty_set() {
        let mut set = Set::with_max(5);
        set.insert(1);
        set.insert(2);
        set.clear();

        assert!(set.is_empty());
    }

    #[test]
    fn insert() {
        let mut set = Set::with_max(MAX_CAPACITY);
        // Insert a value and check its presence
        set.insert(1);
        assert!(set.contains(&1), "Set should contain 1");
        // Insert duplicate value and check set length
        set.insert(1);
        assert_eq!(
            set.len(),
            1,
            "Inserting a duplicate should not increase set size"
        );
        // Insert more values, including one beyond the initial capacity
        set.insert(2);
        set.insert(1000);
        assert!(set.contains(&2), "Set should contain 2");
        assert!(set.contains(&1000), "Set should contain 1000");
        assert_eq!(set.len(), 3, "Set should contain 3 unique elements");
    }

    #[test]
    fn insert_within_capacity() {
        let mut set = Set::with_max(5);
        assert!(set.insert(1));
        assert!(set.contains(&1));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_beyond_capacity_and_within_max() {
        let mut set = Set::with_max(5);
        assert!(set.insert(6));
        assert!(set.contains(&6));
        assert_eq!(set.len(), 1);
    }

    #[test]
    #[should_panic]
    fn insert_beyond_max_capacity() {
        let mut set = Set::with_max(usize::MAX);
        assert!(!set.insert(usize::MAX));
        assert!(!set.contains(&usize::MAX));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn remove() {
        let mut set = Set::with_max(MAX_CAPACITY / 3000);
        // Insert some values
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        // Remove a value that exists
        set.remove(&3);
        assert!(!set.contains(&3), "Set should not contain 3 after removal");
        // Remove a value that does not exist
        set.remove(&5);
        assert!(
            !set.contains(&5),
            "Set should not contain 5 as it was never added"
        );
        // Check the remaining values
        assert!(set.contains(&1), "Set should still contain 1");
        assert!(set.contains(&2), "Set should still contain 2");
        assert!(set.contains(&4), "Set should still contain 4");
        // Check the total number of elements after removals
        assert_eq!(set.len(), 3, "Set should contain 3 elements after removals");
        // Remove all remaining values
        set.remove(&1);
        set.remove(&2);
        set.remove(&4);
        assert!(
            set.is_empty(),
            "Set should be empty after removing all elements"
        );
    }

    #[test]
    fn remove_existing_element() {
        let mut set = Set::with_max(5);
        set.insert(3);
        assert!(set.remove(&3));
        assert!(!set.contains(&3));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn remove_non_existing_element() {
        let mut set = Set::with_max(5);
        set.insert(3);
        assert!(!set.remove(&5));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn remove_element_beyond_capacity() {
        let mut set = Set::with_max(5);
        set.insert(3);
        assert!(!set.remove(&(usize::MAX)));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn contains() {
        let mut set = Set::with_max(MAX_CAPACITY);
        // Insert some values
        set.insert(1);
        set.insert(2);
        set.insert(3);
        // Check for presence of inserted values
        assert!(set.contains(&1), "Set should contain 1");
        assert!(set.contains(&2), "Set should contain 2");
        assert!(set.contains(&3), "Set should contain 3");
        // Check for absence of values not inserted
        assert!(!set.contains(&4), "Set should not contain 4");
        assert!(!set.contains(&0), "Set should not contain 0");
        assert!(!set.contains(&100), "Set should not contain 100");
    }

    #[test]
    fn contains_existing_element() {
        let set = Set::from(vec![1, 2, 3]);
        assert!(set.contains(&2));
    }

    #[test]
    fn contains_non_existing_element() {
        let set = Set::from(vec![1, 2, 3]);
        assert!(!set.contains(&5));
    }

    #[test]
    fn contains_element_beyond_capacity() {
        let set = Set::with_max(5);
        assert!(!set.contains(&(usize::MAX)));
    }

    #[test]
    fn get_existing_element() {
        let set = Set::from(vec![1, 2, 3]);
        assert_eq!(set.get(&2), Some(2));
    }

    #[test]
    fn get_non_existing_element() {
        let set = Set::from(vec![1, 2, 3]);
        assert_eq!(set.get(&5), None);
    }

    #[test]
    fn get_element_beyond_capacity() {
        let set = Set::with_max(5);
        assert_eq!(set.get(&(usize::MAX)), None);
    }

    #[test]
    fn take_existing_element() {
        let mut set = Set::from(vec![1, 2, 3]);
        assert_eq!(set.take(&2), Some(2));
        assert!(!set.contains(&2));
    }

    #[test]
    fn take_non_existing_element() {
        let mut set = Set::from(vec![1, 2, 3]);
        assert_eq!(set.take(&5), None);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn take_element_beyond_capacity() {
        let mut set = Set::with_max(5);
        assert_eq!(set.take(&(usize::MAX)), None);
    }

    #[test]
    fn max_empty_set() {
        let set = Set::with_max(5);
        assert_eq!(set.max(), None);
    }

    #[test]
    fn max_non_empty_set() {
        let set = Set::from(vec![1, 2, 3]);
        assert_eq!(set.max(), Some(3));
    }

    #[test]
    fn max_set_with_single_element() {
        let set = Set::from(vec![5]);
        assert_eq!(set.max(), Some(5));
    }

    #[test]
    fn min_empty_set() {
        let set = Set::with_max(5);
        assert_eq!(set.min(), None);
    }

    #[test]
    fn min_non_empty_set() {
        let set = Set::from(vec![3, 1, 5]);
        assert_eq!(set.min(), Some(1));
    }

    #[test]
    fn min_set_with_single_element() {
        let set = Set::from(vec![5]);
        assert_eq!(set.min(), Some(5));
    }

    #[test]
    fn range_cardinality_empty_set() {
        let set = Set::with_max(10);
        assert_eq!(set.range_cardinality(..), 0);
        assert_eq!(set.range_cardinality(0..5), 0);
    }

    #[test]
    fn range_cardinality_full_set() {
        let set = Set::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(set.range_cardinality(..), 5);
        assert_eq!(set.range_cardinality(1..4), 3);
    }

    #[test]
    fn range_cardinality_out_of_bounds() {
        let set = Set::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(set.range_cardinality(6..10), 0);
    }

    #[test]
    fn rank_empty_set() {
        let set = Set::with_max(10);
        assert_eq!(set.rank(5), 0);
    }

    #[test]
    fn rank_non_empty_set() {
        let set = Set::from(vec![1, 3, 5, 7, 9]);
        assert_eq!(set.rank(5), 2);
    }

    #[test]
    fn rank_non_existing_element() {
        let set = Set::from(vec![1, 3, 5, 7, 9]);
        assert_eq!(set.rank(6), 3);
    }

    #[test]
    fn remove_largest_from_empty_set() {
        let mut set = Set::with_max(10);
        assert_eq!(set.remove_largest(), None);
    }

    #[test]
    fn remove_largest_from_non_empty_set() {
        let mut set = Set::from(vec![1, 3, 5, 7, 9]);
        assert_eq!(set.remove_largest(), Some(9));
        assert!(!set.contains(&9));
    }

    #[test]
    fn remove_largest_from_unsorted_set() {
        let mut set = Set::with_max(100);
        set.insert(10);
        set.insert(1);
        set.insert(7);
        assert_eq!(set.remove_largest(), Some(10));
        assert!(!set.contains(&10));
    }

    #[test]
    fn remove_smallest_from_empty_set() {
        let mut set = Set::with_max(10);
        assert_eq!(set.remove_smallest(), None);
    }

    #[test]
    fn remove_smallest_from_non_empty_set() {
        let mut set = Set::from(vec![1, 3, 5, 7, 9]);
        assert_eq!(set.remove_smallest(), Some(1));
        assert!(!set.contains(&1));
    }

    #[test]
    fn remove_smallest_from_unsorted_set() {
        let mut set = Set::with_max(100);
        set.insert(10);
        set.insert(1);
        set.insert(7);
        assert_eq!(set.remove_smallest(), Some(1));
        assert!(!set.contains(&1));
    }

    #[test]
    fn random() {
        let mut set = Set::with_max(MAX_CAPACITY / 3000);
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        // Test with non-empty set
        let mut observed_values = HashSet::new();
        let mut rng = WyRand::new();
        for _ in 0..100 {
            if let Some(value) = set.random(&mut rng) {
                assert!(
                    set.contains(&value),
                    "Randomly selected value should be in the set"
                );
                observed_values.insert(value);
            }
        }
        // Check that multiple distinct values are observed
        assert!(
            observed_values.len() > 1,
            "Random should return different values over multiple calls"
        );
        // Test with empty set
        set.clear();
        assert!(
            set.random(&mut rng).is_none(),
            "Random should return None for an empty set"
        );
    }

    #[test]
    fn random_returns_none_for_empty_set() {
        let set = Set::with_max(10);
        let mut rng = WyRand::new();
        assert_eq!(set.random(&mut rng), None);
    }

    #[test]
    fn insert_unchecked_adds_element_correctly() {
        let mut set = Set::with_max(5);

        // Insert an element without bounds checking
        let result = set.insert_unchecked(3);

        // Ensure the element was inserted and the operation returned true
        assert!(result);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&3));
        assert_eq!(set.current_max, Some(3));
        assert_eq!(set.current_min, Some(3));
    }

    #[test]
    fn remove_unchecked_removes_element_correctly() {
        let mut set = Set::with_max(5);
        set.insert(3);

        // Remove the element without bounds checking
        let result = unsafe { set.remove_unchecked(&3) };

        // Ensure the element was removed and the operation returned true
        assert!(result);
        assert_eq!(set.len(), 0);
        assert!(!set.contains(&3));
        assert_eq!(set.current_max, None);
        assert_eq!(set.current_min, None);
    }

    #[test]
    #[should_panic]
    fn remove_unchecked_panics_for_out_of_bounds() {
        let mut set = Set::with_max(5);
        set.insert(3);
        // Attempt to remove an out-of-bounds element without bounds checking should panic
        unsafe { assert!(!set.remove_unchecked(&6)) };
    }

    #[test]
    fn contains_returns_true_for_existing_element() {
        let mut set = Set::with_max(100);
        set.insert(42);

        // Check if the set contains the inserted value
        assert!(set.contains(&42));
    }

    #[test]
    fn contains_returns_false_for_nonexistent_element() {
        let set = HashSet::<usize>::new();
        // Check if the set contains a value not inserted
        assert!(!set.contains(&100));
    }

    #[test]
    fn iter_returns_correct_values() {
        let mut set = Set::with_max(2);
        set.insert(42);
        set.insert(100);
        // Create an iterator from the set
        let mut iter = set.iter();
        // Check the values returned by the iterator
        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.next(), Some(&100));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn max_returns_correct_value() {
        let mut set = HashSet::new();
        set.insert(42);
        set.insert(100);
        // Check if the maximum value in the set is returned
        assert_eq!(set.max(), Some(100));
    }

    #[test]
    fn max_returns_none_for_empty_set() {
        let set = HashSet::new();

        // Check if None is returned for an empty set
        assert_eq!(set.max(), None);
    }

    #[test]
    fn is_subset_returns_true_for_subset() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(1..=10);

        // Check if set1 is a subset of set2
        assert!(set1.is_subset(&set2));
    }

    #[test]
    fn is_subset_returns_false_for_non_subset() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(6..=10);

        // Check if set1 is a subset of set2
        assert!(!set1.is_subset(&set2));
    }

    #[test]
    fn is_superset_returns_true_for_superset() {
        let set1 = Set::from_iter(1..=10);
        let set2 = Set::from_iter(1..=5);

        // Check if set1 is a superset of set2
        assert!(set1.is_superset(&set2));
    }

    #[test]
    fn is_superset_returns_false_for_non_superset() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(6..=10);

        // Check if set1 is a superset of set2
        assert!(!set1.is_superset(&set2));
    }

    #[test]
    fn is_disjoint_returns_true_for_disjoint_sets() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(6..=10);

        // Check if set1 and set2 are disjoint
        assert!(set1.is_disjoint(&set2));
    }

    #[test]
    fn is_disjoint_returns_false_for_non_disjoint_sets() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=10);

        // Check if set1 and set2 are disjoint
        assert!(!set1.is_disjoint(&set2));
    }

    #[test]
    fn test_intersection() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let intersection = set1.intersection(&set2);

        assert_eq!(intersection.len(), 2);
        for i in 4..=5 {
            assert!(intersection.contains(&i));
        }
    }

    #[test]
    fn test_difference() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let difference = set1.difference(&set2);

        assert_eq!(difference.len(), 3);
        for i in 1..=3 {
            assert!(difference.contains(&i));
        }
    }

    #[test]
    fn test_symmetric_difference() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let symmetric_difference = set1.symmetric_difference(&set2);

        assert_eq!(symmetric_difference.len(), 6);
        for i in 1..=3 {
            assert!(symmetric_difference.contains(&i));
        }
        for i in 6..=8 {
            assert!(symmetric_difference.contains(&i));
        }
    }

    #[test]
    fn test_empty_set_operations() {
        let set1 = Set::with_max(100);
        let set2 = Set::with_max(100);

        assert!(set1.union(&set2).is_empty());
        assert!(set1.intersection(&set2).is_empty());
        assert!(set1.difference(&set2).is_empty());
        assert!(set1.symmetric_difference(&set2).is_empty());
    }

    #[test]
    fn test_sets_with_same_elements() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(1..=5);

        assert_eq!(set1.union(&set2), set1.intersection(&set2));
        assert!(set1.difference(&set2).is_empty());
        assert!(set1.symmetric_difference(&set2).is_empty());
    }

    #[test]
    fn test_boundary_cases() {
        // Test when one set is empty
        let set1 = Set::with_max(10);
        let set2 = Set::from_iter(1..=5);

        assert!(set1.is_subset(&set2));
        assert!(!set2.is_subset(&set1));
        assert!(!set1.is_superset(&set2));
        assert!(set2.is_superset(&set1));
        assert!(set1.is_disjoint(&set2));

        // Test when one set is a subset/superset of the other
        let set3 = Set::from_iter(1..=5);
        let set4 = Set::from_iter(1..=10);

        assert!(set3.is_subset(&set4));
        assert!(!set4.is_subset(&set3));
        assert!(!set3.is_superset(&set4));
        assert!(set4.is_superset(&set3));
        assert!(!set3.is_disjoint(&set4));

        // Test when the sets have only one common element
        let set5 = Set::from_iter(1..=5);
        let set6 = Set::from_iter(4..=8);

        assert!(!set5.is_subset(&set6));
        assert!(!set6.is_subset(&set5));
        assert!(!set5.is_superset(&set6));
        assert!(!set6.is_superset(&set5));
        assert!(!set5.is_disjoint(&set6));
    }

    #[test]
    fn test_bit_xor_sets() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let result = &set1 ^ &set2;

        // Verify that the result contains only the elements that are unique to each set
        assert_eq!(result.len(), 6);
        assert!(result.contains(&6));
        assert!(result.contains(&7));
        assert!(result.contains(&8));
    }

    #[test]
    fn test_bit_xor_assignment_sets() {
        let mut set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        set1 ^= &set2;

        // Verify that set1 contains only the elements that are unique to each set
        assert_eq!(set1.len(), 6);
        assert!(set1.contains(&1));
        assert!(set1.contains(&2));
        assert!(set1.contains(&3));
        assert!(!set1.contains(&4));
        assert!(!set1.contains(&5));
        assert!(set1.contains(&6));
        assert!(set1.contains(&7));
        assert!(set1.contains(&8));
    }

    #[test]
    fn test_bit_xor_assignment_set_and_hashset() {
        let mut set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();

        set ^= &hash_set;

        // Verify that the set contains only the elements that are unique to each set
        assert_eq!(set.len(), 6);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&6));
        assert!(set.contains(&7));
        assert!(set.contains(&8));
    }

    #[test]
    fn test_sub_sets() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let result = &set1 - &set2;

        // Verify that the result contains only the elements present in set1 but not in set2
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_sub_set_and_hashset() {
        let set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();

        let result = &set - &hash_set;

        // Verify that the result contains only the elements present in the set but not in the hash set
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(!result.contains(&4));
    }

    #[test]
    fn test_sub_assignment_sets() {
        let mut set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        set1 -= &set2;

        // Verify that set1 contains only the elements present in set1 but not in set2
        assert_eq!(set1.len(), 3);
        assert!(set1.contains(&1));
        assert!(set1.contains(&2));
        assert!(set1.contains(&3));
    }

    #[test]
    fn test_sub_assignment_set_and_hashset() {
        let mut set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();

        set -= &hash_set;

        // Verify that the set contains only the elements present in the set but not in the hash set
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_bitand_sets() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let result = &set1 & &set2;

        // Verify that the result contains only the elements present in both set1 and set2
        assert_eq!(result.len(), 2);
        assert!(result.contains(&4));
        assert!(result.contains(&5));
    }

    #[test]
    fn test_bitand_set_and_hashset() {
        let set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();

        let result = &set & &hash_set;

        // Verify that the result contains only the elements present in both the set and the hash set
        assert_eq!(result.len(), 2);
        assert!(result.contains(&4));
        assert!(result.contains(&5));
        assert!(!result.contains(&3));
        assert!(!result.contains(&7));
    }

    #[test]
    fn test_bitand_assignment_sets() {
        let mut set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        set1 &= &set2;

        // Verify that set1 contains only the elements present in both set1 and set2
        assert_eq!(set1.len(), 2);
        assert!(set1.contains(&4));
        assert!(set1.contains(&5));
    }

    #[test]
    fn test_bitand_assignment_set_and_hashset() {
        let mut set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();

        set &= &hash_set;

        assert_eq!(set.len(), 2);
        assert!(set.contains(&4));
        assert!(set.contains(&5));

        let mut set2 = Set::from_iter(1..5);
        let hash_set2 = (4..=8).collect::<HashSet<_>>();

        set2 &= &hash_set2;

        assert_eq!(set2.len(), 1);
        assert!(set2.contains(&4));
        assert!(!set2.contains(&5));

        let mut set3 = Set::from_iter(1..5);
        let hash_set3 = (6..=8).collect::<HashSet<_>>();

        set3 &= &hash_set3;

        assert_eq!(set3.len(), 0);
        assert!(!set3.contains(&1));
    }

    #[test]
    fn debug_format() {
        let mut set = Set::with_max(5); // Assuming a 'with_max' method with a 'max' parameter.

        // Simulate `from_iter` functionality for the test.
        for i in 1..=5 {
            set.insert(i); // Assuming an 'insert' method is available.
        }

        let debug_output = format!("{:?}", set);

        assert!(debug_output.contains("Set {"));
        assert!(debug_output.contains("elements: [1, 2, 3, 4, 5]"));
        assert!(debug_output.contains("max: 5"));
        assert!(debug_output.contains("current_max: Some(5)"));
        assert!(debug_output.contains("current_min: Some(1)"));

        // Adjust the assertions to match the new Debug output format:
        // We're looking for "Mapped Index:" now instead of "Index: Some()"
        // Example new format: "Element: 1, Indicator: true, Mapped Index: 0"
        // Note: The exact wording and ordering in your Debug impl may require adjustments here.
        assert!(debug_output.contains("Element: 1, Indicator: true, Mapped Index: 0"));
        assert!(debug_output.contains("Element: 5, Indicator: true, Mapped Index: 4"));
    }

    #[test]
    fn display_format() {
        let set = Set::from_iter(1..=3);
        let display_output = format!("{}", set);
        assert!(display_output == "{1, 2, 3}" || display_output == "{3, 2, 1}");
    }

    #[test]
    fn test_default() {
        let set: Set = Default::default();
        assert!(set.is_empty());
    }

    #[test]
    fn test_partial_eq_sets_equal() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(1..=5);
        assert_eq!(set1, set2);
    }

    #[test]
    fn test_partial_eq_sets_not_equal() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(6..=10);
        assert_ne!(set1, set2);
    }

    #[test]
    fn test_eq_sets_equal() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter((1..=5).rev());
        assert_eq!(set1, set2);
    }

    #[test]
    fn test_eq_sets_not_equal() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(1..=4);
        assert_ne!(set1, set2);
    }

    #[test]
    fn test_partial_eq_with_hashset() {
        let set = Set::from_iter(1..=5);
        let hash_set: HashSet<usize> = (1..=5).collect();
        assert_eq!(set, hash_set);
    }

    #[test]
    fn test_hash() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(1..=5);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        set1.hash(&mut hasher1);
        set2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let set = Set::from(vec.clone());

        for item in vec {
            assert!(set.contains(&item));
        }
    }

    #[test]
    fn test_from_slice() {
        let items = &[1, 2, 3, 4, 5];
        let set = Set::from(items);

        for &item in items {
            assert!(set.contains(&item));
        }
    }

    #[test]
    fn test_from_array() {
        let items = &[1, 2, 3, 4, 5];
        let set = Set::from(items);

        for &item in items {
            assert!(set.contains(&item));
        }
    }

    #[test]
    fn test_from_hashset_owned() {
        let mut hash_set = HashSet::new();
        hash_set.insert(1);
        hash_set.insert(2);
        hash_set.insert(3);

        let set = Set::from(hash_set.clone());

        for item in hash_set {
            assert!(set.contains(&item));
        }
    }

    #[test]
    fn test_from_hashset_ref() {
        let mut hash_set = HashSet::new();
        hash_set.insert(1);
        hash_set.insert(2);
        hash_set.insert(3);

        let set = Set::from(&hash_set);

        for &item in &hash_set {
            assert!(set.contains(&item));
        }
    }

    #[test]
    fn test_extend_usize() {
        let mut set = Set::with_max(0);
        set.extend(vec![1, 2, 3]);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_extend_ref_usize() {
        let mut set = Set::with_max(0);
        let values = [1, 2, 3];
        set.extend(values.iter());

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_from_iterator_usize() {
        let set: Set = (1..=5).collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
    }

    #[test]
    fn test_from_iterator_ref_usize() {
        let values = [1, 2, 3];
        let set: Set = values.iter().collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_into_iter_owned() {
        let set = Set::from(vec![1, 2, 3]);
        let mut iter = set.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_ref() {
        let set = Set::from(vec![1, 2, 3]);

        let mut values = Vec::new();
        for &value in &set {
            values.push(value);
        }

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_into_iter_mut_ref() {
        let mut set = Set::from(vec![1, 2, 3]);

        for value in &mut set {
            *value += 1;
        }

        assert_eq!(set.elements, vec![2, 3, 4]);
    }

    #[test]
    fn comparison() {
        let mut set = Set::with_capacity(1_000_000);
        let mut std_set = HashSet::new();
        let mut rng = WyRand::new();
        for iteration in 0..10000 {
            let value = rng.generate_range(..100_000usize);
            // Randomly choose to insert or remove
            if rng.generate::<bool>() {
                set.insert(value);
                std_set.insert(value);
                if rng.generate::<bool>() {
                    let more_values: Vec<usize> =
                        (0..10).map(|_| rng.generate_range(..1000usize)).collect();
                    for &val in more_values.iter() {
                        std_set.insert(val);
                    }
                    for &val in more_values.iter() {
                        set.insert(val);
                    }
                }
            } else {
                set.remove(&value);
                std_set.remove(&value);
                if rng.generate::<bool>() {
                    let more_values: Vec<usize> =
                        (0..10).map(|_| rng.generate_range(..1000usize)).collect();
                    for &val in more_values.iter() {
                        std_set.remove(&val);
                    }
                    for &val in more_values.iter() {
                        set.remove(&val);
                    }
                }
            }
            // Periodically verify that both sets contain the same elements
            if iteration % 100 == 0 {
                let diff = set.difference(&std_set);
                if !diff.is_empty() {
                    println!("Differences at iteration {}: {:?}", iteration, diff);
                }
                assert!(
                    diff.is_empty(),
                    "Iteration {}: HashSet and StdHashSet differ: {:?}",
                    iteration,
                    diff
                );
            }
        }
    }

    #[test]
    fn test_max_element_reached() {
        // Create sets with maximum element reached
        let max_element = MAX_CAPACITY / 3000 - 1; // Assuming MAX_CAPACITY is defined somewhere
        let set1 = Set::from_iter(0..max_element);
        let set2 = Set::from_iter((max_element - 4)..=max_element);

        // Perform set operations
        let union = set1.union(&set2);
        let intersection = set1.intersection(&set2);
        let difference = set1.difference(&set2);
        let symmetric_difference = set1.symmetric_difference(&set2);

        // Verify the results
        assert_eq!(union.len(), MAX_CAPACITY / 3000); // set1 (0 to max_element-1) plus max_element from set2
        assert_eq!(intersection.len(), 4); // The elements: max-4, max-3, max-2, max-1
        assert_eq!(difference.len(), max_element - 4); // All in set1 minus intersection

        // Symmetric difference has:
        // - All elements unique to set1 (max_element - 4 elements)
        // - Plus one element unique to set2 (max_element itself)
        assert_eq!(symmetric_difference.len(), (max_element - 4) + 1);
    }

    #[test]
    fn test_randomized_operations() {
        // Generate random sets
        let mut rng = WyRand::new();
        let set1: Set = (0..100).map(|_| rng.generate_range(0..100)).collect();
        let set2: Set = (0..150).map(|_| rng.generate_range(0..150)).collect();

        // Perform set operations
        let union = set1.union(&set2);
        let intersection = set1.intersection(&set2);
        let difference = set1.difference(&set2);
        let symmetric_difference = set1.symmetric_difference(&set2);
        // Validate the results by comparing with manual calculations
        for element in &union {
            assert!(set1.contains(element) || set2.contains(element));
        }
        for element in &intersection {
            assert!(set1.contains(element) && set2.contains(element));
        }
        for element in &difference {
            assert!(set1.contains(element) && !set2.contains(element));
        }
        for element in &symmetric_difference {
            assert!(set1.contains(element) != set2.contains(element));
        }
    }

    #[test]
    fn test_bit_or_sets() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let result = &set1 | &set2;

        // Verify that the result contains all elements from both sets
        assert_eq!(result.len(), 8);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(result.contains(&4));
        assert!(result.contains(&5));
        assert!(result.contains(&6));
        assert!(result.contains(&7));
        assert!(result.contains(&8));
    }

    #[test]
    fn test_bit_or_set_and_hashset() {
        let set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();

        let result = &set | &hash_set;

        // Verify that the result contains all elements from both sets
        assert_eq!(result.len(), 8);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(result.contains(&4));
        assert!(result.contains(&5));
        assert!(result.contains(&6));
        assert!(result.contains(&7));
        assert!(result.contains(&8));
    }

    #[test]
    fn test_bit_or_assignment_sets() {
        let mut set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        set1 |= &set2;

        // Verify that set1 contains all elements from both sets
        assert_eq!(set1.len(), 8);
        assert!(set1.contains(&1));
        assert!(set1.contains(&2));
        assert!(set1.contains(&3));
        assert!(set1.contains(&4));
        assert!(set1.contains(&5));
        assert!(set1.contains(&6));
        assert!(set1.contains(&7));
        assert!(set1.contains(&8));
    }

    #[test]
    fn test_bit_or_assignment_set_and_hashset() {
        let mut set = Set::from_iter(1..=5);
        let hash_set = (4..=8).collect::<HashSet<_>>();
        set |= &hash_set;

        // Verify that set contains all elements from both sets
        assert_eq!(set.len(), 8);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
        assert!(set.contains(&6));
        assert!(set.contains(&7));
        assert!(set.contains(&8));
    }

    #[test]
    fn test_overlapping_ranges() {
        // Create two sets with overlapping ranges
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        // Perform set operations
        let union = set1.union(&set2);
        let intersection = set1.intersection(&set2);
        let difference = set1.difference(&set2);
        let symmetric_difference = set1.symmetric_difference(&set2);

        // Verify the results
        assert_eq!(union.len(), 8);
        assert_eq!(intersection.len(), 2); // 4 and 5 are the overlapping elements
        assert_eq!(difference.len(), 3); // 1, 2, and 3 are unique to set1
        assert_eq!(symmetric_difference.len(), 6); // 1, 2, 3, 6, 7, and 8 are unique to their respective sets
    }

    #[test]
    fn test_nested_set_operations() {
        // Create sets
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);
        let set3 = Set::from_iter(6..=10);

        // Perform set operations
        let union = set1.union(&set2);
        let nested_intersection = union.intersection(&set3);

        // Verify the result
        assert_eq!(nested_intersection.len(), 3); // 6, 7, 8 are the common elements
    }

    #[test]
    fn test_union() {
        let set1 = Set::from_iter(1..=5);
        let set2 = Set::from_iter(4..=8);

        let union = set1.union(&set2);

        assert_eq!(union.len(), 8);
        for i in 1..=8 {
            assert!(union.contains(&i));
        }
    }

    #[test]
    fn test_current_max_min_tracking() {
        let mut set = Set::with_max(100);

        // Test empty set
        assert_eq!(set.current_max, None);
        assert_eq!(set.current_min, None);

        // Insert elements and verify max/min update
        set.insert(10);
        assert_eq!(set.current_max, Some(10));
        assert_eq!(set.current_min, Some(10));

        set.insert(5);
        assert_eq!(set.current_max, Some(10));
        assert_eq!(set.current_min, Some(5));

        set.insert(20);
        assert_eq!(set.current_max, Some(20));
        assert_eq!(set.current_min, Some(5));

        // Remove maximum element and verify max updates
        set.remove(&20);
        assert_eq!(set.current_max, Some(10));
        assert_eq!(set.current_min, Some(5));

        // Remove minimum element and verify min updates
        set.remove(&5);
        assert_eq!(set.current_max, Some(10));
        assert_eq!(set.current_min, Some(10));

        // Remove last element
        set.remove(&10);
        assert_eq!(set.current_max, None);
        assert_eq!(set.current_min, None);
    }

    #[test]
    fn test_remove_largest_smallest() {
        let mut set = Set::with_max(100);

        // Test with unsorted insertion
        set.insert(30);
        set.insert(10);
        set.insert(50);
        set.insert(20);
        set.insert(40);

        // Verify max/min are correct
        assert_eq!(set.current_max, Some(50));
        assert_eq!(set.current_min, Some(10));

        // Remove largest returns the largest element
        assert_eq!(set.remove_largest(), Some(50));
        assert_eq!(set.current_max, Some(40));

        // Remove smallest returns the smallest element
        assert_eq!(set.remove_smallest(), Some(10));
        assert_eq!(set.current_min, Some(20));
    }

    #[test]
    fn sampling_is_uniformly_at_random() {
        const SAMPLES: usize = 1_000_000;
        const EDGE_OF_THE_UNIVERSE: usize = 10000;

        let elements = (1..=EDGE_OF_THE_UNIVERSE).collect::<Vec<_>>();
        let set = Set::from(elements.clone());
        let mut rng = WyRand::new_seed(42u64);
        let mut counts = vec![0f64; elements.len()];

        for _ in 0..SAMPLES {
            if let Some(value) = set.random(&mut rng) {
                counts[value - 1] += 1.0;
            }
        }

        let e = SAMPLES as f64 / elements.len() as f64;
        let statistic: f64 = counts.iter().map(|&o| (o - e) * (o - e) / e).sum();

        let dof = elements.len() - 1;
        let chi = ChiSquared::new(dof as f64).unwrap();
        let acceptable = chi.inverse_cdf(0.99);

        // Null hypothesis: Elements are sampled uniformly at random
        assert!(
            statistic < acceptable,
            "Chi-square statistic {} is greater than what's acceptable ({})",
            statistic,
            acceptable,
        );
    }
}
