from typing import NewType, Protocol, TypeVar

__doc__: str
T_co = TypeVar("T_co", covariant=True, bound=int)
Uint32 = NewType("Uint32", int)
Uint64 = NewType("Uint64", int)
Uint128 = NewType("Uint128", int)

class Hasher(Protocol[T_co]):
    def __init__(self, *, seed: int) -> None:
        """
        Summary
        -------
        Initialise `Hasher` with a a signed 64-bit `seed`.
        The `seed` should not be exposed as it is used to deterministically generate the hash.
        An exposed `seed` would put your service at a higher risk of a DoS attack.

        Parameters
        ----------
        seed (`int`)
            a signed 64-bit seed for the hasher [-2^63, 2^63)

        Example
        -------
        ```python
        >>> from gxhash import GxHash32
        >>> GxHash32(seed=2**63 - 1)
        <builtins.GxHash32 object at 0x...>

        ```
        """

    def hash(self, bytes: bytes, /) -> T_co:
        """
        Summary
        -------
        Hashes `bytes` to an `int`.
        This method has less overhead than `hash_async`.

        Parameters
        ----------
        bytes (`bytes`)
            input bytes

        Returns
        -------
        hash (`int`)
            the hash of the input bytes

        Example
        -------
        ```python
        >>> from gxhash import GxHash64
        >>> hasher = GxHash64(seed=1234)
        >>> hasher.hash(bytes(range(256)))
        12522596144082598891

        ```
        """

    async def hash_async(self, bytes: bytes, /) -> T_co:
        """
        Summary
        -------
        Hashes `bytes` to an `int` asynchronously.
        This method allows you to compute multiple hashes with true multi-threaded parallelism.
        If called sequentially, this method is slightly less performant than the default `hash` method.
        Otherwise, this variant offers the highest throughput.

        Parameters
        ----------
        bytes (`bytes`)
            input bytes

        Returns
        -------
        hash (`int`)
            the hash of the input bytes

        Example
        -------
        ```python
        >>> from gxhash import GxHash128
        >>> from asyncio import run
        >>> hasher = GxHash128(seed=1234)
        >>> run(hasher.hash_async(bytes(range(256))))
        117181821629540739333037011138327886827

        ```
        """

class GxHashAsyncError(Exception):
    """
    Summary
    -------
    This error is raised when an asynchronous hash operation fails.
    """

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
