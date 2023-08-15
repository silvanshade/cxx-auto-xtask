use crate::BoxResult;
use std::process::ExitStatus;

/// # Errors
///
/// Will return `Err` if argument processing fails.
pub fn help(args: &mut pico_args::Arguments, help: &str) -> BoxResult<bool> {
    if args.contains(["-h", "--help"]) {
        println!("{help}");
        while args.opt_free_from_str::<String>()?.is_some() {}
        return Ok(true);
    }
    Ok(false)
}

pub fn result<T>(result: BoxResult<T>) {
    if let Err(err) = result {
        println!("error: {err}");
        let code = 1;
        std::process::exit(code);
    }
}

pub fn subcommand_result(subcommand: &str, result: BoxResult<Option<ExitStatus>>) {
    match result {
        Ok(None) => {},
        Ok(Some(status)) => {
            if !status.success() {
                println!("error: subcommand `{subcommand}` failed with non-zero exit code");
                let code = status.code().unwrap_or(1);
                std::process::exit(code);
            }
        },
        result => crate::handler::result(result),
    }
}

/// # Errors
///
/// Will return `Err` if unused arguments remain in `args`.
pub fn unused(args: &pico_args::Arguments) -> BoxResult<()> {
    use std::borrow::Borrow;
    let unused = args.clone().finish();
    if unused.is_empty() {
        return Ok(());
    }
    let mut message = String::new();
    for str in unused {
        message.push(' ');
        message.push_str(str.to_string_lossy().borrow());
    }
    Err(format!("unrecognized arguments `{message}`").into())
}
