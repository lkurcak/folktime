# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-04-21

### Added

- `no_std` support (no dependency on the Rust standard library).
- `Unit` enum and `with_min_unit()` builder for setting a floor on the
  displayed time unit.
- `#[must_use]` on all builder methods and constructors.
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` derives on `Style`.
- Public time-unit constants (`MIN`, `HOUR`, `DAY`, etc.) in `folktime::duration`.
- No-allocation test that verifies formatting never heap-allocates.
- CI workflow with clippy (pedantic) and tests.
- Publish workflow using crates.io trusted publishing.

### Changed

- `with_style()` and `with_min_unit()` are now `const fn`.
- Formatting methods are now `pub(crate)` instead of `pub`; use
  `Display` via `format!`/`write!` instead of calling them directly.
- Formatter sub-modules are now private.
- Edition updated to 2024.

### Fixed

- License field in `Cargo.toml` now correctly says `MIT OR Apache-2.0`
  (was `MIT OR UNLICENSE`).
- Missing doc-test fence on `Style::OneUnitWhole` variant.

## [0.2.0] - 2024-05-09

### Added

- `Style` enum with `OneUnitFrac` (default), `OneUnitWhole`, and
  `TwoUnitsWhole` formatting styles.
- `with_style()` builder method.

## [0.1.0] - 2024-05-09

### Added

- Initial release.
- `Folktime::duration()` for approximate human-friendly duration formatting.
- Three significant digit formatting with automatic unit selection
  (ns through Gy).
