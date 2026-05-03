load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

_BUILD_CONTENT = """
cc_library(
    name = "cjson",
    srcs = ["cJSON.c"],
    hdrs = ["cJSON.h"],
    includes = ["."],
    visibility = ["//visibility:public"],
)
"""

def _cjson_impl(module_ctx):
    http_archive(
        name = "cjson",
        urls = ["https://github.com/DaveGamble/cJSON/archive/refs/tags/v1.7.18.tar.gz"],
        strip_prefix = "cJSON-1.7.18",
        build_file_content = _BUILD_CONTENT,
    )

cjson = module_extension(implementation = _cjson_impl)
