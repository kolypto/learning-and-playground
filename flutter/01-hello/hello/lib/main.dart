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

  // Behavior: get a new word
  void getNext() {
    current = WordPair.random();

    // notify everyone listening to changes
    notifyListeners();
  }
}

// A Widget
class MyHomePage extends StatelessWidget {
  // build() is called every time the widget's circumstances change.
  // Must return a widget.
  @override
  Widget build(BuildContext context) {
    // Tracks changes to the app's state using `watch()`
    var appState = context.watch<MyAppState>();

    var pair = appState.current;

    // Widget
    return Scaffold(
      // Column: most basic layout widget.
      // Puts children into a column, from top to bottom.
      body: Center(
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

            // Add a button
            ElevatedButton(
                onPressed: () {
                  print('button pressed!');
                  // Get a new pair
                  appState.getNext();
                },
                child: Text('Next'))
          ],
        ),
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
