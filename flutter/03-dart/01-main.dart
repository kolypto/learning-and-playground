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


