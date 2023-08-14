use crate::{command::Context, BoxResult};
use std::process::{Command, ExitStatus};

pub fn test(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-test

USAGE:
xtask test

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
    cmd.args(["test"]);
    cmd.args(["--package", "cxx-auto"]);
    cmd.args(context.tool_args);

    let status = cmd.status()?;

    Ok(Some(status))
}
