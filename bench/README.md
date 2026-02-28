# Benchmarks

All benchmarks are updated regularly and the latest results can be found below. The benchmarks compare `gxhash` against other popular hashing Python libraries. The `hybrid` feature has been enabled for every benchmark.

## Fairness

- All benchmarks are measured before and after a warm-up phase.
- Each benchmark is run multiple times and the average throughput is reported.
- The most performant configuration for each library is used.
- Event loop is torn down between each benchmark.
- Seed and payload(s) are randomly shuffled between each run to avoid caching effects.
- No long-lived reference cycles to avoid interference from the garbage collector.

## 32-bit

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with benchmark results for 32-bit hashes."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-32bit.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with benchmark results for 32-bit hashes with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-32bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## 64-bit

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with benchmark results for 64-bit hashes."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-64bit.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with benchmark results for 64-bit hashes with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-64bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## 128-bit

Without `--features hybrid`.

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 128-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-128bit.svg"
    />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with benchmark results for 128-bit hashes with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-128bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## Asynchronous Hashing

`gxhash` includes first-class support for asynchronous hashing. As the bar plots above show, the asynchronous variant is expected to perform worse in single-hash scenarios because it **may** incur the overhead of spawning a thread. However, when there are concurrent hashing requests, `gxhash` can keep all CPU cores busy and outperform the synchronous variant. In the benchmark below, we used batches of 16 payloads consistently across all payload sizes.

> [!NOTE]
> Although xxHash and MD5 drop the GIL, and can technically perform multithreaded hashing, they do not provide a native async API. The best attempts at using `ThreadPoolExecutor` led to worse performance than their synchronous counterparts. Please submit a PR if you have a better approach for benchmarking these third-party hashers asynchronously.

Without `--features hybrid`.

<div align="center">
    <img alt="Shows a line chart with benchmark results for asynchronous hashing."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched.svg"
    />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
    <img alt="Shows a line chart with benchmark results for asynchronous hashing with VAES."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched-vaes.svg"
    />
</div>

## Reproduction

To produce the benchmark parquet file, run the following command. This will produce a `benchmark.parquet` file in the current directory. Depending on your hardware, The benchmark may take up to an hour to complete.

> [!IMPORTANT]
> You will need 6 GiB of RAM to avoid OOM errors.

```bash
MATURIN_PEP517_ARGS="--features hybrid" sudo -E nice -n -20 ionice -c 1 -n 0 su -c \
  "uv run --reinstall --no-cache --no-dev --locked bench || echo 'SIGILL - Unsupported platform'" $USER
```

You can observe the progress of the benchmark by setting the log level to `DEBUG`.

```bash
PYTHONUNBUFFERED=1 uv run bench DEBUG
```

To generate the plots from the parquet file, run the following command. This will produce the benchmark plots in the current directory.

```bash
cargo run
```

## Acknowledgements

This benchmark suite was inspired by [uv](https://github.com/astral-sh/uv).
