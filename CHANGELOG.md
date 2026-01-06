# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Breaking:** Value only parameter type. Parameter type that has no key but holds a value.
- This changelog.
- Version information to sciformats_web.
- Reading of exported JSON data.

### Changed

- **Breaking (Rust only):** Use of SfError type instead of dynamic error in API functions.
- **Breaking:** JSON export header field name change from "name" to "format".
- Fixed minor navigation issues in sciformats_web.
- Fixed minor documentation issue.
- Updated dependencies.

### Removed

- Excessive logging in sciformats_web.

## [0.1.2] - 2025-10-24

### Added

- Switched to Trusted Publishing for crates.io and npmjs.com.

### Changed

- Updated dependencies.

## [0.1.1] - 2025-10-23

### Added

- Fixed release issue with crates.io.

## [0.1.0] - 2025-10-23

### Added

- Initial release.
