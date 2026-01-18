# Benchmarks

All benchmarks are updated regularly and the latest results can be found below. The benchmarks compare `gxhash` against other popular hashing Python libraries.

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

`gxhash` has first-class support for asynchronous hashing. As seen in the benchmarks above, the asynchronous variant of `gxhash` greatly underperforms due to the overhead of spawning a thread for each hash operation. However, when hashing many items concurrently, `gxhash` is able to fully utilise all CPU cores and outperform its synchronous variant. In the benchmark below, we consistent used batches of 16 payloads across all payload sizes.

<div align="center">
    <img alt="Shows a bar chart with benchmark results for asynchronous hashing."
         src="https://raw.githubusercontent.com/wiki/winstxnhdw/gxhash/resources/throughput-batched.png"
    />
</div>

## Reproduction

To produce the benchmark parquet file, run the following command. This will produce a `benchmark.parquet` file in the current directory.

```bash
uv run bench
```

To generate the plots from the parquet file, run the following command. This will produce the benchmark plots in the current directory.

```bash
cargo run
```
