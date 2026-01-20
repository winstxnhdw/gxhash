# ruff: noqa: S101

from asyncio import run
from typing import Protocol

from gxhash import GxHash32, GxHash64, GxHash128, GxHashAsyncError, Hasher, T_co, Uint32, Uint64, Uint128


class NewHasher(Protocol[T_co]): ...


async def main() -> None:
    try:
        data = bytes(range(256))
        hasher32: Hasher[Uint32] = GxHash32(seed=0)
        hasher64: Hasher[Uint64] = GxHash64(seed=-(2**63))
        hasher128: Hasher[Uint128] = GxHash128(seed=2**63 - 1)
        assert hasher32.hash(data) == await hasher32.hash_async(data)
        assert hasher64.hash(data) == await hasher64.hash_async(data)
        assert hasher128.hash(data) == await hasher128.hash_async(data)
    except GxHashAsyncError as error:
        raise GxHashAsyncError from error


if __name__ == "__main__":
    run(main())
