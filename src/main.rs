// File: src\main.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-11-22
// Description: A high-performance directory tree visualization tool written in Rust with colors, emojis, and gitignore support. Available as both CLI tool and library crate.
// License: MIT

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use clap::{Parser, ArgAction};
use dunce::canonicalize;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use clap_version_flag::colorful_version;

// ANSI Color Codes with True Color (24-bit)
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_WHITE_ON_RED: &str = "\x1b[1;97;41m";
const COLOR_ORANGE: &str = "\x1b[38;5;214m";

// True Color (24-bit) ANSI codes - lighter color
const COLOR_BRIGHT_YELLOW: &str = "\x1b[38;2;255;255;0m";  // #FFFF00
const COLOR_BRIGHT_CYAN: &str = "\x1b[38;2;0;255;255m";    // #00FFFF
const COLOR_LIGHT_MAGENTA_TRUE: &str = "\x1b[38;2;255;128;255m"; // Light magenta

#[derive(Parser)]
#[command(name = "tree2")]
#[command(about = "Print directory tree with file sizes, exclusions, and .gitignore support")]
#[command(disable_version_flag = true)]
struct Cli {
    #[arg(short = 'V', long = "version", action = ArgAction::SetTrue)]
    version: bool,

    #[arg(default_value = ".")]
    path: String,

    /// Exclude directories/files (exact match only)
    #[arg(short, long, num_args = 0..)]
    exclude: Vec<String>,

    /// Copy result to clipboard
    #[arg(short = 'c', long)]
    clipboard: bool,

    // /// Print version information
    // #[arg(short = 'V', long)]
    // version: bool,
}

struct Config {
    excludes: HashSet<String>,
    root_excludes: HashSet<String>,
}

// fn get_version_info() -> String {
//     format!(
//         "{}\nAuthor: Hadi Cahyadi <cumulus13@gmail.com>\nRepository: https://github.com/cumulus13/tree2",
//         env!("CARGO_PKG_VERSION")
//     )
// }

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

// Fixed: exact match only, tidak akan exclude .github jika exclude .git
fn should_exclude(entry: &str, excludes: &HashSet<String>, root_excludes: &HashSet<String>) -> bool {
    excludes.contains(entry) || root_excludes.contains(entry)
}

// fn strip_ansi(text: &str) -> String {
//     let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
//     re.replace_all(text, "").to_string()
// }

fn print_tree(path: &Path, prefix: &str, config: &Config, output: &mut String, use_colors: bool) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => {
            let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>().unwrap_or_default();
            entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            entries
        }
        Err(_) => {
            let permission_text = format!("{}└── 🔒 [Permission Denied]\n", prefix);
            if use_colors {
                let colored = format!("{}{}{}", COLOR_WHITE_ON_RED, permission_text, COLOR_RESET);
                print!("{}", colored);
                output.push_str(&permission_text);
            } else {
                print!("{}", permission_text);
                output.push_str(&permission_text);
            }
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
            let folder_text = format!("{}{}📁 {}/\n", prefix, connector, file_name);
            
            if use_colors {
                let colored = format!("{}{}{}", COLOR_BRIGHT_YELLOW, folder_text, COLOR_RESET);
                print!("{}", colored);
            } else {
                print!("{}", folder_text);
            }
            output.push_str(&folder_text);

            let new_prefix = if idx == entries.len() - 1 {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree(&entry.path(), &new_prefix, config, output, use_colors);
        } else {
            let size = metadata.len();
            let size_str = human_size(size);
            let parts: Vec<&str> = size_str.split_whitespace().collect();
            let (size_value, size_unit) = (parts[0], parts[1]);

            let file_line = format!("{}{}📄 {} ({} {})\n", prefix, connector, file_name, size_value, size_unit);
            
            if use_colors {
                print!("{}{}📄 {} (", COLOR_BRIGHT_CYAN, format!("{}{}", prefix, connector), file_name);
                
                if size == 0 {
                    print!("{}{}", COLOR_WHITE_ON_RED, size_value);
                } else {
                    print!("{}{}", COLOR_LIGHT_MAGENTA_TRUE, size_value);
                }
                print!("{} ", COLOR_RESET);
                print!("{}{}", COLOR_ORANGE, size_unit);
                println!("{})", COLOR_RESET);
            } else {
                print!("{}", file_line);
            }
            
            output.push_str(&file_line);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "-V" || args[1] == "--version") {
        let version = colorful_version!();
        version.print_and_exit();
    }
    
    let cli = Cli::parse();
    let version_str = colorful_version!();

    // Handle version flag manually
    if cli.version {
        // println!("{}", get_version_info());
        println!("{}", version_str);
        std::process::exit(0);
    }

    let path = PathBuf::from(&cli.path);
    let abs_path = match canonicalize(&path) {
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

    let mut output = String::new();
    let use_colors = !cli.clipboard;

    // Print the root directory
    let root_text = format!("📂 {}/\n", abs_path.display());
    
    if use_colors {
        let colored = format!("{}{}{}", COLOR_BRIGHT_YELLOW, root_text, COLOR_RESET);
        print!("{}", colored);
    } else {
        print!("{}", root_text);
    }
    output.push_str(&root_text);

    print_tree(&abs_path, "", &config, &mut output, use_colors);

    // Copy to clipboard if requested
    if cli.clipboard {
        match ClipboardContext::new() {
            Ok(mut ctx) => {
                match ctx.set_contents(output.clone()) {
                    Ok(_) => eprintln!("\n✅ Tree output copied to clipboard!"),
                    Err(e) => eprintln!("\n❌ Failed to copy to clipboard: {}", e),
                }
            }
            Err(e) => eprintln!("\n❌ Failed to access clipboard: {}", e),
        }
    }
}