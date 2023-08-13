use crate::BoxResult;
use std::{path::PathBuf, process::Command};

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
        .flat_map(|val| val.as_str().map(PathBuf::from))
        .next()
        .ok_or("`workspace_root` not found in `cargo metadata` output")?;
    Ok(path)
}
