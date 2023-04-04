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
