use super::MAX_CAPACITY;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};

/// Represents a custom Set implementation.
#[derive(Clone, Serialize, Deserialize)]
pub struct Set {
    pub(super) indicator: Vec<bool>,
    pub(super) elements: Vec<usize>,
    pub(super) pages: Vec<Option<Vec<usize>>>,
    pub(super) max: usize,
    pub(super) current_max: Option<usize>,
    pub(super) current_min: Option<usize>,
}

impl Set {
    pub(super) const PAGE_SIZE: usize = 16;
    pub(super) const PAGE_SHIFT: usize = Self::PAGE_SIZE.trailing_zeros() as usize;
    pub(super) const PAGE_MASK: usize = Self::PAGE_SIZE - 1;

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
            elements: Vec::with_capacity(std::cmp::min(max_element.saturating_add(1), 1024)),
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
            elements: Vec::with_capacity(std::cmp::min(capacity, 1024)),
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
            min_capacity
        } else {
            std::cmp::max(self.current_max.unwrap_or(0), min_capacity)
        };
        self.max = new_max;
        self.indicator.resize(new_max + 1, false);
        self.indicator.shrink_to_fit();

        // Clean up pages
        if !self.pages.is_empty() {
            let max_page_idx = Self::page_indices(new_max).0;
            self.pages.truncate(max_page_idx + 1);
            self.pages.shrink_to_fit();
        }
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
            self.indicator = vec![false; 1];
            self.indicator.shrink_to_fit();
            self.pages.clear();
            self.pages.shrink_to_fit();
        } else {
            // Otherwise resize to fit the current maximum value
            self.max = self.current_max.unwrap_or(0);
            self.indicator.resize(self.max + 1, false);
            self.indicator.shrink_to_fit();

            // Clean up pages that are now out of range
            if !self.pages.is_empty() {
                let max_page_idx = Self::page_indices(self.max).0;
                self.pages.truncate(max_page_idx + 1);
                self.pages.shrink_to_fit();

                // Shrink individual page allocations
                for page in &mut self.pages {
                    if let Some(p) = page {
                        p.shrink_to_fit();
                    }
                }
            }
        }
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
    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
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
        // More efficient clearing - only clear the parts that are actually used
        for &elem in &self.elements {
            self.indicator[elem] = false;
        }
        self.elements.clear();

        // Clear pages more efficiently
        for page in &mut self.pages {
            if let Some(p) = page {
                p.fill(0);
            }
        }

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
        // Fast path: already in bounds
        if value < self.indicator.len() {
            return self.insert_unchecked(value);
        }

        // Check max capacity
        if value >= MAX_CAPACITY {
            return false;
        }

        // Optimized resize for small increments
        if value == self.max + 1 {
            self.indicator.push(false);
            self.max = value;
        } else {
            self.reserve(value);
        }

        self.insert_unchecked(value)
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
        if *value < self.indicator.len() {
            unsafe { self.remove_unchecked(value) }
        } else {
            false
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
        // Safe and almost as fast as unsafe version
        self.indicator.get(*value).copied().unwrap_or(false)
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
        if self.contains(value) {
            Some(*value)
        } else {
            None
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
        if self.remove(value) {
            Some(*value)
        } else {
            None
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
    pub(super) fn page_indices(value: usize) -> (usize, usize) {
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
            std::ops::Bound::Unbounded => self.indicator.len(),
        };

        // Optimized counting using indicator directly
        if end <= self.indicator.len() && start < end {
            self.indicator[start..end].iter().filter(|&&b| b).count()
        } else {
            0
        }
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
        // Fast path for small values
        if value == 0 {
            return 0;
        }

        // Use range_cardinality for efficiency
        self.range_cardinality(0..value)
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
        self.current_max.and_then(|max_val| {
            unsafe { self.remove_unchecked(&max_val) };
            Some(max_val)
        })
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
        self.current_min.and_then(|min_val| {
            unsafe { self.remove_unchecked(&min_val) };
            Some(min_val)
        })
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

        // Update current_max and current_min more efficiently
        match (self.current_max, self.current_min) {
            (None, None) => {
                self.current_max = Some(value);
                self.current_min = Some(value);
            }
            (Some(max), Some(min)) => {
                if value > max {
                    self.current_max = Some(value);
                } else if value < min {
                    self.current_min = Some(value);
                }
            }
            _ => unreachable!("Invariant violated: max and min should both be Some or None"),
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

        // Get the element index from the page
        let elem_index = self.pages[page_idx].as_ref().unwrap()[in_page_idx];

        // Remove the element by swapping with the last
        let last_index = self.elements.len() - 1;

        if elem_index < last_index {
            // Swap with last element
            self.elements.swap(elem_index, last_index);

            // Update the page entry for the swapped element
            let swapped_value = self.elements[elem_index];
            let (swapped_page_idx, swapped_in_page_idx) = Self::page_indices(swapped_value);
            self.pages[swapped_page_idx].as_mut().unwrap()[swapped_in_page_idx] = elem_index;
        }

        // Remove the last element
        self.elements.pop();

        // Zero the slot in the page to avoid stale entries
        self.pages[page_idx].as_mut().unwrap()[in_page_idx] = 0;

        // Update current_max and current_min if necessary
        match (self.current_max, self.current_min) {
            (Some(max), Some(min)) if *value == max || *value == min => {
                if self.is_empty() {
                    self.current_max = None;
                    self.current_min = None;
                } else {
                    // Only recalculate if we removed the max or min
                    if *value == max {
                        self.current_max = self.elements.iter().copied().max();
                    }
                    if *value == min {
                        self.current_min = self.elements.iter().copied().min();
                    }
                }
            }
            _ => {} // No update needed
        }

        true
    }
}
