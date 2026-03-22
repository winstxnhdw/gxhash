#define PY_SSIZE_T_CLEAN
#include <Python.h>

static PyObject *digest_method = NULL;

typedef struct CompatObject CompatObject;

struct CompatObject {
    PyObject ob_base;
    PyObject *hash_func;
    PyObject *kwarg_names;
    PyObject **items;
    Py_ssize_t args_count;
    vectorcallfunc self_vector_call;
    vectorcallfunc vector_call;
};

static struct PyModuleDef module_definition = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "compat",
    .m_size = -1,
};

static void Compat_dealloc(PyObject *const self_obj) {
    CompatObject *const self = (CompatObject *)self_obj;

    if (self->items != NULL) {
        const Py_ssize_t kwargs_count = self->kwarg_names ? PyTuple_GET_SIZE(self->kwarg_names) : 0;

        for (Py_ssize_t i = 0; i < self->args_count + kwargs_count; i++) {
            Py_XDECREF(self->items[i]);
        }

        PyMem_Free(self->items);
    }

    Py_XDECREF(self->hash_func);
    Py_XDECREF(self->kwarg_names);
    Py_TYPE(self)->tp_free(self);
}

static PyObject *Compat_vectorcall(
    PyObject *const self_obj,
    PyObject *const *args,
    const size_t args_flag,
    PyObject *const kwargs
) {
    CompatObject *const self = (CompatObject *)self_obj;
    char *buffer = NULL;
    Py_ssize_t len = 0;

    self->items[0] = args[0];
    PyObject *hash = self->vector_call(self->hash_func, self->items, self->args_count, self->kwarg_names);
    self->items[0] = Py_None;

    PyObject *digest = PyObject_CallMethodNoArgs(hash, digest_method);
    PyBytes_AsStringAndSize(digest, &buffer, &len);
    PyObject *result = PyLong_FromUnsignedNativeBytes(buffer, len, 0);

    Py_DECREF(digest);
    Py_DECREF(hash);
    return result;
}

static PyObject *Compat_new(PyTypeObject *type, PyObject *args, PyObject *kwargs) {
    const Py_ssize_t args_count = PyTuple_GET_SIZE(args);
    const Py_ssize_t kwargs_count = kwargs != NULL ? PyDict_GET_SIZE(kwargs) : 0;
    CompatObject *self = NULL;
    PyObject *key = NULL;
    PyObject *value = NULL;
    Py_ssize_t position = 0;

    if ((self = (CompatObject *)type->tp_alloc(type, 0)) == NULL) {
        goto error;
    }

    if (args_count < 1) {
        PyErr_SetString(PyExc_TypeError, "compat requires at least one argument");
        goto error;
    }

    if ((self->items = PyMem_Calloc(args_count + kwargs_count, sizeof(PyObject *))) == NULL) {
        PyErr_NoMemory();
        goto error;
    }

    if (kwargs_count > 0 && (self->kwarg_names = PyTuple_New(kwargs_count)) == NULL) {
        goto error;
    }

    for (Py_ssize_t i = 1; i < args_count; i++) {
        self->items[i] = Py_NewRef(PyTuple_GET_ITEM(args, i));
    }

    for (Py_ssize_t i = 0; i < kwargs_count && PyDict_Next(kwargs, &position, &key, &value); i++) {
        PyTuple_SET_ITEM(self->kwarg_names, i, Py_NewRef(key));
        self->items[args_count + i] = Py_NewRef(value);
    }

    self->hash_func = Py_NewRef(PyTuple_GET_ITEM(args, 0));
    self->vector_call = PyVectorcall_Function(self->hash_func) ?: PyObject_Vectorcall;
    self->self_vector_call = Compat_vectorcall;
    self->args_count = args_count;
    self->items[0] = Py_None;

    return (PyObject *)self;

error:
    Py_XDECREF(self);
    return NULL;
}

static PyTypeObject Compat_Type = {
    .ob_base = {PyObject_HEAD_INIT(NULL) 0},
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_IMMUTABLETYPE | Py_TPFLAGS_HAVE_VECTORCALL,
    .tp_name = "compat.compat",
    .tp_vectorcall_offset = __builtin_offsetof(CompatObject, self_vector_call),
    .tp_basicsize = sizeof(CompatObject),
    .tp_new = Compat_new,
    .tp_dealloc = Compat_dealloc,
    .tp_call = PyVectorcall_Call,
};

PyMODINIT_FUNC PyInit_compat(void) {
    PyObject *module = NULL;

    if ((digest_method = PyUnicode_InternFromString("digest")) == NULL) {
        goto error;
    }

    if (PyType_Ready(&Compat_Type) < 0) {
        goto error;
    }

    if ((module = PyModule_Create(&module_definition)) == NULL) {
        goto error;
    }

    if (PyModule_AddObjectRef(module, "compat", (PyObject *)&Compat_Type) < 0) {
        goto error;
    }

    return module;

error:
    Py_XDECREF(module);
    return NULL;
}
