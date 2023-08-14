use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

pub fn udeps(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-udep

USAGE:
xtask udeps

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

    crate::validation::validate_rust_toolchain(&toolchain)?;

    let validation = crate::validation::validate_tool(context.config, "cargo-udeps")?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::workspace::project_root()?);
    cmd.args([&format!("+{toolchain}"), "udeps"]);
    cmd.args(["--package", "xtask"]);
    cmd.args(["--package", "cxx-auto"]);
    cmd.args(context.tool_args);
    for (key, value) in validation.env_vars {
        cmd.env(key, value);
    }
    let status = cmd.status()?;

    Ok(Some(status))
}
