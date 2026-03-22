from collections.abc import Buffer

from cityhash import Uint64

def hash(data: str | Buffer, /, seed: int = ...) -> Uint64: ...  # noqa: A001
