cdef extern from "library.cpp" namespace "Tangle":
    cdef void say_hello()

def say_hi():
    return say_hello()