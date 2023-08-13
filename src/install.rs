use crate::{config::Config, BoxResult};
use std::{os::unix::prelude::PermissionsExt, path::Path};

pub(crate) fn create_xtask_bin_dir(project_root_dir: &Path) -> BoxResult<()> {
    let path = project_root_dir.join(".xtask/bin");
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub(crate) fn fetch_xtask_bin(config: &Config, url: &str, tool: &str) -> BoxResult<()> {
    let tool = config.xtask_bin_dir.join(tool);
    {
        let mut reader = ureq::get(url).call()?.into_reader();
        let mut writer = std::fs::File::create(&tool)?;
        std::io::copy(&mut reader, &mut writer)?;
    }
    let mut permissions = std::fs::metadata(&tool)?.permissions();
    permissions.set_mode(0o744);
    Ok(())
}
