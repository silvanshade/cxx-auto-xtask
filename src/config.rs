mod rust_toolchain;
mod xtask;

use std::path::{Path, PathBuf};

pub use rust_toolchain::{RustToolchain, RustToolchainToolchain};
pub use xtask::{
    Xtask,
    XtaskClang,
    XtaskPlatform,
    XtaskPlatformMacos,
    XtaskPlatformMacosSearchPath,
    XtaskRust,
    XtaskRustComponent,
    XtaskRustToolchain,
};

use crate::BoxResult;

pub struct Config {
    pub project_root_dir: PathBuf,
    pub xtask_dir: PathBuf,
    pub xtask_bin_dir: PathBuf,
    pub xtask: Xtask,
    pub rust_toolchain: RustToolchain,
}

impl Config {
    pub fn load(project_root_dir: &Path) -> BoxResult<Self> {
        crate::install::create_xtask_bin_dir(project_root_dir)?;
        let xtask_dir = project_root_dir.join(".xtask");
        let xtask_bin_dir = xtask_dir.join("bin");
        let rust_toolchain: RustToolchain = {
            let path = project_root_dir.join("rust-toolchain.toml");
            let data = std::fs::read_to_string(path)?;
            toml::from_str(&data)?
        };
        let xtask: Xtask = {
            let path = project_root_dir.join("xtask.toml");
            let data = std::fs::read_to_string(path)?;
            toml::from_str(&data)?
        };
        if &format!("nightly-{}", xtask.rust.toolchain.nightly) != &rust_toolchain.toolchain.channel {
            return Err(format!(
                "<xtask.toml>.rust.toolchain.nightly ({}) != <rust_toolchain.toml>.toolchain.channel ({})",
                xtask.rust.toolchain.nightly, rust_toolchain.toolchain.channel,
            )
            .into());
        }
        Ok(Config {
            project_root_dir: project_root_dir.to_path_buf(),
            xtask_dir,
            xtask_bin_dir,
            xtask,
            rust_toolchain,
        })
    }
}

pub mod rust {
    pub mod toolchain {
        use crate::config::Config;

        pub fn stable(_config: &Config) -> &str {
            "stable"
        }

        pub fn nightly(config: &Config) -> &str {
            &config.rust_toolchain.toolchain.channel
        }
    }
}
