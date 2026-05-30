use anyhow::{Context, Result, bail};
use colored::Colorize;
use dirs::cache_dir;
use std::fs;
use std::path::Path;
use crate::schema::{LockFile, PluginsFile, PluginEntry};
use crate::functions::install::{
    head_sha, spinner, read_plugin_manifest,
    resolve_prebuilt_url, download_prebuilt,
    DEFAULT_BUILD
};
use crate::git;

pub fn update_plugins(maybe_plugin: Option<String>) -> Result<()> {
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

    let mut lock: LockFile = if lock_path.exists() {
        let lock_contents = fs::read_to_string(lock_path).context("failed to read plugins.lock")?;
        toml::from_str(&lock_contents).context("failed to parse plugins.lock")?
    } else {
        bail!("plugins.lock not found, run 'eiipm install' first");
    };

    let cache_root = cache_dir()
        .context("could not resolve cache directory")?
        .join("eiipm");

    let targets: Vec<(&String, &PluginEntry)> = match &maybe_plugin {
        Some(name) => {
            match file.plugins.get_key_value(name) {
                Some((k, v)) => vec![(k, v)],
                None => bail!("'{}' is not in plugins.toml", name),
            }
        }
        None => file.plugins.iter().collect(),
    };

    let total = targets.len();
    log::info!(
        "updating {} plugin{}",
        total.to_string().cyan().bold(),
        if total == 1 { "" } else { "s" }
    );

    let mut updated = 0;
    let mut skipped = 0;

    for (repo, entry) in targets {
        match update_one(repo, entry, &cache_root, &mut lock) {
            Ok(true)  => updated += 1,
            Ok(false) => skipped += 1,
            Err(e)    => log::warn!("{} {}: {}", "failed to update".yellow().bold(), repo, e),
        }
    }

    let lock_str = toml::to_string_pretty(&lock).context("failed to serialize lockfile")?;
    fs::write(lock_path, lock_str).context("failed to write plugins.lock")?;

    log::info!(
        "\n{} {} updated, {} already up to date",
        "done!".green().bold(),
        updated.to_string().cyan(),
        skipped.to_string().dimmed(),
    );

    Ok(())
}

// true - Plugin was updated
// false - plugin is up to date
fn update_one(
    repo: &str,
    entry: &PluginEntry,
    cache_root: &Path,
    lock: &mut LockFile,
) -> Result<bool> {
    let ref_ = match entry {
        PluginEntry::Ref(r) => r.as_str(),
        PluginEntry::Config(c) => c.ref_.as_str(),
    };
    let prebuilt_requested = match entry {
        PluginEntry::Config(c) => c.prebuilt.unwrap_or(false),
        _ => false,
    };

    let cache_dir = cache_root.join(repo.replace('/', "__"));
    let short_name = repo.split('/').last().unwrap_or(repo);
    let artifact_dst = Path::new("plugins").join(format!("{}.so", short_name));

    if !cache_dir.exists() {
        bail!("not in cache, run 'eiipm install' first");
    }

    let is_locked = lock.plugin.iter().any(|p| p.repo == repo);
    if !is_locked {
        bail!("not installed, run 'eiipm install' first");
    }

    let artifact_missing = !artifact_dst.exists();

    //  Prebuilts
    if prebuilt_requested {
        let sp = spinner(&format!("{} {}", "fetching".cyan(), repo));

        git::update_to_latest(&cache_dir, ref_, 1)
            .with_context(|| format!("failed to fetch {}", repo))?;

        let sha_after = head_sha(&cache_dir).unwrap_or_default();
        let sha_before = lock.plugin.iter()
            .find(|p| p.repo == repo)
            .map(|p| p.sha.clone())
            .unwrap_or_default();

        if sha_before == sha_after && !artifact_missing {
            sp.finish_with_message(format!(
                "{} {} {}",
                "-".dimmed(),
                repo.white(),
                "already up to date".dimmed(),
            ));
            return Ok(false);
        }

        let plugin_manifest = read_plugin_manifest(&cache_dir);
        let prebuilt_url = plugin_manifest
            .as_ref()
            .and_then(|m| m.prebuilt.as_ref())
            .map(|p| p.url.clone())
            .with_context(|| format!("{} requested prebuilt but plugin.toml has no [plugin.prebuilt] section", repo))?;

        let resolved_url = resolve_prebuilt_url(&prebuilt_url, ref_);

        sp.set_message(format!(
            "{} {} {}",
            "downloading".cyan(),
            repo,
            if artifact_missing { "(restoring missing artifact)".yellow().to_string() } else { "".to_string() }
        ));

        download_prebuilt(&resolved_url, &artifact_dst)
            .with_context(|| format!("failed to download prebuilt for {}", repo))?;

        let finish_msg = if artifact_missing {
            format!("{} {} {}", "✔".green().bold(), repo.white().bold(), "artifact restored".yellow())
        } else {
            format!(
                "{} {} {} {} {}",
                "✔".green().bold(),
                repo.white().bold(),
                "(prebuilt)".dimmed(),
                sha_before[..8.min(sha_before.len())].dimmed(),
                format!("→ {}", &sha_after[..8.min(sha_after.len())]).green(),
            )
        };
        sp.finish_with_message(finish_msg);

        crate::functions::install::upsert_lock(lock, repo, ref_, &sha_after, &artifact_dst);
        return Ok(true);
    }

    // Building method
    let sha_before = head_sha(&cache_dir).unwrap_or_default();

    let sp = spinner(&format!("{} {}", "fetching".cyan(), repo));

    git::update_to_latest(&cache_dir, ref_, 1)
        .with_context(|| format!("failed to fetch {}", repo))?;

    let sha_after = head_sha(&cache_dir).unwrap_or_default();

    if sha_before == sha_after && !artifact_missing {
        sp.finish_with_message(format!(
            "{} {} {}",
            "-".dimmed(),
            repo.white(),
            "already up to date".dimmed(),
        ));
        return Ok(false);
    }

    sp.set_message(format!(
        "{} {} {}",
        "building".cyan(),
        repo,
        if artifact_missing { "(restoring missing artifact)".yellow().to_string() } else { "".to_string() }
    ));

    let plugin_manifest = read_plugin_manifest(&cache_dir);
    let build_cmd = match entry {
        PluginEntry::Config(c) => c.build.as_deref()
            .or_else(|| plugin_manifest.as_ref().and_then(|m| m.build.as_deref()))
            .unwrap_or(DEFAULT_BUILD),
        _ => plugin_manifest.as_ref()
            .and_then(|m| m.build.as_deref())
            .unwrap_or(DEFAULT_BUILD),
    };

    crate::functions::install::run_build(build_cmd, &cache_dir)
        .with_context(|| format!("build failed for {}", repo))?;

    let artifact_rel = match entry {
        PluginEntry::Config(c) => c.artifact.as_deref()
            .or_else(|| plugin_manifest.as_ref().and_then(|m| m.artifact.as_deref()))
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("target/release/lib{}.so", short_name.replace('-', "_"))),
        _ => plugin_manifest.as_ref()
            .and_then(|m| m.artifact.as_deref())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("target/release/lib{}.so", short_name.replace('-', "_"))),
    };

    let artifact_src = cache_dir.join(&artifact_rel);

    if !artifact_src.exists() {
        bail!("artifact not found at {} after build", artifact_src.display());
    }

    fs::copy(&artifact_src, &artifact_dst)
        .with_context(|| format!("failed to copy artifact for {}", repo))?;

    let finish_msg = if artifact_missing {
        format!("{} {} {}", "✔".green().bold(), repo.white().bold(), "artifact restored".yellow())
    } else {
        format!(
            "{} {} {} {}",
            "✔".green().bold(),
            repo.white().bold(),
            sha_before[..8.min(sha_before.len())].dimmed(),
            format!("→ {}", &sha_after[..8.min(sha_after.len())]).green(),
        )
    };
    sp.finish_with_message(finish_msg);

    crate::functions::install::upsert_lock(lock, repo, ref_, &sha_after, &artifact_dst);

    Ok(true)
}
