from ast import Yield, parse, walk
from asyncio import AbstractEventLoop, eager_task_factory, get_running_loop, run
from collections.abc import Callable, Coroutine, Iterable, Iterator
from enum import IntEnum
from hashlib import md5
from inspect import getsource
from itertools import count, islice, product, repeat, takewhile
from logging import INFO, Formatter, Logger, StreamHandler, getLogger
from math import log
from os import urandom
from random import randint
from sys import argv
from time import perf_counter_ns
from typing import NewType, TypedDict

from cityhash import CityHash64WithSeed, CityHash128WithSeed
from farmhash import FarmHash32WithSeed, FarmHash64WithSeed, FarmHash128WithSeed
from gxhash import GxHash32, GxHash64, GxHash128
from gxhash.hashlib import gxhash32, gxhash64, gxhash128
from metrohash import hash64_int, hash128_int
from mmh3 import hash as hash32
from mmh3 import hash128
from polars import LazyFrame, col
from stringzilla import hash as stringzilla_hash
from xxhash import xxh32_intdigest, xxh64_intdigest, xxh128_intdigest

from src.routine import EagerRoutine

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


class EvaluandMetadata[Payload](TypedDict):
    batch_size: int
    payload_size: int
    payloads_warmup: Iterable[Payload]
    payloads: Iterable[Payload]


class Evaluand[Payload, Result](TypedDict):
    name: str
    length: Length
    hasher: Callable[[Payload], Coroutine[None, None, Result]]


class Progress(Iterator[int]):
    __slots__ = ("current", "total")

    def __init__(self, *, total: int) -> None:
        self.current = 0
        self.total = total

    def __next__(self) -> int:
        self.current += 1
        return self.current


async def gather[Result](loop: AbstractEventLoop, coroutines: Iterator[Coroutine[None, None, Result]], /) -> None:
    for future in tuple(map(loop.create_task, coroutines)):
        await future


async def benchmark[Payload, Result](
    evaluand: Evaluand[Payload, Result],
    metadata: EvaluandMetadata[Payload],
) -> EvaluationResult:
    loop = get_running_loop()
    loop.set_task_factory(eager_task_factory)

    hasher = evaluand["hasher"]
    hash_warmup_futures = map(hasher, metadata["payloads_warmup"])
    hash_futures = map(hasher, metadata["payloads"])

    start = perf_counter_ns()
    await gather(loop, hash_warmup_futures)
    end = perf_counter_ns()
    cold_duration = Nanoseconds(end - start)

    start = perf_counter_ns()
    await gather(loop, hash_futures)
    end = perf_counter_ns()
    hot_duration = Nanoseconds(end - start)

    return {
        "name": evaluand["name"],
        "length": evaluand["length"],
        "batch_size": metadata["batch_size"],
        "payload_size": metadata["payload_size"],
        "cold_duration": cold_duration,
        "hot_duration": hot_duration,
    }


def setup_evaluands() -> Iterator[Evaluand[bytes, int]]:
    seed = randint(0, 256)  # noqa: S311

    yield {
        "name": "GxHash32",
        "length": Length.BIT_32,
        "hasher": EagerRoutine(GxHash32(seed=seed).hash),
    }
    yield {
        "name": "GxHash32 (async)",
        "length": Length.BIT_32,
        "hasher": GxHash32(seed=seed).hash_async,
    }
    yield {
        "name": "GxHashLib32",
        "length": Length.BIT_32,
        "hasher": EagerRoutine(lambda payload: int.from_bytes(gxhash32(payload, seed=seed).digest(), "little")),
    }
    yield {
        "name": "XXH32",
        "length": Length.BIT_32,
        "hasher": EagerRoutine(xxh32_intdigest, seed=seed),
    }
    yield {
        "name": "MurmurHash3",
        "length": Length.BIT_32,
        "hasher": EagerRoutine(hash32, seed=seed),
    }
    yield {
        "name": "FarmHash32",
        "length": Length.BIT_32,
        "hasher": EagerRoutine(FarmHash32WithSeed, seed=seed),
    }
    yield {
        "name": "GxHash64",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(GxHash64(seed=seed).hash),
    }
    yield {
        "name": "GxHash64 (async)",
        "length": Length.BIT_64,
        "hasher": GxHash64(seed=seed).hash_async,
    }
    yield {
        "name": "GxHashLib64",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(lambda payload: int.from_bytes(gxhash64(payload, seed=seed).digest(), "little")),
    }
    yield {
        "name": "XXH3",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(xxh64_intdigest, seed=seed),
    }
    yield {
        "name": "CityHash64",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(CityHash64WithSeed, seed=seed),
    }
    yield {
        "name": "FarmHash64",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(FarmHash64WithSeed, seed=seed),
    }
    yield {
        "name": "MetroHash64",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(hash64_int, seed=seed),
    }
    yield {
        "name": "StringZilla",
        "length": Length.BIT_64,
        "hasher": EagerRoutine(stringzilla_hash, seed=seed),
    }
    yield {
        "name": "GxHash128",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(GxHash128(seed=seed).hash),
    }
    yield {
        "name": "GxHash128 (async)",
        "length": Length.BIT_128,
        "hasher": GxHash128(seed=seed).hash_async,
    }
    yield {
        "name": "GxHashLib128",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(lambda payload: int.from_bytes(gxhash128(payload, seed=seed).digest(), "little")),
    }
    yield {
        "name": "XXH128",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(xxh128_intdigest, seed=seed),
    }
    yield {
        "name": "MurmurHash3",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(hash128, seed=seed),
    }
    yield {
        "name": "CityHash128",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(CityHash128WithSeed, seed=seed),
    }
    yield {
        "name": "FarmHash128",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(FarmHash128WithSeed, seed=seed),
    }
    yield {
        "name": "MetroHash128",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(hash128_int, seed=seed),
    }
    yield {
        "name": "MD5",
        "length": Length.BIT_128,
        "hasher": EagerRoutine(lambda payload: int.from_bytes(md5(payload, usedforsecurity=False).digest(), "little")),
    }


def generate_metadata(*, payload_size: int, payload_count: int) -> Iterator[EvaluandMetadata[bytes]]:
    yield {
        "payload_size": payload_size,
        "batch_size": payload_count,
        "payloads_warmup": tuple(urandom(payload_size) for _ in repeat(None, payload_count)),
        "payloads": tuple(urandom(payload_size) for _ in repeat(None, payload_count)),
    }


def replicate(repeats: int, *, progress: Progress, logger: Logger) -> Iterator[None]:
    return (logger.debug("\rRuns: %s/%s", current, progress.total) for current in islice(progress, repeats))


def generate_sizes(base: int, max_size: int) -> Iterator[int]:
    return takewhile(max_size.__ge__, map(pow, repeat(base), count(0)))


def payload_counts(counts: int) -> Iterator[int]:
    return map(pow, repeat(4), range(counts))


def main() -> None:
    handler = StreamHandler()
    handler.terminator = ""
    handler.setFormatter(Formatter("%(message)s"))
    logger = getLogger(__name__)
    logger.setLevel(INFO if len(argv) <= 1 else argv[1])
    logger.addHandler(handler)

    base = 4
    max_size = 256 << 20
    sizes = int(log(max_size, base)) + 1
    counts = 3
    repeats = 60
    steps = sum(type(node) is Yield for node in walk(parse(getsource(setup_evaluands))))
    rows = sizes * counts * steps
    progress = Progress(total=rows * repeats)

    results = (
        run(benchmark(evaluand, metadata))
        for size, count in product(generate_sizes(base, max_size), payload_counts(counts))
        for metadata in generate_metadata(payload_size=size, payload_count=count)
        for evaluand in setup_evaluands()
        for _ in replicate(repeats, progress=progress, logger=logger)
    )

    columns = ("name", "payload_size", "length", "batch_size")
    trimmed = (
        col(duration).is_between(col(duration).quantile(0.05).over(columns), col(duration).quantile(0.95).over(columns))
        for duration in ("cold_duration", "hot_duration")
    )

    dataframe = (
        LazyFrame(results)
        .filter(trimmed)
        .group_by(columns)
        .agg(col("cold_duration").mean(), col("hot_duration").mean())
        .collect(engine="streaming")
    )

    dataframe.sort("batch_size", "payload_size", "length", "hot_duration").show(rows)
    dataframe.write_parquet("benchmarks.parquet")
