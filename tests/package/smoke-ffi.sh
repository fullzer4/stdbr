#!/usr/bin/env bash
set -euo pipefail

artifact_dir=${1:?FFI artifact directory is required}
target=${2:?Rust target is required}
archive="$artifact_dir/stdbr-ffi-$target.tar.gz"
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

test -f "$archive"
tar -xzf "$archive" -C "$work_dir"
package_dir="$work_dir/stdbr-ffi-$target"
test -f "$package_dir/stdbr.h"
test -f "$package_dir/LICENSE"

case "$target" in
  *-linux-*)
    test -f "$package_dir/libstdbr_ffi.a"
    test -f "$package_dir/libstdbr_ffi.so"
    runtime_variable=LD_LIBRARY_PATH
    ;;
  *-apple-darwin)
    test -f "$package_dir/libstdbr_ffi.a"
    test -f "$package_dir/libstdbr_ffi.dylib"
    runtime_variable=DYLD_LIBRARY_PATH
    ;;
  *) echo "unsupported Unix FFI target: $target" >&2; exit 1 ;;
esac

cat > "$work_dir/smoke.c" <<'EOF'
#include "stdbr.h"

int main(void) {
    return stdbr_cpf_is_valid("52998224725") ? 0 : 1;
}
EOF

cc "$work_dir/smoke.c" -I"$package_dir" -L"$package_dir" -lstdbr_ffi -o "$work_dir/smoke"
env "$runtime_variable=$package_dir" "$work_dir/smoke"
