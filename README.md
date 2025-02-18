# Hantracker

[![dependency status](https://deps.rs/repo/github/wildwestrom/hantracker/status.svg)](https://deps.rs/repo/github/wildwestrom/hantracker)

  Tracks your progress learning Chinese Characters.

After you launch the program here's how it works:

1. Put in a list of Chinese characters you wish to learn.
2. You'll see each character and have to say if you know the meaning or not, one by one.
3. The program then shows you your progress by how many you know.

Note:
You must set the proper font for the language you wish to study on your OS. (I tried, but doing this for you was too much work).

## Roadmap

Here are some features I'd like to add.

- [x] Preset lists of characters for testing
- [x] A back button to re-answer a previous question on the test
  - [x] A view of the previously answered character to confirm you actually knew the character
- [x] A way to save and re-load your progress
  - [x] ~Start and continue screens~ Buttons to either start a new test or resume a previous one
  - [x] Know where you last stopped and where to pick up
- [x] ~Get working~ Compileable on multiple platforms
  - [x] Linux
  - [x] MacOS
  - [x] Windows
- [ ] Show information about the previous character to confirm you actually knew it
  - [ ] Make the meanings and readings match the target language (Chinese, Japanese, Korean)
- [ ] Build artifacts with CI
  - [ ] Linux
  - [ ] MacOS
  - [ ] Windows

### Optional Features

- [ ] Have multiple profiles for different lists of characters
- [ ] Render a wallpaper of your learning progress (inspired by the [Wanikani Screensaver](https://community.wanikani.com/t/wanikani-progress-screensaver-for-osx/1583/109))
  - [ ] Gnome
  - [ ] KDE
  - [ ] MacOS
  - [ ] Windows

### Optional Code Quality Stuff

- [ ] Proper error handling (it all got messed up since adding sqlx)

## Build instructions

Dependencies are listed in `flake.nix`.

You should be able to run the following to build the project:

```console
sqlx database create
sqlx migrate run
cargo build --release
```

The name of the database is always `data.sqlite`.

## Licenses

Copyright © 2021 Christian Westrom

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

![GPL Version 3.0 Logo](https://www.gnu.org/graphics/gplv3-or-later.png)

### KanjiDic2

This program uses the [KANJIDIC](http://www.edrdg.org/wiki/index.php/KANJIDIC_Project) dictionary files. These files are the property of the [Electronic Dictionary Research and Development Group](http://www.edrdg.org/wiki/index.php/KANJIDIC_Project), and are used in conformance with the Group's [license](http://www.edrdg.org/edrdg/licence.html).
