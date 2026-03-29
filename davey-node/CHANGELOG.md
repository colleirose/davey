# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

## 0.1.11 - 2026-03-28

### Fixed:

Updated internal dependencies to fix vulnerabilities: https://github.com/cryspen/libcrux/security/advisories/GHSA-435g-fcv3-8j26 & https://github.com/cryspen/hpke-rs/security/advisories/GHSA-g433-pq76-6cmf

## 0.1.10 - 2026-03-02

### Fixed:

Updated the internal OpenMLS dependency to fix a high severity vulnerability: https://github.com/advisories/GHSA-8x3w-qj7j-gqhf

## 0.1.9 - 2025-12-19

### Added:

Support for H265, VP8, VP9 and AV1 encryption.

## 0.1.8 - 2025-11-17

### Fixed:

- Fixed an issue with setting passthrough mode not properly setting the expiry.

## 0.1.7 - 2025-09-23

### Added:

- Added support for H264 encryption.

## 0.1.6 - 2025-06-10

### Fixed:

- Forced resulting commits to only include proposal references. This problem resulted in voice servers not accepting our commits that have add proposals in them.

## 0.1.5 - 2025-06-05

### Fixed:

- `DAVESession` will no longer create a decryptor for itself.
- Fixed an issue where on cleaning up expiring ciphers, it actually cleaned up valid ones.

## 0.1.4 - 2025-06-01

### Fixed:

- Using `DAVESession.canPassthrough` on a user ID that doesn't have a decryptor now just returns `false` instead of throwing an error.
- Fixed return type for `DAVESession.getVerificationCode`.
- Decryptors of users that have left the group will now be removed rather than retained.

## 0.1.3 - 2025-05-27

### Added

- `DAVESession.epoch`
- `DAVESession.ownLeafIndex`

### Changed

- Bumped Node-API version to 6

## 0.1.2 - 2025-04-04

### Added

- Functions for decryptor passthrough: `DAVESession.canPassthrough`, `DAVESession.setPassthroughMode`

### Fixed:

- Fixed setting an external sender possibly not re-creating the group.

## 0.1.1 - 2025-03-21

### Fixed:

- Fixed an issue where encryption did not properly set the codec and led to encryption failures.

## 0.1.0 - 2025-03-21

### Added

- Initial version of package.
