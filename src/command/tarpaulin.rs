use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - Argument processing fails (e.g. invalid arguments)
/// - Tool validation fails (missing tools, incorrect versions, etc.)
/// - The command process fails to start
/// - The command invocation fails with non-zero exit status
pub fn tarpaulin(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-tarpaulin

USAGE:
xtask tarpaulin

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

    let validation = crate::validation::validate_tool(context.config, "cargo-tarpaulin")?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::workspace::project_root()?);
    cmd.args([&format!("+{toolchain}"), "tarpaulin"]);
    cmd.args(["--packages", "cxx-auto"]);
    cmd.args(["--timeout", "120"]);
    cmd.args(["--out", "Xml"]);
    cmd.args(context.tool_args);
    for (key, value) in validation.env_vars {
        cmd.env(key, value);
    }
    let status = cmd.status()?;

    Ok(Some(status))
}
