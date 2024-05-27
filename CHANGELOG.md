# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Do not take `self` for `Value::is_*` methods.

## [0.1.4] - 2024-04-11

### Changed

- Compress the size of numbers.

### Fixed

- Fix panic on unaligned pointer.

## [0.1.3] - 2023-11-20

### Added

- Add `is_*` methods for `Value`, `ValueRef` and `NumberRef`.
- Add `From<Number>`, `From<usize>` and `From<isize>` for `Value`.
- Add `Default` for `Value`.
- Add `PartialEq` for `ValueRef`.
- Add `pointer` for `Value` and `ValueRef`.
- Add `to_number` for `NumberRef`.
- Add `json!` macro. 

## [0.1.2] - 2023-10-30

### Added

- Add `From<&[u8]>` for `Value`.

## [0.1.1] - 2023-10-27

### Added

- Add `ObjectRef::contains_key`.
- Add `to_value`.
- Add feature `serde-json` and `Value::from_text_mut`.

### Fixed

- Remove data of duplicate keys when building objects.

## [0.1.0] - 2023-10-25

- Initial release.
