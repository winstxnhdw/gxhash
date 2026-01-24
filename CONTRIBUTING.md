# Contributing Guide

We welcome all kinds of contributions. _You don't need to be an expert in Python or Rust development to help out._

## Prerequisites

- [Rust](https://rustup.rs/) (nightly)
- [uv](https://docs.astral.sh/uv/)

## Setup

Install all pre-commit hooks.

```bash
make pre-commit
```

## Development

Run all necessary linter(s) and test(s).

```bash
make test
```

You can inspect the built distribution with the following.

```bash
make inspect
```

If you need a clean slate, you can remove all build artifacts with the following.

```bash
make clean
```
