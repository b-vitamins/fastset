# Development Standards for fastset

This document defines the contribution standards and development workflow for agents interacting with this repository.

## Commit Message Standard
- Use **Conventional Commits** (`type: description`), e.g. `feat: add paging support`.
- Common types: `feat`, `fix`, `perf`, `refactor`, `docs`, `style`, `test`, `ci`, `build`, `chore`.
- Keep messages concise (72 characters or less for the summary).
- Provide additional context in the commit body when necessary.

## Commit Sequencing
- Group related changes into atomic commits.
- Order commits logically (tests and docs can accompany related code changes).

## Pull Request Standards
- Title mirrors the main commit summary.
- Description should include:
  - Purpose of the change
  - Summary of implementation
  - Testing steps and results
  - References to issues or discussions if applicable
- PRs must pass `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test`.

## Code Housekeeping
- Update dependencies periodically using `cargo update`.
- Remove dead code and keep the codebase tidy.
- Address technical debt as part of regular maintenance.

## Architecture and Design
- Keep modules small and focused.
- Document public APIs in code using Rust doc comments.
- Avoid introducing unsafe code unless absolutely necessary and document its use.

## Pre-commit Checks
Run the following before committing:
```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Version Management
- Follow [SemVer](https://semver.org/) for crate versions in `Cargo.toml`.
- Tag releases with `vMAJOR.MINOR.PATCH`.
- Document changes in `CHANGELOG.md` under an "Unreleased" section until release.

## CHANGELOG Maintenance
- Add an entry for any user-facing change.
- Use subsections: `Added`, `Changed`, `Fixed`, `Removed`, `Security`.

## Testing Standards
- Keep unit tests in `src/` alongside modules.
- Ensure test coverage for new features.
- Benchmarks reside in `bench/` and are run with `cargo bench`.

## Documentation
- Update `README.md` when public APIs or usage change.
- Keep inline documentation up to date with code changes.

