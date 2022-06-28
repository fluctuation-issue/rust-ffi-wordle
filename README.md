# rust-ffi-wordle

This is a basic implementation of the game [Wordle](https://en.wikipedia.org/wiki/Wordle), using rust Foreign Function Interface.

The core library contains game mechanics. It can be built as a standard rust library (crate-type: `lib`), or as a static library interfaceable in C (crate-type: `staticlib`).

Four interfaces have been developped:
- one GUI with GTK4
- one terminal-oriented interface (ansi)
- one for apple iOS
- one for apple macOS

## Screenshots: interfaces comparisons

|             | Start                                                                                                        | Lost                                                                                                       | Won                                                                                                      |
| ----------- | ------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Ansi        | ![Ansi Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ansi-start.png)               | ![Ansi Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ansi-lost.png)               | ![Ansi Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ansi-won.png)               |
| Gtk4 Dark   | ![Gtk4 Dark Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/gtk4-dark-start.png)     | ![Gtk4 Dark Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/gtk4-dark-lost.png)     | ![Gtk4 Dark Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/gtk4-dark-won.png)     |
| Gtk4 Light  | ![Gtk4 Light Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/gtk4-light-start.png)   | ![Gtk4 Light Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/gtk4-light-lost.png)   | ![Gtk4 Light Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/gtk4-light-won.png)   |
| macOS Light | ![macOS Light Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/macos-light-start.png) | ![macOS Light Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/macos-light-lost.png) | ![macOS Light Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/macos-light-won.png) |
| macOS Dark  | ![macOS Dark Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/macos-dark-start.png)   | ![macOS Dark Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/macos-dark-lost.png)   | ![macOS Dark Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/macos-dark-won.png)   |
| iOS Dark    | ![iOS Dark Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ios-dark-start.png)       | ![iOS Dark Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ios-dark-lost.png)       | ![iOS Dark Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ios-dark-won.png)       |
| iOS Light   | ![iOS Light Start](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ios-light-start.png)     | ![iOS Light Lost](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ios-light-lost.png)     | ![iOS Light Won](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ios-light-won.png)     |

## GUI with GTK4

A very simple front with GTK.

Locales were provided for french and german.

It accepts a single command line argument:

```sh
wordle-gtk4 [file path]
```

If a file path is specified, then words will be loaded from this file (it is assumed a single word lies per line).
A random word from this list is chosen at each game.

In case no files are provided, the list of words defaults to `["wordle", "wordlerust"]`.

## GUI for apple plateforms

GUI for macOS and iOS was developped using [SwiftUI](https://developer.apple.com/xcode/swiftui).

As the Gtk4 interface, the macOS application accepts an optional single parameter: a path to a file containing a list of word (one per line).

The iOS application has a tab view as its entrypoint.
The user can edit a list of words, saved under the sandboxed `Documents` folder.

## Ansi front

Synopsis for the ansi front:

```sh
wordle-ansi help
wordle-ansi --help
wordle-ansi -h
wordle-ansi version
wordle-ansi --version
wordle-ansi -v
wordle-ansi [file path]
```

The ansi front, as the GTK one, takes an optional file path as its unique argument.
The word is picked from the file. If no files are provided, then the words list is loaded from `STDIN`.

However words are loaded, user input is read from `/dev/tty`.

The output is written in the alternate screen buffer, using the CSI escape sequences described [on the ANSI escape code Wikipedia page](https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences).

| Launch from `STDIN`                                                                                               | Launch from file                                                                                                |
| ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| ![Ansi Launch from STDIN](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ansi-launch-stdin.png) | ![Ansi Launch from File](https://github.com/fluctuation-issue/rust-ffi-wordle/blob/assets/ansi-launch-file.png) |
