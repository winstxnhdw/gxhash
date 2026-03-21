# cython: language_level=3, auto_pickle=False, binding=False

from collections.abc import Coroutine
from functools import partial


cdef class EagerRoutine:
    cdef readonly object hasher
    cdef object result

    def __cinit__(self, object func, *args, **kwargs):
        self.hasher = partial(func, *args, **kwargs)

    def __call__(self, object data):
        self.result = StopIteration(self.hasher(data))
        return self

    cpdef send(self, object _):
        raise self.result


Coroutine.register(EagerRoutine)
