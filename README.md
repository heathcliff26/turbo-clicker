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
    - [Fedora Copr](#fedora-copr)
    - [Download binary](#download-binary)
      - [Uninstalling](#uninstalling)
  - [Credits](#credits)

## Screenshots

![](screenshots/window-dark.png#gh-dark-mode-only)
![](screenshots/window-light.png#gh-light-mode-only)

## Installation

### Fedora Copr

The app is available as an rpm by using the fedora copr repository [heathcliff26/turbo-clicker](https://copr.fedorainfracloud.org/coprs/heathcliff26/turbo-clicker/).
1. Enable the copr repository
```bash
sudo dnf copr enable heathcliff26/turbo-clicker
```
2. Install the app
```bash
sudo dnf install turbo-clicker
```

### Download binary

1. Download the [latest release](https://github.com/heathcliff26/turbo-clicker/releases/latest)
2. Unpack the archive
3. Install the app for your user by running:
   - You can install it globally by running the script with `sudo`
```bash
./install.sh -i
```

#### Uninstalling

1. Switch to the folder with the installation script
2. Uninstall by running:
   - Run with `sudo` if you installed it globally
```bash
./install.sh -u
```
3. Delete the folder.

## Credits

Frontend framework: [slint](https://slint.dev/)
