#!/bin/bash

set -e

base_dir="$(dirname "${BASH_SOURCE[0]}" | xargs realpath)"

APP_ID="io.github.heathcliff26.turbo-clicker"
BINARY="turbo-clicker"

usr_dir="$HOME/.local"
if [ "$(id -u)" -eq 0 ]; then
    usr_dir="/usr"
fi

bin_dir="${usr_dir}/bin"
libexec_dir="${usr_dir}/libexec"
share_dir="${usr_dir}/share"

help() {
    echo "Integrate Turbo Clicker with common desktop environments."
    echo
    echo "Usage: -i | --install    -- install desktop file"
    echo "       -u | --uninstall  -- uninstall desktop file"
    echo "       -h | --help       -- show usage"
}

install_app() {
    echo "Installing binary to ${bin_dir}/${BINARY}"
    install -Dm755 "${base_dir}/${BINARY}" "${bin_dir}/${BINARY}"
    install -Dm755 "${base_dir}/turbo-clicker-pkexec-wrapper.sh" "${libexec_dir}/turbo-clicker-pkexec-wrapper.sh"
    [ "$(id -u)" -eq 0 ] || sed -i "s#/usr/bin/turbo-clicker#${bin_dir}/${BINARY}#g" "${libexec_dir}/turbo-clicker-pkexec-wrapper.sh"

    echo "Installing desktop file"
    install -Dm644 "${base_dir}/${APP_ID}.desktop" "${share_dir}/applications/${APP_ID}.desktop"
    [ "$(id -u)" -eq 0 ] || sed -i "s#/usr/libexec#${libexec_dir}#g" "${share_dir}/applications/${APP_ID}.desktop"

    echo "Installing icon"
    install -Dm644 "${base_dir}/${APP_ID}.svg" "${share_dir}/icons/hicolor/scalable/apps/${APP_ID}.svg"

    xdg-desktop-menu forceupdate
    xdg-icon-resource forceupdate
}

uninstall_app() {
    echo "Removing binary"
    rm -f "${bin_dir}/${BINARY}" "${libexec_dir}/turbo-clicker-pkexec-wrapper.sh"

    echo "Removing desktop file and icon"
    rm -f "${share_dir}/icons/hicolor/scalable/apps/${APP_ID}.svg" "${share_dir}/applications/${APP_ID}.desktop"
}

while [[ "$#" -gt 0 ]]; do
    case $1 in
    -i | --install)
        install_app
        exit 0
        ;;
    -u | --uninstall)
        uninstall_app
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
done

help
