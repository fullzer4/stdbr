def _cbindgen_repository_impl(repository_ctx):
    executable = repository_ctx.os.environ.get("CBINDGEN")
    if not executable:
        fail("CBINDGEN is not set; run Bazel through `nix develop`")

    executable_path = repository_ctx.path(executable)
    if not executable_path.exists:
        fail("CBINDGEN does not exist: {}".format(executable))

    repository_ctx.symlink(executable_path, "cbindgen")
    repository_ctx.file(
        "BUILD.bazel",
        """exports_files(
    ["cbindgen"],
    visibility = ["//visibility:public"],
)
""",
    )

cbindgen_repository = repository_rule(
    implementation = _cbindgen_repository_impl,
    configure = True,
    environ = ["CBINDGEN"],
)
