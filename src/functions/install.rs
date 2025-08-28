use super::{FileEntry, InstalledPackage, PackageRootMeta, http_get_string, load_db, save_db};
use colored::Colorize;
use dirs;
use glob::glob;
use log::{info, trace};
use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;

use crate::git::{init_and_fetch, update_to_latest};

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

    let repo_fs_path = eiipm_dir.join(format!("cache/{}", repo_name));

    // Init and fetch or fetch and clean repo
    if !repo_fs_path.exists() {
        info!(
            "Cloning repository {} to {}",
            meta.src.underline(),
            repo_fs_path.display()
        );
        let _repo = init_and_fetch(&meta.src, &repo_fs_path, &meta.commit_hash, 1)
            .map_err(|e| format!("Failed to fetch commit: {}", e))?;
    } else {
        info!("Repository exists, fetching latest changes");
        let _repo = update_to_latest(&repo_fs_path, &meta.commit_hash, 1)
            .map_err(|e| format!("Failed to fetch commit and clean state: {}", e))?;
    }

    // Optional build step
    if let Some(build_cmd) = &meta.build {
        info!("Running build command: {}", build_cmd);
        let status = Command::new("sh")
            .arg("-c")
            .arg(build_cmd)
            .current_dir(&repo_fs_path)
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
        // handle *, ** etc. in file entry
        let files: Vec<(std::path::PathBuf, std::path::PathBuf)> = match file_entry {
            FileEntry::Flat(f) => glob(&repo_fs_path.join(f).to_string_lossy())
                .expect("Invalid glob")
                .filter_map(Result::ok)
                .map(|src| {
                    let tgt = target_base_dir.join(src.file_name().expect("Invalid file name"));
                    (src, tgt)
                })
                .collect(),

            FileEntry::Detailed { src, dest } => glob(&repo_fs_path.join(src).to_string_lossy())
                .expect("Invalid glob")
                .filter_map(Result::ok)
                .map(|src_path| {
                    let tgt = match dest {
                        Some(d) => target_base_dir.join(d),
                        None => target_base_dir.join(src),
                    };
                    (src_path, tgt)
                })
                .collect(),
        };

        for (source, target) in files {
            if !source.exists() {
                return Err(format!("File '{}' not found", source.display()).into());
            }

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source, &target)?;
            installed_files.push(target.to_string_lossy().to_string());
        }
    }

    // Update DB if its not a theme
    if !(meta.pkg_type == "theme") {
        let mut db = load_db()?;
        db.packages.insert(
            meta.name.clone(),
            InstalledPackage {
                repo_fs_path: repo_fs_path.to_string_lossy().to_string(),
                installed_files: installed_files,
                copy_files: meta.files.clone(),
                pkg_type: meta.pkg_type.clone(),
                upstream_src: meta.src.clone(),
                installed_hash: meta.commit_hash.clone(),
                manifest_url: raw_manifest_url,
                build_command: meta.build.clone(),
            },
        );
        save_db(&db)?;
    }

    info!(
        "Installation complete for '{}'",
        package_name.yellow().bold()
    );
    Ok(())
}
