use std::io::{self, Write};

pub fn confirm(prompt: &str) -> bool {
    let mut input = String::new();
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
