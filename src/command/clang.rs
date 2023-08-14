use crate::{command::Context, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn clang(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
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

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    let clang_subcommand: String = context
        .subcommand
        .ok_or_else(|| "expected a subcommand for `xtask clang`")?;

    crate::handler::unused(context.args)?;

    let validation = crate::validation::validate_tool(context.config, &format!("clang-{clang_subcommand}"))?;

    let status = match &*clang_subcommand {
        "format" => {
            let tool = context.config.xtask_bin_dir.join("run-clang-format.py");
            let mut cmd = Command::new("python3");
            cmd.args([tool.as_os_str()]);
            if !context.tool_args.contains(&OsString::from("--clang-format-executable")) {
                let clang_format_tool = validation
                    .tools
                    .get("clang-format")
                    .ok_or_else(|| "`clang-format` not found")?;
                cmd.args(["--clang-format-executable", clang_format_tool]);
            }
            cmd.args(context.tool_args);
            for (key, value) in validation.env_vars {
                cmd.env(key, value);
            }
            cmd.status()?
        },
        "tidy" => {
            {
                let config = context.config;
                let mut args = pico_args::Arguments::from_vec(vec!["build".into()]);
                let tool_args = vec![];
                let context = Context::new(config, &mut args, tool_args);
                let result = crate::command::cmake(context);
                crate::handler::subcommand_result("cmake", result);
            }
            let run_clang_tidy_tool = validation
                .tools
                .get("run-clang-tidy")
                .ok_or_else(|| "`run-clang-tidy` not found")?;
            let mut cmd = Command::new(run_clang_tidy_tool);
            if !context.tool_args.contains(&OsString::from("-clang-tidy-binary")) {
                let clang_tidy_tool = validation
                    .tools
                    .get("clang-tidy")
                    .ok_or_else(|| "`clang-tidy` not found")?;
                cmd.args(["-clang-tidy-binary", clang_tidy_tool]);
            }
            cmd.args(context.tool_args);
            for (key, value) in validation.env_vars {
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
