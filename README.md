# Rounal

**Rust TUI application that lets you explore and view `journarlctl` logs in a clean and interactive terminal interface.**

https://github.com/user-attachments/assets/86488fcc-a152-42e6-9051-cec17c9bce0e



## Table of contents
- [Features](#features)
- [Motivation](#motivation)
- [Installation](#installation)
- [Usage](#usage)
- [Customization](#customization)
- [Inspiration](#inspiration)
- [Future work](#future-work)


### Features

- Filter logs by priority (emerg, alert, err, etc.)
- Navigate easily with Vim-style or arrow keybindings
- Copy (yank) logs to clipboard
- Customizable colors and behavior via config
- Async fetching of journal entries for better performance

### Motivation

I found that finding and reading logs generated by `systemd` can sometimes be a bit overwhelming. This project tries to solve this with a user interface, that supports some `Vim`-like movements (future work).

This project serve as my submission to my university course, `Modern user interfaces, 2025`. Also, I have recently found Rust to be a interesting programming language and wanted to learn it by creating a project. 

### Installation

##### Prerequisites

- Linux system with `systemd` and `journalctl` available.
- Rust and Cargo (`rustup` recommended): https://rustup.rs
- A terminal that supports UTF-8 characters.
- Commands that the program will run are listed below:

```sh
systemctl list-unit-files --type=service --all
###
systemctl list-units --type=service --all
###
sudo journalctl -u <selected-service> -r -p <1-7>
```

---

#### Via .deb package
Visit the [Releases](https://github.com/williamlempinen/rounal/releases) page and download the latest `.deb` file:

```sh
# Install
sudo dpkg -i rounal_0.1.0_amd64.deb
# Running after install
rounal
```

#### Building from source
```sh
git clone git@github.com:williamlempinen/rounal.git
###
cd rounal
###
cargo build --release
###
target/release/rounal
###
```

### Usage
**j** / **k** or **arrow keys** to move cursor

**Enter** to select a service and view its logs

**/** to search by service name or timestamp

**K** to open current line in modal, created for long log messages

**c** to go back from selected service logs

**y** to yank the log to your clipboard

**?** for help

**E** to read short docs

**q** or **Esc** to quit


### Customization

Some configurations are loaded from `app_config.toml`, for example color configurations. These can be modified to create customized UI. Current color settings are selected based on the `GitHub Dark Default` -theme.


### Inspiration

- **gitui**: https://github.com/gitui-org/gitui
- **atuin**: https://github.com/atuinsh/atuin

### Bugs

There are might be problems regarding copying messages to clipboard. There is ongoing discussion about fixes, [Issues](https://github.com/1Password/arboard/issues)

Terminal's `sudo password` prompt is not `catched` in any way, which can lead to not desired behavior of the ui.

### Future work
- Full rewrite
- Ability to filter services based on states (sub, load, etc.)
- Highlighting search matches
- Horizontal scrolling for longer messages
- More responsive layout
