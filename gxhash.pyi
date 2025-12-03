from typing import NewType, Protocol, TypeVar

T_co = TypeVar("T_co", covariant=True, bound=int)
Int32 = NewType("Int32", int)
Int64 = NewType("Int64", int)
Int128 = NewType("Int128", int)

class GxHashAsyncError(Exception):
    """
    Summary
    -------
    This error is raised when an asynchronous hash operation fails.
    """

class Hasher(Protocol[T_co]):
    def __init__(self, *, seed: int) -> None:
        """
        Summary
        -------
        Initialise `Hasher` with a `seed`.
        The `seed` should not be exposed as it is used to deterministically generate the hash.
        An exposed `seed` would put your service at a higher risk of a DoS attack.

        Parameters
        ----------
        seed (`int`)
            a seed for the hasher

        Example
        -------
        ```python
        hasher = GxHash32(seed=1234)
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
        print(f"Hash is {hasher.hash(bytes([42] * 1000))}!")
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
        hasher = GxHash128(seed=1234)
        print(f"Hash is {await hasher.hash_async(bytes([42] * 1000))}!")
        ```
        """

class GxHash32(Hasher[Int32]):
    """
    Summary
    -------
    This class exposes GxHash's 32-bit hash methods.
    """

class GxHash64(Hasher[Int64]):
    """
    Summary
    -------
    This class exposes GxHash's 64-bit hash methods.
    """

class GxHash128(Hasher[Int128]):
    """
    Summary
    -------
    This class exposes GxHash's 128-bit hash methods.
    """
