use colored::{Colorize};
use dirs;
use log::{debug, info, trace};
use reqwest::blocking::get;
use serde::{Deserialize};
use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;
use super::{FileEntry, InstalledPackage, save_db, load_db}; 

use crate::git::{
    clone_https,
    pull_but_reclone_on_fail
};

#[derive(Deserialize, Debug)]
struct PackageRootMeta {
    metadata: PackageMeta,
}

#[derive(Deserialize, Debug)]
struct PackageMeta {
    name: String,
    #[serde(rename = "type")]
    pkg_type: String,
    src: String,
    files: Vec<FileEntry>,
    build: Option<String>, // Optional build command
}

pub fn install_package(package_name: &str) -> Result<(), Box<dyn Error>> {
    info!("> Installing package '{}'", package_name.yellow().bold());

    let raw_manifest_url = format!(
        "https://raw.githubusercontent.com/Ewwii-sh/eii-manifests/main/manifests/{}.toml",
        package_name
    );
    trace!("Fetching manifest from {}", raw_manifest_url.underline());
    let toml_content = http_get_string(&raw_manifest_url)?;
    let root_meta: PackageRootMeta = toml::from_str(&toml_content)?;
    let meta = &root_meta.metadata;

    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let eiipm_dir = home_dir.join(".eiipm");
    fs::create_dir_all(&eiipm_dir)?;

    let repo_name = meta
        .src
        .rsplit('/')
        .next()
        .ok_or("Invalid src URL")?
        .strip_suffix(".git")
        .unwrap_or_else(|| meta.src.rsplit('/').next().unwrap());

    let repo_path = eiipm_dir.join(format!("cache/{}", repo_name));

    // Clone or pull repo
    if !repo_path.exists() {
        info!("Cloning repository {} to {}", meta.src.underline(), repo_path.display());
        let _repo = clone_https(&meta.src, &repo_path, Some(1))
            .map_err(|e| format!("Git clone failed: {}", e))?;
    } else {
        info!("Repository exists, pulling latest changes");
        pull_but_reclone_on_fail(&meta.src, &repo_path, Some(1))
            .map_err(|e| format!("Git pull failed: {}", e))?;
    }

    // Optional build step
    if let Some(build_cmd) = &meta.build {
        info!("Running build command: {}", build_cmd);
        let status = Command::new("sh")
            .arg("-c")
            .arg(build_cmd)
            .current_dir(&repo_path)
            .status()?;
        if !status.success() {
            return Err(format!("Build failed for package '{}'", package_name).into());
        }
    }

    // Determine target directory
    let target_base_dir = match meta.pkg_type.as_str() {
        "binary" => home_dir.join(".eiipm/bin"),
        "theme" => env::current_dir()?,
        "library" => home_dir.join(format!(".eiipm/lib/{}", package_name)),
        other => return Err(format!("Unknown package type '{}'", other).into()),
    };
    fs::create_dir_all(&target_base_dir)?;

    // Copy files and track them
    let mut installed_files = Vec::new();
    for file_entry in &meta.files {
        let (source, target) = match file_entry {
            FileEntry::Flat(f) => {
                let src = repo_path.join(f);
                let tgt = target_base_dir.join(src.file_name().ok_or_else(|| format!("Invalid file name '{}'", f))?);
                (src, tgt)
            }
            FileEntry::Detailed { src, dest } => {
                let src_path = repo_path.join(src);
                let tgt = match dest {
                    Some(d) => target_base_dir.join(d),
                    None => target_base_dir.join(src),
                };
                (src_path, tgt)
            }
        };

        if !source.exists() {
            return Err(format!("File '{}' not found", source.display()).into());
        }

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source, &target)?;
        installed_files.push(target.to_string_lossy().to_string());
    }

    // Update DB
    let mut db = load_db()?;
    db.packages.insert(
        meta.name.clone(),
        InstalledPackage {
            repo_path: repo_path.to_string_lossy().to_string(),
            installed_files: installed_files,
            copy_files: meta.files.clone(),
            pkg_type: meta.pkg_type.clone(),
            upstream_src: meta.src.clone(),
            build_command: meta.build.clone(),
        },
    );
    save_db(&db)?;

    info!("Installation complete for '{}'", package_name.yellow().bold());
    Ok(())
}

fn http_get_string(url: &str) -> Result<String, Box<dyn Error>> {
    debug!("Sending GET request to {}", url);
    let response = get(url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL {}: HTTP {}", url, response.status()).into());
    }
    Ok(response.text()?)
}
