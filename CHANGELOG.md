<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

## [0.6.0]

Released on 2026-04-11

### Fixed

- update all dependencies (#158)
- update all dependencies (#128)
- update all dependencies (#121)

### Changed

- update `pygments` (2c32586)
- increase minimum `maturin` version to fix Windows install (#124)
- separate lazy hashlib note from warning (5c0aa09)
- bump rustls-webpki (#130)
- bump quinn-proto (#122)
- turn on pedantic lints (adbe362)
- remove nightly tag (7b04b99)
- rename positional parameter names in stub (0b43d12)
- add footnote (87338ee)

## [0.5.2]

Released on 2026-03-06

### Fixed

- update all dependencies (#114)

## [0.5.1]

Released on 2026-02-27

### Fixed

- ensure that `frozen` and `immutable_type` are applied

### Changed

- allow rust toolchain override

## [0.5.0]

Released on 2026-02-21

### Fixed

- re-export `__doc__` to `hashlib` module

### Changed

- no longer expose stubs to users

## [0.4.1]

Released on 2026-02-19

### Added

- add `hashlib`-compatible API

### Changed

- rewrite hex encoding with SIMD (#89)
- do not spawn new thread under 4 MiB (#42)
- use absolute links for better viewing on PyPi
- preserve stubs with mixed project layout

## [0.3.1]

Released on 2026-01-23

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
