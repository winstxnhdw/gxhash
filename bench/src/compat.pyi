from _hashlib import HASH
from collections.abc import Buffer, Callable
from typing import Concatenate

def compat[**P](
    hashlib_function: Callable[Concatenate[Buffer, P], HASH],
    *args: P.args,
    **kwargs: P.kwargs,
) -> Callable[[bytes], int]: ...
