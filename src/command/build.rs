use crate::{config::Config, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn build(
    _config: &Config,
    args: &mut pico_args::Arguments,
    tool_args: Vec<OsString>,
) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-build

USAGE:
xtask build

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cargo command
"#
    .trim();

    if crate::handler::help(args, help)? {
        return Ok(None);
    }

    crate::handler::unused(args)?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::cargo::project_root()?);
    cmd.args(["build"]);
    cmd.args(["--package", "cxx-auto"]);
    cmd.args(tool_args);

    let status = cmd.status()?;

    Ok(Some(status))
}
