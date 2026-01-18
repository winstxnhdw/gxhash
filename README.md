# gxhash-py

[![GitHub](https://img.shields.io/badge/GitHub-black?logo=github)](https://github.com/winstxnhdw/gxhash)
[![codecov](https://codecov.io/github/winstxnhdw/gxhash/graph/badge.svg?token=L5FVMJTQUB)](https://codecov.io/github/winstxnhdw/gxhash)
[![python](https://img.shields.io/badge/python-3.8%20|%203.9%20|%203.10%20|%203.11%20|%203.12%20|%203.13%20|%203.14-blue)](https://www.python.org/)
[![main.yml](https://github.com/winstxnhdw/gxhash/actions/workflows/main.yml/badge.svg)](https://github.com/winstxnhdw/gxhash/actions/workflows/main.yml)
[![lint.yml](https://github.com/winstxnhdw/gxhash/actions/workflows/lint.yml/badge.svg)](https://github.com/winstxnhdw/gxhash/actions/workflows/lint.yml)
[![formatter.yml](https://github.com/winstxnhdw/gxhash/actions/workflows/formatter.yml/badge.svg)](https://github.com/winstxnhdw/gxhash/actions/workflows/formatter.yml)

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
- Pure Rust backend with zero additional Python runtime overhead.
- First-class support for parallel [asynchronous hashing](bench/README.md#asynchronous-hashing).
- Predictable, clean API with complete type safety.

## Installation

`gxhash` is available on PyPI and can be installed via `pip`.

```bash
pip install gxhash
```

> [!IMPORTANT]\
> This is only possible on systems that support `VAES` and `AVX2` instruction sets.

For the best throughput, you can allow `gxhash` to use wider registers by passing the `MATURIN_PEP517_ARGS` environment variable.

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

## Testing

You can run a comprehensive suite of tests with the following.

```bash
cargo test
```
