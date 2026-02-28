from _hashlib import HASH
from collections.abc import Callable
from collections.abc import Set as AbstractSet
from typing import Literal, Protocol

from gxhash.buffer import Buffer as Buffer

algorithms_available: AbstractSet[str]
algorithms_guaranteed: AbstractSet[str]

class FileLike(Protocol):
    def fileno(self) -> int: ...

class BytesIOLike(Protocol):
    def getbuffer(self) -> Buffer: ...

def new(
    name: Literal["gxhash32", "gxhash64", "gxhash128"],
    data: Buffer = b"",
    *,
    seed: int = 0,
    usedforsecurity: bool = False,
    **kwargs: object,
) -> HASH:
    """
    Summary
    -------
    Returns a hash object implementing the given hash algorithm; optionally initialised with `data`.
    Note that GxHash is not an incremental hasher,
    and all inputs provided to the `update` method are accumulated internally.

    Parameters
    ----------
    name (`str`)
        the name of the hash algorithm to use; must be one of "gxhash32", "gxhash64", or "gxhash128"

    data (`Buffer?`)
        input data to initialise the hasher

    seed (`int?`)
        a signed 64-bit seed for the hasher [-2^63, 2^63)

    usedforsecurity (`bool?`)
        this parameter has no effect and is only present for compatibility with `hashlib`

    Returns
    -------
    gxhash (`HASH`)
        the GxHash object

    Example
    -------
    ```python
    >>> from gxhash.hashlib import new
    >>> hasher = new("gxhash32", b"hello", seed=42)
    >>> hasher.hexdigest()
    '9ffaa800'

    ```
    """

def file_digest(
    fileobj: BytesIOLike | FileLike,
    digest: str | Callable[[], HASH],
    *,
    seed: int = 0,
    **kwargs: object,
) -> HASH:
    """
    Summary
    -------
    Returns a hash object implementing the given hash algorithm, with the hash of the file-like object.

    Parameters
    ----------
    fileobj (`BytesIOLike | FileLike`)
        a file-like object with a fileno() method

    digest (`str | Callable[[], HASH]`)
        the name of the hash algorithm to use, or a zero-argument callable that returns a new hash object

    seed (`int?`)
        a signed 64-bit seed for the hasher [-2^63, 2^63); only used if `digest` is a string

    Returns
    -------
    gxhash (`HASH`)
        the hash object with the file's hash

    Example
    -------
    ```python
    >>> from gxhash.hashlib import file_digest
    >>> from io import BytesIO
    >>> file = BytesIO(b"hello")
    >>> hasher = file_digest(file, "gxhash32", seed=42)
    >>> hasher.digest()
    b'\x9f\xfa\xa8\x00'

    ```
    """

def gxhash32(data: Buffer = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH:
    """
    Summary
    -------
    Returns a GxHash32 hash object; optionally initialised with `data`.
    Note that GxHash is not an incremental hasher,
    and all inputs provided to the `update` method are accumulated internally.

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
    >>> from gxhash.hashlib import gxhash32
    >>> hasher = gxhash32(b"hello", seed=1234)
    >>> hasher.hexdigest()
    '72d987dc'

    ```
    """

def gxhash64(data: Buffer = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH:
    """
    Summary
    -------
    Returns a GxHash64 hash object; optionally initialised with `data`.
    Note that GxHash is not an incremental hasher,
    and all inputs provided to the `update` method are accumulated internally.

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
    >>> from gxhash.hashlib import gxhash64
    >>> hasher = gxhash64(b"hello", seed=1234)
    >>> hasher.hexdigest()
    '72d987dc0ecdfa46'

    ```
    """

def gxhash128(data: Buffer = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH:
    """
    Summary
    -------
    Returns a GxHash128 hash object; optionally initialised with `data`.
    Note that GxHash is not an incremental hasher,
    and all inputs provided to the `update` method are accumulated internally.

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
    >>> from gxhash.hashlib import gxhash128
    >>> hasher = gxhash128(b"hello", seed=1234)
    >>> hasher.hexdigest()
    '72d987dc0ecdfa46a11353e202d601bc'

    ```
    """
