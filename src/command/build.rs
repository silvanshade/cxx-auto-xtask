use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

pub fn build(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-build

USAGE:
xtask build

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cargo command
"#
    .trim();

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    crate::handler::unused(context.args)?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::workspace::project_root()?);
    cmd.args(["build"]);
    cmd.args(["--package", "cxx-auto"]);
    cmd.args(context.tool_args);

    let status = cmd.status()?;

    Ok(Some(status))
}
