use super::load_db;
use crate::opts::ListArgs;
use log::{error, info};
use std::error::Error;

pub fn list_packages(list_args: ListArgs) -> Result<(), Box<dyn Error>> {
    let db = load_db()?;

    if list_args.total_count {
        info!("Total: {}", db.packages.len());
        return Ok(());
    }

    if let Some(pkg) = list_args.query {
        // Find the package in the DB
        if let Some(package) = db.packages.get(&pkg) {
            if list_args.verbose {
                info!(
                    "{}\n  Type: {}\n  Repo: {}\n  Build: {}\n  Files:\n    {}",
                    pkg,
                    package.pkg_type,
                    package.repo_fs_path,
                    package
                        .build_command
                        .clone()
                        .unwrap_or_else(|| "None".into()),
                    package.installed_files.join("\n    ")
                );
            } else {
                info!("{}", pkg);
            }
        } else {
            error!("Package '{}' not found", pkg);
        }
        return Ok(());
    }

    // If no query, list all packages
    for (name, package) in &db.packages {
        if list_args.verbose {
            info!(
                "{}\n  Type: {}\n  Repo: {}\n  Build: {}\n  Files:\n    {}",
                name,
                package.pkg_type,
                package.repo_fs_path,
                package
                    .build_command
                    .clone()
                    .unwrap_or_else(|| "None".into()),
                package.installed_files.join("\n    ")
            );
        } else {
            info!("{}", name);
        }
    }

    Ok(())
}
