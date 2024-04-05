# Changelog

## [Unreleased]

## [0.4.1] - 2024-04-05
### Added
- Added `CHANGELOG.md` to document the project's history and changes.

### Removed
- Removed default use of `jemalloc` as the global allocator.
- Added Chi-squared test for uniform random sampling to the test battery.

## [0.4.0] - 2024-04-02
### Added
- Introduced paging mechanism to reduce memory footprint.

## [0.3.0] - 2024-03-23
### Changed
- Changed the license from GPL-3.0 to MIT.

## [0.2.1] - 2024-03-22
### Changed
- Minor performance improvement in the random method through DMA.

## [0.2.0] - 2024-03-14
### Added
- Added `shrink_to` and `shrink_to_fit` methods to `Set` for memory optimization.

### Fixed
- Patch for `insert_unchecked` bug to handle potential safety issues.
- Ensured safety by calling `reserve` on `elements` for `insert_unchecked` to prevent possible buffer overflow errors.
