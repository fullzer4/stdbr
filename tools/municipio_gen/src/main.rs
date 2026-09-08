use municipio_gen_lib::{generate, has_drift};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("municipio_gen: {message}");
            ExitCode::from(code)
        }
    }
}

fn run() -> Result<(), (u8, String)> {
    let root = env::var_os("BUILD_WORKSPACE_DIRECTORY")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().expect("cannot determine current directory"));
    let source = root.join("tools/municipio_gen/ibge_municipios.json");
    let target = root.join("core/src/municipio.rs");
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "generate".to_owned());

    let mut input = source.clone();
    let mut update_source = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                input = PathBuf::from(
                    args.next()
                        .ok_or_else(|| data_error("--input requires a path"))?,
                );
            }
            "--update-source" => update_source = true,
            _ => return Err(data_error(format!("unknown argument: {arg}"))),
        }
    }

    match command.as_str() {
        "generate" => {
            let changed = generate(&input, &target, &source, update_source).map_err(data_error)?;
            println!(
                "municipality data {}",
                if changed { "updated" } else { "is current" }
            );
            Ok(())
        }
        "check" => {
            let input_text = fs::read_to_string(&input).map_err(io_error(&input))?;
            let target_text = fs::read_to_string(&target).map_err(io_error(&target))?;
            if has_drift(&input_text, &target_text).map_err(data_error)? {
                Err((3, "generated municipality data has drift; run `bazel run //tools/municipio_gen -- generate`".to_owned()))
            } else {
                println!("generated municipality data is current");
                Ok(())
            }
        }
        _ => Err(data_error(format!(
            "unknown command: {command}; expected generate or check"
        ))),
    }
}

fn data_error(message: impl Into<String>) -> (u8, String) {
    (2, message.into())
}

fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> (u8, String) + '_ {
    move |error| (2, format!("failed to access {}: {error}", path.display()))
}
