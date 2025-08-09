use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use log::{debug, info, error, trace};
use std::path::PathBuf;
use colored::{Colorize, ColoredString};

#[derive(Deserialize, Debug)]
struct PackageRootMeta {
    metadata: PackageMeta,
}

#[derive(Deserialize, Debug)]
struct PackageMeta {
    name: String,
    version: f32,
    install_url: String,
}

pub fn install_package(package_name: &str) -> Result<(), Box<dyn Error>> {
    info!("> Installing package '{}'", package_name.yellow().bold());

    let raw_manifest_url = format!(
        "https://raw.githubusercontent.com/Ewwii-sh/eii-manifests/main/manifests/{}.toml",
        package_name
    );
    trace!("  Constructed manifest URL:\n    {}", raw_manifest_url.underline());

    let toml_content = http_get_string(&raw_manifest_url).map_err(|e| {
        error!("  [ERROR] Error fetching manifest for package '{}': {}", package_name.yellow(), e.to_string().red());
        e
    })?;
    debug!("  Fetched manifest content:\n{}", toml_content.dimmed());

    let root_meta: PackageRootMeta = toml::from_str(&toml_content).map_err(|e| {
        error!("  [ERROR] Failed to parse manifest TOML for package '{}': {}", package_name.yellow(), e.to_string().red());
        e
    })?;
    trace!("  Parsed manifest metadata:\n    {:?}", root_meta.metadata);

    let mut install_script_path = std::env::temp_dir();
    install_script_path.push(format!("{}.sh", package_name));

    info!("  Downloading install script:");
    info!("    From: {}", root_meta.metadata.install_url.underline());
    info!("    To:   {}", install_script_path.display().to_string().bright_yellow());

    download_file(&root_meta.metadata.install_url, install_script_path.to_str().unwrap()).map_err(|e| {
        error!("  [ERROR] Failed to download install script for package '{}': {}", package_name.yellow(), e.to_string().red());
        e
    })?;

    info!("  Running install script: {}", install_script_path.display().to_string().yellow());

    run_script(install_script_path.to_str().unwrap(), package_name)?;

    info!("  Installation completed successfully for package '{}'", package_name.yellow().bold());

    std::fs::remove_file(install_script_path)?;

    Ok(())
}

fn http_get_string(url: &str) -> Result<String, Box<dyn Error>> {
    debug!("  Sending GET request to {}", url.dimmed());

    let response = get(url)?;

    if !response.status().is_success() {
        error!("  [ERROR] Failed to fetch URL {}: HTTP {}", url.yellow(), response.status());
        return Err(format!("Failed to fetch URL {}: HTTP {}", url, response.status()).into());
    }

    let body = response.text()?;
    debug!("  Received response body ({} bytes)", body.len());
    Ok(body)
}

fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    debug!("  Sending GET request to download file from {}", url.dimmed());

    let response = get(url)?;

    if !response.status().is_success() {
        error!("  [ERROR] Failed to download file from {}: HTTP {}", url.yellow(), response.status());
        return Err(format!("Failed to download file from {}: HTTP {}", url, response.status()).into());
    }

    let content = response.text()?;
    debug!("  Downloaded file content length: {} bytes", content.len());
    debug!("  Creating file at '{}'", output_path.dimmed());

    let mut file = File::create(output_path)?;
    file.write_all(content.as_bytes())?;

    trace!("  Successfully wrote to '{}'", output_path.bright_green());

    Ok(())
}

fn run_script(script_path: &str, package_name: &str) -> Result<(), Box<dyn Error>> {
    info!("  Executing script: {}", script_path.yellow());

    let output = Command::new("sh")
        .arg(script_path)
        .output()?; // capture output

    if !output.status.success() {
        error!("  [ERROR] Script failed with status: {}", format!("{:?}", output.status).red());
        error!("  [ERROR] stderr:");
        for line in String::from_utf8_lossy(&output.stderr).lines() {
            error!("    {}", line.red());
        }
        return Err(format!("Script execution failed with status: {:?}", output.status).into());
    } else {
        info!("{}", "  Script executed successfully.".bright_green());
        debug!("  Script stdout:\n{}", String::from_utf8_lossy(&output.stdout).dimmed());
    }

    Ok(())
}
