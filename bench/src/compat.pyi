from _hashlib import HASH
from collections.abc import Buffer, Callable
from typing import Concatenate

from cityhash import Uint32, Uint64, Uint128

def compat32[**P](
    hashlib_function: Callable[Concatenate[Buffer, P], HASH],
    *args: P.args,
    **kwargs: P.kwargs,
) -> Callable[[Buffer], Uint32]: ...
def compat64[**P](
    hashlib_function: Callable[Concatenate[Buffer, P], HASH],
    *args: P.args,
    **kwargs: P.kwargs,
) -> Callable[[Buffer], Uint64]: ...
def compat128[**P](
    hashlib_function: Callable[Concatenate[Buffer, P], HASH],
    *args: P.args,
    **kwargs: P.kwargs,
) -> Callable[[Buffer], Uint128]: ...
