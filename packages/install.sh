#!/bin/bash

base_dir="$(dirname "${BASH_SOURCE[0]}" | xargs realpath)"

APP_ID="io.github.heathcliff26.turbo-clicker"
BINARY="turbo-clicker"

bin_dir="/usr/local/bin"
icon_dir="/usr/share/icons/hicolor"

help() {
    echo "Integrate Turbo Clicker with common desktop environments."
    echo
    echo "Usage: -i | --install    -- install desktop file"
    echo "       -u | --uninstall  -- uninstall desktop file"
    echo "       -h | --help       -- show usage"
}

install() {
    echo "Installing binary to ${bin_dir}/${BINARY}"
    sudo cp "${base_dir}/${BINARY}" "${bin_dir}/${BINARY}"

    echo "Installing desktop file"
    sudo xdg-desktop-menu install "${base_dir}/${APP_ID}.desktop"

    echo "Installing icon"
    sudo mkdir -p "${icon_dir}/scalable/apps"
    sudo cp "${base_dir}/${APP_ID}.svg" "${icon_dir}/scalable/apps/${APP_ID}.svg"

    xdg-desktop-menu forceupdate
    xdg-icon-resource forceupdate
}

uninstall() {
    echo "Removing binary"
    sudo rm "${bin_dir}/${BINARY}"

    echo "Removing desktop file and icon"
    sudo xdg-desktop-menu uninstall "${APP_ID}.desktop"
    sudo rm "${icon_dir}/scalable/apps/${APP_ID}.svg"
}

while [[ "$#" -gt 0 ]]; do
    case $1 in
    -i | --install)
        install
        exit 0
        ;;
    -u | --uninstall)
        uninstall
        exit 0
        ;;
    -h | --help)
        help
        exit 0
        ;;
    *)
        echo "Unknown argument: $1"
        help
        exit 1
        ;;
    esac
    shift
done

help
