#!/usr/bin/env bash
set -euo pipefail

target=${1:?Rust target is required}
header=${2:?header path is required}
output_dir=${3:?output directory is required}
license=${4:-LICENSE}
release_dir="target/$target/release"
archive_name="stdbr-ffi-$target"
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT
archive_dir="$work_dir/$archive_name"

case "$target" in
  *-linux-*) libraries=(libstdbr_ffi.a libstdbr_ffi.so) ;;
  *-apple-darwin) libraries=(libstdbr_ffi.a libstdbr_ffi.dylib) ;;
  *-windows-msvc) libraries=(stdbr_ffi.dll stdbr_ffi.lib) ;;
  *) echo "unsupported FFI target: $target" >&2; exit 1 ;;
esac

mkdir -p "$archive_dir" "$output_dir"
output_dir=$(realpath "$output_dir")
cp "$header" "$archive_dir/stdbr.h"
cp "$license" "$archive_dir/LICENSE"
for library in "${libraries[@]}"; do
  test -f "$release_dir/$library"
  cp "$release_dir/$library" "$archive_dir/$library"
done

tar -C "$work_dir" -czf "$output_dir/$archive_name.tar.gz" "$archive_name"
