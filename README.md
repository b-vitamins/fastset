# fastset

![Crates.io](https://img.shields.io/crates/v/fastset)
![docs.rs](https://img.shields.io/docsrs/fastset)
![License](https://img.shields.io/crates/l/fastset)
![GitHub Workflow Status](https://github.com/b-vitamins/fastset/actions/workflows/rust.yml/badge.svg)

Fast set implementation for dense, bounded integer collections, offering quick updates and random access.

## Features

 - Tailored for unsigned integer (`usize`) elements, ideal for index-based applications
 - Fast insertion, removal, and membership check
 - `random` method for uniform random sampling
 - Paging mechanism to somewhat mitigate the large memory footprint[^1]

 Note that while paging improves the existing memory footprint, 
 `fastset::Set` **is still not** a good solution for memory constrained applications 
 or for applications with storage need for sparse elements spread over an extended range.
 For integers twice as sparse as the page size, the `fastset::Set` with paging 
 has peak heap allocation ~ 8x that of `std::collections::HashSet`.

 [^1]: A paging mechanism is introduced in `0.4.0` that reduces the memory-footprint of `fastset::Set`.
 With the paging feature, `fastset::Set` achieves ~ 50% reduction in peak heap memory allocations 
 with no additional performance overhead.

## Benchmarks

 | Operation | `fastset::Set` | `hashbrown::HashSet` | `std::collections::HashSet` |
 |-----------|----------------|----------------------|-----------------------------|
 | insert    | 1.1632 ns      | 4.7105 ns            | 14.136 ns                   |
 | remove    | 1.1647 ns      | 3.0459 ns            | 10.625 ns                   |
 | contains  | 932.81 ps      | 985.38 ps            | 13.827 ns                   |
 | random    | 651.26 ps      | N/A                  | N/A                         |

 - CPU: AMD Ryzen™ 5 5600G with Radeon™ Graphics x 12
 - RAM: 58.8 GiB
 - OS: Guix System, 64-bit

## Usage

 ```rust
 use fastset::{set, Set};
 use nanorand::WyRand;

    let mut set = set![5, 10, 15, 20, 25, 30]; // Initialize set with elements
    assert!(set.contains(&5)); // Check for element presence

    set.insert(35); // Insert a new element
    assert!(set.contains(&35));

    set.remove(&5); // Remove an element
    assert!(!set.contains(&5));

    if let Some(taken) = set.take(&10) { // Remove and return an element
        assert_eq!(taken, 10);
    }

    let mut rng = WyRand::new();
    if let Some(element) = set.random(&mut rng) { // Get a random element
        set.remove(&element); // Remove the randomly selected element
        assert!(!set.contains(&element));
    }

    println!("Set: {:?}, Length: {}", set, set.len()); // Display the set and its length
 ```

## Delphic Sets

 `fastset::Set`, as implemented here, meets the conditions for being a Delphic set [1], [2]:

 Let Ω be a discrete universe. A set (S ⊆ Ω) is considered a member of a Delphic family if it supports the
 following operations within O(log |Ω|) time:

 - **Membership**: Verify if any element (x ∈ Ω) exists within (S).
 - **Cardinality**: Determine the size of (S), i.e., (|S|).
 - **Sampling**: Draw a uniform random sample from (S).

 A unit test in `src/set.rs` verifies the uniform sampling property with a basic [Chi-squared test](https://en.wikipedia.org/wiki/Chi-squared_test).

 ```rust
 use fastset::Set;
 use nanorand::WyRand;
 use statrs::distribution::{ChiSquared, ContinuousCDF};
 
 fn sampling_is_uniformly_at_random() {
     const SAMPLES: usize = 1_000_000;
     const EDGE_OF_THE_UNIVERSE: usize = 10000;

     let elements = (1..=EDGE_OF_THE_UNIVERSE).collect::<Vec<_>>();
     let set = Set::from(elements.clone());
     let mut rng = WyRand::new_seed(42u64);
     let mut counts = vec![0f64; elements.len()];

 (0..SAMPLES).for_each(|_| {
    if let Some(value) = set.random(&mut rng) {
        counts[value - 1] += 1.0;
    }
 });

     let e = SAMPLES as f64 / elements.len() as f64;
     let statistic: f64 = counts.iter().map(|&o| { (o - e) * (o - e) / e }).sum();

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
 ```

## References

 [1]: **Chakraborty, Sourav, N. V. Vinodchandran, and Kuldeep S. Meel.** *"Distinct Elements in Streams: An Algorithm for the (Text) Book."* arXiv preprint arXiv:2301.10191 (2023).

 [2]: **Meel, Kuldeep S., Sourav Chakraborty, and N. V. Vinodchandran.** *"Estimation of the Size of Union of Delphic Sets: Achieving Independence from Stream Size."* Proceedings of the 41st ACM SIGMOD-SIGACT-SIGAI Symposium on Principles of Database Systems. 2022.

## Development Setup

Run `./setup-dev.sh` once with internet access to prepare a fully
offline development environment. The script installs Rust, fetches all
crate dependencies, and runs the test suite.
