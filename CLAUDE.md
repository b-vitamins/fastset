# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Test
- `cargo build` - Build the project
- `cargo build --verbose` - Build with verbose output
- `cargo test` - Run all tests
- `cargo test --verbose` - Run tests with verbose output
- `cargo test -- --nocapture` - Run tests with output visible

### Benchmarks
- `cargo bench` - Run performance benchmarks
- `cargo bench --bench set` - Run specific set benchmarks

### Documentation
- `cargo doc` - Generate documentation
- `cargo doc --open` - Generate and open documentation

### Quality
- `cargo clippy` - Run linting
- `cargo fmt` - Format code
- `cargo check` - Fast compilation check

## Architecture

### Core Design
`fastset` is a specialized set implementation optimized for dense, bounded integer collections (`usize` elements). The main `Set` struct uses:

- **Bit vector indicator**: Fast O(1) membership checks via `Vec<bool>`
- **Element storage**: Compact `Vec<usize>` for iteration and sampling
- **Paging mechanism**: Memory optimization for sparse data (16-element pages)

### Module Structure
- `src/set/core.rs` - Main `Set` implementation with constructors and core logic
- `src/set/operators.rs` - Set operations (union, intersection, difference)
- `src/set/iterators.rs` - Iterator implementations 
- `src/set/conversions.rs` - Type conversions (From/Into traits)
- `src/set/traits.rs` - Trait implementations (Debug, PartialEq, etc.)
- `src/set/ops.rs` - SetOps trait defining core operations
- `src/set/tests.rs` - Unit tests including Chi-squared uniformity test

### Key Constants
- `MAX_CAPACITY: usize = 1_000_000_000` - Global capacity limit
- `PAGE_SIZE: usize = 16` - Paging granularity for memory optimization

### Delphic Set Properties
The implementation satisfies Delphic set requirements with O(log |Ω|) operations:
- Membership testing via bit vector lookup
- Cardinality through element count tracking  
- Uniform random sampling with Chi-squared validation

### Performance Characteristics
Optimized for dense integer sets with:
- Sub-nanosecond insert/remove/contains operations
- Uniform random sampling in ~650 picoseconds
- Memory usage ~8x HashSet for sparse data (2x page size)
- 50% memory reduction with paging vs non-paged version