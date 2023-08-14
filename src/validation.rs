use crate::{config::Config, BoxError, BoxResult};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    process::Command,
};

pub fn validate_tool(config: &Config, tool: &str) -> BoxResult<BTreeMap<OsString, OsString>> {
    let mut env_vars = BTreeMap::default();
    match tool {
        "clang" => {
            env_vars.extend(validate_clang_tool(config, tool)?);
        },
        "clang++" => {
            env_vars.extend(validate_clang_tool(config, tool)?);
        },
        "clangd" => {
            env_vars.extend(validate_clang_tool(config, tool)?);
        },
        "clang-format" => {
            env_vars.extend(validate_clang_tool(config, tool)?);
            validate_python3()?;
            validate_xtask_bin(config, "run-clang-format.py", true)?;
        },
        "clang-tidy" => {
            env_vars.extend(validate_clang_tool(config, tool)?);
            env_vars.extend(validate_other_tool("run-clang-tidy", &env_vars)?);
        },
        "cargo-clippy" => {
            validate_cargo_component(config, tool)?;
        },
        "cargo-fmt" => {
            validate_cargo_component(config, tool)?;
        },
        "cargo-miri" => {
            validate_cargo_component(config, tool)?;
        },
        "cargo-tarpaulin" => {
            validate_cargo_tool(tool)?;
        },
        "cargo-udeps" => {
            validate_cargo_tool(tool)?;
        },
        "cargo-valgrind" => {
            validate_cargo_tool(tool)?;
        },
        "cmake" => {
            validate_other_tool(tool, &env_vars)?;
        },
        "ninja" => {
            validate_other_tool(tool, &env_vars)?;
        },
        _ => {
            return Err(format!("unrecognized tool: `{tool}`").into());
        },
    }
    Ok(env_vars)
}

fn validate_clang_tool(config: &Config, tool: &str) -> BoxResult<BTreeMap<OsString, OsString>> {
    fn check_tool(
        config: &Config,
        matcher: &Regex,
        tool: &str,
        env_vars: BTreeMap<OsString, OsString>,
    ) -> BoxResult<BTreeMap<OsString, OsString>> {
        // check `tool` with no suffix
        let mut cmd = Command::new(tool);
        cmd.arg("--version");
        for (key, value) in env_vars.iter() {
            cmd.env(key, value);
        }
        let output = cmd.output()?;
        if output.status.success() {
            let haystack = String::from_utf8(output.stdout)?;
            if let Some(version) = matcher
                .captures(&haystack)
                .and_then(|captures| captures.get(1).map(|m| m.as_str()))
            {
                if version.starts_with(&config.xtask.clang.version) {
                    return Ok(env_vars);
                } else {
                    return Err(format!(
                        "`{tool}` failed validation; expected version compatible with `{version}` but found `{actual}`",
                        version = config.xtask.clang.version,
                        actual = version,
                    )
                    .into());
                }
            }
            if matcher.is_match(&haystack) {
                Ok(env_vars)
            } else {
                Err(format!("`{tool}` failed validation; ensure you are using the official clang toolchain").into())
            }
        } else {
            Err(format!("`{tool}` failed with non-zero exit code").into())
        }
    }

    if let Some(matcher) = config.xtask.clang.matchers.get(tool) {
        let matcher = regex::Regex::new(matcher)?;
        return
            // check `tool` with no suffix
            check_tool(config, &matcher, tool, BTreeMap::default())

            // check `tool` with suffix
            .or_else(|_err| {
                let tool = &format!("{}-{}", tool, config.xtask.clang.suffix);
                let env = BTreeMap::default();
                check_tool(config, &matcher, tool, env)
            })

            // check `tool` in search paths
            .or_else(|_err| {
                #[allow(unused_mut)]
                let mut paths = if let Some(path) = std::env::var_os("PATH") {
                    std::env::split_paths(&path).collect::<BTreeSet<_>>()
                } else {
                    BTreeSet::default()
                };

                // on macOS, add the homebrew install location to the PATH for a final test
                #[cfg(target_os = "macos")]
                paths.extend(crate::detection::detect_macos_clang_paths(config)?);

                std::env::join_paths(paths)
                    .map_err(Into::into)
                    .and_then(|path| {
                        let env = BTreeMap::from_iter([("PATH".into(), path)]);
                        check_tool(config, &matcher, tool, env)
                    })
            });
        // check if any of the above succeeded
    }

    Err(format!("could not find matcher for tool: `{tool}`").into())
}

fn validate_cargo_component(config: &Config, tool: &str) -> BoxResult<()> {
    let tool = tool.strip_prefix("cargo-").unwrap_or(tool);
    let component_name = if tool == "doc" { "rustdoc" } else { tool };
    if let Some(value) = config.xtask.rust.components.get(component_name) {
        let toolchain = if value.toolchain == "nightly" {
            crate::config::rust::toolchain::nightly(config)
        } else {
            crate::config::rust::toolchain::stable(config)
        };
        let mut cmd = Command::new("cargo");
        cmd.args([&format!("+{toolchain}"), tool]);
        cmd.args(["--help"]);
        cmd.stdout(std::process::Stdio::null());
        if !cmd.status()?.success() {
            return Err(format!("`cargo +{toolchain} {tool} --help` failed with non-zero exit code").into());
        }
    } else {
        return Err(format!("unrecognized component: `{tool}`").into());
    }
    Ok(())
}

fn validate_cargo_tool(tool: &str) -> BoxResult<()> {
    let mut cmd = Command::new(tool);
    cmd.args(["--help"]);
    cmd.stdout(std::process::Stdio::null());
    let status = cmd.status().map_err(|err| -> BoxError {
        if err.kind() == std::io::ErrorKind::NotFound {
            format!("could not find `{tool}` in path").into()
        } else {
            err.into()
        }
    })?;
    if !status.success() {
        return Err(format!("`{tool}` failed with non-zero exit code").into());
    }
    Ok(())
}

fn validate_other_tool(tool: &str, env_vars: &BTreeMap<OsString, OsString>) -> BoxResult<BTreeMap<OsString, OsString>> {
    let mut cmd = Command::new(tool);
    match tool {
        "ninja" => cmd.args(["--version"]),
        _ => cmd.args(["--help"]),
    };
    cmd.stderr(std::process::Stdio::null());
    cmd.stdout(std::process::Stdio::null());
    for (key, value) in env_vars.iter() {
        cmd.env(key, value);
    }
    let status = cmd.status().map_err(|err| -> BoxError {
        if err.kind() == std::io::ErrorKind::NotFound {
            format!("could not find `{tool}` in path").into()
        } else {
            err.into()
        }
    })?;
    if !status.success() {
        return Err(format!("`{tool}` failed with non-zero exit code").into());
    }
    Ok(BTreeMap::default())
}

fn validate_python3() -> BoxResult<()> {
    let mut cmd = Command::new("python3");
    cmd.args(["--help"]);
    cmd.stdout(std::process::Stdio::null());
    let status = cmd.status().map_err(|err| -> BoxError {
        if err.kind() == std::io::ErrorKind::NotFound {
            format!("could not find `python3` in path").into()
        } else {
            err.into()
        }
    })?;
    if !status.success() {
        return Err("`python3` failed with non-zero exit code".into());
    }
    Ok(())
}

pub fn validate_rust_toolchain(toolchain: &str) -> BoxResult<()> {
    let mut cmd = Command::new("rustup");
    cmd.args(["toolchain", "list"]);
    let output = cmd.output()?;
    if output.status.success() {
        for entry in String::from_utf8(output.stdout)?.lines() {
            if entry.starts_with(toolchain) {
                return Ok(());
            }
        }
    } else {
        return Err("`rustup toolchain list` failed with non-zero exit code".into());
    }
    Err(format!(
        "could not find toolchain `{}`\nPerhaps you need to install it with `rustup install toolchain {}`?",
        toolchain, toolchain
    )
    .into())
}

fn validate_xtask_bin(config: &Config, tool: &str, retry: bool) -> BoxResult<()> {
    let needs_install = match tool {
        "run-clang-format.py" => {
            let tool = config.xtask_bin_dir.join(tool);
            let mut cmd = Command::new("python3");
            cmd.args([tool.as_os_str(), "--help".as_ref()]);
            cmd.stderr(std::process::Stdio::null());
            cmd.stdout(std::process::Stdio::null());
            if !cmd.status()?.success() {
                // return Err(format!("`python3 {} --help` failed with non-zero exit code", tool.display()).into());
                Ok::<_, BoxError>(Some(
                    "https://raw.githubusercontent.com/Sarcasm/run-clang-format/master/run-clang-format.py",
                ))
            } else {
                Ok::<_, BoxError>(None)
            }
        },
        _ => {
            return Err(format!("unrecognized xtask bin `{tool}`").into());
        },
    }?;
    if let Some(url) = needs_install {
        crate::install::fetch_xtask_bin(config, url, tool)?;
        if retry {
            validate_xtask_bin(config, tool, false)?;
        }
    }
    Ok(())
}
