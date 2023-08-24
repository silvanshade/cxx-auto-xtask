use crate::{command::Context, BoxResult};
use std::{
    ffi::OsString,
    process::{Command, ExitStatus},
};

pub fn help() -> &'static str {
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
    help
}

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - Argument processing fails (e.g. invalid arguments)
/// - Tool validation fails (missing tools, incorrect versions, etc.)
/// - The command process fails to start
/// - The command invocation fails with non-zero exit status
pub fn clang(context: Context<'_>) -> BoxResult<Option<ExitStatus>> {
    let help = help();

    if crate::handler::help(context.args, help)? {
        return Ok(None);
    }

    let clang_subcommand: String = context.subcommand.ok_or("expected a subcommand for `xtask clang`")?;

    crate::handler::unused(context.args)?;

    let status = match &*clang_subcommand {
        "format" => {
            let run_clang_format_tool = &context.config.cmake_context.bin_run_clang_format;
            let mut cmd = Command::new(run_clang_format_tool);
            if !context.tool_args.contains(&OsString::from("--clang-format-executable")) {
                let clang_format_tool = context.config.cmake_context.bin_clang_format.as_str();
                cmd.args(["--clang-format-executable", clang_format_tool]);
            }
            cmd.args(context.tool_args);
            cmd.status()?
        },
        "tidy" => {
            {
                let mut cmd = Command::new("cargo");
                cmd.args(["check"]);
                let status = cmd.status()?;
                crate::handler::subcommand_result("cargo check", Ok(Some(status)));
            }
            // {
            //     let config = context.config;
            //     let mut args = pico_args::Arguments::from_vec(vec!["build".into()]);
            //     let tool_args = vec![];
            //     let context = Context::new(config, &mut args, tool_args);
            //     let result = crate::command::cmake(context);
            //     crate::handler::subcommand_result("cmake", result);
            // }
            let run_clang_tidy_tool = &context.config.cmake_context.bin_run_clang_tidy;
            let mut cmd = Command::new(run_clang_tidy_tool);
            if !context.tool_args.contains(&OsString::from("-clang-tidy-binary")) {
                let clang_tidy_tool = context.config.cmake_context.bin_clang_tidy.as_str();
                cmd.args(["-clang-tidy-binary", clang_tidy_tool]);
            }
            cmd.args(context.tool_args);
            cmd.status()?
        },
        _ => {
            println!("{help}\n");
            return Err(format!("unrecognized `xtask clang` subcommand `{clang_subcommand}`").into());
        },
    };

    Ok(Some(status))
}
