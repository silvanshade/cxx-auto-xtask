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

    let cmake_subcommand: String = context.args.free_from_str()?;

    crate::handler::unused(context.args)?;

    let mut validation = crate::validation::Validation::default();
    validation.combine(crate::validation::validate_tool(context.config, "cmake")?);
    validation.combine(crate::validation::validate_tool(context.config, "ninja")?);

    let status = if cmake_subcommand == "build" {
        let mut cmd = Command::new("cmake");
        cmd.args(["-G", "Ninja"]);
        cmd.args(["-S", "."]);
        cmd.args(["-B", "build"]);
        cmd.args(context.tool_args);
        for (key, value) in validation.env_vars {
            cmd.env(key, value);
        }
        cmd.current_dir(&context.config.project_root_dir);
        cmd.status()?
    } else {
        println!("{help}\n");
        return Err(format!("unrecognized cmake subcommand `{cmake_subcommand}`").into());
    };

    Ok(Some(status))
}
