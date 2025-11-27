# gxhash-py

[![uv](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/uv/main/assets/badge/v0.json)](https://github.com/astral-sh/uv)
[![python](https://img.shields.io/badge/python-3.8%20|%203.9%20|%203.10%20|%203.11%20|%203.12%20|%203.13%20|%203.14-blue)](https://www.python.org/)

Python bindings for [GxHash](https://github.com/ogxd/gxhash), a blazingly fast and robust non-cryptographic hashing algorithm.

## Features

- **Blazingly Fast**: Minimal-overhead binding to leverage the full speed of GxHash.
- **Zero Python**: Pure Rust backend with zero additional Python runtime overhead.
- **Async-Ready**: Tokio-powered async hashing for fast and efficient concurrency.
- **Fully Typesafe**: Predictable, clean API with complete type safety.

## Installation

### pip

`gxhash` is available on PyPI and can be installed via `pip`.

```bash
pip install gxhash
```

For best performance, you can enable the `hybrid` by passing the following `build-args` to the `--config-settings` flag.

```bash
pip install gxhash --config-settings build-args="--features hybrid"
```

By default, `gxhash` uses your system's vectorisation features. You can disable this by setting the relevant `RUSTFLAGS`.

```bash
RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+aes,+avx2" pip install gxhash
```

### uv

If you are using [uv](https://docs.astral.sh/uv/), you can enable the `hybrid` feature by adding the following `config-settings` under the `tool.uv` section.

```bash
uv add gxhash
```

```toml
[tool.uv]
config-settings = { build-args = "--features hybrid" }
```

## Usage

Hashing bytes.

```python
from gxhash import GxHash32

def main():
    gxhash = GxHash32(seed=0)
    result = gxhash.hash(b"Hello, world!")

if __name__ == "__main__":
    main()
```

Hashing bytes asynchronously.

```python
from asyncio import run
from gxhash import GxHash128

async def main():
    gxhash = GxHash128(seed=0)
    result = await gxhash.hash_async(b"Hello, world!")

if __name__ == "__main__":
    run(main())
```
