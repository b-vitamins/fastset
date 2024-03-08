# fastset

![Crates.io](https://img.shields.io/crates/v/fastset)
![docs.rs](https://img.shields.io/docsrs/fastset)
![License](https://img.shields.io/crates/l/fastset)
![GitHub Workflow Status](https://github.com/b-vitamins/fastset/actions/workflows/rust.yml/badge.svg)

Fast set implementation for dense, bounded integer collections. Provides quick updates and random access.

The `fastset` crate provides a custom `Set` implementation, optimized for managing collections of `usize` values. Use cases involve having to manage indices of other data structures of a special kind, whose elements are densely packed within a known range and the insert and delete operations are voluminous, i.e., operation predictability and performance take precedence over memory footprint.

Example use cases:
    - For storing lattice sites of interest in stochastic cellular automata simulations
    - Managing available or used indices in large arrays.
    - Tracking slots in memory pools.
    
fastset::Set is not a good solution for memory contrained applications or for applications with storage need for sparse elements spread over a extended range.

## Features

- **Specialized for `usize`**: Tailored specifically for handling `usize` values, ideal for indexing scenarios.
- **High-Performance Operations**: Uses direct memory access for fast insertions, deletions, and random access.
- **Random Access**: Includes a `random` method to retrieve a random element from the set, essential for simulations and randomized algorithms.
- **Optimized for Dense Elements**: Efficiency is maximized when elements are closely packed within a pre-determined range.
- **Predictable Memory Usage**: Memory usage, even if will a large footprint, is predictable and directly related to the maximum element value.

## Benchmarks

Benchmarks comparing `fastset::Set` with `hashbrown::HashSet` and `std::collections::HashSet`:

| Operation | `Set (fastset)`  | `HashSet (hashbrown)` | `HashSet (std)`    |
|-----------|------------------|-----------------------|--------------------|
| insert    | 3.3772-3.4153 ns | 7.8488 - 7.9377 ns    | 21.632 - 21.811 ns |
| remove    | 3.0707-3.0856 ns | 5.1704 - 5.2657 ns    | 13.469 - 13.655 ns |
| contains  | 2.9405-2.9808 ns | 3.2842 - 3.3088 ns    | 16.706 - 16.988 ns |
| random    | 2.7410-2.7724 ns | N/A                   | N/A                |

Benchmarks were conducted on a machine with the following specifications:
- Processor: AMD Ryzen™ 5 5600G with Radeon™ Graphics x 12
- Memory: 58.8 GiB
- Operating System: Guix System
- OS Type: 64-bit

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
    if let Some(element) = set.random(&mut rng) {
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

## Note

This crate was developed in the context of Monte Carlo simulations of spin systems. In such simulations, it's required to track and provide fast access to elements of a given sign (positive or negative energy) from an array whose elements (spin energies) are rapidly changing signs due to Monte Carlo updates. The `fastset` `Set` facilitates relatively efficient tracking and manipulation of these elements over extended periods, improving the performance of simulations.