use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use fastset::Set;
use hashbrown::HashSet as HashBrownSet;
use nanorand::{Rng, WyRand};
use std::collections::HashSet;

/// Benchmark basic operations (insert, remove, contains, random)
fn bench_basic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_operations");

    // Setup test data
    let mut set = Set::with_capacity(100_000);
    let mut hashset: HashSet<usize> = HashSet::with_capacity(100_000);
    let mut hashbrownset: HashBrownSet<usize> = HashBrownSet::with_capacity(100_000);
    let mut rng = WyRand::new();
    let data: Vec<usize> = (0..1000)
        .map(|_| rng.generate_range(0usize..=10000))
        .collect();

    // Pre-populate sets for remove/contains benchmarks
    for i in 0..500 {
        set.insert(data[i]);
        hashset.insert(data[i]);
        hashbrownset.insert(data[i]);
    }

    group.bench_function("insert (Set)", |b| {
        b.iter(|| {
            set.insert(black_box(data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("insert (HashSet)", |b| {
        b.iter(|| {
            hashset.insert(black_box(data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("insert (hashbrown HashSet)", |b| {
        b.iter(|| {
            hashbrownset.insert(black_box(data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("remove (Set)", |b| {
        b.iter(|| {
            set.remove(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("remove (HashSet)", |b| {
        b.iter(|| {
            hashset.remove(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("remove (hashbrown HashSet)", |b| {
        b.iter(|| {
            hashbrownset.remove(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("contains (Set)", |b| {
        b.iter(|| {
            set.contains(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("contains (HashSet)", |b| {
        b.iter(|| {
            hashset.contains(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("contains (hashbrown HashSet)", |b| {
        b.iter(|| {
            hashbrownset.contains(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    group.bench_function("random (Set)", |b| {
        b.iter(|| {
            set.random(&mut rng);
        })
    });

    group.finish();
}

/// Benchmark operations with different set sizes
fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    let sizes = vec![100, 1_000, 10_000, 100_000];
    let mut rng = WyRand::new();

    for size in sizes {
        // Benchmark insertion scaling
        group.bench_with_input(BenchmarkId::new("insert", size), &size, |b, &size| {
            b.iter_batched(
                || Set::with_capacity(size * 2),
                |mut set| {
                    for _ in 0..size {
                        set.insert(rng.generate_range(0..size * 2));
                    }
                },
                BatchSize::SmallInput,
            );
        });

        // Benchmark memory allocation scaling
        group.bench_with_input(BenchmarkId::new("allocation", size), &size, |b, &size| {
            b.iter(|| Set::with_capacity(black_box(size)));
        });

        // Benchmark iteration scaling
        group.bench_with_input(BenchmarkId::new("iteration", size), &size, |b, &size| {
            let mut set = Set::with_capacity(size);
            for i in 0..size {
                set.insert(i);
            }
            b.iter(|| {
                let sum: usize = set.iter().sum();
                black_box(sum);
            });
        });
    }

    group.finish();
}

/// Benchmark different access patterns
fn bench_access_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("access_patterns");
    let mut rng = WyRand::new();
    let set_size = 10_000;

    // Sequential access pattern
    group.bench_function("sequential_insert", |b| {
        b.iter_batched(
            || Set::with_capacity(set_size),
            |mut set| {
                for i in 0..set_size {
                    set.insert(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    // Random access pattern
    group.bench_function("random_insert", |b| {
        b.iter_batched(
            || {
                let data: Vec<usize> = (0..set_size)
                    .map(|_| rng.generate_range(0..set_size * 2))
                    .collect();
                (Set::with_capacity(set_size * 2), data)
            },
            |(mut set, data)| {
                for value in data {
                    set.insert(value);
                }
            },
            BatchSize::SmallInput,
        );
    });

    // Sparse access pattern
    group.bench_function("sparse_insert", |b| {
        b.iter_batched(
            || Set::with_capacity(1_000_000),
            |mut set| {
                for i in 0..1000 {
                    set.insert(i * 1000);
                }
            },
            BatchSize::SmallInput,
        );
    });

    // Dense access pattern
    group.bench_function("dense_insert", |b| {
        b.iter_batched(
            || Set::with_capacity(1000),
            |mut set| {
                for i in 0..1000 {
                    set.insert(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark set operations (union, intersection, difference)
fn bench_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_operations");
    let mut rng = WyRand::new();

    let sizes = vec![(100, 100), (1000, 1000), (10000, 1000), (1000, 10000)];

    for (size1, size2) in sizes {
        let id = format!("{}x{}", size1, size2);

        // Create test sets
        let mut set1 = Set::with_capacity(size1 * 2);
        let mut set2 = Set::with_capacity(size2 * 2);
        let mut hashset1 = HashSet::with_capacity(size1);
        let mut hashset2 = HashSet::with_capacity(size2);

        // Populate with overlapping data
        for _ in 0..size1 {
            let val = rng.generate_range(0..size1 * 2);
            set1.insert(val);
            hashset1.insert(val);
        }
        for _ in 0..size2 {
            let val = rng.generate_range(0..size2 * 2);
            set2.insert(val);
            hashset2.insert(val);
        }

        // Union benchmarks
        group.bench_with_input(BenchmarkId::new("union_fastset", &id), &(), |b, _| {
            b.iter(|| {
                let result = &set1 | &set2;
                black_box(result);
            });
        });

        group.bench_with_input(BenchmarkId::new("union_hashset", &id), &(), |b, _| {
            b.iter(|| {
                let result: HashSet<_> = hashset1.union(&hashset2).copied().collect();
                black_box(result);
            });
        });

        // Intersection benchmarks
        group.bench_with_input(
            BenchmarkId::new("intersection_fastset", &id),
            &(),
            |b, _| {
                b.iter(|| {
                    let result = &set1 & &set2;
                    black_box(result);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("intersection_hashset", &id),
            &(),
            |b, _| {
                b.iter(|| {
                    let result: HashSet<_> = hashset1.intersection(&hashset2).copied().collect();
                    black_box(result);
                });
            },
        );

        // Difference benchmarks
        group.bench_with_input(BenchmarkId::new("difference_fastset", &id), &(), |b, _| {
            b.iter(|| {
                let result = &set1 - &set2;
                black_box(result);
            });
        });

        group.bench_with_input(BenchmarkId::new("difference_hashset", &id), &(), |b, _| {
            b.iter(|| {
                let result: HashSet<_> = hashset1.difference(&hashset2).copied().collect();
                black_box(result);
            });
        });
    }

    group.finish();
}

/// Benchmark memory-related operations
fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");

    // Benchmark clear operation
    group.bench_function("clear_small", |b| {
        b.iter_batched(
            || {
                let mut set = Set::with_capacity(100);
                for i in 0..100 {
                    set.insert(i);
                }
                set
            },
            |mut set| {
                set.clear();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("clear_large", |b| {
        b.iter_batched(
            || {
                let mut set = Set::with_capacity(100_000);
                for i in 0..10_000 {
                    set.insert(i * 10);
                }
                set
            },
            |mut set| {
                set.clear();
            },
            BatchSize::SmallInput,
        );
    });

    // Benchmark shrink operations
    group.bench_function("shrink_to_fit", |b| {
        b.iter_batched(
            || {
                let mut set = Set::with_capacity(100_000);
                for i in 0..1000 {
                    set.insert(i);
                }
                set
            },
            |mut set| {
                set.shrink_to_fit();
            },
            BatchSize::SmallInput,
        );
    });

    // Benchmark clone operation
    group.bench_function("clone_small", |b| {
        let mut set = Set::with_capacity(100);
        for i in 0..100 {
            set.insert(i);
        }
        b.iter(|| {
            let cloned = set.clone();
            black_box(cloned);
        });
    });

    group.bench_function("clone_large", |b| {
        let mut set = Set::with_capacity(100_000);
        for i in 0..10_000 {
            set.insert(i * 10);
        }
        b.iter(|| {
            let cloned = set.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

/// Benchmark edge cases and worst-case scenarios
fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");
    let mut rng = WyRand::new();

    // Benchmark repeated insertions of the same element
    group.bench_function("duplicate_inserts", |b| {
        let mut set = Set::with_capacity(100);
        set.insert(50);
        b.iter(|| {
            set.insert(black_box(50));
        });
    });

    // Benchmark alternating insert/remove
    group.bench_function("alternating_insert_remove", |b| {
        let mut set = Set::with_capacity(100);
        let mut insert = true;
        b.iter(|| {
            if insert {
                set.insert(black_box(50));
            } else {
                set.remove(black_box(&50));
            }
            insert = !insert;
        });
    });

    // Benchmark contains on non-existent elements
    group.bench_function("contains_miss", |b| {
        let mut set = Set::with_capacity(10_000);
        for i in 0..5000 {
            set.insert(i * 2); // Only even numbers
        }
        b.iter(|| {
            let val = rng.generate_range(0..5000) * 2 + 1; // Only odd numbers
            set.contains(black_box(&val));
        });
    });

    // Benchmark max/min operations
    group.bench_function("max_min_operations", |b| {
        let mut set = Set::with_capacity(10_000);
        for _ in 0..1000 {
            set.insert(rng.generate_range(0..10_000));
        }
        b.iter(|| {
            let max = set.max();
            let min = set.min();
            black_box((max, min));
        });
    });

    group.finish();
}

/// Benchmark iterator performance
fn bench_iterators(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterators");

    let sizes = vec![100, 1_000, 10_000];

    for size in sizes {
        // Create a set with the given size
        let mut set = Set::with_capacity(size);
        for i in 0..size {
            set.insert(i);
        }

        // Benchmark iteration and sum
        group.bench_with_input(BenchmarkId::new("iter_sum", size), &set, |b, set| {
            b.iter(|| {
                let sum: usize = set.iter().sum();
                black_box(sum);
            });
        });

        // Benchmark iteration and count
        group.bench_with_input(BenchmarkId::new("iter_count", size), &set, |b, set| {
            b.iter(|| {
                let count = set.iter().count();
                black_box(count);
            });
        });

        // Benchmark iteration with filter
        group.bench_with_input(BenchmarkId::new("iter_filter", size), &set, |b, set| {
            b.iter(|| {
                let evens: Vec<_> = set.iter().filter(|&&x| x % 2 == 0).collect();
                black_box(evens);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_operations,
    bench_scaling,
    bench_access_patterns,
    bench_set_operations,
    bench_memory_operations,
    bench_edge_cases,
    bench_iterators
);
criterion_main!(benches);
