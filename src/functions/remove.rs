use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;
use crate::schema::{LockFile, PluginsFile};
use crate::utils;

pub fn remove_plugin(plugin: String) -> Result<()> {
    let toml_path = Path::new("plugins.toml");
    let lock_path = Path::new("plugins.lock");

    if !toml_path.exists() {
        bail!("plugins.toml not found, run 'eiipm init' first");
    }

    // Parse plugins.toml
    let toml_contents = fs::read_to_string(toml_path).context("failed to read plugins.toml")?;
    let mut file: PluginsFile = toml::from_str(&toml_contents).context("failed to parse plugins.toml")?;

    if !file.plugins.contains_key(&plugin) {
        bail!("'{}' is not in plugins.toml", plugin);
    }

    let mut lock: Option<LockFile> = if lock_path.exists() {
        let lock_contents = fs::read_to_string(lock_path).context("failed to read plugins.lock")?;
        Some(toml::from_str(&lock_contents).context("failed to parse plugins.lock")?)
    } else {
        None
    };

    // Find artifact path from lockfile before we remove the entry
    let artifact = lock.as_ref()
        .and_then(|l| l.plugin.iter().find(|p| p.repo == plugin))
        .map(|p| p.artifact.clone());

    // Confirm
    if let Some(ref path) = artifact {
        log::info!("this will remove {} and delete {}", plugin.cyan(), path.dimmed());
    } else {
        log::info!("this will remove {} from plugins.toml (no installed artifact found)", plugin.cyan());
    }

    if !utils::confirm("continue?") {
        log::info!("{}", "aborted".dimmed());
        return Ok(());
    }

    // Remove from plugins.toml
    file.plugins.shift_remove(&plugin);
    let updated_toml = toml::to_string_pretty(&file).context("failed to serialize plugins.toml")?;
    fs::write(toml_path, updated_toml).context("failed to write plugins.toml")?;
    log::info!("{} {} from plugins.toml", "removed".green().bold(), plugin);

    // Remove artifact from plugins/
    if let Some(ref path) = artifact {
        let artifact_path = Path::new(path);
        if artifact_path.exists() {
            fs::remove_file(artifact_path)
                .with_context(|| format!("failed to delete artifact {}", path))?;
            log::info!("{} {}", "deleted".green().bold(), path.dimmed());
        }
    }

    // Remove from lockfile
    if let Some(ref mut l) = lock {
        l.plugin.retain(|p| p.repo != plugin);
        let updated_lock = toml::to_string_pretty(l).context("failed to serialize lockfile")?;
        fs::write(lock_path, updated_lock).context("failed to write plugins.lock")?;
        log::info!("{} {} from plugins.lock", "removed".green().bold(), plugin);
    }

    Ok(())
}
