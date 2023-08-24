use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - Argument processing fails (e.g. invalid arguments)
/// - Tool validation fails (missing tools, incorrect versions, etc.)
/// - The command process fails to start
/// - The command invocation fails with non-zero exit status
pub fn cmake(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-cmake

USAGE:
xtask cmake [SUBCOMMAND]

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cmake subcommand

SUBCOMMANDS:
    build
"#
    .trim();

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    let Some(cmake_subcommand) = context.args.opt_free_from_str::<String>()? else {
        println!("{help}\n");
        return Ok(None);
    };

    crate::handler::unused(context.args)?;

    let status = if cmake_subcommand == "build" {
        let mut cmd = Command::new("cmake");
        cmd.args(["-G", "Ninja"]);
        cmd.args(["-S", "."]);
        cmd.args(["-B", "build"]);
        cmd.args(context.tool_args);
        cmd.current_dir(&context.config.cargo_metadata.workspace_root);
        cmd.status()?
    } else {
        println!("{help}\n");
        return Err(format!("unrecognized `xtask cmake` subcommand `{cmake_subcommand}`").into());
    };

    Ok(Some(status))
}
