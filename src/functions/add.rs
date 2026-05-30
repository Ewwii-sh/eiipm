use crate::opts::AddArgs;
use crate::schema::{PluginsFile, PluginEntry, PluginConfig};
use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::path::Path;
use std::fs;

pub fn add_plugin(args: AddArgs) -> Result<()> {
    let toml_path = Path::new("plugins.toml");

    if !toml_path.exists() {
        bail!("plugins.toml does not exist, run 'eiipm init' first");
    }

    let contents = fs::read_to_string(toml_path)
        .context("failed to read plugins.toml")?;
    let mut file: PluginsFile = toml::from_str(&contents)
        .context("failed to parse plugins.toml")?;

    if file.plugins.contains_key(&args.plugin) {
        bail!("{} is already in plugins.toml", args.plugin);
    }

    let ref_ = args.ref_.unwrap_or_else(|| "main".to_string());

    let needs_config = args.prebuilt || args.build.is_some() || args.artifact.is_some();

    let entry = if needs_config {
        PluginEntry::Config(PluginConfig {
            ref_,
            prebuilt: if args.prebuilt { Some(true) } else { None },
            build: args.build,
            artifact: args.artifact,
        })
    } else {
        PluginEntry::Ref(ref_)
    };

    file.plugins.insert(args.plugin.clone(), entry);

    let updated = toml::to_string_pretty(&file)
        .context("failed to serialize plugins.toml")?;
    fs::write(toml_path, updated)
        .context("failed to write plugins.toml")?;

    log::info!("{} {} to plugins.toml", "added".green().bold(), args.plugin.cyan());
    log::info!("{} run {} to install it", "tip:".dimmed(), "eiipm install".cyan());

    Ok(())
}
