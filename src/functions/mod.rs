pub mod checkupdate;
pub mod clearcache;
pub mod install;
pub mod list;
pub mod listcache;
pub mod purgecache;
pub mod search;
pub mod uninstall;
pub mod update;

use dirs;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub const DB_FILE: &str = ".local/share/eiipm/installed.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct PackageDB {
    packages: HashMap<String, InstalledPackage>,
}

/// Metadata structs
#[derive(Deserialize, Debug)]
pub struct PackageRootMeta {
    metadata: PackageMeta,
}

#[derive(Deserialize, Debug)]
pub struct PackageMeta {
    name: String,
    #[serde(rename = "type")]
    pkg_type: String,
    src: String,
    #[serde(rename = "commit")]
    commit_hash: String, // hash of the commit to install
    files: Vec<FileEntry>,
    build: Option<String>, // Optional build command
}

// Wait there dev!
// if you add a new value to InstalledPackage, eiipm will break
// no... no... eiipm wont break, but old db's that use the old
// struct will break... So, remember to add `#[serde(default)]`.
// #[serde(default)] is our lord and savior if we need to add a new value.
#[derive(Deserialize, Serialize, Debug)]
pub struct InstalledPackage {
    repo_fs_path: String, // path to cached repo. E.g. ~/.eiipm/cache/<REPO_NAME>
    installed_files: Vec<String>,
    copy_files: Vec<FileEntry>,
    pkg_type: String,
    upstream_src: String,
    installed_hash: String,
    manifest_url: String,
    build_command: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum FileEntry {
    Flat(String),
    Detailed { src: String, dest: Option<String> },
}

pub fn load_db() -> Result<PackageDB, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let db_path = home_dir.join(DB_FILE);
    if db_path.exists() {
        let content = fs::read_to_string(&db_path)?;
        let db: PackageDB = toml::from_str(&content)?;
        Ok(db)
    } else {
        Ok(PackageDB {
            packages: HashMap::new(),
        })
    }
}

pub fn save_db(db: &PackageDB) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let db_path = home_dir.join(DB_FILE);
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(db)?;
    fs::write(db_path, content)?;
    Ok(())
}

// Sending requests to url's
pub fn http_get_string(url: &str) -> Result<String, Box<dyn Error>> {
    log::debug!("Sending GET request to {}", url);
    let response = get(url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL {}: HTTP {}", url, response.status()).into());
    }
    Ok(response.text()?)
}

pub fn is_update_needed_for(package_name: &str) -> Result<(bool, String), Box<dyn Error>> {
    let mut db = load_db()?;

    if let Some(pkg) = db.packages.get_mut(package_name) {
        let upstream_manifest_raw = http_get_string(&pkg.manifest_url)?;
        let root_manifest: PackageRootMeta = toml::from_str(&upstream_manifest_raw)?;
        let upstream_manifest = root_manifest.metadata;

        Ok((
            upstream_manifest.commit_hash != pkg.installed_hash,
            upstream_manifest.commit_hash,
        ))
    } else {
        Err(format!("Package `{}` not found in DB", package_name).into())
    }
}
