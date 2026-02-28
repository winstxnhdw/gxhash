# Benchmarks

All benchmarks are updated regularly and the latest results can be found below. The benchmarks compare `gxhash` against other popular hashing Python libraries. If you suspect any bias or  would like to see another library included, please submit an [issue](https://github.com/winstxnhdw/gxhash/issues/new) or a [pull request](https://github.com/winstxnhdw/gxhash/compare).

- [Fairness](#fairness)
- [Throughput](#throughput)
  - [32-bit Throughput](#32-bit-throughput)
  - [64-bit Throughput](#64-bit-throughput)
  - [128-bit Throughput](#128-bit-throughput)
  - [Asynchronous Hashing Throughput](#asynchronous-hashing-throughput)
- [Latency](#latency)
  - [32-bit Latency](#32-bit-latency)
  - [64-bit Latency](#64-bit-latency)
  - [128-bit Latency](#128-bit-latency)
- [Reproduction](#reproduction)
- [Acknowledgements](#acknowledgements)

## Fairness

- All benchmarks are measured before and after a warm-up phase.
- Each benchmark is run multiple times and the average wall time is reported.
- The top and bottom 5% of the results are discarded to mitigate outliers.
- The most performant configuration for each library is used.
- Event loop is torn down between each benchmark.
- Seed and payload(s) are randomly shuffled between each run to avoid caching effects.
- No long-lived reference cycles to avoid interference from the garbage collector.

## Throughput

The throughput benchmarks measure the number of bytes that can be hashed per second. The bar charts below show the average throughput for each library across 32-bit, 64-bit, and 128-bit hashes. The payload size for the throughput benchmarks is 64 KiB, which is a common size for hashing operations in real-world applications.

### 32-bit Throughput

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with throughput benchmark results for 32-bit hashes."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-32bit.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with throughput benchmark results for 32-bit hashes with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-32bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

### 64-bit Throughput

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with throughput benchmark results for 64-bit hashes."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-64bit.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with throughput benchmark results for 64-bit hashes with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-64bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

### 128-bit Throughput

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with throughput benchmark results for 128-bit hashes."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-128bit.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with throughput benchmark results for 128-bit hashes with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-128bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

### Asynchronous Hashing Throughput

`gxhash` includes first-class support for asynchronous hashing. As the bar charts above illustrate, the asynchronous variant is expected to perform worse in single-hash scenarios because it **may** incur the overhead of spawning a thread. However, when there are concurrent hashing requests, `gxhash` can keep all CPU cores busy and outperform the synchronous variant. In the benchmark below, we used batches of 16 payloads across all payload sizes.

> [!NOTE]
> Although xxHash and MD5 drop the GIL, and can technically perform multithreaded hashing, they do not provide a native async API. The best attempts at using `ThreadPoolExecutor` led to worse performance than their synchronous counterparts. Please submit a PR if you have a better approach for benchmarking these third-party hashers asynchronously.

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a line chart with throughput benchmark results for asynchronous hashing."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a line chart with throughput benchmark results for asynchronous hashing with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched-vaes.svg"
  />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## Latency

The latency benchmarks measure the time taken to hash a single payload. The latency values in the charts below **should not** be taken literally, as the Python benchmark harness incurs significant overhead. In practice, the latency is on the order of nanoseconds. Still, the relative latency between libraries is apparent.

### 32-bit Latency

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with latency benchmark results for 32-bit hashing."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/latency-32bit.svg"
  />
</div>

<p align="center">
  <i>Lower is better</i>
</p>

With `--features hybrid`.

<div align="center">
    <img alt="Shows a bar chart with latency benchmark results for 32-bit hashing with VAES."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/latency-32bit-vaes.svg"
    />
</div>

<p align="center">
  <i>Lower is better</i>
</p>

### 64-bit Latency

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with latency benchmark results for 64-bit hashing."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/latency-64bit.svg"
  />
</div>

<p align="center">
  <i>Lower is better</i>
</p>

With `--features hybrid`.

<div align="center">
    <img alt="Shows a bar chart with latency benchmark results for 64-bit hashing with VAES."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/latency-64bit-vaes.svg"
    />
</div>

<p align="center">
  <i>Lower is better</i>
</p>

### 128-bit Latency

Without `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with latency benchmark results for 128-bit hashing."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/latency-128bit.svg"
  />
</div>

<p align="center">
  <i>Lower is better</i>
</p>

With `--features hybrid`.

<div align="center">
  <img alt="Shows a bar chart with latency benchmark results for 128-bit hashing with VAES."
    src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/latency-128bit-vaes.svg"
  />
</div>

<p align="center">
  <i>Lower is better</i>
</p>

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
