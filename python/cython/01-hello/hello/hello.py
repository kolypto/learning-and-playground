import cython

# a cfunc (cdef) won't be visible from Python space
# a ccall (cpdef) will also create a Python wrapper (creates some overhead)

@cython.ccall
# @cython.exceptval(...)
def hello(name):
    return f'Hello {name}'


# Compile:
# $ python setup.py build_ext --inplace
# $ cython -3 -a hello.py  

# Also use this to auto compile all modules (don't in production!)
import pyximport
pyximport.install(pyimport=True)
