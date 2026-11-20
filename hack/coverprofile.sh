#!/bin/bash

set -e

base_dir="$(dirname "${BASH_SOURCE[0]}" | xargs realpath | xargs dirname)"
bin_dir="${base_dir}/bin"

pushd "${base_dir}" >/dev/null

excludes=("/*" "target/*" "**/test.rs" "src/test/*")

grcov=grcov
if ! command -v grcov >/dev/null 2>&1; then
    grcov="${bin_dir}/grcov"
    if [ ! -f "${grcov}" ]; then
        echo "Installing grcov"
        mkdir -p "${bin_dir}"
        target="$(rustc -vV | sed -n 's/^host: //p')"
        archive="${bin_dir}/grcov-${target}.tar.bz2"
        curl -fsSL "https://github.com/mozilla/grcov/releases/latest/download/grcov-${target}.tar.bz2" -o "${archive}"
        tar -xjf "${archive}" -C "${bin_dir}"
        rm "${archive}"
    fi
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

name="$(cargo metadata --no-deps --format-version 1 | jq -r .packages[0].name)"
name_pattern="${name//-/_}-*"
binary_path="./target/debug/deps/"
if [ ! -d "${binary_path}" ]; then
    test_binary="$(find "./target/debug/build/${name}" -type f -perm -111 -name "${name_pattern}" -print -quit 2>/dev/null)"
    if [ -n "${test_binary}" ]; then
        binary_path="$(dirname "${test_binary}")/"
    fi
fi

rm -rf target/coverage/html
rm -f target/coverage/lcov.info

${grcov} . --binary-path "${binary_path}" -s . -t html --branch --ignore-not-existing "${ignored_flags[@]}" -o target/coverage/html
${grcov} . --binary-path "${binary_path}" -s . -t lcov --branch --ignore-not-existing "${ignored_flags[@]}" -o target/coverage/lcov.info

if compgen -G "./*.profraw" >/dev/null; then
    rm ./*.profraw
fi

popd >/dev/null
