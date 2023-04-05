# Dart Syntax

## Variables

`late` variables: a non-null variable that's initialized after its declaration.
Example: top-level variable:

```dart
// A non-null variable initialized after its declaration
late String description;

/** Doc comment: ** or ///
 * Use [name] to refer to methods and fields
 */
void main() {
    // If you fail to initialize a later variable, runtime error occurs
    description = 'Feijoada!';
}
```

`final` variables can be set once. `const` variable is a compile-time constant:

```dart
final name = 'Bob';
const pi = 3.14;
```

`const` can also be used to create constant *values* and declare constructors that create constant values.

```dart
// Const value
var list = const [];

// Const constructor
// A constant object has all fields immutable.
class Point {
    const Point();
}
```

type checks and casts:

```dart
const Object i = 3; // any

// Type cast
const list = [i as int];

// Type check
const map = {if (i is int) i: 'int'};

// Spread
const set = {if (list is List<int>) ...list};
```

## Operators

Usual: `++`, `--`, `cond? value : value`

Cascade: `..`, `?..` (sequence of operations on the same object)

Null-safe, non-null assert: `?`, `?.`, `!`

Assign if null: `a ??= value`

If null then: `value ?? value`

Type test: `is` object has the specified type, `is!` object has not the specified type

Arithmetic: `+-*/`, `%`, `~/` divide, returning an integer (floored)

Shift: `<<`, `>>`, `>>>` (unsigned shift right)

## Metadata

Use `@deprecated()` and `@override()`.

You can retrieve metadata at runtime using reflection.

Custom annotation is just a class:

```dart
class Todo {
  final String who;
  final String what;

  const Todo(this.who, this.what);
}

@Todo('me', 'Implement this function')
void doSomething() {
}
```

## Libraries

Every dart app is a library. Identifiers that start with `_` are not exported.

Imports:

```dart
import 'dart:html';

// The "package:" scheme specifies libraries provided by a package manager, such as the "pub" tool
import 'package:test/test.dart';
```

Library prefix:

```dart
import 'package:lib1/lib1.dart';
import 'package:lib2/lib2.dart' as lib2;

// import here
Element element1 = Element();

// import with prefix
lib2.Element element2 = lib2.Element();
```

Import some names:

```dart
// Import ONLY foo
import 'package:lib1/lib1.dart' show foo;

// Import all EXCEPT foo
import 'package:lib2/lib2.dart' hide foo;
```

Deferred loading (only for JS):

```dart
import 'package:greetings/hello.dart' deferred as hello;

Future<void> greet() async {
    // `await` until the library is loaded
    await hello.loadLibrary(); // can be called multiple times

    // LIMITATION: you can not use types from a deferred library! Move types to a shared file.

    hello.printGreeting();
}
```


# Types

## Built-In Types

Basic types:

* `int` (64bit), `double` (64bit), `String` (UTF16), `bool`
* `List`, `Set`, `Map`
* `Rune` (character)
* `Symbol`: an operator or identifier: `#name` (minification won't change them)
* `null`

Special types:

* `Object`: any
* `Enum`: superclass of all enums
* `Future`, `Stream`: async stuff
* `Iterable`: used in for-loops
* `Never`: indicates that an expression can never successfully finish evaluating
* `dynamic`: disable static checking. Check at runtime. 
* `void`: the value is never used.

Strings:

```dart
var s = 'value';

// String interpolation
assert('${s.toUpperCase()}' ==  'VALUE');

// Multiline strings
var s1 = '''
a
b
''';

// Raw string
var regexp = r'.*\(\)';
```

List:

```dart
var list = [1,2,3];  // List<int> implied
var list2 = [0, ...list]; // spread operator. Also null-aware: `...?list`

assert(list[0] == 1);
assert(list.length == 3);

// Collection if: add an element if condition holds
var nav = ['Home', 'Furniture', 'Plants', if (promoActive) 'Outlet'];

// Collection for: add elements
var listOfStrings = ['#0', for (var i in list) '#$i'];
```

Set:

```dart
var halogens = {'fluorine', 'chlorine', 'bromine', 'iodine', 'astatine'}; // Set<String>
halogens.add('moonshine');
```

Maps:

```dart
var nobleGases = {  // Map<int, String>
  2: 'helium',
  10: 'neon',
  18: 'argon',
};

assert(nobleGases[999] == null);  // missing key: gives `null`
```


# Typedefs

Type alias:


```dart
typedef IntList = List<int>;
IntList il = [1, 2, 3];

// Type alias with type parameters:
typedef ListMapper<X> = Map<X, List<X>>;
```

Function typedef:

```dart
// Function type
typedef Compare<T> = int Function(T a, T b);

// Check it
int sort(int a, int b) => a - b;
assert(sort is Compare<int>);
```





# Generics

Ensure that a type is non-nullable:

```dart
class Foo<T extends Object> {
  // Any type provided to Foo for T must be non-nullable.
}
```

Limit:

```dart
class Foo<T extends SomeBaseClass> {
    //...
}
```

Generic method:

```dart
T first<T>(List<T> ts) {
  // Do some initial work or error checking, then...
  T tmp = ts[0];
  // Do some additional checking or processing...
  return tmp;
}
```


