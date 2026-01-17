from collections.abc import Awaitable, Callable, Iterator
from enum import IntEnum
from hashlib import md5
from itertools import product
from os import urandom
from random import randint
from time import perf_counter_ns
from typing import Concatenate, NewType, TypedDict

from gxhash import GxHash32, GxHash64, GxHash128
from polars import LazyFrame, col
from uvloop import run
from xxhash import xxh32_intdigest, xxh64_intdigest, xxh128_intdigest

Nanoseconds = NewType("Nanoseconds", int)


class Length(IntEnum):
    BIT_32 = 32
    BIT_64 = 64
    BIT_128 = 128


class Evaluand(TypedDict):
    name: str
    length: int
    hasher: Callable[[bytes], Awaitable[int]]
    payload_warmup: bytes
    payload: bytes


class EvaluationResult(TypedDict):
    name: str
    length: int
    payload_size: int
    cold_duration: Nanoseconds
    hot_duration: Nanoseconds


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
    payload_warmup = kwargs["payload_warmup"]
    payload = kwargs["payload"]

    start = perf_counter_ns()
    await hasher(payload_warmup)
    end = perf_counter_ns()
    cold_duration = Nanoseconds(end - start)

    start = perf_counter_ns()
    await hasher(payload)
    end = perf_counter_ns()
    hot_duration = Nanoseconds(end - start)

    return {
        "name": kwargs["name"],
        "length": kwargs["length"],
        "payload_size": len(payload),
        "cold_duration": cold_duration,
        "hot_duration": hot_duration,
    }


def create_evaluands(seed: int, payload_warmup: bytes, payload: bytes) -> tuple[Evaluand, ...]:
    gxhash32 = GxHash32(seed=seed)
    gxhash64 = GxHash64(seed=seed)
    gxhash128 = GxHash128(seed=seed)

    return (
        {
            "name": "GxHash32",
            "length": Length.BIT_32,
            "hasher": async_wrapper(gxhash32.hash),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "XXH32",
            "length": Length.BIT_32,
            "hasher": async_wrapper(xxh32_intdigest, seed=seed),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "GxHash64",
            "length": Length.BIT_64,
            "hasher": async_wrapper(gxhash64.hash),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "XXH3",
            "length": Length.BIT_64,
            "hasher": async_wrapper(xxh64_intdigest, seed=seed),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "GxHash128",
            "length": Length.BIT_128,
            "hasher": async_wrapper(gxhash128.hash),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "XXH128",
            "length": Length.BIT_128,
            "hasher": async_wrapper(xxh128_intdigest, seed=seed),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "MD5",
            "length": Length.BIT_128,
            "hasher": async_wrapper(
                lambda payload: int.from_bytes(md5(payload, usedforsecurity=False).digest(), "big"),
            ),
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "GxHash32 (async)",
            "length": Length.BIT_32,
            "hasher": gxhash32.hash_async,
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "GxHash64 (async)",
            "length": Length.BIT_64,
            "hasher": gxhash64.hash_async,
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
        {
            "name": "GxHash128 (async)",
            "length": Length.BIT_128,
            "hasher": gxhash128.hash_async,
            "payload_warmup": payload_warmup,
            "payload": payload,
        },
    )


def generate_sizes() -> Iterator[int]:
    return (4**i for i in range(1, 16))


def generate_seeds() -> Iterator[int]:
    return (randint(0, 256) for _ in range(4))  # noqa: S311


async def bench() -> None:
    evaluands = (
        evaluand
        for seed, size in product(generate_seeds(), generate_sizes())
        for evaluand in create_evaluands(seed=seed, payload_warmup=urandom(size), payload=urandom(size))
        for _ in range(30)
    )

    dataframe = (
        LazyFrame([await benchmark(evaluand) for evaluand in evaluands])
        .group_by(col("name"), col("payload_size"), col("length"))
        .agg(col("cold_duration").mean(), col("hot_duration").mean())
        .collect()
    )

    dataframe.show(10_000)
    dataframe.write_parquet("benchmarks.parquet")


def main() -> None:
    run(bench())
