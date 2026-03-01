# Contributing Guide

We welcome all kinds of contributions. _You don't need to be an expert in Python or Rust development to help out._ The `gxhash-py` project has many CI checks in place to ensure that you can contribute with confidence.

<br>
<details>
<summary>An exhaustive (probably) list of checks for <code>gxhash-py</code></summary>
<br>

- Package installation matrix across all non-/free-threaded Python 3.8–3.14 and Ubuntu/macOS/Windows on x64/ARM64
- Cargo matrix test (Rust/PyO3) across all non-/free-threaded Python 3.8–3.14 and Ubuntu/macOS/Windows on x64/ARM64
- Rust MSRV override build matrix test across Ubuntu/macOS/Windows
- Rust code coverage generation
- Clippy linting for all Rust workspaces
- Python formatting
- Python doc/test/lint/type-checking across Python 3.8–3.14
- CodSpeed performance regression analysis test
- Source distribution content analysis with manual review gate on publish
- Smoke test with privileged approval gate on publish
- Smart release rollback on publish failure
- Automated version bump and changelog/release body generation with commitizen
- Mandatory lockfile updates
- Commitizen conventional commit message validation
- Trailing whitespace check
- Large file check
- Case conflict check
- End-of-file fixer
- Byte order marker fix
- TOML/YAML validation
- Mixed line ending check
- Symlink check
- Merge conflict check
- Private key detection
- Commit body blank line enforcement

</details>

## Prerequisites

- [Rust](https://rustup.rs/) (nightly)
- [uv](https://docs.astral.sh/uv/)

## Setup

Install all pre-commit hooks.

```bash
uv run prek install -t pre-commit -t commit-msg
```

If you are a volunteer, you can get away without installing the `commit-msg` hook.

```bash
uv run prek install -t pre-commit
```

## Development

After you are done finalising your changes, you can run all the necessary linter(s) and test(s) with the following.

```bash
uv run prek run
```

If you are working on a performance-sensitive change, you can run the benchmarks with `cargo`.

```bash
cargo bench --locked --manifest-path=perf/Cargo.toml
```

## Miscellaneous

If you need to see what is shipped to `pypi`, you can inspect the built distribution with the following.

```bash
make inspect
```

If you need a clean slate, you can remove all build artifacts with the following.

```bash
make clean
```
