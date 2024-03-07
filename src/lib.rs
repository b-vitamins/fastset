//! # fastset Crate
//!
//! The `fastset` crate provides a custom `Set` implementation, optimized for managing collections of `usize` values. It is particularly tailored for use cases involving indices of other data structures, where elements are densely packed within a known range and the application demands high volumes of insert and delete operations.
//!
//! ## Rationale
//!
//! In many applications, especially those dealing with indices or identifiers, the need arises for a data structure that can efficiently handle a dense range of `usize` values. Traditional set implementations might not always offer the best balance between performance and memory usage for such specific scenarios. The `fastset` crate addresses this by offering a `Set` that:
//! - **Specializes in `usize` elements**, making it ideal for scenarios like indexing.
//! - **Optimizes for densely packed elements**, leveraging this characteristic to enhance performance.
//! - **Provides high-performance operations**, especially for insertions and deletions, critical for applications requiring dynamic set manipulations.
//! - **Has predictable memory usage**, which, while not minimal, is bounded and directly related to the specified maximum element value.
//!
//! ## Applicability
//!
//! The `fastset` `Set` is designed for environments where operation predictability and performance take precedence over minimal memory usage. It excels in managing dense, bounded collections of `usize` elements, particularly in scenarios with a high frequency of insertions and deletions. Example use cases include:
//! - Managing available or used indices in large arrays.
//! - Tracking slots in memory pools.
//! - Any application where elements are dense, have a bounded range, and require frequent dynamic manipulation.
//!
//! However, it may not be the best fit for applications where sparse elements span a wide range or where minimizing memory footprint is a primary concern.
//!
//! ## Key Features
//!
//! - **Specialized for `usize`**: Tailored specifically for handling `usize` values, ideal for indexing scenarios.
//! - **Optimized for Dense Elements**: Efficiency is maximized when elements are closely packed within a pre-determined range.
//! - **High-Performance Operations**: Engineered for fast insertions, deletions, and random access.
//! - **Predictable Memory Usage**: While not designed for minimal memory footprint, its usage is predictable and directly related to the maximum element value specified upon creation.
//! - **Random Access**: Includes a `random` method to retrieve a random element from the set, essential for simulations and randomized algorithms.
//!
//!
//! ## Performance Benchmarks
//!
//! Benchmarks comparing `fastset::Set` with `hashbrown::HashSet` and `std::collections::HashSet`:
//!
//! | Operation  | `fastset::Set` (ns) | `hashbrown::HashSet` (ns) | `std::collections::HashSet` (ns) |
//! |------------|---------------------|---------------------------|----------------------------------|
//! | insert     | 3.3772 - 3.4153     | 7.8488 - 7.9377           | 21.632 - 21.811                  |
//! | remove     | 3.0707 - 3.0856     | 5.1704 - 5.2657           | 13.469 - 13.655                  |
//! | contains   | 2.9405 - 2.9808     | 3.2842 - 3.3088           | 16.706 - 16.988                  |
//! | random     | 2.7410 - 2.7724     | N/A                       | N/A                              |
//!
//! Benchmarks were conducted on a machine with the following specifications:
//! - Processor: AMD Ryzen™ 5 5600G with Radeon™ Graphics x 12
//! - Memory: 58.8 GiB
//! - Operating System: Guix System
//! - OS Type: 64-bit
//!
//! Performance times are given in nanoseconds (ns) and represent a range from multiple benchmarking runs. Lower numbers 
//! indicate better performance.
//! 
//! ## Usage
//!
//! ```rust
//!use fastset::{set, Set};
//!use nanorand::WyRand;
//!
//!fn main() {
//!    // Create a set with some initial elements
//!    let mut set = set![5, 10, 15, 20, 25, 30]; 
//!
//!    // Check if certain elements are present in the set
//!    assert!(set.contains(&5));
//!    assert!(set.contains(&15));
//!    assert!(set.contains(&25));
//!
//!    // Display the current elements and the set length
//!    println!("Initial set: {}, Length: {}", set, set.len());
//!
//!    // Insert a new element into the set
//!    if set.insert(35) { println!("Inserted 35 into the set"); }
//!    println!("Set after inserting 35: {}, Length: {}", set, set.len());
//!    assert!(set.contains(&20));
//!
//!    // Remove an element from the set
//!    if set.remove(&5) { println!("Removed 5 from the set"); }
//!    println!("Set after removal: {}, Length: {}", set, set.len());
//!    assert!(!set.contains(&5));
//!
//!    // Try to take an element from the set, removing it in the process
//!    if let Some(taken) = set.take(&10) { println!("Took element {} from the set", taken); }
//!    println!("Set after take: {}, Length: {}", set, set.len());
//!    assert!(!set.contains(&10));
//!
//!    // Use the random method to get a random element from the set
//!    let mut rng = WyRand::new();
//!    if let Some(element) = set.random(&mut rng).copied() {
//!        println!("Randomly selected element: {}", element);
//!        assert!(set.contains(&element));
//!        set.remove(&element);
//!        println!("Removed {} from the set", element);
//!        assert!(!set.contains(&element));
//!    }
//!    println!("Set after removal: {}, Length: {}", set, set.len());
//!
//!    // Display the current elements and the set length
//!    println!("Final set: {}, Length: {}", set, set.len());
//!}
//! ```
//!
//! This example showcases the basic functionality provided by the `fastset` crate, including the crucial `random` method for Monte Carlo simulations and other applications requiring random access to elements.
//!
//! ## Note
//!
//! This crate was developed in the context of Monte Carlo simulations of spin systems. In such simulations, it's required to track and provide fast access to elements of a given sign (positive or negative energy) from an array whose elements (spin energies) are rapidly changing signs due to Monte Carlo updates. The `fastset` `Set` facilitates relatively efficient tracking and manipulation of these elements over extended periods, improving the performance of simulations.

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
