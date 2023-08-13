use crate::{config::Config, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn clang(
    config: &Config,
    args: &mut pico_args::Arguments,
    tool_args: Vec<OsString>,
) -> BoxResult<Option<ExitStatus>> {
    let help = r#"
xtask-clang

USAGE:
xtask clang [SUBCOMMAND]

FLAGS:
-h, --help          Prints help information
-- '...'            Extra arguments to pass to the clang subcommand

SUBCOMMANDS:
    format          Run run-clang-format.py on the project's C++ code
                    Use `-- --help` to see the usage for run-clang-format.py
    tidy            Run run-clang-tidy      on the project's C++ code
                    Use `-- --help` to see the usage for run-clang-tidy
"#
    .trim();

    if crate::handler::help(args, help)? {
        return Ok(None);
    }

    let clang_subcommand: String = args.free_from_str()?;

    crate::handler::unused(args)?;

    let env_vars = crate::validation::validate_tool(config, &format!("clang-{clang_subcommand}"))?;

    let status = match &*clang_subcommand {
        "format" => {
            let tool = config.xtask_bin_dir.join("run-clang-format.py");
            let mut cmd = Command::new("python3");
            cmd.args([tool.as_os_str()]);
            cmd.args(tool_args);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        "tidy" => {
            let mut cmd = Command::new("run-clang-tidy");
            cmd.args(tool_args);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        _ => {
            println!("{help}\n");
            return Err(format!("unrecognized clang subcommand `{clang_subcommand}`").into());
        },
    };

    Ok(Some(status))
}
