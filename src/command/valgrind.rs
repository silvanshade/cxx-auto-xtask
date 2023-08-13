use crate::{config::Config, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn valgrind(
    config: &Config,
    args: &mut pico_args::Arguments,
    tool_args: Vec<OsString>,
) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-valgrind

USAGE:
xtask valgrind [SUBCOMMAND]

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cargo command

SUBCOMMANDS:
    run             Run the project's binary with cargo-valgrind
    test            Run the project's tests  with cargo-valgrind
"#
    .trim();

    if crate::handler::help(args, help)? {
        return Ok(None);
    }

    let valgrind_subcommand: String = args.free_from_str()?;

    crate::handler::unused(args)?;

    let env_vars = crate::validation::validate_tool(config, "cargo-valgrind")?;

    let status = match &*valgrind_subcommand {
        "run" => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(crate::cargo::project_root()?);
            cmd.args(["valgrind"]);
            cmd.args([valgrind_subcommand]);
            cmd.args(["--features", "valgrind"]);
            cmd.args(tool_args);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        "test" => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(crate::cargo::project_root()?);
            cmd.args(["valgrind"]);
            cmd.args([valgrind_subcommand]);
            cmd.args(["--features", "valgrind"]);
            cmd.args(tool_args);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        _ => {
            println!("{help}\n");
            return Err(format!("unrecognized valgrind subcommand `{valgrind_subcommand}`").into());
        },
    };

    Ok(Some(status))
}
