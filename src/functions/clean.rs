use anyhow::{Context, Result};
use colored::Colorize;
use dirs::cache_dir;
use std::fs;
use std::path::Path;
use crate::utils;

pub fn clean_cache() -> Result<()> {
    let cache_root = cache_dir()
        .context("could not resolve cache directory")?
        .join("eiipm");

    if !cache_root.exists() {
        log::info!("{}", "cache is already empty".dimmed());
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(&cache_root)
        .context("failed to read cache dir")?
        .filter_map(|e| e.ok())
        .collect();

    if entries.is_empty() {
        log::info!("{}", "cache is already empty".dimmed());
        return Ok(());
    }

    log::info!("this will delete {}:", cache_root.display().to_string().cyan());
    for entry in &entries {
        log::info!("  {}", entry.file_name().to_string_lossy().dimmed());
    }

    if !utils::confirm("delete the entire eiipm cache?") {
        log::info!("{}", "aborted".dimmed());
        return Ok(());
    }

    fs::remove_dir_all(&cache_root)
        .context("failed to delete cache directory")?;

    log::info!("{} cache cleared", "done!".green().bold());
    Ok(())
}

pub fn clean_plugins() -> Result<()> {
    let plugins_dir = Path::new("plugins");
    let lock_path = Path::new("plugins.lock");

    if !plugins_dir.exists() {
        log::info!("{}", "plugins/ does not exist, nothing to clean".dimmed());
        return Ok(());
    }

    let on_disk: Vec<_> = fs::read_dir(plugins_dir)
        .context("failed to read plugins/ dir")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "so"))
        .collect();

    if on_disk.is_empty() {
        log::info!("{}", "plugins/ is already empty".dimmed());
        return Ok(());
    }

    let tracked: Vec<String> = if lock_path.exists() {
        let contents = fs::read_to_string(lock_path).context("failed to read plugins.lock")?;
        let lock: crate::schema::LockFile = toml::from_str(&contents)
            .context("failed to parse plugins.lock")?;
        lock.plugin.into_iter().map(|p| p.artifact).collect()
    } else {
        vec![]
    };

    let untracked: Vec<_> = on_disk
        .iter()
        .filter(|e| {
            let path = e.path().to_string_lossy().to_string();
            !tracked.contains(&path)
        })
        .collect();

    if untracked.is_empty() {
        log::info!("{}", "nothing to clean, all artifacts are tracked".dimmed());
        return Ok(());
    }

    log::info!("untracked artifacts:");
    for entry in &untracked {
        log::info!("  {}", entry.path().display().to_string().dimmed());
    }

    if !utils::confirm("delete untracked artifacts?") {
        log::info!("{}", "aborted".dimmed());
        return Ok(());
    }

    for entry in untracked {
        fs::remove_file(entry.path())
            .with_context(|| format!("failed to delete {}", entry.path().display()))?;
    }

    log::info!("{} plugins/ cleaned", "done!".green().bold());
    Ok(())
}
