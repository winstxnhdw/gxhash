# gxhash-py

[![GitHub](https://img.shields.io/badge/GitHub-black?logo=github)](https://github.com/winstxnhdw/gxhash)
[![PyPi](https://img.shields.io/pypi/v/gxhash)](https://pypi.python.org/pypi/gxhash)
[![python](https://img.shields.io/badge/python-3.8%20|%203.9%20|%203.10%20|%203.11%20|%203.12%20|%203.13%20|%203.14-blue)](https://www.python.org/)
[![codecov](https://codecov.io/github/winstxnhdw/gxhash/graph/badge.svg?token=L5FVMJTQUB)](https://codecov.io/github/winstxnhdw/gxhash)
[![main.yml](https://github.com/winstxnhdw/gxhash/actions/workflows/main.yml/badge.svg)](https://github.com/winstxnhdw/gxhash/actions/workflows/main.yml)

<p align="center">
  <picture align="center">
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-cropped-dark.png" width=50%>
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-cropped-light.png" width=50%>
    <img alt="Shows a bar chart with benchmark results." src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-cropped-light.png">
  </picture>
</p>

<p align="center">
  <i>128-bit hash throughput (MiB/s)</i>
</p>

Python bindings for [GxHash](https://github.com/ogxd/gxhash), a blazingly fast and robust non-cryptographic hashing algorithm.

## Highlights

- [Fastest non-cryptographic hash algorithm](https://github.com/winstxnhdw/gxhash/blob/main/bench/README.md) of its class.
- Pure Rust backend with zero additional Python runtime overhead.
- Zero-copy data access across the FFI boundary via the [buffer protocol](https://docs.python.org/3/c-api/buffer.html).
- Support for [async hashing](https://github.com/winstxnhdw/gxhash/tree/main/bench#asynchronous-hashing) with multithreaded parallelism for non-blocking applications.
- Passes all [SMHasher](https://github.com/rurban/smhasher) tests and produces high-quality, hardware-accelerated 32/64/128-bit hashes.
- Guaranteed [stable hashes](https://github.com/ogxd/gxhash?tab=readme-ov-file#hashes-stability) across all supported platforms.
- Provides a [performant](https://github.com/winstxnhdw/gxhash/tree/main/bench#128-bit), drop-in replacement for the built-in [hashlib](https://docs.python.org/3/library/hashlib.html) module.
- SIMD-accelerated [hashlib](https://docs.python.org/3/library/hashlib.html) hex digest encoding via SSSE3/NEON.
- Fully-typed, clean API with uncompromising [strict-mode](https://github.com/microsoft/pyright/blob/main/docs/configuration.md#diagnostic-settings-defaults) conformance across all major type checkers.
- Zero-dependency installations on all platforms supported by [maturin](https://github.com/PyO3/maturin) and [puccinialin](https://github.com/konstin/puccinialin).

## Installation

`gxhash` is available on [PyPI](https://pypi.python.org/pypi/gxhash) and can be installed via `pip`.

```bash
pip install gxhash
```

For the best throughput, you can allow `gxhash` to use wider registers by installing with the `MATURIN_PEP517_ARGS` environment variable.

> [!WARNING]\
> This is only possible on systems that support `VAES` and `AVX2` instruction sets. Running on unsupported hardware will result in an illegal instruction error at **runtime**.

```bash
MATURIN_PEP517_ARGS="--features hybrid" pip install gxhash
```

By default, `gxhash` attempts to detect and use your system's vectorisation features. You can manually control this by setting the specific `RUSTFLAGS` for your machine. For x64 systems, the minimum required features are `aes` and `sse2`.

```bash
RUSTFLAGS="-C target-feature=+aes,+ssse3" pip install gxhash
```

For ARM64 systems, the minimum required features are `aes` and `neon`.

```bash
RUSTFLAGS="-C target-feature=+aes,+neon" pip install gxhash
```

## Supported Platforms

`gxhash` is well supported across a wide range of platforms, thanks in part to [maturin](https://github.com/PyO3/maturin), and more specifically [puccinialin](https://github.com/konstin/puccinialin). Therefore, `gxhash` supports [all platforms](https://www.maturin.rs/platform_support.html) that `maturin` and `puccinialin` support. `gxhash` is also actively tested on the following platforms:

- Ubuntu 24.04 x64
- Ubuntu 24.04 ARM64
- macOS 15 x64
- macOS 15 ARM64
- Windows Server 2025 x64
- Windows 11 ARM64

## Usage

Hashing bytes.

```python
from gxhash import GxHash32

def main() -> None:
    gxhash = GxHash32(seed=0)
    result = gxhash.hash(b"Hello, world!")

if __name__ == "__main__":
    main()
```

Hashing bytes asynchronously.

```python
from asyncio import run
from gxhash import GxHash128

async def main() -> None:
    gxhash = GxHash128(seed=0)
    result = await gxhash.hash_async(b"Hello, world!")

if __name__ == "__main__":
    run(main())
```

As a drop-in replacement for `hashlib`.

> [!WARNING]\
> [GxHash](https://github.com/ogxd/gxhash) is not an incremental hasher, and all inputs provided to the `update` method will be accumulated internally. This can lead to an unexpected increase in memory usage if you are expecting streaming behaviour.
> Also note that hash computation in `gxhash.hashlib` functions are deferred and only computed when `digest` or `hexdigest` is called.

```python
from gxhash.hashlib import gxhash128

def main() -> None:
    hasher = gxhash128(data=b"Hello, world!", seed=0)
    result = hasher.hexdigest()

if __name__ == "__main__":
    main()
```

## Contribute

Read the [CONTRIBUTING.md](https://github.com/winstxnhdw/gxhash/blob/main/CONTRIBUTING.md) docs for development setup and guidelines.
