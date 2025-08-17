use colored::Colorize;
use log::{info, error};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use super::load_db;

pub fn purge_cache() -> Result<(), Box<dyn std::error::Error>> {
    let db = load_db()?; 
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let cache_root = home_dir.join(".eiipm/cache");

    if !cache_root.exists() {
        info!("Cache directory does not exist. Nothing to purge.");
        return Ok(());
    }

    let db_repos: HashSet<_> = db
        .packages
        .values()
        .map(|pkg| {
            Path::new(&pkg.repo_path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();

    let mut orphan_count = 0;

    for entry in fs::read_dir(&cache_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
            if !db_repos.contains(&dir_name) {
                orphan_count += 1;
                info!("Removing orphaned cache directory: {}", path.display());
                if let Err(e) = fs::remove_dir_all(&path) {
                    error!("Failed to remove {}: {}", path.display(), e);
                }
            }
        }
    }

    if orphan_count == 0 {
        info!("{}", "Cache is clean. Nothing to remove.".green());
    } else {
        info!("{}", format!("\nRemoved {} orphaned cache director{}", orphan_count, if orphan_count == 1 { "y" } else { "ies" }).yellow());
    }

    Ok(())
}
