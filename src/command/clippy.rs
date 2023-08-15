use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - Argument processing fails (e.g. invalid arguments)
/// - The command process fails to start
/// - The command invocation fails with non-zero exit status
pub fn clippy(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-clippy

USAGE:
xtask clippy

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cargo command
"#
    .trim();

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    crate::handler::unused(context.args)?;

    let toolchain = crate::config::rust::toolchain::nightly(context.config);

    crate::validation::validate_rust_toolchain(toolchain)?;

    let validation = crate::validation::validate_tool(context.config, "cargo-clippy")?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::workspace::project_root()?);
    cmd.args([&format!("+{toolchain}"), "clippy"]);
    cmd.args(["--package", "xtask"]);
    cmd.args(["--package", "cxx-auto"]);
    cmd.args(context.tool_args);
    cmd.args(["--", "-D", "warnings"]);
    for (key, value) in validation.env_vars {
        cmd.env(key, value);
    }
    let status = cmd.status()?;

    Ok(Some(status))
}
