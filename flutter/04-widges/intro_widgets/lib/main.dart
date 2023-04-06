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
    return Material(
      // Column: top bar + screen space
      child: Column(
        children: [
          // Top bar
          const TopBar(
            appTitle: "Example App",
            actions: [
              // Passing widgets as arguments is a powerful technique
              IconButton(
                icon: Icon(Icons.search),
                tooltip: "Search",
                onPressed: null, // TODO: handle
              ),
            ],
          ),
          // Take all available space
          // NOTE: use "flex: 2" to determine the ratio for multiple Expanded() widgets
          Expanded(
            // Default home screen
            child: HomeScreen(),
          ),
        ],
      ),
    );
  }
}

// Top bar
class TopBar extends StatelessWidget {
  // Fields in a Widget subclass are always marked as "final"
  final String appTitle;

  // List of actions
  final List<Widget> actions;

  const TopBar({required this.appTitle, required this.actions});

  @override
  Widget build(BuildContext context) {
    // Container: the box, sized
    return Container(
      // Height, in logical pixels
      height: 56,
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
              // Large text
              appTitle,
              style: Theme.of(context).primaryTextTheme.titleLarge,
            ),
          ),

          // Additional actions
          ...actions
        ],
      ),
    );
  }
}

// Home page
class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Placeholder(
      child: Center(child: Text("🍅", style: TextStyle(fontSize: 250))),
    );
  }
}
