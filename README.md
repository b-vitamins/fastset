# fastset

![Crates.io](https://img.shields.io/crates/v/fastset)
![docs.rs](https://img.shields.io/docsrs/fastset)
![License](https://img.shields.io/crates/l/fastset)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/b-vitamins/fastset/Rust)

Fast set implementation for dense, bounded integer collections. Provides quick updates and random access.

The `fastset` crate provides a custom `Set` implementation, optimized for managing collections of `usize` values. It is particularly tailored for use cases involving indices of other data structures, where elements are densely packed within a known range and the application demands high volumes of insert and delete operations.

## Rationale

In many applications, especially those dealing with indices or identifiers, the need arises for a data structure that can efficiently handle a dense range of `usize` values. Traditional set implementations might not always offer the best balance between performance and memory usage for such specific scenarios. The `fastset` crate addresses this by offering a `Set` that:
- **Specializes in `usize` elements**, making it ideal for scenarios like indexing.
- **Optimizes for densely packed elements**, leveraging this characteristic to enhance performance.
- **Provides high-performance operations**, especially for insertions and deletions, critical for applications requiring dynamic set manipulations.
- **Has predictable memory usage**, which, while not minimal, is bounded and directly related to the specified maximum element value.

## Applicability

The `fastset` `Set` is designed for environments where operation predictability and performance take precedence over minimal memory usage. It excels in managing dense, bounded collections of `usize` elements, particularly in scenarios with a high frequency of insertions and deletions. Example use cases include:
- Managing available or used indices in large arrays.
- Tracking slots in memory pools.
- Any application where elements are dense, have a bounded range, and require frequent dynamic manipulation.

However, it may not be the best fit for applications where sparse elements span a wide range or where minimizing memory footprint is a primary concern.

## Key Features

- **Specialized for `usize`**: Tailored specifically for handling `usize` values, ideal for indexing scenarios.
- **Optimized for Dense Elements**: Efficiency is maximized when elements are closely packed within a pre-determined range.
- **High-Performance Operations**: Engineered for fast insertions, deletions, and random access.
- **Predictable Memory Usage**: While not designed for minimal memory footprint, its usage is predictable and directly related to the maximum element value specified upon creation.
- **Random Access**: Includes a `random` method to retrieve a random element from the set, essential for simulations and randomized algorithms.

## Note

This crate was developed in the context of Monte Carlo simulations of spin systems. In such simulations, it's required to track and provide fast access to elements of a given sign (positive or negative energy) from an array whose elements (spin energies) are rapidly changing signs due to Monte Carlo updates. The `fastset` `Set` facilitates relatively efficient tracking and manipulation of these elements over extended periods, improving the performance of simulations.

## Usage

```rust
use fastset::{set, Set};
use nanorand::WyRand;

fn main() {
    // Create a set with some initial elements
    let mut set = set![5, 10, 15, 20, 25, 30]; 

    // Check if certain elements are present in the set
    assert!(set.contains(&5));
    assert!(set.contains(&15));
    assert!(set.contains(&25));

    // Display the current elements and the set length
    println!("Initial set: {}, Length: {}", set, set.len());

    // Insert a new element into the set
    if set.insert(35) { println!("Inserted 35 into the set"); }
    println!("Set after inserting 35: {}, Length: {}", set, set.len());
    assert!(set.contains(&20));

    // Remove an element from the set
    if set.remove(&5) { println!("Removed 5 from the set"); }
    println!("Set after removal: {}, Length: {}", set, set.len());
    assert!(!set.contains(&5));

    // Try to take an element from the set, removing it in the process
    if let Some(taken) = set.take(&10) { println!("Took element {} from the set", taken); }
    println!("Set after take: {}, Length: {}", set, set.len());
    assert!(!set.contains(&10));

    // Use the random method to get a random element from the set
    let mut rng = WyRand::new();
    if let Some(element) = set.random(&mut rng).copied() {
        println!("Randomly selected element: {}", element);
        assert!(set.contains(&element));
        set.remove(&element);
        println!("Removed {} from the set", element);
        assert!(!set.contains(&element));
    }
    println!("Set after removal: {}, Length: {}", set, set.len());

    // Display the current elements and the set length
    println!("Final set: {}, Length: {}", set, set.len());
}
```
