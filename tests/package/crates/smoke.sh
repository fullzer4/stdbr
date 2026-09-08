#!/usr/bin/env bash
set -euo pipefail

artifact_dir=${1:?crate artifact directory is required}
expected_version=${2:-}
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

archives=("$artifact_dir"/*.crate)
if [[ ${#archives[@]} -ne 2 ]]; then
  echo "expected 2 crate archives, found ${#archives[@]}" >&2
  exit 1
fi

for archive in "${archives[@]}"; do
  tar -xzf "$archive" -C "$work_dir"
done

core_dirs=("$work_dir"/stdbr-core-*)
root_dirs=("$work_dir"/stdbr-*)
core_dir=${core_dirs[0]}
root_dir=''
for candidate in "${root_dirs[@]}"; do
  if [[ $(basename "$candidate") != stdbr-core-* ]]; then
    root_dir=$candidate
  fi
done
test -d "$core_dir"
test -n "$root_dir"
test -f "$core_dir/LICENSE"
test -f "$root_dir/LICENSE"

if [[ -n "$expected_version" ]]; then
  [[ $(basename "$core_dir") == "stdbr-core-$expected_version" ]]
  [[ $(basename "$root_dir") == "stdbr-$expected_version" ]]
fi

mkdir -p "$work_dir/consumer/src" "$work_dir/cargo-home" "$work_dir/target"
cat > "$work_dir/consumer/Cargo.toml" <<EOF
[package]
name = "stdbr-package-smoke"
version = "0.0.0"
edition = "2024"

[dependencies]
stdbr = { path = "$root_dir" }

[patch.crates-io]
stdbr-core = { path = "$core_dir" }
EOF
cat > "$work_dir/consumer/src/main.rs" <<'EOF'
fn main() {
    assert!(stdbr::cpf::is_valid("52998224725"));
}
EOF

CARGO_HOME="$work_dir/cargo-home" CARGO_TARGET_DIR="$work_dir/target" \
  cargo run --offline --manifest-path "$work_dir/consumer/Cargo.toml"
