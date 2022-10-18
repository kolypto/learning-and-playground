from setuptools import setup, Extension
from Cython.Build import cythonize

setup(
    name='Hello world app',
    ext_modules=cythonize(
        # One way:
        # ["hello/hello.py", "hello/primes.py",],
        # Second way:
        Extension(
            'hello',
            ['hello/hello.py', 'hello/primes.py'],
            annotate=True,
        ),
        annotate=True,
    ),
    zip_safe=False,
)
