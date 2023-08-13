use crate::{config::Config, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn fmt(
    config: &Config,
    args: &mut pico_args::Arguments,
    tool_args: Vec<OsString>,
) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-format

USAGE:
xtask format

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the cargo command
"#
    .trim();

    if crate::handler::help(args, help)? {
        return Ok(None);
    }

    crate::handler::unused(args)?;

    let toolchain = crate::config::rust::toolchain::nightly(config);

    crate::validation::validate_rust_toolchain(&toolchain)?;

    let env_vars = crate::validation::validate_tool(config, "cargo-fmt")?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::cargo::project_root()?);
    cmd.args([&format!("+{toolchain}"), "fmt", "--all"]);
    cmd.args(tool_args);
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    let status = cmd.status()?;

    Ok(Some(status))
}
