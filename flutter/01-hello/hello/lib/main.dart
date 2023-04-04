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
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepOrange),
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
}

// A Widget
class MyHomePage extends StatelessWidget {
  // build() is called every time the widget's circumstances change.
  // Must return a widget.
  @override
  Widget build(BuildContext context) {
    // Tracks changes to the app's state using `watch()`
    var appState = context.watch<MyAppState>();

    return Scaffold(
      // Column: most basic layout widget.
      // Puts children into a column, from top to bottom.
      body: Column(
        children: [
          // Two text elements
          Text('A random idea:'),
          Text(appState.current.asLowerCase), // takes an app state

          // Add a button
          ElevatedButton(
              onPressed: () {
                print('button pressed!');
              },
              child: Text('Next'))
        ],
      ),
    );
  }
}
