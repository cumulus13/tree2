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
use regex::Regex;

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

    /// Ignore file(s) to use (e.g., .gitignore, .dockerignore). If not specified, all ignore files including .pt will be used
    #[arg(short = 'i', long, num_args = 0..)]
    ignore_file: Vec<String>,

    /// Exception patterns (supports wildcards and regex). Patterns matching these will NOT be excluded
    #[arg(short = 'e', long = "exception", num_args = 0..)]
    exceptions: Vec<String>,

    /// Show hidden system folders (.git, .svn, etc.) - by default these are always hidden
    #[arg(short = 'a', long = "all")]
    show_all: bool,
}

struct Config {
    excludes: HashSet<String>,
    root_excludes: HashSet<String>,
    exception_patterns: Vec<Pattern>,
}

enum Pattern {
    Wildcard(String),
    Regex(Regex),
    Exact(String),
}

impl Pattern {
    fn matches(&self, text: &str) -> bool {
        match self {
            Pattern::Wildcard(pattern) => wildcard_match(pattern, text),
            Pattern::Regex(re) => re.is_match(text),
            Pattern::Exact(exact) => text == exact,
        }
    }

    fn from_string(s: &str) -> Result<Self, String> {
        // If it starts with regex: prefix, treat as regex
        if let Some(pattern) = s.strip_prefix("regex:") {
            match Regex::new(pattern) {
                Ok(re) => Ok(Pattern::Regex(re)),
                Err(e) => Err(format!("Invalid regex '{}': {}", pattern, e)),
            }
        }
        // If it contains * or ?, treat as wildcard
        else if s.contains('*') || s.contains('?') {
            Ok(Pattern::Wildcard(s.to_string()))
        }
        // Otherwise, exact match
        else {
            Ok(Pattern::Exact(s.to_string()))
        }
    }
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    
    wildcard_match_recursive(&pattern_chars, &text_chars, 0, 0)
}

fn wildcard_match_recursive(pattern: &[char], text: &[char], p_idx: usize, t_idx: usize) -> bool {
    if p_idx == pattern.len() {
        return t_idx == text.len();
    }

    match pattern[p_idx] {
        '*' => {
            // Try matching zero or more characters
            for i in t_idx..=text.len() {
                if wildcard_match_recursive(pattern, text, p_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        '?' => {
            // Match exactly one character
            if t_idx < text.len() {
                wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
            } else {
                false
            }
        }
        c => {
            // Match exact character
            if t_idx < text.len() && text[t_idx] == c {
                wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
            } else {
                false
            }
        }
    }
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

fn load_ignore_file(path: &Path, filename: &str) -> HashSet<String> {
    let ignore_path = path.join(filename);
    if !ignore_path.exists() {
        return HashSet::new();
    }

    match fs::read_to_string(ignore_path) {
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

fn load_all_ignore_files(path: &Path, specific_files: Option<&[String]>, show_all: bool) -> HashSet<String> {
    let mut all_excludes = HashSet::new();

    // Default excludes - always ignored unless --all flag is used
    if !show_all {
        let default_excludes = vec![
            ".git",
            ".svn",
            ".hg",
            ".bzr",
            "_darcs",
            "CVS",
            ".DS_Store",
            "Thumbs.db",
            "desktop.ini",
        ];
        
        // Add default excludes
        for exclude in default_excludes {
            all_excludes.insert(exclude.to_string());
        }
    }

    // List of common ignore files
    let ignore_files = vec![
        ".gitignore",
        ".dockerignore",
        ".npmignore",
        ".eslintignore",
        ".prettierignore",
        ".hgignore",
        ".terraformignore",
        ".helmignore",
        ".gcloudignore",
        ".cfignore",
        ".slugignore",
        ".pt",  // Default .pt file
    ];

    if let Some(files) = specific_files {
        // If specific files are provided, only load those
        for filename in files {
            let excludes = load_ignore_file(path, filename);
            all_excludes.extend(excludes);
        }
    } else {
        // Load all ignore files
        for filename in ignore_files {
            let excludes = load_ignore_file(path, filename);
            all_excludes.extend(excludes);
        }
    }

    all_excludes
}

fn should_exclude(
    entry: &str, 
    excludes: &HashSet<String>, 
    root_excludes: &HashSet<String>,
    exception_patterns: &[Pattern]
) -> bool {
    // Check if matches any exception pattern - if yes, don't exclude
    for pattern in exception_patterns {
        if pattern.matches(entry) {
            return false;
        }
    }

    // Check manual excludes (exact match)
    if excludes.contains(entry) {
        return true;
    }

    // Check ignore file patterns (support wildcards)
    for pattern in root_excludes {
        // If pattern contains wildcard, use wildcard matching
        if pattern.contains('*') || pattern.contains('?') {
            if wildcard_match(pattern, entry) {
                return true;
            }
        } else {
            // Exact match
            if entry == pattern {
                return true;
            }
        }
    }

    false
}

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

    // Filter out excluded entries first
    let filtered_entries: Vec<_> = entries
        .iter()
        .filter(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            !should_exclude(&file_name, &config.excludes, &config.root_excludes, &config.exception_patterns)
        })
        .collect();

    for (idx, entry) in filtered_entries.iter().enumerate() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        let connector = if idx == filtered_entries.len() - 1 { "└── " } else { "├── " };
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

            let new_prefix = if idx == filtered_entries.len() - 1 {
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

    // Load ignore files
    let ignore_file_excludes = if cli.ignore_file.is_empty() {
        load_all_ignore_files(&abs_path, None, cli.show_all)
    } else {
        load_all_ignore_files(&abs_path, Some(&cli.ignore_file), cli.show_all)
    };

    // Parse exception patterns
    let mut exception_patterns = Vec::new();
    for exception in &cli.exceptions {
        match Pattern::from_string(exception) {
            Ok(pattern) => exception_patterns.push(pattern),
            Err(e) => {
                eprintln!("Warning: {}", e);
            }
        }
    }

    let config = Config {
        excludes: cli.exclude.into_iter().collect(),
        root_excludes: ignore_file_excludes,
        exception_patterns,
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