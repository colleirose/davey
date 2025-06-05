# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5] - 2025-06-05

### Fixed:

- `DAVESession` will no longer create a decryptor for itself.
- Fixed an issue where on cleaning up expiring ciphers, it actually cleaned up valid ones.

## [0.1.4] - 2025-06-01

### Fixed:

- Using `DAVESession.canPassthrough` on a user ID that doesn't have a decryptor now just returns `false` instead of throwing an error.
- Fixed return type for `DAVESession.getVerificationCode`.
- Decryptors of users that have left the group will now be removed rather than retained.

## [0.1.3] - 2025-05-27

### Added

- `DAVESession.epoch`
- `DAVESession.ownLeafIndex`

### Changed

- Bumped Node-API version to 6

## [0.1.2] - 2025-04-04

### Added

- Functions for decryptor passthrough: `DAVESession.canPassthrough`, `DAVESession.setPassthroughMode`

### Fixed:

- Fixed setting an external sender possibly not re-creating the group.

## [0.1.1] - 2025-03-21

### Fixed:

- Fixed an issue where encryption did not properly set the codec and led to encryption failures.

## [0.1.0] - 2025-03-21

### Added

- Initial version of package.

[unreleased]: https://github.com/Snazzah/davey/compare/v0.1.5...HEAD
[0.1.5]: https://github.com/Snazzah/davey/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/Snazzah/davey/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/Snazzah/davey/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/Snazzah/davey/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Snazzah/davey/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Snazzah/davey/releases/tag/v0.1.0
