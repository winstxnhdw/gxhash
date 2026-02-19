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

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 32-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-32bit.png"
    />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## 64-bit

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 64-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-64bit.png"
    />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## 128-bit

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 128-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-128bit.png"
    />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## Asynchronous Hashing

`gxhash` includes first-class support for asynchronous hashing. As the previous benchmarks above show, the asynchronous variant performs significantly worse in single-hash scenarios because it incurs the overhead of spawning a thread per operation. However, when hashing many items concurrently, `gxhash` can keep all CPU cores busy and outperform the synchronous variant. In the benchmark below, we used batches of 16 payloads consistently across all payload sizes.

> [!NOTE]\
> Although xxHash and MD5 drop the GIL, and can technically perform multithreaded hashing, they do not provide a native async API. The best attempts at using `ThreadPoolExecutor` led to worse performance than their synchronous counterparts. Please submit a PR if you have a better approach for benchmarking xxHash asynchronously.

<div align="center">
    <img alt="Shows a bar chart with benchmark results for asynchronous hashing."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched.png"
    />
</div>

<p align="center">
  <i>Higher is better</i>
</p>

## Reproduction

To produce the benchmark parquet file, run the following command. This will produce a `benchmark.parquet` file in the current directory. The benchmark may take a few hours to complete, depending on your hardware.

> [!IMPORTANT]\
> You will need 6 GiB of RAM to avoid OOM errors.

```bash
MATURIN_PEP517_ARGS="--features hybrid" uv run bench
```

You can observe the progress of the benchmark by setting the log level to `DEBUG`.

```bash
MATURIN_PEP517_ARGS="--features hybrid" PYTHONUNBUFFERED=1 uv run bench DEBUG
```

To generate the plots from the parquet file, run the following command. This will produce the benchmark plots in the current directory.

```bash
cargo run
```

## Acknowledgements

This benchmark suite was inspired by [uv](https://github.com/astral-sh/uv).
