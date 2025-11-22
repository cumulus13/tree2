// File: src\main.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-11-22
// Description: A high-performance directory tree visualization tool written in Rust with colors, emojis, and gitignore support. Available as both CLI tool and library crate.
// License: MIT

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;

// ANSI Color Codes with True Color (24-bit) and fallback
const COLOR_RESET: &str = "\x1b[0m";
// const COLOR_RED: &str = "\x1b[91m";
// const COLOR_RED: &str = "\x1b[1;38;2;255;100;100m"; 
const COLOR_RED: &str = "\x1b[1;97;41m"; 
const COLOR_ORANGE: &str = "\x1b[38;5;214m";

// True Color (24-bit) ANSI codes - lighter color
const COLOR_BRIGHT_YELLOW: &str = "\x1b[38;2;255;255;0m";  // #FFFF00
const COLOR_BRIGHT_CYAN: &str = "\x1b[38;2;0;255;255m";    // #00FFFF
const COLOR_LIGHT_MAGENTA_TRUE: &str = "\x1b[38;2;255;128;255m"; // Light magenta

#[derive(Parser)]
#[command(name = "tree2")]
#[command(about = "Print directory tree with file sizes, exclusions, and .gitignore support")]
#[command(version)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, long, num_args = 0..)]
    exclude: Vec<String>,
}

struct Config {
    excludes: HashSet<String>,
    root_excludes: HashSet<String>,
}

fn human_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;

    for unit in units.iter() {
        if size < 1024.0 {
            return format!("{:.2} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:.2} PB", size)
}

fn load_gitignore(path: &Path) -> HashSet<String> {
    let gitignore_path = path.join(".gitignore");
    if !gitignore_path.exists() {
        return HashSet::new();
    }

    match fs::read_to_string(gitignore_path) {
        Ok(content) => {
            content.lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with('#'))
                .map(|line| line.trim_end_matches('/').to_string())
                .collect()
        }
        Err(_) => HashSet::new(),
    }
}

fn should_exclude(entry: &str, excludes: &HashSet<String>, root_excludes: &HashSet<String>) -> bool {
    excludes.iter().any(|ex| entry == ex || entry.starts_with(ex)) ||
    root_excludes.iter().any(|ex| entry == ex || entry.starts_with(ex))
}

fn print_tree(path: &Path, prefix: &str, config: &Config) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => {
            let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>().unwrap_or_default();
            entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            entries
        }
        Err(_) => {
            let permission_text = format!("{}└── 🔒 [Permission Denied]", prefix);
            println!("{}{}{}", COLOR_RED, permission_text, COLOR_RESET);
            return;
        }
    };

    for (idx, entry) in entries.iter().enumerate() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        if should_exclude(&file_name, &config.excludes, &config.root_excludes) {
            continue;
        }

        let connector = if idx == entries.len() - 1 { "└── " } else { "├── " };
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        if metadata.is_dir() {
            // Folder in light yellow (#FFFF00)
            let folder_text = format!("{}{}📁 {}/", prefix, connector, file_name);
            println!("{}{}{}", COLOR_BRIGHT_YELLOW, folder_text, COLOR_RESET);

            let new_prefix = if idx == entries.len() - 1 {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree(&entry.path(), &new_prefix, config);
        } else {
            let size = metadata.len();
            let size_str = human_size(size);
            let parts: Vec<&str> = size_str.split_whitespace().collect();
            let (size_value, size_unit) = (parts[0], parts[1]);

            // File with light cyan color (#00FFFF)
            print!("{}{}📄 {} (", COLOR_BRIGHT_CYAN, format!("{}{}", prefix, connector), file_name);

            // Size value
            if size == 0 {
                print!("{}{}", COLOR_RED, size_value);
            } else {
                print!("{}{}", COLOR_LIGHT_MAGENTA_TRUE, size_value);
            }

            print!("{} ", COLOR_RESET);

            // Unit size in orange
            print!("{}{}", COLOR_ORANGE, size_unit);
            println!("{})", COLOR_RESET);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let path = PathBuf::from(&cli.path);
    let abs_path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let gitignore_excludes = load_gitignore(&abs_path);

    let config = Config {
        excludes: cli.exclude.into_iter().collect(),
        root_excludes: gitignore_excludes,
    };

    // Print the root directory in bright yellow
    let root_text = format!("📂 {}/", abs_path.display());
    println!("{}{}{}", COLOR_BRIGHT_YELLOW, root_text, COLOR_RESET);

    print_tree(&abs_path, "", &config);
}
