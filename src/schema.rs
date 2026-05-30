use serde::{Serialize, Deserialize};
use indexmap::IndexMap;

// plugins.toml schema

#[derive(Deserialize, Serialize)]
pub struct PluginsFile {
    pub plugins: IndexMap<String, PluginEntry>
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum PluginEntry {
    Ref(String),
    Config(PluginConfig),
}

#[derive(Deserialize, Serialize)]
pub struct PluginConfig {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub build: Option<String>,
    pub artifact: Option<String>,
    pub prebuilt: Option<bool>,
}

// plugins.lock schema

#[derive(Deserialize, Serialize)]
pub struct LockFile {
    pub version: u32,
    #[serde(default)]
    pub plugin: Vec<LockedPlugin>,
}

#[derive(Deserialize, Serialize)]
pub struct LockedPlugin {
    pub repo: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
    pub artifact: String,
    pub built_at: String,
}

// plugin.toml schema

#[derive(Deserialize)]
pub struct PluginManifest {
    pub plugin: PluginManifestInner,
}

#[derive(Deserialize)]
pub struct PluginManifestInner {
    pub build: Option<String>,
    pub artifact: Option<String>,
    pub prebuilt: Option<PrebuiltConfig>,
}

#[derive(Deserialize)]
pub struct PrebuiltConfig {
    /// Direct URL to the binary, supports {version} and {arch} placeholders
    /// e.g. "https://github.com/user/repo/releases/download/{version}/libwidget-{arch}.so"
    pub url: String,
}
