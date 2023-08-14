use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

pub fn valgrind(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
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

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    let valgrind_subcommand: String = context.args.free_from_str()?;

    crate::handler::unused(context.args)?;

    let validation = crate::validation::validate_tool(context.config, "cargo-valgrind")?;

    let status = match &*valgrind_subcommand {
        "run" => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(crate::workspace::project_root()?);
            cmd.args(["valgrind"]);
            cmd.args([valgrind_subcommand]);
            cmd.args(["--features", "valgrind"]);
            cmd.args(context.tool_args);
            for (key, value) in validation.env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        "test" => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(crate::workspace::project_root()?);
            cmd.args(["valgrind"]);
            cmd.args([valgrind_subcommand]);
            cmd.args(["--features", "valgrind"]);
            cmd.args(context.tool_args);
            for (key, value) in validation.env_vars {
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
