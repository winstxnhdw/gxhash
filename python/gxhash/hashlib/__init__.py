from _hashlib import HASH as HASH
from typing import Protocol

from gxhash.gxhashlib import file_digest as file_digest
from gxhash.gxhashlib import gxhash32 as gxhash32
from gxhash.gxhashlib import gxhash64 as gxhash64
from gxhash.gxhashlib import gxhash128 as gxhash128
from gxhash.gxhashlib import new as new
from gxhash.hashlib.algorithms import algorithms_available as algorithms_available
from gxhash.hashlib.algorithms import algorithms_guaranteed as algorithms_guaranteed


class Buffer(Protocol):
    def __buffer__(self, flags: int, /) -> memoryview: ...


class FileLike(Protocol):
    def fileno(self) -> int: ...


class BytesIOLike(Protocol):
    def getbuffer(self) -> Buffer: ...
