# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate
### Added
- Show `__awaitee` type name ([#2]).

### Changed
- The `-h` parameter now filters also types, not only fields.

### Fixed
- Support nightly after 2024-03-22 ([#4]).

[#2]: https://github.com/loyd/top-type-sizes/issues/2
[#4]: https://github.com/loyd/top-type-sizes/pull/4

## [0.1.5] - 2023-03-16
### Added
- More useful examples.
- CI checks.

### Changed
- Use `cargo-release`.
- Merge variants after applying `-h` and `-s` transformations.

### Fixed
- Specify missing metadata in the package's info.
- Now `-h` also affects discriminants.

## [0.1.4] - 2023-03-07
### Added
- Support `upvar` and `local` fields.

### Changed
- Initial release with CHANGELOG.

<!-- next-url -->
[Unreleased]: https://github.com/loyd/top-type-sizes/compare/v0.1.5...HEAD
[0.1.5]: https://github.com/loyd/top-type-sizes/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/loyd/top-type-sizes/releases/tag/v0.1.4
