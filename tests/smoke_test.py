# ruff: noqa: S101, PLR2004, PLR0915

from asyncio import run
from typing import Protocol

from gxhash import GxHash32, GxHash64, GxHash128, GxHashAsyncError, Hasher, T_co, Uint32, Uint64, Uint128
from gxhash.hashlib import HASH, Buffer, gxhash32, gxhash64, gxhash128


class NewHasher(Protocol[T_co]): ...


async def main() -> None:
    try:
        data: Buffer = bytes(range(256))
        additional_data = b"additional data"
        extra_data: Buffer = data + additional_data
        hasher32: Hasher[Uint32] = GxHash32(seed=0)
        hasher64: Hasher[Uint64] = GxHash64(seed=-(2**63))
        hasher128: Hasher[Uint128] = GxHash128(seed=2**63 - 1)
        gxhashlib32: HASH = gxhash32(data, seed=0, usedforsecurity=False, string=None)
        gxhashlib64: HASH = gxhash64(data, seed=-(2**63), usedforsecurity=False, string=None)
        gxhashlib128: HASH = gxhash128(data, seed=2**63 - 1, usedforsecurity=False, string=None)
        gxhashlib32_copy: HASH = gxhashlib32.copy()
        gxhashlib64_copy: HASH = gxhashlib64.copy()
        gxhashlib128_copy: HASH = gxhashlib128.copy()
        assert hasher32.hash(data) != hasher32.hash(extra_data)
        assert hasher64.hash(data) != hasher64.hash(extra_data)
        assert hasher128.hash(data) != hasher128.hash(extra_data)
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
        assert id(gxhashlib32) != id(gxhashlib32_copy)
        assert id(gxhashlib64) != id(gxhashlib64_copy)
        assert id(gxhashlib128) != id(gxhashlib128_copy)
        assert gxhashlib32_copy.digest() == gxhashlib32.digest()
        assert gxhashlib64_copy.digest() == gxhashlib64.digest()
        assert gxhashlib128_copy.digest() == gxhashlib128.digest()
        gxhashlib32.update(additional_data)
        gxhashlib64.update(additional_data)
        gxhashlib128.update(additional_data)
        assert gxhashlib32.digest() == hasher32.hash(extra_data).to_bytes(4, "little")
        assert gxhashlib64.digest() == hasher64.hash(extra_data).to_bytes(8, "little")
        assert gxhashlib128.digest() == hasher128.hash(extra_data).to_bytes(16, "little")
        assert gxhashlib32.__class__.__name__ == "HASH"
        assert gxhashlib64.__class__.__name__ == "HASH"
        assert gxhashlib128.__class__.__name__ == "HASH"
    except GxHashAsyncError as error:
        raise GxHashAsyncError from error


if __name__ == "__main__":
    run(main())
