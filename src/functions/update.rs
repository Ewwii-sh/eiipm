use colored::Colorize;
use glob::glob;
use log::{debug, error, info};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::{FileEntry, InstalledPackage, is_update_needed_for, load_db, save_db};

use crate::git::{init_and_fetch, update_to_latest};

pub fn update_package(package_name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let mut db = load_db()?;

    if let Some(name) = package_name {
        if let Some(pkg) = db.packages.get_mut(name) {
            info!(
                "Checking package '{}' [{}]",
                name.yellow().bold(),
                pkg.pkg_type
            );

            // there is no way that it can be a theme
            // but... what if?
            if pkg.pkg_type == "theme" {
                info!("Skipping theme package '{}'", name.yellow().bold());
            } else {
                let need_update = is_update_needed_for(&name)?;
                if need_update.0 {
                    info!("> Updating '{}'", name.yellow().bold());
                    update_file(pkg, &name, need_update.1)?;
                    info!("Successfully updated '{}'", name.yellow().bold());
                } else {
                    info!("Package '{}' is already up-to-date", name.yellow().bold());
                }
            }
        } else {
            info!("Package '{}' not found in database", name.yellow());
        }
    } else {
        info!("> Updating all packages...");
        for (name, pkg) in db.packages.iter_mut() {
            info!(
                "Checking package '{}' [{}]",
                name.yellow().bold(),
                pkg.pkg_type
            );

            if pkg.pkg_type == "theme" {
                info!("Skipping theme package '{}'", name.yellow().bold());
                continue;
            }

            let need_update = is_update_needed_for(&name)?;
            if need_update.0 {
                info!("> Updating '{}'", name.yellow().bold());
                update_file(pkg, &name, need_update.1)?;
                info!("Successfully updated '{}'", name.yellow().bold());
            } else {
                info!("Package '{}' is already up-to-date", name.yellow().bold());
            }
        }
    }

    save_db(&db)?;
    Ok(())
}

fn update_file(
    pkg: &mut InstalledPackage,
    package_name: &str,
    commit_hash: String,
) -> Result<(), Box<dyn Error>> {
    let repo_fs_path = PathBuf::from(&pkg.repo_fs_path);

    // Clone/Pull latest changes
    debug!("Pulling latest version of {} using git...", package_name);

    // Init and fetch or fetch and clean repo
    if !repo_fs_path.exists() {
        info!(
            "Cloning repository {} to {}",
            pkg.upstream_src.underline(),
            repo_fs_path.display()
        );
        let _repo = init_and_fetch(&pkg.upstream_src, &repo_fs_path, &commit_hash, 1)
            .map_err(|e| format!("Failed to fetch commit: {}", e))?;
    } else {
        info!("Repository exists, fetching latest changes");
        let _repo = update_to_latest(&repo_fs_path, &commit_hash, 1)
            .map_err(|e| format!("Failed to fetch commit and clean state: {}", e))?;
    }

    // Optional build step
    if let Some(build_cmd) = &pkg.build_command {
        info!("Running build command: {}", build_cmd);
        let status = Command::new("sh")
            .arg("-c")
            .arg(build_cmd)
            .current_dir(&repo_fs_path)
            .status()?;
        if !status.success() {
            return Err(format!("Build failed for package '{}'", pkg.repo_fs_path).into());
        }
    }

    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;

    // Determine target directory
    let target_base_dir = match pkg.pkg_type.as_str() {
        "binary" => home_dir.join(".eiipm/bin"),
        // "theme" => env::current_dir()?, // updating theme is risky
        "library" => home_dir.join(format!(".eiipm/lib/{}", package_name)),
        other => return Err(format!("Unknown package type '{}'", other).into()),
    };

    // Just an extra caution
    if pkg.pkg_type == "theme" {
        error!("A theme was found in update script... skipping...");
        return Ok(());
    }

    // Copy updated files to targets
    for file_entry in &pkg.copy_files {
        // handle *, **, etc. in file entry
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
                return Err(format!("File '{}' not found in repo", source.display()).into());
            }

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(&source, &target)?;
            info!("Copied {} -> {}", source.display(), target.display());
        }
    }

    pkg.installed_hash = commit_hash;

    Ok(())
}
