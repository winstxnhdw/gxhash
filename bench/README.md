# Benchmarks

All benchmarks are updated regularly and the latest results can be found below. The benchmarks compare `gxhash` against other popular hashing Python libraries. The `hybrid` feature has been enabled for every benchmark.

## 32-bit

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 32-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-32bit.png"
    />
</div>

## 64-bit

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 64-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-64bit.png"
    />
</div>

## 128-bit

<div align="center">
    <img alt="Shows a bar chart with benchmark results for 128-bit hashes."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-128bit.png"
    />
</div>

## Asynchronous Hashing

`gxhash` includes first-class support for asynchronous hashing. As the benchmarks above show, the asynchronous variant performs significantly worse in single-hash scenarios because it incurs the overhead of spawning a thread per operation. When hashing many items concurrently, however, `gxhash` can keep all CPU cores busy and outperform the synchronous variant. In the benchmark below, we used batches of 16 payloads consistently across all payload sizes.

<div align="center">
    <img alt="Shows a bar chart with benchmark results for asynchronous hashing."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched.png"
    />
</div>

## Reproduction

To produce the benchmark parquet file, run the following command. This will produce a `benchmark.parquet` file in the current directory.

```bash
MATURIN_PEP517_ARGS="--features hybrid" uv run bench
```

To generate the plots from the parquet file, run the following command. This will produce the benchmark plots in the current directory.

```bash
cargo run
```

## Acknowledgements

This benchmark suite was inspired by [uv](https://github.com/astral-sh/uv).
