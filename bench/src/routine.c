#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <cpython/genobject.h>

typedef struct EagerRoutineObject EagerRoutineObject;

struct EagerRoutineObject {
  PyObject ob_base;
  PyObject *result;
  PyObject *callback;
  PyObject *kwarg_names;
  PyObject **items;
  vectorcallfunc self_vector_call;
  vectorcallfunc vector_call;
};

static struct PyModuleDef module_definition = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "routine",
    .m_size = -1,
};

static void EagerRoutine_dealloc(PyObject *const self_obj) {
  EagerRoutineObject *const self = (EagerRoutineObject *)self_obj;

  if (self->items != NULL) {
    for (Py_ssize_t i = 0; i < 1 + (self->kwarg_names ? PyTuple_GET_SIZE(self->kwarg_names) : 0); i++) {
      Py_XDECREF(self->items[i]);
    }

    PyMem_Free(self->items);
  }

  Py_XDECREF(self->callback);
  Py_XDECREF(self->kwarg_names);
  Py_XDECREF(self->result);
  Py_TYPE(self)->tp_free(self);
}

static PyObject *EagerRoutine_vectorcall(
    PyObject *const self_obj,
    PyObject *const *args,
    const size_t args_flag,
    PyObject *const kwargs
) {
  EagerRoutineObject *const self = (EagerRoutineObject *)self_obj;
  self->items[0] = args[0];
  self->result = self->vector_call(self->callback, self->items, 1, self->kwarg_names);
  self->items[0] = Py_None;

  return Py_NewRef(self_obj);
}

static PySendResult EagerRoutine_am_send(PyObject *const self_obj, PyObject *const arg, PyObject **presult) {
  EagerRoutineObject *const self = (EagerRoutineObject *)self_obj;
  *presult = self->result;
  self->result = Py_None;

  return PYGEN_RETURN;
}

static PyObject *EagerRoutine_new(PyTypeObject *type, PyObject *args, PyObject *kwargs) {
  const Py_ssize_t kwargs_count = kwargs ? PyDict_GET_SIZE(kwargs) : 0;
  EagerRoutineObject *self = NULL;
  PyObject *key = NULL;
  PyObject *value = NULL;
  Py_ssize_t position = 0;

  if ((self = (EagerRoutineObject *)type->tp_alloc(type, 0)) == NULL) {
    goto error;
  }

  if (PyTuple_GET_SIZE(args) < 1) {
    PyErr_SetString(PyExc_TypeError, "EagerRoutine requires at least one argument");
    goto error;
  }

  if ((self->items = PyMem_Calloc(kwargs_count + 1, sizeof(PyObject *))) == NULL) {
    PyErr_NoMemory();
    goto error;
  }

  if (kwargs_count > 0 && (self->kwarg_names = PyTuple_New(kwargs_count)) == NULL) {
    goto error;
  }

  for (Py_ssize_t i = 0; i < kwargs_count && PyDict_Next(kwargs, &position, &key, &value); i++) {
    PyTuple_SET_ITEM(self->kwarg_names, i, Py_NewRef(key));
    self->items[1 + i] = Py_NewRef(value);
  }

  self->callback = Py_NewRef(PyTuple_GET_ITEM(args, 0));
  self->vector_call = PyVectorcall_Function(self->callback) ?: PyObject_Vectorcall;
  self->self_vector_call = EagerRoutine_vectorcall;
  self->result = Py_None;
  self->items[0] = Py_None;

  return (PyObject *)self;

error:
  Py_XDECREF(self);
  return NULL;
}

static PyAsyncMethods EagerRoutine_async = {
    .am_send = EagerRoutine_am_send,
};

static PyTypeObject EagerRoutine_Type = {
    .ob_base = {PyObject_HEAD_INIT(NULL) 0},
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_IMMUTABLETYPE | Py_TPFLAGS_HAVE_VECTORCALL,
    .tp_name = "routine.EagerRoutine",
    .tp_vectorcall_offset = __builtin_offsetof(EagerRoutineObject, self_vector_call),
    .tp_basicsize = sizeof(EagerRoutineObject),
    .tp_new = EagerRoutine_new,
    .tp_dealloc = EagerRoutine_dealloc,
    .tp_call = PyVectorcall_Call,
    .tp_as_async = &EagerRoutine_async,
};

PyMODINIT_FUNC PyInit_routine(void) {
  PyObject *coroutine_abc = NULL;
  PyObject *register_abc = NULL;
  PyObject *module = NULL;

  if (PyType_Ready(&EagerRoutine_Type) < 0) {
    goto error;
  }

  if ((coroutine_abc = PyImport_ImportModuleAttrString("collections.abc", "Coroutine")) == NULL) {
    goto error;
  }

  if ((register_abc = PyObject_CallMethod(coroutine_abc, "register", "O", (PyObject *)&EagerRoutine_Type)) == NULL) {
    goto error;
  }

  if ((module = PyModule_Create(&module_definition)) == NULL) {
    goto error;
  }

  if (PyModule_AddObjectRef(module, "EagerRoutine", (PyObject *)&EagerRoutine_Type) < 0) {
    goto error;
  }

  Py_DECREF(register_abc);
  Py_DECREF(coroutine_abc);
  return module;

error:
  Py_XDECREF(module);
  Py_XDECREF(register_abc);
  Py_XDECREF(coroutine_abc);
  return NULL;
}
