# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1]

Released on 2026-01-22

### Changed

- exclude changelog from sdist

## [0.3.0]

Released on 2026-01-20

### Added

- align runtime and interpreter lifetimes
- explicitly specify unsigned int
- allow users to import thin classes
- use more specific differentiating types using `NewType`
- allow user to subclass `Hasher`

### Fixed

- throw error if user tries to instantiate `Hasher`
- implement `__getitem__` to support generics only 3.8

### Changed

- default to nightly toolchain
- defer runtime build cost to ctor
- use native PyO3 async API
- support free-threaded Python
- add project URLs

## [0.2.7]

Released on 2025-12-01

### Fixed

- include README in sdist
