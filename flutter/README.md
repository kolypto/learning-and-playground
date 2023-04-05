# Flutter





# topics
# Install Flutter on Linux

```console
$ sudo snap install flutter --classic
$ flutter sdk-path
$ flutter doctor
```

<!--
Now install Android Studio: <https://developer.android.com/studio>
1. install command-line tools only (without the studio)

```console
$ ./bin/sdkmanager --sdk_root=. --licenses
$ ./bin/sdkmanager --sdk_root=. --install "cmdline-tools;latest"
```-->


Here's how to install Flutter and Android SDK without Android Studio on Ubuntu:

```console
$ sudo apt install openjdk-8-jre
$ sudo apt install git
$ sudo apt install sdkmanager
$ sudo apt install clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev

$ sdkmanager --list
$ sudo sdkmanager --install "cmdline-tools;latest"
$ sudo sdkmanager --install "build-tools;33.0.2"
$ sudo sdkmanager --install "platform-tools;33.0.32
$ sudo sdkmanager --install "platforms;android-33"

$ sudo /opt/android-sdk/cmdline-tools/latest/bin/sdkmanager --list
$ sudo /opt/android-sdk/cmdline-tools/latest/bin/sdkmanager --install "system-images;android-33;google_apis;x86_64"

$ flutter config --android-sdk /opt/android-sdk/
$ flutter doctor --android-licenses
$ flutter doctor
```

Create an emulator:

```console
$ /opt/android-sdk/cmdline-tools/latest/bin/avdmanager create avd -n PixelXL -d "pixel_xl" --abi google_apis/x86_64 -k "system-images;android-33;google_apis;x86_64"
```






# 01-hello/hello/lib


# 01-hello/hello/lib/main.dart

```dart
import 'package:english_words/english_words.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

void main() {
  // Tell Flutter to run the app
  runApp(MyApp());
}

// App: extends StatelessWidget. The app itself is a widget.
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    // The state is provided to the app using `ChangeNotifierProvider`: allows any widget to get hold of the state
    return ChangeNotifierProvider(
      // Create the app state: the data app needs to function
      create: (context) => MyAppState(),
      // The app uses Material
      child: MaterialApp(
        // It has a name
        title: 'Namer App',
        // Theme settings
        theme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        ),
        // set the "home" widget
        home: MyHomePage(),
      ),
    );
  }
}

// App state: the data app needs to function
// The class extends `ChangeNotifier`: it can notify others about its own changes.
class MyAppState extends ChangeNotifier {
  // Random word pair
  var current = WordPair.random();

  // Behavior: get a new word
  void getNext() {
    current = WordPair.random();

    // notify everyone listening to changes
    notifyListeners();
  }

  // Behavior: remember a word pair
  var favorites = <WordPair>[];

  void toggleFavorite() {
    if (favorites.contains(current)) {
      favorites.remove(current);
    } else {
      favorites.add(current);
    }
    notifyListeners();
  }
}

// A Widget
class GeneratorPage extends StatelessWidget {
  // build() is called every time the widget's circumstances change.
  // Must return a widget.
  @override
  Widget build(BuildContext context) {
    // Tracks changes to the app's state using `watch()`
    var appState = context.watch<MyAppState>();

    // The word pair
    var pair = appState.current;

    // Prepare the icon
    IconData icon;
    if (appState.favorites.contains(pair)) {
      icon = Icons.favorite;
    } else {
      icon = Icons.favorite_border;
    }

    // Widget
    // Column: most basic layout widget.
    // Puts children into a column, from top to bottom.
    return Center(
      child: Column(
        // Centered
        mainAxisAlignment: MainAxisAlignment.center,
        // Widgets
        children: [
          // Two text elements
          Text('A random idea:'),
          BigCard(pair: pair), // takes an app state

          // Space between
          SizedBox(height: 10),

          // Buttons row
          Row(
            mainAxisSize: MainAxisSize.min, // center
            children: [
              ElevatedButton.icon(
                  icon: Icon(icon),
                  onPressed: () {
                    appState.toggleFavorite();
                  },
                  label: Text('Like')),
              ElevatedButton(
                onPressed: () {
                  print('button pressed!');
                  // Get a new pair
                  appState.getNext();
                },
                child: Text('Next'),
              ),
            ],
          )
        ],
      ),
    );
  }
}

// Widget: piece of text
class BigCard extends StatelessWidget {
  const BigCard({
    super.key,
    required this.pair,
  });

  final WordPair pair;

  @override
  Widget build(BuildContext context) {
    // Get theme
    final theme = Theme.of(context);

    // Improve
    // `displayMedium` is a large style for display text. For short, important text.
    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
    );

    // Added with refactoring: wrap with widget, wrap with padding
    return Card(
      // Colorize
      color: theme.colorScheme.primary,
      // color: Colors.blue,

      // Children
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Text(
          pair.asLowerCase,
          style: style,
          // For screen readers: make sure that "madcat" is read as "mad cat"
          semanticsLabel: "${pair.first} ${pair.second}",
        ),
      ),
    );
  }
}

// Home screen:
// LEFT: menu
// RIGHT: current page
// Stateful widget: a widget that has State.
class _MyHomePageState extends State<MyHomePage> {
  // Current page
  var selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    // Choose a widget
    Widget page;
    switch (selectedIndex) {
      case 0:
        page = GeneratorPage();
        break;
      case 1:
        // Placeholder() displays a mock box
        // page = Placeholder();
        page = FavoritesPage();
        break;
      // Fail-fast
      default:
        throw UnimplementedError('not widget for $selectedIndex');
    }

    // Builder
    return LayoutBuilder(
        // builder() is called every time the constraints change: window resized, phone rotated, a widget grows in size, etc
        builder: (context, constraints) {
      // Scaffold
      return Scaffold(
        // Row() with two children: SafeArea() and Expanded()
        body: Row(
          children: [
            // SafeArea() ensures that its child is not obscured by a hardware notch or a status bar
            SafeArea(
              // NavigationRail()
              child: NavigationRail(
                // extended: `true` to show labels next to icons
                // Depends on "virtual pixels" size
                extended: constraints.maxWidth >= 600,

                // Destinations
                destinations: [
                  NavigationRailDestination(
                    icon: Icon(Icons.home),
                    label: Text('Home'),
                  ),
                  NavigationRailDestination(
                    icon: Icon(Icons.favorite),
                    label: Text('Favorites'),
                  ),
                ],

                // Current selected destination
                selectedIndex: selectedIndex, // depends
                // When selected: do this
                onDestinationSelected: (value) {
                  // setState() makes sure that the change is actually recorded
                  setState(() {
                    selectedIndex = value;
                    print('selected: $value');
                  });
                },
              ),
            ),
            // Expanded() expresses a layout where a child takes as much of the remaining room as possible
            Expanded(
              // Container() is colored
              child: Container(
                color: Theme.of(context).colorScheme.primaryContainer,
                // Child: the current page
                child: page,
              ),
            ),
          ],
        ),
      );
    });
  }
}

// MyHomePage widget
class MyHomePage extends StatefulWidget {
  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class FavoritesPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    // Watch app state
    var appState = context.watch<MyAppState>();

    // Empty
    if (appState.favorites.isEmpty) {
      return Center(child: Text("No favorites yet"));
    }

    // ListView(): a column that scrolls
    return ListView(children: [
      Padding(
        padding: const EdgeInsets.all(20),
        child: Text('You have ${appState.favorites.length} favorites:'),
      ),
      for (var fav in appState.favorites)
        ListTile(leading: Icon(Icons.favorite), title: Text(fav.asString))
    ]);
  }
}

```





# 03-dart


# 03-dart/01-main.dart

```dart
// Importing core libraries
import 'dart:io';
import 'dart:math';

/** Entry point
 */
void main(){
    // Variables
    var name = 'Voyager' + " " + "I";
    var year = 1977;
    var flybyObjects = ['Jupiter', 'Saturn', 'Uranus', 'Neptune'];

    // Condition
    if (year > 1900) {
        print("Name: $name $year $flybyObjects");
    } else if (year > 1800) {
        print("Quite old");
    } else {
        // Exception
        throw Exception("Too old");
    }

    // Use object
    print(Spacecraft(name, DateTime(1977, 9, 5)).describe());
}

// Function
int getValue(int value) {
    return (
        // arrow functions (lambda)
        (v) => v
    )(value);
}


// Classes
class Spacecraft {
    String name;
    DateTime? launch; // nullable

    // Read-only non-final property
    int? get year => launch?.year;  // null-safe access

    // Constructor
    Spacecraft(this.name, this.launch)
        : assert(name.length > 0) // initializer. Useful for assertions and final params.
        {
            // ... constructor code
        }

    // Named constructor
    Spacecraft.unlaunched(String name) 
        : this(name, null);  // forwards to the default one

    // Method
    String describe() {
        if (this.launch == null){
            return '$name; Unlaunched';
        } else {
            // property access: by name: `launch`
            // "!" indicates it's not nullable here
            int years = DateTime.now().difference(launch!).inDays ~/ 365;
            return "$name; launched: $launch ($years ago)";
        }
    }
}


// Enums
enum PlanetType { terrestial, gas, ice }

// Enum with fields and methods
enum Planet {
    // values
    mercury(PlanetType.terrestial, moons: 0, hasRings: false),
    venus(PlanetType.terrestial, moons: 0, hasRings: false);
    //...

    // A const constructor
    // positional arg, required named args
    const Planet(this.type, {required this.moons, required this.hasRings});

    // Fields are final
    final PlanetType type;
    final int moons;
    final bool hasRings;

    // getter method
    bool get isGiant => type == PlanetType.gas || type == PlanetType.ice;
}


// Dart has single inheritance
class Orbiter extends Spacecraft {
    double altitude;

    // Constructor: uses `super` to refer to parent
    Orbiter(super.name, DateTime super.launch, this.altitude);
}

// Mixin: reuse code in multiple class hierarchies
mixin Piloted {
    // Mixins are for methods, not variables
    int get astronauts {
        return 1;
    }
}
class PilotedCraft extends Spacecraft with Piloted {
    PilotedCraft(super.name, super.launch);
}


// Dart has no interfaces, but every class is an interface:
class MockSpaceship implements Spacecraft {
    var name = "Test";
    var launch = DateTime(2000, 1, 1);
    final year = 2000;
    String describe(){ return "test"; }
}

// Abstract class
abstract class Describable {
    void describe();
}



// Async function.
// Also see: `async*` stream
Future<void> createFile(Iterable<String> objects) async {
    for (final object in objects){
        try{
            // Open file
            var file = File('$object.txt');
            await file.create();
            await file.writeAsString("Description: $object");
        // Catching exceptions
        } on IOException catch(e) { 
            print("Cannot create description for $object: $e");
            rethrow;
        } finally {
            //...
        }
    }
}



```

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



