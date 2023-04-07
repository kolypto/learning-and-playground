import 'package:flutter/material.dart';

void main() {
  // runApp() takes a Widget and makes it the root of a widget tree.
  // Flutter forces the root widget to cover the screen
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  // A widget’s main job is to implement a build() function
  @override
  Widget build(BuildContext context) {
    // Many Material Design widgets need to be inside of a MaterialApp() to display properly:
    // e.g. inherit theme data
    //
    // It also includes `Navigator`: manages stack of widgets identified by "routes".
    // Navigator lets you transition smoothly between screens of your app.
    return MaterialApp(
      // App title: used in the OS task switcher
      title: "App Title",
      // Color scheme for the whole app. We'll refer to these colors for text and fill
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
      ),
      // SafeArea(): make sure it's not covered with a notch or something
      home: SafeArea(
        child: MyScaffold(),
      ),
    );
  }
}

// Scaffold: the "whole" of the application
class MyScaffold extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    // Material: a canvas on which the UI appears
    // We will not use this approach.
    var oldWidget = Material(
      // Column: top bar + screen space
      child: Column(
        children: [
          // Top bar
          const TopBar(title: '...', actions: []),
          // Take all available space
          // NOTE: use "flex: 2" to determine the ratio for multiple Expanded() widgets
          Expanded(
            // Default home screen
            child: HomeScreen(),
          ),
        ],
      ),
    );

    // Instead of Material(child: Column(...)) we'll use Scaffold:
    // it's an application template, with: app bar, body, FAB buttons, etc
    return Scaffold(
      appBar: AppBar(
        title: Text("Example App"),
        leading: const IconButton(
          icon: Icon(Icons.menu),
          tooltip: "Menu",
          onPressed: null, // TODO: handle
        ),
        actions: const [
          // Passing widgets as arguments is a powerful technique
          IconButton(
            icon: Icon(Icons.search),
            tooltip: "Search",
            onPressed: null, // TODO: handle
          ),
        ],
      ),
      body: HomeScreen(),
      // FAB button (Floating Action Button)
      floatingActionButton: FloatingActionButton(
        tooltip: "Add", // used by screen readers
        onPressed: null,
        child: Icon(Icons.add),
      ),
    );
  }
}

// My Top Bar. Unused, because we have `AppBar` :)
class TopBar extends StatelessWidget implements PreferredSizeWidget {
  // Fields in tdget subclass are always marked as "final"
  final String title;

  // List of actions
  final List<Widget> actions;

  const TopBar({required this.title, required this.actions});

  @override
  Widget build(BuildContext context) {
    // Container: the box, sized
    return Container(
      // Height, in logical pixels
      height: 560,
      // Horizontal padding
      padding: const EdgeInsets.symmetric(horizontal: 8),
      // Color: refer to the current theme
      decoration: BoxDecoration(color: Theme.of(context).primaryColorDark),
      // Row: horizontal layout
      child: Row(
        children: [
          // Icon
          // Be sure to add this to pubspec: >> uses-material-design: true
          const IconButton(
            icon: Icon(Icons.menu),
            tooltip: "Menu",
            onPressed: null, // TODO: handle
          ),

          // Spacer, title
          Expanded(
            child: Text(
              title,
              // style: Theme.of(context).primaryTextTheme.titleLarge, // inherited from Scaffold
            ),
          ),

          // Additional actions
          ...actions
        ],
      ),
    );
  }

  @override
  Size get preferredSize => Size(-1, 56);
}

// Home page
class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Center(
      child: Tomato(),
    );
  }
}

// Stateful widgets know how to generate `State` objects: they are used to hold state.
// The Widget class is the configuration for a state. Holds values provided by a parent.
// All Widget fields are final.
class Tomato extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _TomatoState();
}

// State objects are perstistent between calls to build(), allowing them to remember information.
// Widget objects, on the other hand, are temporary: used to conduct a presentation of the application in its current state.
// So a Widget is temporary, a State is persistent.
class _TomatoState extends State<Tomato> {
  double size = 50;

  void increaseSize() {
    setState(() {
      size *= 1.2;
    });
  }

  void resetSize() {
    setState(() {
      size = 50;
    });
  }

  @override
  Widget build(BuildContext context) {
    // GestureDetector() makes things sensitive to input gestures
    return GestureDetector(
      child: Text("🍅", style: TextStyle(fontSize: size)),
      onTap: () {
        increaseSize();
      },
      onLongPress: () {
        resetSize();
      },
    );
  }
}
