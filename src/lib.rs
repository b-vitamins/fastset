//! # fastset
//!
//! ![Crates.io](https://img.shields.io/crates/v/fastset)
//! ![docs.rs](https://img.shields.io/docsrs/fastset)
//! ![License](https://img.shields.io/crates/l/fastset)
//! ![GitHub Workflow Status](https://github.com/b-vitamins/fastset/actions/workflows/rust.yml/badge.svg)
//!
//! Fast set implementation for dense, bounded integer collections, offering quick updates and random access.
//!
//! ## Features
//!
//! - Tailored for `usize` elements, ideal for index-based applications.
//! - Fast insertions, deletions, and random access through direct memory operations.
//! - Paging mechanism to enhance memory usage efficiency.
//! - `random` method for selecting random elements, crucial for simulations.
//!
//! ## Use Cases:
//! - Tracking interest points within stochastic cellular automata simulations.
//! - Managing available or used indices in arrays.
//! - Tracking slots in memory pools.

//! 0.4.0 introduces a paging mechanism that reduces the memory-footprint of fastset::Set. 
//! With the paging feature, `fastset::Set` achieves ~ 100% reduction in peak heap memory allocations 
//! with no additional performance overhead.
//! 
//! Note that while paging improves the existing memory footprint, 
//! `fastset::Set` **is still not** a good solution for memory constrained applications 
//! or for applications with storage need for sparse elements spread over an extended range.
//! For integers twice as sparse as the page size, the `fastset::Set` with paging 
//! has peak heap allocation ~ 8x that of `HashSet`.
//!
//! ## Benchmarks
//!
//! Performance comparisons between `fastset::Set`, `hashbrown::HashSet`, and `std::collections::HashSet`:
//!
//! | Operation | `fastset::Set` | `hashbrown::HashSet` | `std::collections::HashSet` |
//! |-----------|----------------|----------------------|-----------------------------|
//! | insert    | 1.1632 ns      | 4.7105 ns            | 14.136 ns                   |
//! | remove    | 1.1647 ns      | 3.0459 ns            | 10.625 ns                   |
//! | contains  | 932.81 ps      | 985.38 ps            | 13.827 ns                   |
//! | random    | 651.26 ps      | N/A                  | N/A                         |
//!
//! Benchmarks were performed on the following system:
//! - CPU: AMD Ryzen™ 5 5600G with Radeon™ Graphics x 12
//! - RAM: 58.8 GiB
//! - OS: Guix System, 64-bit
//!
//! ## Usage
//!
//! ```rust
//! use fastset::{set, Set};
//! use nanorand::WyRand;
//!
//!     // Create a set with some initial elements
//!     let mut set = set![5, 10, 15, 20, 25, 30];
//!
//!     // Check if certain elements are present in the set
//!     assert!(set.contains(&5));
//!     assert!(set.contains(&15));
//!     assert!(set.contains(&25));
//!
//!     // Display the current elements and the set length
//!     println!("Initial set: {}, Length: {}", set, set.len());
//!
//!     // Insert a new element into the set
//!     if set.insert(35) { println!("Inserted 35 into the set"); }
//!     println!("Set after inserting 35: {}, Length: {}", set, set.len());
//!     assert!(set.contains(&20));
//!
//!     // Remove an element from the set
//!     if set.remove(&5) { println!("Removed 5 from the set"); }
//!     println!("Set after removal: {}, Length: {}", set, set.len());
//!     assert!(!set.contains(&5));
//!
//!     // Try to take an element from the set, removing it in the process
//!     if let Some(taken) = set.take(&10) { println!("Took element {} from the set", taken); }
//!     println!("Set after take: {}, Length: {}", set, set.len());
//!     assert!(!set.contains(&10));
//!
//!     // Use the random method to get a random element from the set
//!     let mut rng = WyRand::new();
//!     if let Some(element) = set.random(&mut rng) {
//!         println!("Randomly selected element: {}", element);
//!         assert!(set.contains(&element));
//!         set.remove(&element);
//!         println!("Removed {} from the set", element);
//!         assert!(!set.contains(&element));
//!     }
//!     println!("Set after removal: {}, Length: {}", set, set.len());
//!
//!     // Display the current elements and the set length
//!     println!("Final set: {}, Length: {}", set, set.len());
//! ```
//!
mod set;
pub use set::{Set, SetOps};
/// The maximum capacity for the Set.
///
/// CAUTION: Setting the set's largest element or capacity near MAX_CAPACITY
/// can significantly impact memory usage. For instance, with MAX_CAPACITY
/// set to 1 billion, the `Set` could require approximately 16 GB of memory
/// under certain conditions due to storage of elements, their indicators,
/// and indexing structures. Ensure adequate memory is available when
/// approaching this limit.
const MAX_CAPACITY: usize = 1_000_000_000;

/// Macro for creating a `Set` with the given elements.
///
/// # Example
///
/// ```
/// use fastset::{Set, set};
/// let set = set![1, 2, 3];
/// ```
#[macro_export]
macro_rules! set {
    ($($element:expr),*) => {{
        let mut new_set = Set::new(30000); // Adjusted for crate-level visibility
        $(new_set.insert($element);)*
        new_set
    }};
}

/// Macro for removing elements from a `Set`.
///
/// # Example
///
/// ```
/// # use fastset::{Set, remove};
/// let mut my_set = Set::new(10);
/// my_set.insert(1);
/// my_set.insert(2);
///
/// remove!(my_set, 1, 2);
/// ```
#[macro_export]
macro_rules! remove {
    ($set:expr, $($element:expr),*) => {
        $( $set.remove(&$element); )*
    };
}

/// Macro for inserting elements into a `Set`.
///
/// # Example
///
/// ```
/// # use fastset::{Set, insert};
/// let mut my_set = Set::new(10);
///
/// insert!(my_set, 1, 2, 3);
/// ```
#[macro_export]
macro_rules! insert {
    ($set:expr, $($element:expr),*) => {
        $( $set.insert($element); )*
    };
}

/// Macro for selecting a random element from a `Set`.
///
/// If no random number generator is provided, it will use `WyRand` as the default.
///
/// # Example
///
/// ```
/// # use fastset::{Set, random};
/// # use nanorand::{WyRand, Rng};
/// let mut my_set = Set::new(10);
/// my_set.insert(1);
/// my_set.insert(2);
///
/// let random_elem = random!(my_set); // Use default RNG
///
/// let mut rng = WyRand::new();
/// let random_elem_custom_rng = random!(my_set, &mut rng); // Use custom RNG
/// ```
#[macro_export]
macro_rules! random {
    ($set:expr, $rng:expr) => {{
        $set.random($rng)
    }};
    ($set:expr) => {{
        let mut rng = WyRand::new();
        $set.random(&mut rng)
    }};
}
