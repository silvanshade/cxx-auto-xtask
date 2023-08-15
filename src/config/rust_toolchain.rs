use serde::Deserialize;

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
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
}
