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

use crate::{BoxError, BoxResult};

pub struct Config {
    pub project_root_dir: PathBuf,
    pub xtask_dir: PathBuf,
    pub xtask_bin_dir: PathBuf,
    pub xtask: Xtask,
    pub rust_toolchain: RustToolchain,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` under the following circumstances:
    /// - Creating the `.xtask/bin` directory fails
    /// - Reading the `rust-toolchaim.toml` file as text fails
    /// - Converting the `rust-toolchain.toml` file text to TOML after loading fails
    /// - Reading the `xtask.toml` file as text fails
    /// - Converting the `xtask.toml` file text to TOML after loading fails
    /// - The `xtask.rust.toolchain.nightly` and `rust_toolchain.toolchain.channel` values disagree
    pub fn load(project_root_dir: &Path) -> BoxResult<Self> {
        crate::install::create_xtask_bin_dir(project_root_dir)?;
        let xtask_dir = project_root_dir.join(".xtask");
        let xtask_bin_dir = xtask_dir.join("bin");
        let rust_toolchain: RustToolchain = {
            let path = project_root_dir.join("rust-toolchain.toml");
            let data = std::fs::read_to_string(&path).map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    format!("path not found: {}", path.display()).into()
                } else {
                    BoxError::from(err)
                }
            })?;
            toml::from_str(&data)?
        };
        let xtask: Xtask = {
            let path = project_root_dir.join("xtask.toml");
            let data = std::fs::read_to_string(&path).map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    format!("path not found: {}", path.display()).into()
                } else {
                    BoxError::from(err)
                }
            })?;
            toml::from_str(&data)?
        };
        if format!("nightly-{}", xtask.rust.toolchain.nightly) != rust_toolchain.toolchain.channel {
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

        #[must_use]
        pub fn stable(_config: &Config) -> &str {
            "stable"
        }

        #[must_use]
        pub fn nightly(config: &Config) -> &str {
            &config.rust_toolchain.toolchain.channel
        }
    }
}
