use super::{InstalledPackage, save_db, load_db}; 
use log::info;
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use colored::Colorize;

pub fn update_package(package_name: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut db = load_db()?;

    if let Some(name) = package_name {
        if let Some(pkg) = db.packages.get_mut(&name) {
            info!("> Updating package '{}'", name.yellow().bold());
            update_file(pkg)?;
            info!("Successfully updated '{}'", name.yellow().bold());
        } else {
            info!("Package '{}' not found in database", name.yellow());
        }
    } else {
        info!("> Updating all packages...");
        for (name, pkg) in db.packages.iter_mut() {
            info!("Updating '{}'", name.yellow().bold());
            update_file(pkg)?;
        }
    }

    save_db(&db)?;
    Ok(())
}

fn update_file(pkg: &mut InstalledPackage) -> Result<(), Box<dyn Error>> {
    let repo_path = PathBuf::from(&pkg.repo_path);

    // Pull latest changes
    let output = Command::new("git")
        .args(&["-C", repo_path.to_str().unwrap(), "pull"])
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "Git pull failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
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

    // Copy updated files to targets
    for file in &pkg.files {
        let source = repo_path.join(
            PathBuf::from(file)
                .file_name()
                .ok_or("Invalid filename")?,
        );
        let target = PathBuf::from(file);
        if source.exists() {
            fs::copy(&source, &target)?;
        }
    }

    Ok(())
}
