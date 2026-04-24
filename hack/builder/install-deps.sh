#!/bin/bash

set -ex

arches="arm64"

for arch in ${arches}; do
    echo "Adding architecture ${arch}"
    dpkg --add-architecture "${arch}"
done

echo "Updating package lists"
apt-get update

echo "Installing native dependencies"
apt-get install -y --no-install-recommends --no-install-suggests \
        appstream \
        qt6-base-dev \
        qt6-wayland-dev \
        libfontconfig-dev \
        libxkbcommon-dev

echo "Adding rust target for architecture x86_64"
rustup target add "x86_64-unknown-linux-gnu"

for arch in ${arches}; do
    case "${arch}" in
        arm64)
            pkg_arch="aarch64"
            ;;
        *)
            pkg_arch="${arch}"
    esac

    echo "Adding rust target for architecture ${arch}"
    rustup target add "${pkg_arch}-unknown-linux-gnu"

    echo "Installing dependencies for architecture ${arch}"
    apt-get install -y --no-install-recommends --no-install-suggests \
        "gcc-${pkg_arch}-linux-gnu" \
        "g++-${pkg_arch}-linux-gnu" \
        "qt6-base-dev:${arch}" \
        "qt6-wayland-dev:${arch}" \
        "libfontconfig-dev:${arch}" \
        "libxkbcommon-dev:${arch}"
done

echo "Installing clippy"
rustup component add clippy

echo "Installing rustfmt"
rustup component add rustfmt

echo "Installing goreleaser"
curl -SL -o goreleaser.tar.gz "https://github.com/goreleaser/goreleaser/releases/download/${GORELEASER_VERSION}/goreleaser_Linux_x86_64.tar.gz"
tar -xzf goreleaser.tar.gz -C "/usr/local/bin" goreleaser
rm goreleaser.tar.gz
goreleaser --version

echo "Cleaning up"
apt-get clean
rm -rf /var/lib/apt/lists/* /var/cache/apt/*
