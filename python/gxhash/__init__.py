from typing import NewType, Protocol, TypeVar

from . import gxhash

__doc__ = gxhash.__doc__
__all__ = gxhash.__all__  # type: ignore[reportUnsupportedDunderAll]

T_co = TypeVar("T_co", covariant=True, bound=int, default=int)
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
        hasher = GxHash32(seed=2**63 - 1)
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
        hasher = GxHash64(seed=1234)
        print(f"Hash is {hasher.hash(bytes(range(256)))}!")
        ```
        """
        ...

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
        hasher = GxHash128(seed=1234)
        print(f"Hash is {await hasher.hash_async(bytes(range(256)))}!")
        ```
        """
        ...


def __getattr__(name: str) -> object:
    return getattr(gxhash, name)
