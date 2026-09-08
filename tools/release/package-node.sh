#!/usr/bin/env bash
set -euo pipefail

native_dir=${1:?native artifact directory is required}
output_dir=${2:?output directory is required}
source_dir=${3:-bindings/nodejs}
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/../.." && pwd)
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT
staging_dir="$work_dir/nodejs"

declare -A binaries=(
  [darwin-arm64]=stdbr.darwin-arm64.node
  [darwin-x64]=stdbr.darwin-x64.node
  [linux-arm64-gnu]=stdbr.linux-arm64-gnu.node
  [linux-x64-gnu]=stdbr.linux-x64-gnu.node
  [linux-x64-musl]=stdbr.linux-x64-musl.node
  [win32-x64-msvc]=stdbr.win32-x64-msvc.node
)

mkdir -p "$output_dir"
output_dir=$(realpath "$output_dir")
cp -R "$source_dir" "$staging_dir"
rm -rf "$staging_dir/node_modules" "$staging_dir/target"
node "$script_dir/sync-npm-version.mjs" "$staging_dir"
cp "$repo_root/LICENSE" "$staging_dir/LICENSE"

for platform in "${!binaries[@]}"; do
  binary=${binaries[$platform]}
  test -f "$native_dir/$binary"
  test -d "$staging_dir/npm/$platform"
  cp "$repo_root/LICENSE" "$staging_dir/npm/$platform/LICENSE"
  cp "$native_dir/$binary" "$staging_dir/npm/$platform/$binary"
  npm pack "$staging_dir/npm/$platform" --ignore-scripts --pack-destination "$output_dir"
done

npm pack "$staging_dir" --ignore-scripts --pack-destination "$output_dir"

packages=("$output_dir"/*.tgz)
if [[ ${#packages[@]} -ne 7 ]]; then
  echo "expected 7 npm packages, found ${#packages[@]}" >&2
  exit 1
fi
