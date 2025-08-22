use colored::Colorize;
use log::{error, info};
use std::env;
use std::path::PathBuf;

pub fn check_eiipm_in_path() {
    let target_dir: PathBuf = match dirs::home_dir() {
        Some(dir) => dir.join(".eiipm/bin"),
        None => {
            error!("Cannot get home directory");
            return;
        }
    };

    let path_var = env::var("PATH").expect("Could not get PATH environment variable");
    let paths: Vec<PathBuf> = path_var.split(':').map(PathBuf::from).collect();

    if !paths.iter().any(|p| p == &target_dir) {
        let msg = format!(
            "\n!! WARNING: {0} is NOT in your PATH\n\
            ------------------------------------------------------------\n\
            >> To fix this, add the following line to your shell config\n\
            (e.g. ~/.bashrc, ~/.zshrc):\n\n\
            {1} {2}\n\n\
            >> Without this, the 'eiipm' binaries inside {0}\n\
            will not work correctly.\n\n\
            >> For more details, see the documentation:\n\
            {3}\n\
            ------------------------------------------------------------\n",
            target_dir.display(),
            "export PATH=\"$HOME/.eiipm/bin:$PATH\"".yellow(),
            "(adjust if necessary)".dimmed(),
            "https://ewwii-sh.github.io/docs/package-manager/overview/#adding-eiipm-to-path"
                .underline()
                .blue()
        );

        info!("{}", msg.yellow()); // dont use warn!() as it will add [WARN] label
    }
}
