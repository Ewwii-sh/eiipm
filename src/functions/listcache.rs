use log::{error, info};
use std::fs;

pub fn list_all_cache() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let cache_root = home_dir.join(".eiipm/cache");

    if !cache_root.is_dir() {
        error!("Cache directory not found.");
        return Ok(());
    }

    info!("Directories in cache:");
    for entry in fs::read_dir(cache_root)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            info!("  {}", entry_path.display());
        }
    }
    Ok(())
}
