from hashlib import _Hash
from typing import Protocol

HASH = _Hash

class Buffer(Protocol):
    def __buffer__(self, flags: int, /) -> memoryview: ...

def gxhash32(data: Buffer = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH:
    """
    Summary
    -------
    Returns a GxHash32 hash object; optionally initialised with `data`.

    Parameters
    ----------
    data (`Buffer?`)
        input data to initialise the hasher

    seed (`int?`)
        a signed 64-bit seed for the hasher [-2^63, 2^63)

    usedforsecurity (`bool?`)
        this parameter has no effect and is only present for compatibility with `hashlib`

    Returns
    -------
    gxhash (`HASH`)
        the GxHash32 hash object

    Example
    -------
    ```python
    hasher = gxhash32(b"hello", seed=1234)
    print(f"Hash is {hasher.hexdigest()}!")
    ```
    """

def gxhash64(data: Buffer = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH:
    """
    Summary
    -------
    Returns a GxHash64 hash object; optionally initialised with `data`.


    Parameters
    ----------
    data (`Buffer?`)
        input data to initialise the hasher

    seed (`int?`)
        a signed 64-bit seed for the hasher [-2^63, 2^63)

    usedforsecurity (`bool?`)
        this parameter has no effect and is only present for compatibility with `hashlib`

    Returns
    -------
    gxhash (`HASH`)
        the GxHash64 hash object

    Example
    -------
    ```python
    hasher = gxhash64(b"hello", seed=1234)
    print(f"Hash is {hasher.hexdigest()}!")
    ```
    """

def gxhash128(data: Buffer = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH:
    """
    Summary
    -------
    Returns a GxHash128 hash object; optionally initialised with `data`.


    Parameters
    ----------
    data (`Buffer?`)
        input data to initialise the hasher

    seed (`int?`)
        a signed 64-bit seed for the hasher [-2^63, 2^63)

    usedforsecurity (`bool?`)
        this parameter has no effect and is only present for compatibility with `hashlib`

    Returns
    -------
    gxhash (`HASH`)
        the GxHash128 hash object

    Example
    -------
    ```python
    hasher = gxhash128(b"hello", seed=1234)
    print(f"Hash is {hasher.hexdigest()}!")
    ```
    """
