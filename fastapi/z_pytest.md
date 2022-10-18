# pytest primer

## Command-Line

Show all available *fixtures* (injectable dependencies):

    $ pytest --fixtures

Stop on first error:

    $ pytest -x
    
show local variables:

    $ pytest -l
    
drop to pdb for first 3 failures, use `breakpoint()` in the code, or:

    $ pytest --pdb --maxfail=3
    
show 10 slowest tests:

    $ pytest --durations=10
    
do not capture stdout:

    $ pytest -s

Re-run failures:

* `--lf` - to only re-run the failures.
* `--ff` - to run the failures first and then the rest of the tests.






















## Python tests


### Tests

* Any file named `test_*.py`
* Any class named `Test*` (optional; don't have to use classes)
* Any function named `test_*()`









### Assertions

Simply use `assert`, and pytest will provide nice detailed output:

```python
import pytest

def test_function():
    assert f() == 4

    with pytest.raises(ValueError):
        5/0
```












### Fixtures

Fixtures are automatically provided to tests. Recognized by names.

```python
# conftest.py
# Shared fixture functions

# Use for services, sharing data, etc

import pytest

# scope:
# 'function': invoke once per test function (default)
# 'module': invoke once per module
# 'session': invoked once per test run
# callable: function(fixture_name, config) that determines the scope on the fly
#
# autouse=True: enable for every test that sees it
@pytest.fixture(scope="module")
def some_connection():
    with connect() as connection:
        # yield it, clean-up afterwards
        yield connection


# Alternative teardown
@pytest.fixture(scope="module")
def some_connection(request: FixtureRequest):  # can use fixtures themselves: DI
    ...

    def fin():
        ...

    request.addfinalizer(fin)
    return ...

# A fixture can yield a factory function with parameters that tests can use.yiel
@pytest.fixture(scope="module")
def some_connection(request: FixtureRequest):
    def factory(param):
        ...
    
    yield factory

    ... # clean-up

```

A parameterized fixture. 
Will cause every test using it to run twice:

```python
@pytest.fixture(scope="module", 
                # Run every test twice, for every parameter
                params=["smtp.gmail.com", "mail.python.org"],
                # Nicer, human-readable names (in case you used magic numbers for parameters)
                ids=['gmail', 'python']
)
def some_connection(request):
    connection = connect(request.param)  # 1) gmail, 2) python
    yield connection

# Parameters can be marked, e.g. with tags, or for skipping:
@pytest.fixture(..., params=[
    ...,
    pytest.param(2, marks=pytest.mark.skip)
])
def some_connection(request): pass
```

Use fixtures without arguments:

```python
# For one test
@pytest.mark.usefixtures("cleandir", "anotherfixture")
def test(): ...

# For the whole module
pytestmark = pytest.mark.usefixtures("cleandir")
```

Auto-using fixtures for the whole class:

```python
@pytest.fixture(scope="module")
def db():
    return DB()  # DB connection


# Notice how this class is used as a ... scope of sorts
class TestClass:
    # This fixture will begin&rollback a transaction for the whole class
    # autouse: will be invoked for all tests in view
    @pytest.fixture(autouse=True)
    def transact(self, request, db):
        db.begin(request.function.__name__)
        yield
        db.rollback()

    def test_method1(self, db):
        assert db.intransaction == ["test_method1"]

    def test_method2(self, db):
        assert db.intransaction == ["test_method2"]
```

To override a fixture, just use a function with the same name.
The `super()` fixture will be provided as an argument:

```python
@pytest.fixture()
def db(db):
    ...
```

Override a fixture with a fixed value:

```python
# Feed a constant value
@pytest.mark.parametrize('username', ['directly-overridden-username'])
def test_username(username):
    assert username == 'directly-overridden-username'
```











### Attributes

```python
import pytest

pytest.skip # - always skip a test function
pytest.skipif # - skip a test function if a certain condition is met
pytest.xfail # - produce an “expected failure” outcome if a certain condition is met
pytest.parametrize # - to perform multiple calls to the same test function.
```

example:

```python
# Skip a failing test
@pytest.mark.skip(reason="no way of currently testing this")
def failing_test(): 
    pytest.skip("unsupported configuration")  # another way

# Conditional skip
@pytest.mark.skipif(sys.version_info < (3, 6), reason="requires python3.6 or higher")
def test_function():
    ...

# Give up; not too good, but not fatal either
@pytest.mark.xfail(raises=RuntimeError)  # expected error
def test_function2():
    import slow_module
    if slow_module.slow_function():
        pytest.xfail("slow_module taking too long")  # another way
```

expected failures can be used with parameters:

```python
@pytest.mark.parametrize(
    ("n", "expected"),
    [
        (1, 2),
        # known bug
        pytest.param(1, 3, marks=pytest.mark.xfail(reason="some bug")),
        # known not working 
        pytest.param(
            10, 11, marks=pytest.mark.skipif(sys.version_info >= (3, 0), reason="py2k")
        ),
    ],
)
def test_increment(n, expected):
    assert n + 1 == expected
```









### Monkeypatching other objects

Use the `monkeypatch` fixture.
All modifications will be undone after the requesting test function or fixture has finished. 

```python
monkeypatch: MonkeyPatch
monkeypatch.setattr(obj, name, value, raising=True)
monkeypatch.delattr(obj, name, raising=True)
monkeypatch.setitem(mapping, name, value)
monkeypatch.delitem(obj, name, raising=True)
monkeypatch.setenv(name, value, prepend=False)
monkeypatch.delenv(name, raising=True)
monkeypatch.syspath_prepend(path)
monkeypatch.chdir(path)
```

Can it as a context manager:

```python
with monkeypatch.context() as m:
    m.setattr(...)
m.undo()  # undo all changes. ALL changes.
```














### Parameterizing

Run the function multiple times, with different inputs:

```python
@pytest.mark.parametrize(
    ('input', 'expeced'), 
    [("3+5", 8), ("2+4", 6), ("6*9", 42)])
def test_eval(input, expected):
    assert eval(input) == expected
```
