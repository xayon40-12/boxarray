# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.2] - 2026-01-13

### Fixes

- Accept pull request [Allocation improvements](https://github.com/xayon40-12/boxarray/pull/2) from [MuffinTastic](https://github.com/MuffinTastic):
  - Use `alloc` instead of `alloc_zeroed` to avoid unnecessary zero padding.
  - Use `std::ptr::write` to avoid dropping the data in the previously allocated array (as there is no data to be dropped). 

## [1.3.1] - 2025-01-05

### Fixes

- Properly handle [Zero-Sized Type](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).

### Added

- Example usage with a [Zero-Sized Type](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).
