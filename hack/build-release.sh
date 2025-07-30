#!/bin/bash

set -e

base_dir="$(dirname "${BASH_SOURCE[0]}" | xargs realpath | xargs dirname)"

bin_dir="${base_dir}/dist"
name="$(yq -r '.package.name' "${base_dir}/Cargo.toml")"

[ -d "${bin_dir}" ] || mkdir -p "${bin_dir}"

cargo build --release

case "$(uname -m)" in
x86_64 | amd64)
    arch="amd64"
    ;;
aarch64 | arm64)
    arch="arm64"
    ;;
*)
    arch="$(uname -m)"
    ;;
esac

tmp_dir="${base_dir}/tmp/${name}-${arch}"
[ -e "${tmp_dir}" ] && rm -rf "${tmp_dir}"
mkdir -p "${tmp_dir}"
cp packages/* "${base_dir}/target/release/${name}" "${tmp_dir}/"
tar -C "${tmp_dir}" -czf "${bin_dir}/${name}_linux-${arch}.tar.gz" .
rm -rf "${tmp_dir}"
