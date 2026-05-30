use crate::git;
use crate::schema::{LockFile, LockedPlugin, PluginEntry, PluginsFile, PluginManifest, PluginManifestInner};
use anyhow::{bail, Context, Result};
use colored::Colorize;
use dirs::cache_dir;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const DEFAULT_BUILD: &str = "cargo build --release";

pub fn install_plugins() -> Result<()> {
    let toml_path = Path::new("plugins.toml");
    let lock_path = Path::new("plugins.lock");

    if !toml_path.exists() {
        bail!("plugins.toml not found, run 'eiipm init' first");
    }

    let contents = fs::read_to_string(toml_path).context("failed to read plugins.toml")?;
    let file: PluginsFile = toml::from_str(&contents).context("failed to parse plugins.toml")?;

    if file.plugins.is_empty() {
        log::info!("{}", "nothing to install".dimmed());
        return Ok(());
    }

    //  handle lockfile
    let mut lock: LockFile = if lock_path.exists() {
        let lock_contents = fs::read_to_string(lock_path).context("failed to read plugins.lock")?;
        toml::from_str(&lock_contents).context("failed to parse plugins.lock")?
    } else {
        LockFile { version: 1, plugin: vec![] }
    };

    let cache_root = cache_dir()
        .context("could not resolve cache directory")?
        .join("eiipm");
    fs::create_dir_all(&cache_root).context("failed to create cache dir")?;
    fs::create_dir_all("plugins").context("failed to create plugins/ dir")?;

    let total = file.plugins.len();
    log::info!("installing {} plugin{}", total, if total == 1 { "" } else { "s" });

    for (repo, entry) in &file.plugins {
        install_one(repo, entry, &cache_root, &mut lock)?;
    }

    let lock_str = toml::to_string_pretty(&lock).context("failed to serialize lockfile")?;
    fs::write(lock_path, lock_str).context("failed to write plugins.lock")?;

    log::info!("\n{} all plugins installed", "done!".green().bold());
    Ok(())
}

fn install_one(
    repo: &str,
    entry: &PluginEntry,
    cache_root: &Path,
    lock: &mut LockFile,
) -> Result<()> {
    if lock.plugin.iter().any(|p| p.repo == repo) {
        log::info!("{} {} {}", "-".dimmed(), repo.dimmed(), "already installed, skipping".dimmed());
        return Ok(());
    }

    let ref_ = match entry {
        PluginEntry::Ref(r) => r.as_str(),
        PluginEntry::Config(c) => c.ref_.as_str(),
    };
    let build_override = match entry {
        PluginEntry::Config(c) => c.build.as_deref(),
        _ => None,
    };
    let artifact_override = match entry {
        PluginEntry::Config(c) => c.artifact.as_deref(),
        _ => None,
    };
    let prebuilt_requested = match entry {
        PluginEntry::Config(c) => c.prebuilt.unwrap_or(false),
        _ => false,
    };

    let cache_dir = cache_root.join(repo.replace('/', "__"));
    let repo_url = format!("https://github.com/{}.git", repo);
    let short_name = repo.split('/').last().unwrap_or(repo);
    let artifact_dst = PathBuf::from("plugins").join(format!("{}.so", short_name));

    //  Using prebuilts
    if prebuilt_requested {
        let sp = spinner(&format!("{} {}", "fetching".cyan(), repo));

        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir)
                .with_context(|| format!("failed to clear stale cache for {}", repo))?;
        }
        git::init_and_fetch(&repo_url, &cache_dir, ref_, 1)
            .with_context(|| format!("failed to clone {}", repo))?;

        sp.finish_with_message(format!("{} {}", "fetched".green(), repo));

        let plugin_manifest = read_plugin_manifest(&cache_dir);

        let prebuilt_url = plugin_manifest
            .as_ref()
            .and_then(|m| m.prebuilt.as_ref())
            .map(|p| p.url.clone())
            .with_context(|| format!("{} requested prebuilt but plugin.toml has no [plugin.prebuilt] section", repo))?;

        let resolved_url = resolve_prebuilt_url(&prebuilt_url, ref_);

        let sp = spinner(&format!("{} {} {}", "downloading".cyan(), repo, resolved_url.dimmed()));
        download_prebuilt(&resolved_url, &artifact_dst)
            .with_context(|| format!("failed to download prebuilt for {}", repo))?;
        sp.finish_with_message(format!(
            "{} {} {}",
            "✔".green().bold(),
            repo.white().bold(),
            "(prebuilt)".dimmed(),
        ));

        let sha = head_sha(&cache_dir).unwrap_or_else(|_| "unknown".to_string());
        upsert_lock(lock, repo, ref_, &sha, &artifact_dst);
        return Ok(());
    }

    // Building
    let sp = spinner(&format!("{} {}", "fetching".cyan(), repo));

    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir)
            .with_context(|| format!("failed to clear stale cache for {}", repo))?;
    }
    git::init_and_fetch(&repo_url, &cache_dir, ref_, 1)
        .with_context(|| format!("failed to clone {}", repo))?;

    sp.finish_with_message(format!("{} {}", "fetched".green(), repo));

    let plugin_manifest = read_plugin_manifest(&cache_dir);

    let build_cmd = build_override
        .or_else(|| plugin_manifest.as_ref().and_then(|m| m.build.as_deref()))
        .unwrap_or(DEFAULT_BUILD);

    let artifact_rel = artifact_override
        .or_else(|| plugin_manifest.as_ref().and_then(|m| m.artifact.as_deref()))
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("target/release/lib{}.so", short_name.replace('-', "_")));

    let sp = spinner(&format!("{} {}", "building".cyan(), repo));
    run_build(build_cmd, &cache_dir)
        .with_context(|| format!("build failed for {}", repo))?;
    sp.finish_with_message(format!("{} {}", "built".green(), repo));

    let artifact_src = cache_dir.join(artifact_rel);

    if !artifact_src.exists() {
        bail!(
            "artifact not found at {} after build! Check plugin.toml or pass --artifact",
            artifact_src.display()
        );
    }

    fs::copy(&artifact_src, &artifact_dst)
        .with_context(|| format!("failed to copy artifact for {}", repo))?;

    log::info!("{} {}", "installed".green().bold(), artifact_dst.display());

    let sha = head_sha(&cache_dir).unwrap_or_else(|_| "unknown".to_string());
    upsert_lock(lock, repo, ref_, &sha, &artifact_dst);

    Ok(())
}

// == Helpers ==

pub fn resolve_prebuilt_url(url: &str, ref_: &str) -> String {
    url
        .replace("{version}", ref_)
        .replace("{arch}", std::env::consts::ARCH)
        .replace("{os}", std::env::consts::OS)
}

pub fn download_prebuilt(url: &str, dst: &Path) -> Result<()> {
    let response = ureq::get(url)
        .call()
        .with_context(|| format!("HTTP request failed for {}", url))?;

    if response.status() != 200 {
        bail!("server returned HTTP {} for {}", response.status(), url);
    }

    let mut out = fs::File::create(dst)
        .with_context(|| format!("failed to create {}", dst.display()))?;

    std::io::copy(&mut response.into_body().as_reader(), &mut out)
        .context("failed to write downloaded binary")?;

    Ok(())
}

pub fn upsert_lock(lock: &mut LockFile, repo: &str, ref_: &str, sha: &str, artifact_dst: &Path) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default();

    lock.plugin.retain(|p| p.repo != repo);
    lock.plugin.push(LockedPlugin {
        repo: repo.to_string(),
        sha: sha.to_string(),
        ref_: ref_.to_string(),
        artifact: artifact_dst.to_string_lossy().to_string(),
        built_at: now,
    });
}

pub fn spinner(msg: &str) -> ProgressBar {
    let sp = ProgressBar::new_spinner();
    sp.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    sp.enable_steady_tick(Duration::from_millis(80));
    sp.set_message(msg.to_string());
    sp
}

pub fn run_build(cmd: &str, cwd: &Path) -> Result<()> {
    let mut parts = cmd.split_whitespace();
    let bin = parts.next().context("build command is empty")?;
    let args: Vec<&str> = parts.collect();

    let debug = log::max_level() >= log::LevelFilter::Debug;

    let mut command = Command::new(bin);
    command.args(&args).current_dir(cwd);

    if !debug {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = command
        .status()
        .with_context(|| format!("failed to spawn '{}'", cmd))?;

    if !status.success() {
        bail!("command '{}' exited with {}", cmd, status);
    }
    Ok(())
}

pub fn head_sha(repo_path: &Path) -> Result<String> {
    let repo = git2::Repository::open(repo_path)?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}

pub fn read_plugin_manifest(cache_dir: &Path) -> Option<PluginManifestInner> {
    let path = cache_dir.join("plugin.toml");
    let contents = fs::read_to_string(path).ok()?;
    let manifest: PluginManifest = toml::from_str(&contents).ok()?;
    Some(manifest.plugin)
}
