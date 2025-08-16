use super::{InstalledPackage, save_db, load_db}; 
use log::{info, debug};
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::process::Command;
use colored::Colorize;

use crate::git::{
    clone_https,
    pull_with_rebase_or_reclone
};

pub fn update_package(package_name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let mut db = load_db()?;

    if let Some(name) = package_name {
        if let Some(pkg) = db.packages.get_mut(name) {
            info!("> Updating package '{}'", name.yellow().bold());
            update_file(pkg, &name)?;
            info!("Successfully updated '{}'", name.yellow().bold());
        } else {
            info!("Package '{}' not found in database", name.yellow());
        }
    } else {
        info!("> Updating all packages...");
        for (name, pkg) in db.packages.iter_mut() {
            info!("Updating '{}'", name.yellow().bold());
            update_file(pkg, name)?;
        }
    }

    save_db(&db)?;
    Ok(())
}

fn update_file(pkg: &mut InstalledPackage, package_name: &str) -> Result<(), Box<dyn Error>> {
    let repo_path = PathBuf::from(&pkg.repo_path);

    // Clone/Pull latest changes
    debug!("Pulling latest version of {} using git...", package_name);

    if !repo_path.exists() {
        info!("Cache not found. Cloning repository {} to {}", pkg.upstream_src.underline(), repo_path.display());
        let _repo = clone_https(&pkg.upstream_src, &repo_path, Some(1))
            .map_err(|e| format!("Git clone failed: {}", e))?;
    } else {
        info!("Repository is cached, pulling latest changes");
        pull_with_rebase_or_reclone(&pkg.upstream_src, &repo_path, Some(1))
            .map_err(|e| format!("Git pull failed: {}", e))?;
    }

    // Optional build step
    if let Some(build_cmd) = &pkg.build_command {
        info!("Running build command: {}", build_cmd);
        let status = Command::new("sh")
            .arg("-c")
            .arg(build_cmd)
            .current_dir(&repo_path)
            .status()?;
        if !status.success() {
            return Err(format!("Build failed for package '{}'", pkg.repo_path).into());
        }
    }

    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;

    // Determine target directory
    let target_base_dir = match pkg.pkg_type.as_str() {
        "binary" => home_dir.join(".eiipm/bin"),
        "theme" => env::current_dir()?,
        "library" => home_dir.join(format!(".eiipm/lib/{}", package_name)),
        other => return Err(format!("Unknown package type '{}'", other).into()),
    };

    // Copy updated files to targets
    for file in &pkg.copy_files {
        let source = repo_path.join(file);
        let target = target_base_dir.join(file);
        if !source.exists() {
            return Err(format!("File '{}' not found in repo", source.display()).into());
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source, &target)?;
        info!("Copied {} -> {}", source.display(), target.display());
    }


    Ok(())
}
