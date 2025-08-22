use super::http_get_string;
use log::info;
use std::error::Error;
use colored::Colorize;
use crate::opts::SearchArgs;

pub fn search_package(
    package_name: &str,
    flags: SearchArgs
) -> Result<(), Box<dyn Error>> {
    info!("> Searching for '{}'", package_name.yellow().bold());

    let raw_manifest_url = format!(
        "https://raw.githubusercontent.com/Ewwii-sh/eii-manifests/main/manifests/{}.toml",
        package_name
    );

    if let Ok(response) = http_get_string(&raw_manifest_url) {
        info!("{}", format!(
            "\nPackage with name '{}' is found in eii-manifests!",
            package_name
        ).green());

        if flags.log_metadata {
            info!("{}", format!(
                "\n--- Metadata for '{}' ---\n{}",
                package_name.cyan().bold(),
                indent_lines(&response, 4)
            ));
        }
    } else {
        info!("{}", format!(
            "Package '{}' not found.",
            package_name.red().bold()
        ));
    }

    Ok(())
}

fn indent_lines(s: &str, spaces: usize) -> String {
    let padding = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{}{}", padding, line))
        .collect::<Vec<_>>()
        .join("\n")
}
