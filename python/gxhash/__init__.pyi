from typing import NewType, Protocol, TypeVar

T_co = TypeVar("T_co", covariant=True, bound=int, default=int)
Uint32 = NewType("Uint32", int)
Uint64 = NewType("Uint64", int)
Uint128 = NewType("Uint128", int)

class GxHashAsyncError(Exception):
    """
    Summary
    -------
    This error is raised when an asynchronous hash operation fails.
    """

class Hasher(Protocol[T_co]):
    def __init__(self, *, seed: int) -> None: ...
    def hash(self, bytes: bytes, /) -> T_co: ...
    async def hash_async(self, bytes: bytes, /) -> T_co: ...

class GxHash32(Hasher[Uint32]):
    """
    Summary
    -------
    This class exposes GxHash's 32-bit hash methods.
    """

class GxHash64(Hasher[Uint64]):
    """
    Summary
    -------
    This class exposes GxHash's 64-bit hash methods.
    """

class GxHash128(Hasher[Uint128]):
    """
    Summary
    -------
    This class exposes GxHash's 128-bit hash methods.
    """
