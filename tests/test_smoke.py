# ruff: noqa: S101, PLR2004, PLR0915

from __future__ import annotations

from _hashlib import HASH
from asyncio import run
from collections.abc import Generator
from contextlib import contextmanager
from hashlib import md5
from io import BytesIO
from sys import version_info
from tempfile import NamedTemporaryFile

from gxhash import GxHash32, GxHash64, GxHash128, GxHashAsyncError
from gxhash.hashlib import (
    algorithms_available,
    algorithms_guaranteed,
    file_digest,
    gxhash32,
    gxhash64,
    gxhash128,
    new,
)


def equal(hasher1: HASH, hasher2: HASH) -> bool:
    return (
        hasher1.digest() == hasher2.digest()
        and hasher1.hexdigest() == hasher2.hexdigest()
        and hasher1.name == hasher2.name
        and hasher1.block_size == hasher2.block_size
        and hasher1.digest_size == hasher2.digest_size
    )


@contextmanager
def raises(exception: type[BaseException]) -> Generator[None]:
    raised = False
    try:
        yield
    except exception:
        raised = True
    finally:
        assert raised


async def test_smoke() -> None:
    try:
        data = bytes(range(256))
        hashlib_md5 = md5(usedforsecurity=False)
        file = BytesIO(data)
        temporary_file = NamedTemporaryFile(delete=False)  # noqa: SIM115
        temporary_file.write(data)
        additional_data = b"additional data"
        extra_data = data + additional_data
        hasher32 = GxHash32(seed=0)
        hasher64 = GxHash64(seed=-(2**63))
        hasher128 = GxHash128(seed=2**63 - 1)
        gxhashlib32: HASH = gxhash32(data, seed=0, usedforsecurity=False, string=None)
        gxhashlib64: HASH = gxhash64(data, seed=-(2**63), usedforsecurity=False, string=None)
        gxhashlib128: HASH = gxhash128(data, seed=2**63 - 1, usedforsecurity=False, string=None)
        new_gxhashlib32 = new("gxhash32", data, seed=0, usedforsecurity=False, string=None)
        new_gxhashlib64 = new("gxhash64", data, seed=-(2**63), usedforsecurity=False, string=None)
        new_gxhashlib128 = new("gxhash128", data, seed=2**63 - 1, usedforsecurity=False, string=None)
        gxhashlib32_from_file = file_digest(file, "gxhash32", seed=0, usedforsecurity=False, string=None)
        gxhash64_digest_from_file = file_digest(file, "gxhash64", seed=-(2**63), usedforsecurity=False, string=None)
        gxhash128_digest_from_file = file_digest(file, "gxhash128", seed=2**63 - 1, usedforsecurity=False, string=None)
        gxhashlib32_copy = gxhashlib32.copy()
        gxhashlib64_copy = gxhashlib64.copy()
        gxhashlib128_copy = gxhashlib128.copy()
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
        assert equal(new_gxhashlib32, gxhashlib32)
        assert equal(new_gxhashlib64, gxhashlib64)
        assert equal(new_gxhashlib128, gxhashlib128)
        assert equal(gxhashlib32_from_file, gxhashlib32)
        assert equal(gxhash64_digest_from_file, gxhashlib64)
        assert equal(gxhash128_digest_from_file, gxhashlib128)
        assert equal(gxhashlib32_copy, gxhashlib32)
        assert equal(gxhashlib64_copy, gxhashlib64)
        assert equal(gxhashlib128_copy, gxhashlib128)
        gxhashlib32.update(additional_data)
        gxhashlib64.update(additional_data)
        gxhashlib128.update(additional_data)
        assert gxhashlib32.digest() == hasher32.hash(extra_data).to_bytes(4, "little")
        assert gxhashlib64.digest() == hasher64.hash(extra_data).to_bytes(8, "little")
        assert gxhashlib128.digest() == hasher128.hash(extra_data).to_bytes(16, "little")
        assert issubclass(gxhashlib32.__class__, hashlib_md5.__class__)
        assert issubclass(gxhashlib64.__class__, hashlib_md5.__class__)
        assert issubclass(gxhashlib128.__class__, hashlib_md5.__class__)
        assert isinstance(gxhashlib32, hashlib_md5.__class__)
        assert isinstance(gxhashlib64, hashlib_md5.__class__)
        assert isinstance(gxhashlib128, hashlib_md5.__class__)
        assert algorithms_available == algorithms_guaranteed == {"gxhash32", "gxhash64", "gxhash128"}
        temporary_file.close()
        with raises(AttributeError):
            hasher32.foo = 1  # pyright: ignore[reportAttributeAccessIssue]
            hasher64.foo = 1  # pyright: ignore[reportAttributeAccessIssue]
            hasher128.foo = 1  # pyright: ignore[reportAttributeAccessIssue]
        if version_info >= (3, 10):
            with raises(TypeError):
                GxHash32.foo = 1  # pyright: ignore[reportAttributeAccessIssue]
                GxHash64.foo = 1  # pyright: ignore[reportAttributeAccessIssue]
                GxHash128.foo = 1  # pyright: ignore[reportAttributeAccessIssue]
                type(gxhashlib32).foo = 1  # pyright: ignore[reportAttributeAccessIssue]
                type(gxhashlib64).foo = 1  # pyright: ignore[reportAttributeAccessIssue]
                type(gxhashlib128).foo = 1  # pyright: ignore[reportAttributeAccessIssue]
        raise GxHashAsyncError  # noqa: TRY301
    except GxHashAsyncError:
        pass


if __name__ == "__main__":
    run(test_smoke())
