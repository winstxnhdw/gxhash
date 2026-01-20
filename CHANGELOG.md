# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0]

Released on 2026-01-20

### Added

- add benchmark plot
- align runtime and interpreter lifetimes
- explicitly specify unsigned int
- allow users to import thin classes
- use more specific differentiating types using `NewType`
- use custom extension and allow user to subclass `Hasher`
- add `gxhash` Python binding

### Fixed

- export actual `TypeVar`
- remove nightly feature
- share runtime across instances
- throw error if user tries to instantiate `Hasher`
- implement `__getitem__` to support generics only 3.8
- include README in sdist

### Changed

- default to nightly toolchain
- do not use `uvloop` - in case someone on windows wants to reproduce the benchmark
- clean up plotting script
- defer cost of building runtime to module init
- update lockfile
- defer runtime build cost to ctor
- exclude `tests/` directory
- remove redundant `?`
- use native PyO3 async API
- support free-threaded Python
- add project URLs
