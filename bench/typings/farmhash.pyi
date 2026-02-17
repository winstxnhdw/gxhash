from collections.abc import Buffer

class FarmHash32WithSeed:
    def __new__(cls, data: str | Buffer, /, seed: int = ...) -> int: ...

class FarmHash64WithSeed:
    def __new__(cls, data: str | Buffer, /, seed: int = ...) -> int: ...

class FarmHash128WithSeed:
    def __new__(cls, data: str | Buffer, /, seed: int = ...) -> int: ...
