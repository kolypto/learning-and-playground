# Installation

Install: ionic, `native-run` to run native binaries on devices and emulators, and `cordova-res` to generate native app icons:

```console
$ npm install -g @ionic/cli
```

Start with an app: "blank", "tabs", or "sidemenu"

```console
$ ionic start <app-name> --type=angular --capacitor
```

With Angular:

```console
$ npm install @ionic/angular@latest --save
```

To add Ionic to an existing Angular project:

```console
$ ng add @ionic/angular
```

# Install plugins

In a project, here's how you install the necessary Capacitor plugins:

```console
$ npm install @capacitor/camera @capacitor/preferences @capacitor/filesystem
```
