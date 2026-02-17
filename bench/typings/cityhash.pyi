from collections.abc import Buffer

class CityHash64WithSeed:
    def __new__(cls, data: str | Buffer, /, seed: int = ...) -> int: ...

class CityHash128WithSeed:
    def __new__(cls, data: str | Buffer, /, seed: int = ...) -> int: ...
