#!/bin/bash

set -e

base_dir="$(dirname "${BASH_SOURCE[0]}" | xargs realpath | xargs dirname)"
translations_dir="${base_dir}/translations"
app_name="turbo-clicker"

pot_file="${translations_dir}/${app_name}.pot"

pushd "${base_dir}" >/dev/null

echo "Extracting translations"
# shellcheck disable=SC2038
find ui/ -name \*.slint | xargs slint-tr-extractor -o "${pot_file}"

for lang in "en" "de"; do
    echo "Updating ${lang} translations"
    msgmerge -U --lang="${lang}" "${translations_dir}/${lang}/LC_MESSAGES/${app_name}.po" "${pot_file}"
    rm "${translations_dir}/${lang}/LC_MESSAGES/${app_name}.po~"
done

rm "${pot_file}"

popd >/dev/null
