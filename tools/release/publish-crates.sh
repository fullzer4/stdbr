#!/usr/bin/env bash
set -euo pipefail

artifact_dir=${1:?crate artifact directory is required}
version=${2:?release version is required}
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

publish_crate() {
  local name=$1
  local archive="$artifact_dir/$name-$version.crate"
  local status

  test -f "$archive"
  status=$(curl --silent --show-error --output /dev/null --write-out '%{http_code}' \
    "https://crates.io/api/v1/crates/$name/$version")
  case "$status" in
    200) echo "$name $version already exists on crates.io; skipping"; return ;;
    404) ;;
    *) echo "crates.io returned HTTP $status for $name $version" >&2; exit 1 ;;
  esac

  tar -xzf "$archive" -C "$work_dir"
  cargo publish --no-verify --manifest-path "$work_dir/$name-$version/Cargo.toml"
}

publish_crate stdbr-core

for attempt in {1..10}; do
  if publish_crate stdbr; then
    exit 0
  fi
  if [[ $attempt -eq 10 ]]; then
    echo "stdbr could not be published after $attempt attempts" >&2
    exit 1
  fi
  sleep 15
done
