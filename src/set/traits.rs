use super::core::Set;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

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
                let (page_idx, in_page_idx) = Set::page_indices(e);
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
