# 2022 Nutriblocks Kids Recipe App - App

The app code repository for my 2022 NCEA assessment (& scholarship?) project.
Built for [Nutriblocks](https://nutriblocks.co.nz/).

## Layout

-   `.dart_tool/` - Ignorable, generated by Dart/Flutter init.
-   `android/` - Android config files.
-   `build/` - Generated by Flutter for the builds of the app.
-   `ios/` - iOS config files.
-   `lib/` - Source code of the app. Get familiar with it.
-   `test/` - Automatic tests of the app.
-   `web/` - Web config files.
-   `windows/` - Windows config files.
-   `analysis_options.yaml` - Dart/Flutter linting config.
-   `pubspec.lock` - Dependencies of the project. Do not untrack! This is important so that everyone has the same dependencies instead of different versions of them!
-   `pubspec.yaml` - Flutter config file. Dependencies and other configuration options go here.
-   `recipeapp.iml` - Further Flutter config stuff.

&nbsp;

# How to Run the Code Locally

Install Dart and Flutter. To check if it is installed, run the following

```
$ flutter --version
Flutter 2.10.5 • channel stable • https://github.com/flutter/flutter.git
Framework • revision 5464c5bac7 (3 weeks ago) • 2022-04-18 09:55:37 -0700
Engine • revision 57d3bac3dd
Tools • Dart 2.16.2 • DevTools 2.9.2
```

If you do not have them installed, download them from [here](https://docs.flutter.dev/get-started/install). Make sure you install everything that is required, checking with `$ flutter doctor`. Flutter, the Android toolchain and Android Studio, and HTTP Host Availability should be successful. VS Code is also recommended. Chrome is not required.

## Development

Firstly, I would recommend installing VS Code and the verified Flutter extension. This makes running and debugging a bit easier, and also has support for automatic hot reloading.

If using the extension, open the `/app/lib/main.dart` file and press `Shift+F5`. It should open the Android emulator (or may prompt to select a device and launch after selecting). Wait a bit for the code to compile and it should load. Making changes to the files and saving should cause the changes to show.

## Production

> Todo
