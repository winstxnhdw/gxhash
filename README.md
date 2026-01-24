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
    <img alt="Shows a bar chart with benchmark results." src="https://i.ibb.co/SXSh79gL/throughput-128bit.png">
  </picture>
</p>

<p align="center">
  <i>128-bit hash throughput (MiB/s)</i>
</p>

Python bindings for [GxHash](https://github.com/ogxd/gxhash), a blazingly fast and robust non-cryptographic hashing algorithm.

## Highlights

- [Fastest non-cryptographic hash algorithm](bench/README.md) of its class.
- Guaranteed [stable hashes](https://github.com/ogxd/gxhash?tab=readme-ov-file#hashes-stability) across all supported platforms.
- Provides a type-safe drop-in replacement for the built-in [hashlib](https://docs.python.org/3/library/hashlib.html) module.
- Zero-copy data access across the FFI boundary via the [buffer protocol](https://docs.python.org/3/c-api/buffer.html).
- Pure Rust backend with zero additional Python runtime overhead.
- First-class support for parallel [asynchronous hashing](bench/README.md#asynchronous-hashing).
- Predictable, clean API with complete type safety.

## Installation

`gxhash` is available on PyPI and can be installed via `pip`.

```bash
pip install gxhash
```

For the best throughput, you can allow `gxhash` to use wider registers by installing with the `MATURIN_PEP517_ARGS` environment variable.

> [!WARNING]\
> This is only possible on systems that support `VAES` and `AVX2` instruction sets. Running on unsupported hardware will result in an illegal instruction error at **runtime**.

```bash
MATURIN_PEP517_ARGS="--features hybrid" pip install gxhash
```

By default, `gxhash` uses your system's vectorisation features. You can disable this by setting the relevant `RUSTFLAGS`.

```bash
RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+aes,+avx2" pip install gxhash
```

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

> [!NOTE]\
> Hash computation in `gxhash.hashlib` functions are deferred and only computed when `digest` or `hexdigest` is called.

```python
from gxhash.hashlib import gxhash128

def main() -> None:
    hasher = gxhash128(data=b"Hello, world!", seed=0)
    result = hasher.hexdigest()

if __name__ == "__main__":
    main()
```

## Contribute

Read the [CONTRIBUTING.md](CONTRIBUTING.md) docs for development setup and guidelines.
