# cedr

![Alt text](docs/media/image.png)

Chinese English dictionary in Rust. Uses CC-CEDICT. [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/)

## Features and roadmap

This is mostly a project for my own benefit at the moment, so the roadmap is based on my personal needs. I welcome that to change - if anyone else finds this project useful, please feel free to open an issue with a suggestion or file a PR.

- [x] Display
    - [x] Simplified characters
    - [x] Traditional characters
    - [x] Pinyin
- [x] Dictionary downloads and updates
    - [ ] Multiple dictionary sources
- [x] Search
    - [x] Automatic detection
    - [x] Chinese characters (traditional and simplified)
    - [x] Pinyin
    - [x] English meaning
    - [x] Search history
    - [x] Link to individual character definitions in multiple character words
- [x] Collections (Card decks)
    - [x] Create and add
    - [ ] Manage decks
    - [x] Import/Export
- [ ] Learning
    - [x] [Design thoughts](/docs/memorization-system.md)
    - [ ] SRS-like reviews
    - [ ] Default HSK decks
    - [ ] Learn by character dependency (Automatically learn all individual characters of a compound word first)
    - [ ] Import/Export progress
- [ ] Theming
    - [ ] Dark theme
- [ ] Multiple device data sync
- [ ] CI task to generate installer
    - [ ] Windows
    - [ ] Mac
    - [ ] Linux
- [ ] Mobile app (iOS)

## Non-goals

This project aims to be a useful way to study chinese vocab, but will never come close to something like Pleco. It instead focuses more on vocab learning and usage, as opposed to an all-in-one application for everything-Chinese-language.

## Installation

### Release

Go to the releases page to find installers

### Manual

1. Install Tauri prereqs ([See guide](https://beta.tauri.app/guides/prerequisites/))
2. Install Tauri CLI (`cargo install tauri-cli --version "^2.0.0-alpha"`)
3. Build application (`cargo tauri build`)
4. Install application from `target` folder

## Folders

Uses [`etcetera`](https://docs.rs/etcetera/latest/etcetera/) for storing data. See crate for details on expected directories for your platform.

## Usage

On first start (or whenever you wish to update the dictionary), click `Settings > Update dictionary`. This will take several seconds.

---

### Generating an ICO

```
cargo install icopng
icopng src-tauri/icons/128x128.png src-tauri/icons/icon.ico
```

### License

GPL-v3 probably for the application. I don't like it but whatever, it's not a bad choice for programs. Just kind of annoying for libraries. idk you can convince me otherwise on this probably