from asyncio import gather, run
from collections.abc import Awaitable, Callable, Iterable, Iterator
from enum import IntEnum
from hashlib import md5
from itertools import product
from os import urandom
from random import randint
from time import perf_counter_ns
from typing import Concatenate, NewType, TypedDict

from cityhash import CityHash64WithSeed, CityHash128WithSeed
from farmhash import FarmHash32WithSeed, FarmHash64WithSeed, FarmHash128WithSeed
from gxhash import GxHash32, GxHash64, GxHash128
from gxhash.hashlib import gxhash32, gxhash64, gxhash128
from metrohash import hash64_int, hash128_int
from mmh3 import mmh3_32_uintdigest, mmh3_x64_128_uintdigest
from polars import LazyFrame, col
from xxhash import xxh32_intdigest, xxh64_intdigest, xxh128_intdigest

Nanoseconds = NewType("Nanoseconds", int)


class Length(IntEnum):
    BIT_32 = 32
    BIT_64 = 64
    BIT_128 = 128


class EvaluationResult(TypedDict):
    name: str
    length: Length
    batch_size: int
    payload_size: int
    cold_duration: Nanoseconds
    hot_duration: Nanoseconds


class EvaluandMetadata(TypedDict):
    batch_size: int
    payload_size: int
    payloads_warmup: Iterable[bytes]
    payloads: Iterable[bytes]


class Evaluand(EvaluandMetadata):
    name: str
    length: Length
    hasher: Callable[[bytes], Awaitable[int]]


async def wrap_async[**P](
    hasher: Callable[Concatenate[bytes, P], int],
    payload: bytes,
    *args: P.args,
    **kwargs: P.kwargs,
) -> int:
    return hasher(payload, *args, **kwargs)


def async_wrapper[**P](
    hasher: Callable[Concatenate[bytes, P], int],
    *args: P.args,
    **kwargs: P.kwargs,
) -> Callable[[bytes], Awaitable[int]]:
    return lambda payload: wrap_async(hasher, payload, *args, **kwargs)


async def benchmark(kwargs: Evaluand) -> EvaluationResult:
    hasher = kwargs["hasher"]
    hash_warmup_futures = map(hasher, kwargs["payloads_warmup"])
    hash_futures = map(hasher, kwargs["payloads"])

    start = perf_counter_ns()
    await gather(*hash_warmup_futures)
    end = perf_counter_ns()
    cold_duration = Nanoseconds(end - start)

    start = perf_counter_ns()
    await gather(*hash_futures)
    end = perf_counter_ns()
    hot_duration = Nanoseconds(end - start)

    return {
        "name": kwargs["name"],
        "length": kwargs["length"],
        "batch_size": kwargs["batch_size"],
        "payload_size": kwargs["payload_size"],
        "cold_duration": cold_duration,
        "hot_duration": hot_duration,
    }


def create_evaluands(*, payload_size: int, payload_count: int) -> Iterator[Evaluand]:
    seed = randint(0, 256)  # noqa: S311
    payloads_warmup = tuple(urandom(payload_size) for _ in range(payload_count))
    payloads = tuple(urandom(payload_size) for _ in range(payload_count))
    metadata: EvaluandMetadata = {
        "payload_size": payload_size,
        "payloads_warmup": payloads_warmup,
        "payloads": payloads,
        "batch_size": payload_count,
    }

    yield {
        **metadata,
        "name": "GxHash32",
        "length": Length.BIT_32,
        "hasher": async_wrapper(GxHash32(seed=seed).hash),
    }
    yield {
        **metadata,
        "name": "GxHash32 (async)",
        "length": Length.BIT_32,
        "hasher": GxHash32(seed=seed).hash_async,
    }
    yield {
        **metadata,
        "name": "GxHashLib32",
        "length": Length.BIT_32,
        "hasher": async_wrapper(lambda payload: int.from_bytes(gxhash32(payload, seed=seed).digest(), "big")),
    }
    yield {
        **metadata,
        "name": "XXH32",
        "length": Length.BIT_32,
        "hasher": async_wrapper(xxh32_intdigest, seed=seed),
    }
    yield {  # MurmurHash3 does not support kwargs
        **metadata,
        "name": "MurmurHash3",
        "length": Length.BIT_32,
        "hasher": async_wrapper(lambda payload: mmh3_32_uintdigest(payload, seed)),
    }
    yield {
        **metadata,
        "name": "FarmHash32",
        "length": Length.BIT_32,
        "hasher": async_wrapper(FarmHash32WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "GxHash64",
        "length": Length.BIT_64,
        "hasher": async_wrapper(GxHash64(seed=seed).hash),
    }
    yield {
        **metadata,
        "name": "GxHash64 (async)",
        "length": Length.BIT_64,
        "hasher": GxHash64(seed=seed).hash_async,
    }
    yield {
        **metadata,
        "name": "GxHashLib64",
        "length": Length.BIT_64,
        "hasher": async_wrapper(lambda payload: int.from_bytes(gxhash64(payload, seed=seed).digest(), "big")),
    }
    yield {
        **metadata,
        "name": "XXH3",
        "length": Length.BIT_64,
        "hasher": async_wrapper(xxh64_intdigest, seed=seed),
    }
    yield {
        **metadata,
        "name": "CityHash64",
        "length": Length.BIT_64,
        "hasher": async_wrapper(CityHash64WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "FarmHash64",
        "length": Length.BIT_64,
        "hasher": async_wrapper(FarmHash64WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "MetroHash64",
        "length": Length.BIT_64,
        "hasher": async_wrapper(hash64_int, seed=seed),
    }
    yield {
        **metadata,
        "name": "GxHash128",
        "length": Length.BIT_128,
        "hasher": async_wrapper(GxHash128(seed=seed).hash),
    }
    yield {
        **metadata,
        "name": "GxHash128 (async)",
        "length": Length.BIT_128,
        "hasher": GxHash128(seed=seed).hash_async,
    }
    yield {
        **metadata,
        "name": "GxHashLib128",
        "length": Length.BIT_128,
        "hasher": async_wrapper(lambda payload: int.from_bytes(gxhash128(payload, seed=seed).digest(), "big")),
    }
    yield {
        **metadata,
        "name": "XXH128",
        "length": Length.BIT_128,
        "hasher": async_wrapper(xxh128_intdigest, seed=seed),
    }
    yield {  # MurmurHash3 does not support kwargs
        **metadata,
        "name": "MurmurHash3",
        "length": Length.BIT_128,
        "hasher": async_wrapper(lambda payload: mmh3_x64_128_uintdigest(payload, seed)),
    }
    yield {
        **metadata,
        "name": "CityHash128",
        "length": Length.BIT_128,
        "hasher": async_wrapper(CityHash128WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "FarmHash128",
        "length": Length.BIT_128,
        "hasher": async_wrapper(FarmHash128WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "MetroHash128",
        "length": Length.BIT_128,
        "hasher": async_wrapper(hash128_int, seed=seed),
    }
    yield {
        **metadata,
        "name": "MD5",
        "length": Length.BIT_128,
        "hasher": async_wrapper(lambda payload: int.from_bytes(md5(payload, usedforsecurity=False).digest(), "big")),
    }


def generate_sizes() -> Iterator[int]:
    return (4**i for i in range(1, 14))


def payload_counts() -> Iterator[int]:
    return iter((1, 4, 16))


def main() -> None:
    results = (
        run(benchmark(evaluand))
        for size, count in product(generate_sizes(), payload_counts())
        for evaluand in create_evaluands(payload_size=size, payload_count=count)
        for _ in range(30)
    )

    dataframe = (
        LazyFrame(results)
        .group_by(col("name"), col("payload_size"), col("length"), col("batch_size"))
        .agg(col("cold_duration").mean(), col("hot_duration").mean())
        .collect(engine="streaming")
    )

    dataframe.sort("batch_size", "payload_size", "length", "hot_duration").show(10_000)
    dataframe.write_parquet("benchmarks.parquet")
