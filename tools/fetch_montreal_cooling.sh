#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repository_dir=$(CDPATH= cd -- "$script_dir/.." && pwd)
temporary_dir=$(mktemp -d)
trap 'rm -rf "$temporary_dir"' EXIT HUP INT TERM

archive_url='https://www.astro.umontreal.ca/~bergeron/CoolingModels/CoolingModels/AllSequences.tar.gz'
curl -L --fail --silent --show-error "$archive_url" \
    -o "$temporary_dir/AllSequences.tar.gz"
tar -xzf "$temporary_dir/AllSequences.tar.gz" -C "$temporary_dir"
rustc --edition 2024 "$script_dir/reduce_montreal_cooling.rs" \
    -o "$temporary_dir/reduce_montreal_cooling"
"$temporary_dir/reduce_montreal_cooling" \
    "$temporary_dir" \
    "$repository_dir/assets/scientific_models/white_dwarf_cooling.local.ron"

echo "Generated assets/scientific_models/white_dwarf_cooling.local.ron from the official Montréal archive."
