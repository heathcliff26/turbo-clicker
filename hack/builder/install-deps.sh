#!/bin/bash

set -ex

arches="amd64 arm64"

case "$(uname -m)" in
    x86_64|amd64)
        current_arch="amd64"
        ;;
    aarch64|arm64)
        current_arch="arm64"
        ;;
    *)
        current_arch="$(uname -m)"
esac

for arch in ${arches}; do
    if [ "${arch}" != "${current_arch}" ]; then
        echo "Adding architecture ${arch}"
        dpkg --add-architecture "${arch}"
    fi
done

echo "Updating package lists"
apt-get update

echo "Installing native dependencies"
apt-get install -y --no-install-recommends --no-install-suggests \
        build-essential \
        pkg-config \
        cmake \
        appstream \
        qt6-base-dev \
        qt6-wayland-dev \
        libfontconfig-dev \
        libxkbcommon-dev

for arch in ${arches}; do
    case "${arch}" in
        amd64)
            pkg_arch="x86-64"
            rust_target="x86_64-unknown-linux-gnu"
            ;;
        arm64)
            pkg_arch="aarch64"
            rust_target="aarch64-unknown-linux-gnu"
            ;;
        *)
            pkg_arch="${arch}"
            rust_target="${arch}-unknown-linux-gnu"
    esac

    echo "Adding rust target for architecture ${arch}"
    rustup target add "${rust_target}"

    if [ "${arch}" == "${current_arch}" ]; then
        continue
    fi

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
arch=$(uname -m)
if [ "${arch}" == "aarch64" ]; then
    arch="arm64"
fi
curl -SL -o goreleaser.tar.gz "https://github.com/goreleaser/goreleaser/releases/download/${GORELEASER_VERSION}/goreleaser_Linux_${arch}.tar.gz"
tar -xzf goreleaser.tar.gz -C "/usr/local/bin" goreleaser
rm goreleaser.tar.gz
goreleaser --version

echo "Cleaning up"
apt-get clean
rm -rf /var/lib/apt/lists/* /var/cache/apt/*
