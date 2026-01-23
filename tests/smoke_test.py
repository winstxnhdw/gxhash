# ruff: noqa: S101, PLR2004

from asyncio import run
from typing import Protocol

from gxhash import GxHash32, GxHash64, GxHash128, GxHashAsyncError, Hasher, T_co, Uint32, Uint64, Uint128
from gxhash.hashlib import HASH, Buffer, gxhash32, gxhash64, gxhash128


class NewHasher(Protocol[T_co]): ...


async def main() -> None:
    try:
        data: Buffer = bytes(range(256))
        hasher32: Hasher[Uint32] = GxHash32(seed=0)
        hasher64: Hasher[Uint64] = GxHash64(seed=-(2**63))
        hasher128: Hasher[Uint128] = GxHash128(seed=2**63 - 1)
        gxhashlib32: HASH = gxhash32(data, seed=0, usedforsecurity=False, string=None)
        gxhashlib64: HASH = gxhash64(data, seed=-(2**63), usedforsecurity=False, string=None)
        gxhashlib128: HASH = gxhash128(data, seed=2**63 - 1, usedforsecurity=False, string=None)
        assert hasher32.hash(data) == await hasher32.hash_async(data)
        assert hasher64.hash(data) == await hasher64.hash_async(data)
        assert hasher128.hash(data) == await hasher128.hash_async(data)
        assert gxhashlib32.digest() == hasher32.hash(data).to_bytes(4, "little")
        assert gxhashlib64.digest() == hasher64.hash(data).to_bytes(8, "little")
        assert gxhashlib128.digest() == hasher128.hash(data).to_bytes(16, "little")
        assert gxhashlib32.hexdigest() == hasher32.hash(data).to_bytes(4, "little").hex()
        assert gxhashlib64.hexdigest() == hasher64.hash(data).to_bytes(8, "little").hex()
        assert gxhashlib128.hexdigest() == hasher128.hash(data).to_bytes(16, "little").hex()
        assert gxhashlib32.digest_size == 4
        assert gxhashlib64.digest_size == 8
        assert gxhashlib128.digest_size == 16
        assert gxhashlib32.block_size == 1
        assert gxhashlib64.block_size == 1
        assert gxhashlib128.block_size == 1
        assert gxhashlib32.name == "gxhash32"
        assert gxhashlib64.name == "gxhash64"
        assert gxhashlib128.name == "gxhash128"
        assert gxhashlib32.copy().digest() == gxhashlib32.digest()
        assert gxhashlib64.copy().digest() == gxhashlib64.digest()
        assert gxhashlib128.copy().digest() == gxhashlib128.digest()
        gxhashlib32.update(b"additional data")
        gxhashlib64.update(b"additional data")
        gxhashlib128.update(b"additional data")
        assert gxhashlib32.digest() == hasher32.hash(data + b"additional data").to_bytes(4, "little")
        assert gxhashlib64.digest() == hasher64.hash(data + b"additional data").to_bytes(8, "little")
        assert gxhashlib128.digest() == hasher128.hash(data + b"additional data").to_bytes(16, "little")
        assert gxhashlib32.__class__.__name__ == "HASH"
        assert gxhashlib64.__class__.__name__ == "HASH"
        assert gxhashlib128.__class__.__name__ == "HASH"
    except GxHashAsyncError as error:
        raise GxHashAsyncError from error


if __name__ == "__main__":
    run(main())
