#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

jobs="${QUALITY_JOBS:-2}"
memory_mb="${QUALITY_MEMORY_MB:-4096}"
coverage_min_lines="${COVERAGE_MIN_LINES:-95}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-$jobs}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

run() {
  printf '+ '
  printf '%q ' "$@"
  printf '\n'
  "$@"
}

flake() {
  run nix flake check path:. --no-update-lock-file --max-jobs "$jobs" --cores "$jobs"
}

bazel_gate() {
  run bazel test \
    --jobs="$jobs" \
    --local_resources="memory=$memory_mb" \
    //:all_tests
}

rust() {
  run cargo fmt --all -- --check
  run cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
  run env RUSTDOCFLAGS=-Dwarnings cargo doc --locked \
    --package stdbr-core --package stdbr --all-features --no-deps
}

coverage() {
  run cargo llvm-cov --locked --package stdbr-core --all-features \
    --jobs "$jobs" --fail-under-lines "$coverage_min_lines"
}

fuzz_build() {
  : "${FUZZ_CARGO:?FUZZ_CARGO is provided by the Nix devshell}"
  : "${FUZZ_RUSTC:?FUZZ_RUSTC is provided by the Nix devshell}"
  run env RUSTC="$FUZZ_RUSTC" "$FUZZ_CARGO" fuzz build --fuzz-dir fuzz
}

msrv() {
  : "${MSRV_CARGO:?MSRV_CARGO is provided by the Nix devshell}"
  : "${MSRV_RUSTC:?MSRV_RUSTC is provided by the Nix devshell}"
  run env RUSTC="$MSRV_RUSTC" "$MSRV_CARGO" check \
    --locked --package stdbr-core --package stdbr --all-features
}

no_std() {
  run cargo check --locked --package stdbr-core --no-default-features
  run cargo check --locked --package stdbr --no-default-features
}

audit() {
  run cargo audit --deny warnings
}

semver() {
  : "${SEMVER_TOOLCHAIN_BIN:?SEMVER_TOOLCHAIN_BIN is provided by the Nix devshell}"
  run env PATH="$SEMVER_TOOLCHAIN_BIN:$PATH" cargo semver-checks check-release --package stdbr-core
  run env PATH="$SEMVER_TOOLCHAIN_BIN:$PATH" cargo semver-checks check-release --package stdbr
}

usage() {
  printf 'usage: %s {all|flake|bazel|rust|coverage|fuzz-build|msrv|no-std|audit|semver}\n' "$0" >&2
  exit 2
}

case "${1:-all}" in
  all)
    flake
    bazel_gate
    rust
    coverage
    msrv
    no_std
    audit
    semver
    ;;
  flake | rust | coverage | msrv | audit | semver)
    "$1"
    ;;
  fuzz-build)
    fuzz_build
    ;;
  bazel)
    bazel_gate
    ;;
  no-std)
    no_std
    ;;
  *)
    usage
    ;;
esac
