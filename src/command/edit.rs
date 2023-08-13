use crate::{config::Config, BoxError, BoxResult};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn edit(
    config: &Config,
    args: &mut pico_args::Arguments,
    tool_args: Vec<OsString>,
) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-edit

USAGE:
xtask edit [<editor>] [-- <...>]

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the editor command
"#
    .trim();

    if crate::handler::help(args, help)? {
        return Ok(None);
    }

    crate::handler::unused(args)?;

    let mut env_vars = BTreeMap::new();

    env_vars.extend(crate::validation::validate_tool(config, "clang++")?);
    env_vars.extend(crate::validation::validate_tool(config, "clangd")?);

    let toolchain = crate::config::rust::toolchain::nightly(config);

    crate::validation::validate_rust_toolchain(&toolchain)?;

    env_vars.extend(crate::validation::validate_tool(config, "cargo-clippy")?);
    env_vars.extend(crate::validation::validate_tool(config, "cargo-fmt")?);

    let (editor, editor_args) = crate::detection::detect_editor(args)?;
    let mut cmd = Command::new(&editor);
    cmd.current_dir(crate::cargo::project_root()?);
    cmd.args(editor_args);
    cmd.args(tool_args);
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    let status = cmd.status().map_err(|err| -> BoxError {
        if err.kind() == std::io::ErrorKind::NotFound {
            format!("editor `{editor}` not found in path").into()
        } else {
            err.into()
        }
    })?;

    Ok(Some(status))
}
