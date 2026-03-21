#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <cpython/genobject.h>

static struct PyModuleDef module_definition = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "routine",
    .m_size = -1,
};

typedef struct {
  PyObject ob_base;
  PyObject *callback;
  PyObject *stack;
  PyObject *kwarg_names;
  PyObject *result;
} EagerRoutineObject;

static void EagerRoutine_dealloc(PyObject *const self_obj) {
  EagerRoutineObject *const self = (EagerRoutineObject *)self_obj;
  Py_XDECREF(self->callback);
  Py_XDECREF(self->stack);
  Py_XDECREF(self->kwarg_names);
  Py_XDECREF(self->result);
  Py_TYPE(self)->tp_free(self);
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

  if ((self->stack = PyTuple_New(kwargs_count + 1)) == NULL) {
    goto error;
  }

  if ((self->kwarg_names = PyTuple_New(kwargs_count)) == NULL) {
    goto error;
  }

  self->callback = Py_NewRef(PyTuple_GET_ITEM(args, 0));
  self->result = Py_None;

  for (Py_ssize_t i = 0; i < kwargs_count && PyDict_Next(kwargs, &position, &key, &value); i++) {
    PyTuple_SET_ITEM(self->kwarg_names, i, Py_NewRef(key));
    PyTuple_SET_ITEM(self->stack, 1 + i, Py_NewRef(value));
  }

  PyTuple_SET_ITEM(self->stack, 0, Py_NewRef(Py_None));
  return (PyObject *)self;

error:
  Py_XDECREF(self);
  return NULL;
}

static PyObject *EagerRoutine_call(PyObject *const self_obj, PyObject *args, PyObject *kwds) {
  PyObject *data = NULL;

  if (!PyArg_ParseTuple(args, "O:__call__", &data)) {
    return NULL;
  }

  EagerRoutineObject *const self = (EagerRoutineObject *)self_obj;
  PyObject **const items = &PyTuple_GET_ITEM(self->stack, 0);
  items[0] = data;

  if ((self->result = PyObject_Vectorcall(self->callback, items, 1, self->kwarg_names)) == NULL) {
    items[0] = Py_None;
    return NULL;
  }

  items[0] = Py_None;
  return Py_NewRef(self_obj);
}

static PySendResult EagerRoutine_am_send(PyObject *const self_obj, PyObject *Py_UNUSED(arg), PyObject **presult) {
  EagerRoutineObject *const self = (EagerRoutineObject *)self_obj;
  *presult = self->result;
  self->result = Py_None;

  return PYGEN_RETURN;
}

static PyAsyncMethods EagerRoutine_async = {
    .am_send = (sendfunc)EagerRoutine_am_send,
};

static PyTypeObject EagerRoutine_Type = {
    .ob_base = {PyObject_HEAD_INIT(NULL) 0},
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_IMMUTABLETYPE,
    .tp_name = "routine.EagerRoutine",
    .tp_basicsize = sizeof(EagerRoutineObject),
    .tp_new = EagerRoutine_new,
    .tp_dealloc = EagerRoutine_dealloc,
    .tp_call = EagerRoutine_call,
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
