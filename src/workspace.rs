use crate::BoxResult;
use std::{path::PathBuf, process::Command};

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - The command process for `cargo metadata --format-version=1` fails to start
/// - The command invocation fails with non-zero exit status
/// - The command invocation fails to produce valid UTF-8 output
/// - The command invocation fails to produce valid JSON output
/// - `workspace_root` is not found in the JSON output
pub fn project_root() -> BoxResult<PathBuf> {
    let data = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()?;
    if !data.status.success() {
        let err = String::from_utf8(data.stderr)?;
        return Err(format!("`cargo metadata` failed: \"{err}\"").into());
    }
    let json = serde_json::from_str::<serde_json::Value>(&String::from_utf8(data.stdout)?)?;
    let path = json
        .get("workspace_root")
        .iter()
        .find_map(|val| val.as_str().map(PathBuf::from))
        .ok_or("`workspace_root` not found in `cargo metadata` output")?;
    Ok(path)
}
