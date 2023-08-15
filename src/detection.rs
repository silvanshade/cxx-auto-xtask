use crate::BoxResult;
use std::process::Command;

fn split_editor_command(editor: &str) -> BoxResult<(String, Vec<String>)> {
    let mut words = editor.split_whitespace();
    if let Some(editor) = words.next() {
        let editor = editor.into();
        let editor_args = words
            .filter_map(|word| if word == "--wait" { None } else { Some(word.into()) })
            .collect();
        return Ok((editor, editor_args));
    }
    Err("editor command must not be empty".into())
}

/// # Errors
///
/// Will return `Err` if the editor cannot be detected from the environment.
pub fn detect_editor(args: &mut pico_args::Arguments) -> BoxResult<(String, Vec<String>)> {
    if let Ok(editor) = args.free_from_str::<String>() {
        return split_editor_command(&editor);
    }
    if let Some(editor) = detect_git_core_editor() {
        return split_editor_command(&editor);
    }
    if let Ok(editor) = std::env::var("VISUAL") {
        return split_editor_command(&editor);
    }
    if let Ok(editor) = std::env::var("EDITOR") {
        return split_editor_command(&editor);
    }
    Err("could not determine editor\nSpecify <editor> command or set $VISUAL or $EDITOR".into())
}

fn detect_git_core_editor() -> Option<String> {
    let mut cmd = Command::new("git");
    cmd.args(["config", "--get", "core.editor"]);
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            if let Ok(editor) = String::from_utf8(output.stdout) {
                return Some(editor);
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
use crate::config::{Config, XtaskPlatformMacosSearchPath};
#[cfg(target_os = "macos")]
use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub fn detect_macos_clang_paths(config: &Config) -> BoxResult<impl Iterator<Item = PathBuf>> {
    let version = config.xtask.clang.version.as_str();
    let major_version = version.split('.').next().unwrap_or(version);
    let mut paths = vec![];
    for entry in &config.xtask.clang.platform.macos.search_paths {
        match entry {
            XtaskPlatformMacosSearchPath::Homebrew => {
                let formula = format!("llvm@{major_version}");
                if let Some(prefix) = detect_homebrew_prefix(&formula)? {
                    if let Some(parent) = prefix.parent() {
                        let path = parent.join(formula).join("bin");
                        paths.push(path);
                    }
                }
            },
        }
    }
    Ok(paths.into_iter())
}

#[cfg(target_os = "macos")]
fn detect_homebrew_prefix(formula: &str) -> BoxResult<Option<PathBuf>> {
    let mut cmd = Command::new("brew");
    cmd.args(["--prefix", formula]);
    let output = cmd.output()?;
    if output.status.success() {
        if let Ok(prefix) = String::from_utf8(output.stdout) {
            return Ok(Some(PathBuf::from(prefix.trim())));
        }
    }
    Ok(None)
}
