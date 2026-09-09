#!/usr/bin/env bash
set -euo pipefail

artifact_dir=$(realpath "${1:?crate artifact directory is required}")
version=${2:?release version is required}
source_dir=$(realpath "${3:?release source directory is required}")

(
  cd "$source_dir"
  RUSTC_BOOTSTRAP=1 cargo -Z package-workspace package --locked --no-verify \
    -p stdbr-core -p stdbr
)

for name in stdbr-core stdbr; do
  cmp "$artifact_dir/$name-$version.crate" "$source_dir/target/package/$name-$version.crate"
done

publish_crate() {
  local name=$1
  local manifest=$2
  local archive="$artifact_dir/$name-$version.crate"
  local status

  test -f "$archive"
  status=$(curl --silent --show-error --output /dev/null --write-out '%{http_code}' \
    --user-agent "stdbr-release-workflow/$version (https://github.com/fullzer4/stdbr)" \
    "https://crates.io/api/v1/crates/$name/$version")
  case "$status" in
    200) echo "$name $version already exists on crates.io; skipping"; return ;;
    404) ;;
    *) echo "crates.io returned HTTP $status for $name $version" >&2; exit 1 ;;
  esac

  cargo publish --locked --no-verify --manifest-path "$manifest"
}

publish_crate stdbr-core "$source_dir/core/Cargo.toml"

for attempt in {1..10}; do
  if publish_crate stdbr "$source_dir/Cargo.toml"; then
    exit 0
  fi
  if [[ $attempt -eq 10 ]]; then
    echo "stdbr could not be published after $attempt attempts" >&2
    exit 1
  fi
  sleep 15
done
