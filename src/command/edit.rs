use crate::{command::Context, BoxError, BoxResult};
use std::process::{Command, ExitStatus};

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - Argument processing fails (e.g. invalid arguments)
/// - Tool validation fails (missing tools, incorrect versions, etc.)
/// - The command process fails to start
/// - The command invocation fails with non-zero exit status
pub fn edit(context: Context<'_>, editor: &str, editor_args: Vec<String>) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-edit

USAGE:
xtask edit [<editor>] [-- <...>]

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the editor command
"#
    .trim();

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    crate::handler::unused(context.args)?;

    let mut validation = crate::validation::Validation::default();

    validation.combine(crate::validation::validate_tool(context.config, "clang++")?);
    validation.combine(crate::validation::validate_tool(context.config, "clangd")?);

    let toolchain = crate::config::rust::toolchain::nightly(context.config);

    crate::validation::validate_rust_toolchain(toolchain)?;

    validation.combine(crate::validation::validate_tool(context.config, "cargo-clippy")?);
    validation.combine(crate::validation::validate_tool(context.config, "cargo-fmt")?);

    let mut cmd = Command::new(editor);
    cmd.current_dir(crate::workspace::project_root()?);
    cmd.args(editor_args);
    cmd.args(context.tool_args);
    for (key, value) in validation.env_vars {
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
