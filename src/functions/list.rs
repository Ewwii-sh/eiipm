use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;
use crate::schema::{LockFile, PluginsFile};

pub fn list_plugins() -> Result<()> {
    let toml_path = Path::new("plugins.toml");
    let lock_path = Path::new("plugins.lock");

    if !toml_path.exists() {
        bail!("plugins.toml not found, run 'eiipm init' first");
    }

    let toml_contents = fs::read_to_string(toml_path).context("failed to read plugins.toml")?;
    let file: PluginsFile = toml::from_str(&toml_contents).context("failed to parse plugins.toml")?;

    if file.plugins.is_empty() {
        log::info!("{}", "no plugins declared in plugins.toml".dimmed());
        return Ok(());
    }

    let lock: Option<LockFile> = if lock_path.exists() {
        let lock_contents = fs::read_to_string(lock_path).context("failed to read plugins.lock")?;
        Some(toml::from_str(&lock_contents).context("failed to parse plugins.lock")?)
    } else {
        None
    };

    log::info!("{} plugins\n", file.plugins.len().to_string().cyan().bold());

    for (repo, entry) in &file.plugins {
        let ref_ = match entry {
            crate::schema::PluginEntry::Ref(r) => r.as_str(),
            crate::schema::PluginEntry::Config(c) => c.ref_.as_str(),
        };

        let locked = lock.as_ref().and_then(|l| l.plugin.iter().find(|p| &p.repo == repo));

        match locked {
            Some(lp) => {
                let short_sha = &lp.sha[..8.min(lp.sha.len())];
                let artifact_exists = Path::new(&lp.artifact).exists();
                let status = if artifact_exists {
                    "installed".green().bold()
                } else {
                    "missing artifact".yellow().bold()
                };
                log::info!(
                    "  {} {} {} {}",
                    repo.white().bold(),
                    format!("({})", ref_).dimmed(),
                    format!("@ {}", short_sha).dimmed(),
                    status,
                );
            }
            None => {
                log::info!(
                    "  {} {} {}",
                    repo.white().bold(),
                    format!("({})", ref_).dimmed(),
                    "not installed".red().bold(),
                );
            }
        }
    }

    Ok(())
}
