def _cbindgen_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.header_name)

    cmd = " ".join([
        "cbindgen",
        "--config",
        ctx.file.config.path,
        "--lockfile",
        ctx.file.lockfile.path,
        "--crate",
        ctx.attr.crate_name,
        "--output",
        output.path,
        ctx.file.manifest.dirname,
    ])

    all_inputs = (
        ctx.files.srcs +
        ctx.files.workspace_srcs +
        [ctx.file.config, ctx.file.lockfile, ctx.file.manifest]
    )

    ctx.actions.run_shell(
        inputs = all_inputs,
        outputs = [output],
        command = cmd,
        use_default_shell_env = True,
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
            doc = "Rust source files of the FFI crate.",
        ),
        "config": attr.label(
            allow_single_file = [".toml"],
            doc = "cbindgen.toml configuration file.",
        ),
        "manifest": attr.label(
            allow_single_file = ["Cargo.toml"],
            doc = "Cargo.toml of the workspace root.",
        ),
        "lockfile": attr.label(
            allow_single_file = ["Cargo.lock"],
            doc = "Cargo.lock of the workspace.",
        ),
        "workspace_srcs": attr.label_list(
            allow_files = True,
            doc = "All workspace Cargo.toml + source files needed by cargo metadata.",
        ),
        "crate_name": attr.string(
            doc = "Name of the crate to generate bindings for.",
        ),
        "header_name": attr.string(
            default = "stdbr.h",
            doc = "Name of the output header file.",
        ),
    },
    doc = "Generates a C header file from Rust sources using cbindgen.",
)
