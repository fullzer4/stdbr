load("@bazel_skylib//rules:write_file.bzl", "write_file")

LOCAL_TAGS = ["no-sandbox", "local"]
GOLDEN_ENV = {"GOLDEN_JSON": "$(rootpath :gen_golden)"}

_WS_PREFIX = [
    "#!/usr/bin/env bash",
    "set -euo pipefail",
    'WS=$(dirname "$(readlink -f "$TEST_SRCDIR/_main/Cargo.toml")")',
    'export GOLDEN_JSON="$TEST_SRCDIR/_main/$GOLDEN_JSON"',
]

def _ws_cmd(build_cmd):
    return (
        "OUT=$$PWD/$@ && " +
        "WS=$$(dirname $$(readlink -f $(location //:Cargo.toml))) && " +
        "cd $$WS && " + build_cmd + " >&2 && touch $$OUT"
    )

def parity_binding_test(name, build_cmd, runner_lines, test_file):
    native.genrule(
        name = "build_" + name,
        srcs = ["//:Cargo.toml"],
        outs = [name + "_built.stamp"],
        cmd = _ws_cmd(build_cmd),
        tags = LOCAL_TAGS,
    )

    write_file(
        name = "gen_{}_runner".format(name),
        out = "run_{}.sh".format(name),
        content = _WS_PREFIX + runner_lines,
        is_executable = True,
    )

    native.sh_test(
        name = "parity_" + name,
        srcs = [":gen_{}_runner".format(name)],
        data = [":gen_golden", ":build_" + name, test_file, "//:Cargo.toml"],
        env = GOLDEN_ENV,
        tags = LOCAL_TAGS,
    )
