pub mod install;
pub mod uninstall;
pub mod update;
pub mod list;

use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dirs;

pub const DB_FILE: &str = ".local/share/eiipm/installed.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct PackageDB {
    packages: HashMap<String, InstalledPackage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstalledPackage {
    repo_path: String,
    installed_files: Vec<String>,
    copy_files: Vec<String>,
    pkg_type: String,
    build_command: Option<String>,
}

pub fn load_db() -> Result<PackageDB, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let db_path = home_dir.join(DB_FILE);
    if db_path.exists() {
        let content = fs::read_to_string(&db_path)?;
        let db: PackageDB = toml::from_str(&content)?;
        Ok(db)
    } else {
        Ok(PackageDB { packages: HashMap::new() })
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
