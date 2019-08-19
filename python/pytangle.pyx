# cdef extern from "tangle.h":
#     cdef void count_to(int i)

cdef extern from "tangle.h":
    void count_to(int i)

def say_hi():
    return count_to(5)

def count(i):
    return count_to(i)
