use crate::{config::Config, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn miri(
    config: &Config,
    args: &mut pico_args::Arguments,
    tool_args: Vec<OsString>,
) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-miri

USAGE:
xtask miri [SUBCOMMAND]

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cargo command

SUBCOMMANDS:
    run             Run the project's binary with cargo-miri
    test            Run the project's tests  with cargo-miri
"#
    .trim();

    if crate::handler::help(args, help)? {
        return Ok(None);
    }

    let miri_subcommand: String = args.free_from_str()?;

    crate::handler::unused(args)?;

    let toolchain = crate::config::rust::toolchain::nightly(config);

    crate::validation::validate_rust_toolchain(&toolchain)?;

    let env_vars = crate::validation::validate_tool(config, "cargo-miri")?;

    let status = match &*miri_subcommand {
        "run" => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(crate::cargo::project_root()?);
            cmd.args([&format!("+{toolchain}"), "miri"]);
            cmd.args([miri_subcommand]);
            cmd.args(tool_args);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        "test" => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(crate::cargo::project_root()?);
            cmd.args([&format!("+{toolchain}"), "miri"]);
            cmd.args([miri_subcommand]);
            cmd.args(tool_args);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        _ => {
            println!("{help}\n");
            return Err(format!("unrecognized miri subcommand `{miri_subcommand}`").into());
        },
    };

    Ok(Some(status))
}
