[![CI](https://github.com/heathcliff26/turbo-clicker/actions/workflows/ci.yaml/badge.svg?event=push)](https://github.com/heathcliff26/turbo-clicker/actions/workflows/ci.yaml)
[![Coverage Status](https://coveralls.io/repos/github/heathcliff26/turbo-clicker/badge.svg)](https://coveralls.io/github/heathcliff26/turbo-clicker)
[![Editorconfig Check](https://github.com/heathcliff26/turbo-clicker/actions/workflows/editorconfig-check.yaml/badge.svg?event=push)](https://github.com/heathcliff26/turbo-clicker/actions/workflows/editorconfig-check.yaml)
[![Renovate](https://github.com/heathcliff26/turbo-clicker/actions/workflows/renovate.yaml/badge.svg)](https://github.com/heathcliff26/turbo-clicker/actions/workflows/renovate.yaml)

# Turbo Clicker

GUI based auto-clicker for Linux. It uses uinput and should thus work independently of wayland or x11.

## Table of Contents

- [Turbo Clicker](#turbo-clicker)
  - [Table of Contents](#table-of-contents)
  - [Screenshots](#screenshots)
  - [Installation](#installation)
    - [Download binary](#download-binary)
      - [Uninstalling](#uninstalling)
    - [Flatpak](#flatpak)
    - [RPM](#rpm)
  - [Credits](#credits)

## Screenshots

![](screenshots/window-dark.png#gh-dark-mode-only)
![](screenshots/window-light.png#gh-light-mode-only)

## Installation

### Download binary

1. Download the latest release
2. Unpack the archive into your installation folder
3. Switch to the installation folder
4. Install the desktop file to list the app in the search by running:
```bash
./install-desktop.sh -i
```

#### Uninstalling

1. Switch to the installation folder
2. Uninstall the desktop file by running:
```bash
./install-desktop.sh -u
```
3. Delete the installation folder.

### Flatpak

The app can be installed from [flathub](https://github.com/flathub/io.github.heathcliff26.turbo-clicker)

### RPM
TODO?

## Credits

Frontend framework: [slint](https://slint.dev/)
