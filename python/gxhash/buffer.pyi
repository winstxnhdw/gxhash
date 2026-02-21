from typing import Protocol

class Buffer(Protocol):
    def __buffer__(self, flags: int, /) -> memoryview: ...
