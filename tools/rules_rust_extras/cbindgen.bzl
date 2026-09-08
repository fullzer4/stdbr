def _cbindgen_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.header_name)

    args = ctx.actions.args()
    args.add("--config")
    args.add(ctx.file.config)
    args.add("--output")
    args.add(output)
    args.add(ctx.file.crate_root)

    ctx.actions.run(
        executable = ctx.executable._cbindgen,
        arguments = [args],
        inputs = depset(ctx.files.srcs + [ctx.file.config, ctx.file.crate_root]),
        outputs = [output],
        tools = [ctx.attr._cbindgen[DefaultInfo].files_to_run],
        mnemonic = "Cbindgen",
        progress_message = "Generating C header %{output}",
    )

    return [
        DefaultInfo(files = depset([output])),
        CcInfo(
            compilation_context = cc_common.create_compilation_context(
                headers = depset([output]),
                system_includes = depset([output.dirname]),
            ),
        ),
    ]

cbindgen = rule(
    implementation = _cbindgen_impl,
    attrs = {
        "srcs": attr.label_list(
            allow_files = [".rs"],
            mandatory = True,
            doc = "All local Rust source files parsed by cbindgen.",
        ),
        "crate_root": attr.label(
            allow_single_file = [".rs"],
            mandatory = True,
            doc = "Root Rust source file, normally src/lib.rs.",
        ),
        "config": attr.label(
            allow_single_file = [".toml"],
            mandatory = True,
            doc = "cbindgen.toml configuration file.",
        ),
        "header_name": attr.string(
            default = "stdbr.h",
            doc = "Name of the output header file.",
        ),
        "_cbindgen": attr.label(
            default = Label("@nix_cbindgen//:cbindgen"),
            allow_files = True,
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Generates a C header from local Rust source files.",
)
