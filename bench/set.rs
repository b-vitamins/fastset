use fastset::Set;
use std::collections::HashSet;
use hashbrown::HashSet as HashBrownSet;
use nanorand::{Rng, WyRand};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("fastset");
    let mut set = Set::with_capacity(100_000);
    let mut hashset: HashSet<usize> = HashSet::with_capacity(100_000);
    let mut hashbrownset: HashBrownSet<usize> = HashBrownSet::with_capacity(100_000);
    let mut rng = WyRand::new();
    let data: Vec<usize> = (0..1000).map(|_| rng.generate_range(0usize..=10000)).collect();

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

    set.clear();
    data.iter().for_each(|&x| { set.insert(x); });
    group.bench_function("remove (Set)", |b| {
        b.iter(|| {
            set.remove(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    hashset.clear();
    data.iter().for_each(|&x| { hashset.insert(x); });
    group.bench_function("remove (HashSet)", |b| {
        b.iter(|| {
            hashset.remove(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    hashbrownset.clear();
    data.iter().for_each(|&x| { hashset.insert(x); });
    group.bench_function("remove (hashbrown HashSet)", |b| {
        b.iter(|| {
            hashbrownset.remove(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    set.clear();
    data.iter().for_each(|&x| { set.insert(x); });
    group.bench_function("contains (Set)", |b| {
        b.iter(|| {
            set.contains(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    hashset.clear();
    data.iter().for_each(|&x| { hashset.insert(x); });
    group.bench_function("contains (HashSet)", |b| {
        b.iter(|| {
            hashset.contains(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    hashbrownset.clear();
    data.iter().for_each(|&x| { hashset.insert(x); });
    group.bench_function("contains (hashbrown HashSet)", |b| {
        b.iter(|| {
            hashbrownset.contains(black_box(&data[rng.generate_range(0usize..1000)]));
        })
    });

    set.clear();
    data.iter().for_each(|&x| { set.insert(x); });
    group.bench_function("random (Set)", |b| {
        b.iter(|| { set.random(&mut rng); })
    });

    group.finish();
}

criterion_group!(benches, bench_set);
criterion_main!(benches);
