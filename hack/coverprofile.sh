#!/bin/bash

set -e

base_dir="$(dirname "${BASH_SOURCE[0]}" | xargs realpath | xargs dirname)"

pushd "${base_dir}" >/dev/null

excludes=("/*" "target/*")

if ! command -v grcov >/dev/null 2>&1; then
    echo "Installing grcov"
    cargo install grcov
fi

if ! rustup component list --installed | grep -q llvm-tools; then
    echo "Installing llvm-tools"
    rustup component add llvm-tools
fi

if compgen -G "./*.profraw" >/dev/null; then
    rm ./*.profraw
fi

# Ensure test are run and not cached
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw"

cargo test

ignored_flags=()
for exclude in "${excludes[@]}"; do
    ignored_flags+=("--ignore" "$exclude")
done

grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing "${ignored_flags[@]}" -o target/coverage/html
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing "${ignored_flags[@]}" -o target/coverage/lcov.info

if compgen -G "./*.profraw" >/dev/null; then
    rm ./*.profraw
fi

popd >/dev/null
