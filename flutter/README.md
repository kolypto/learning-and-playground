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

