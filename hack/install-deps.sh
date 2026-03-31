#!/bin/bash

set -e

install_apt () {
    packages="qt6-base-dev qt6-wayland-dev libfontconfig-dev libxkbcommon-dev"
    run_install=no

    for package in ${packages}; do
        if ! dpkg -l "${package}" > /dev/null 2>&1; then
            run_install=yes
            break
        fi
    done

    if [ "$run_install" = "yes" ]; then
        echo "Installing missing dependencies..."
        sudo apt-get update
        #shellcheck disable=SC2086
        sudo apt-get install -y ${packages}
    else
        echo "All dependencies are already installed."
    fi
}

install_dnf () {
    packages="qt6-qtbase-devel qt6-qtwayland-devel fontconfig-devel"
    run_install=no

    for package in ${packages}; do
        if ! rpm -q "${package}" > /dev/null 2>&1; then
            run_install=yes
            break
        fi
    done

    if [ "$run_install" = "yes" ]; then
        echo "Installing missing dependencies..."
        #shellcheck disable=SC2086
        sudo dnf install -y ${packages}
    else
        echo "All dependencies are already installed."
    fi
}

source /etc/os-release

case "${ID}" in
    "ubuntu"|"debian")
        install_apt
        ;;
    "fedora")
        install_dnf
        ;;
    *)
        echo "Unknown OS ${ID}, you need to manage dependencies manually"
esac
