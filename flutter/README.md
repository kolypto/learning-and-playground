# Flutter





# readme-files
Flutter
=======

# Installation

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

class MyHomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    return Scaffold(
      body: Column(
        children: [
          Text('A random idea:'),
          Text(appState.current.asLowerCase),

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

```

