from ast import Yield, parse, walk
from asyncio import AbstractEventLoop, eager_task_factory, get_running_loop, run
from collections.abc import Callable, Coroutine, Generator, Iterable, Iterator
from enum import IntEnum
from hashlib import md5
from inspect import getsource
from itertools import count, product, repeat, takewhile
from logging import INFO, Formatter, Logger, StreamHandler, getLogger
from math import log
from os import urandom
from random import randint
from sys import argv
from time import perf_counter_ns
from typing import Concatenate, NewType, Self, TypedDict

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
    hasher: Callable[[bytes], Coroutine[None, None, int]]


class Progress:
    __slots__ = ("current", "step", "total")

    def __init__(self, *, total: int, step: int = 1) -> None:
        self.total = total
        self.step = step
        self.current = 0

    def __next__(self) -> tuple[int, int]:
        self.current += self.step
        return self.current, self.total


class EagerCoroutine[**P, R](Coroutine[None, None, R]):
    __slots__ = ("args", "data", "hasher", "kwargs")

    def __init__(self, hasher: Callable[Concatenate[bytes, P], R], *args: P.args, **kwargs: P.kwargs) -> None:
        self.hasher = hasher
        self.args = args
        self.kwargs = kwargs

    def __call__(self, data: bytes, /) -> Self:
        self.data = data
        return self

    def __await__(self) -> Generator[None, None, R]:
        return self.hasher(self.data, *self.args, **self.kwargs)
        yield

    def send(self, _: None) -> None:
        raise StopIteration(self.hasher(self.data, *self.args, **self.kwargs))

    def throw(self, typ: type[BaseException] | BaseException, *_) -> None:
        raise typ

    def close(self) -> None:
        return


async def gather[T](coroutines: Iterator[Coroutine[None, None, T]], /, *, loop: AbstractEventLoop) -> None:
    for future in tuple(map(loop.create_task, coroutines)):
        await future


async def benchmark(kwargs: Evaluand) -> EvaluationResult:
    loop = get_running_loop()
    loop.set_task_factory(eager_task_factory)

    hasher = kwargs["hasher"]
    hash_warmup_futures = map(hasher, kwargs["payloads_warmup"])
    hash_futures = map(hasher, kwargs["payloads"])

    start = perf_counter_ns()
    await gather(hash_warmup_futures, loop=loop)
    end = perf_counter_ns()
    cold_duration = Nanoseconds(end - start)

    start = perf_counter_ns()
    await gather(hash_futures, loop=loop)
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


def create_evaluands(
    *,
    payload_size: int,
    payload_count: int,
    progress: Progress,
    logger: Logger,
) -> Iterator[Evaluand]:
    seed = randint(0, 256)  # noqa: S311
    payloads_warmup = tuple(urandom(payload_size) for _ in repeat(None, payload_count))
    payloads = tuple(urandom(payload_size) for _ in repeat(None, payload_count))
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
        "hasher": EagerCoroutine(GxHash32(seed=seed).hash),
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
        "hasher": EagerCoroutine(lambda payload: int.from_bytes(gxhash32(payload, seed=seed).digest(), "little")),
    }
    yield {
        **metadata,
        "name": "XXH32",
        "length": Length.BIT_32,
        "hasher": EagerCoroutine(xxh32_intdigest, seed=seed),
    }
    yield {
        **metadata,
        "name": "MurmurHash3",
        "length": Length.BIT_32,
        "hasher": EagerCoroutine(hash32, seed=seed),
    }
    yield {
        **metadata,
        "name": "FarmHash32",
        "length": Length.BIT_32,
        "hasher": EagerCoroutine(FarmHash32WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "GxHash64",
        "length": Length.BIT_64,
        "hasher": EagerCoroutine(GxHash64(seed=seed).hash),
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
        "hasher": EagerCoroutine(lambda payload: int.from_bytes(gxhash64(payload, seed=seed).digest(), "little")),
    }
    yield {
        **metadata,
        "name": "XXH3",
        "length": Length.BIT_64,
        "hasher": EagerCoroutine(xxh64_intdigest, seed=seed),
    }
    yield {
        **metadata,
        "name": "CityHash64",
        "length": Length.BIT_64,
        "hasher": EagerCoroutine(CityHash64WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "FarmHash64",
        "length": Length.BIT_64,
        "hasher": EagerCoroutine(FarmHash64WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "MetroHash64",
        "length": Length.BIT_64,
        "hasher": EagerCoroutine(hash64_int, seed=seed),
    }
    yield {
        **metadata,
        "name": "StringZilla",
        "length": Length.BIT_64,
        "hasher": EagerCoroutine(stringzilla_hash, seed=seed),
    }
    yield {
        **metadata,
        "name": "GxHash128",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(GxHash128(seed=seed).hash),
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
        "hasher": EagerCoroutine(lambda payload: int.from_bytes(gxhash128(payload, seed=seed).digest(), "little")),
    }
    yield {
        **metadata,
        "name": "XXH128",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(xxh128_intdigest, seed=seed),
    }
    yield {
        **metadata,
        "name": "MurmurHash3",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(hash128, seed=seed),
    }
    yield {
        **metadata,
        "name": "CityHash128",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(CityHash128WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "FarmHash128",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(FarmHash128WithSeed, seed=seed),
    }
    yield {
        **metadata,
        "name": "MetroHash128",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(hash128_int, seed=seed),
    }
    yield {
        **metadata,
        "name": "MD5",
        "length": Length.BIT_128,
        "hasher": EagerCoroutine(
            lambda payload: int.from_bytes(md5(payload, usedforsecurity=False).digest(), "little")
        ),
    }

    logger.debug("\rRuns: %s/%s", *next(progress))


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
    steps = sum(type(node) is Yield for node in walk(parse(getsource(create_evaluands))))
    progress = Progress(total=sizes * counts * repeats * steps, step=steps)

    results = (
        run(benchmark(evaluand))
        for size, count in product(generate_sizes(base, max_size), payload_counts(counts))
        for evaluand in create_evaluands(payload_size=size, payload_count=count, progress=progress, logger=logger)
        for _ in repeat(None, repeats)
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

    dataframe.sort("batch_size", "payload_size", "length", "hot_duration").show(10_000)
    dataframe.write_parquet("benchmarks.parquet")
