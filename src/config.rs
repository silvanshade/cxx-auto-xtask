use crate::{BoxError, BoxResult};
use camino::Utf8PathBuf;
use serde::Deserialize;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CMakeContext {
    pub bin_clang_format: Utf8PathBuf,
    pub bin_clang_tidy: Utf8PathBuf,
    pub bin_run_clang_format: Utf8PathBuf,
    pub bin_run_clang_tidy: Utf8PathBuf,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Deserialize)]
pub struct RustToolchain {
    pub toolchain: RustToolchainToolchain,
}

#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Deserialize)]
pub struct RustToolchainToolchain {
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Utf8PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
}

pub struct Config {
    pub cmake_context: CMakeContext,
    pub cargo_metadata: cargo_metadata::Metadata,
    pub rust_toolchain: RustToolchain,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` under the following circumstances:
    /// - `cargo metadata` fails
    /// - Reading the `rust-toolchain.toml` file as text fails
    pub fn load() -> BoxResult<Self> {
        let cargo_metadata = cargo_metadata::MetadataCommand::new().exec()?;
        let cmake_context = {
            let path = cargo_metadata.workspace_root.join("build/cxx-auto-context.json");
            let data = std::fs::read_to_string(&path).map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    format!("Path not found: {}", path.as_std_path().display()).into()
                } else {
                    BoxError::from(err)
                }
            })?;
            serde_json::from_str(&data)?
        };
        let rust_toolchain: RustToolchain = {
            let path = cargo_metadata.workspace_root.join("rust-toolchain.toml");
            let data = std::fs::read_to_string(&path).map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    format!("Path not found: {}", path.as_std_path().display()).into()
                } else {
                    BoxError::from(err)
                }
            })?;
            toml::from_str(&data)?
        };
        Ok(Config {
            cmake_context,
            cargo_metadata,
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
