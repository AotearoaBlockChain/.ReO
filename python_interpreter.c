#include <Python.h>

void initialize_python() {
    Py_Initialize();
}

void finalize_python() {
    Py_Finalize();
}

const char* execute_python_code(const char* code) {
    PyObject *pGlobals, *pLocals, *pValue;
    
    // Initialize Python interpreter
    initialize_python();
    
    // Create dictionaries for global and local namespaces
    pGlobals = PyDict_New();
    pLocals = PyDict_New();
    
    // Execute the Python code
    pValue = PyRun_String(code, Py_file_input, pGlobals, pLocals);
    
    // Handle errors
    if (pValue == NULL) {
        PyErr_Print();
        finalize_python();
        return "Error executing Python code";
    }
    
    // Convert the result to a C string
    PyObject *pResultStr = PyObject_Str(pValue);
    const char *result = PyUnicode_AsUTF8(pResultStr);
    
    // Clean up
    Py_DECREF(pValue);
    Py_DECREF(pGlobals);
    Py_DECREF(pLocals);
    
    // Finalize Python interpreter
    finalize_python();
    
    return result;
}
