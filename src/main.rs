// File: src\main.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-11-22
// Description: A high-performance directory tree visualization tool written in Rust with colors, emojis, and gitignore support. Available as both CLI tool and library crate.
// License: MIT

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
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
const COLOR_BRIGHT_GREEN: &str = "\x1b[38;2;0;255;128m";   // #00FF80 for symlinks
const COLOR_GRAY: &str = "\x1b[38;2;160;160;160m";         // #A0A0A0 for meta info
const COLOR_BRIGHT_WHITE: &str = "\x1b[38;2;230;230;230m"; // #E6E6E6 for permissions

#[derive(Parser)]
#[command(name = "tree2")]
#[command(about = "Print directory tree with file sizes, exclusions, and .gitignore support\nFully compatible with Linux tree command options.")]
#[command(disable_version_flag = true)]
struct Cli {
    #[arg(short = 'V', long = "version", action = ArgAction::SetTrue)]
    version: bool,

    #[arg(default_value = ".")]
    path: String,

    // ── Original tree2 flags ──────────────────────────────────────────────────

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

    // ── Linux tree compatibility flags ───────────────────────────────────────

    /// List directories only (no files)
    #[arg(short = 'd', long = "dirs-only")]
    dirs_only: bool,

    /// Print the full path prefix for each file
    #[arg(short = 'f', long = "full-path")]
    full_path: bool,

    /// Follow symbolic links like directories
    #[arg(short = 'l', long = "follow-links")]
    follow_links: bool,

    /// Max display depth of the directory tree
    #[arg(short = 'L', long = "level")]
    level: Option<usize>,

    /// List only files matching the given wildcard pattern (e.g. "*.rs")
    #[arg(short = 'P', long = "pattern")]
    pattern: Option<String>,

    /// Exclude files matching the given wildcard pattern (e.g. "*.o")
    #[arg(short = 'I', long = "ignore-pattern")]
    ignore_pattern: Option<String>,

    /// Ignore case when matching -P / -I patterns
    #[arg(long = "ignore-case")]
    ignore_case: bool,

    /// List directories before files
    #[arg(long = "dirsfirst")]
    dirsfirst: bool,

    /// Sort output by last modification time (oldest first)
    #[arg(short = 't', long = "sort-time")]
    sort_time: bool,

    /// Reverse the order of the sort
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    /// Print the protections (permissions) for each file (like ls -l)
    #[arg(short = 'p', long = "protections")]
    protections: bool,

    /// Display file owner name (or UID if name unavailable)
    #[arg(short = 'u', long = "owner")]
    owner: bool,

    /// Display file group name (or GID if name unavailable)
    #[arg(short = 'g', long = "group")]
    group: bool,

    /// Print size of each file in bytes
    #[arg(short = 's', long = "size")]
    size_bytes: bool,

    /// Print size in human-readable format (default behaviour for tree2; explicit flag for compat)
    #[arg(short = 'h', long = "human-readable")]
    human_readable: bool,

    /// Like -h but use SI units (powers of 1000 instead of 1024)
    #[arg(long = "si")]
    si_units: bool,

    /// Print last modification date/time for each file
    #[arg(short = 'D', long = "date")]
    date: bool,

    /// Append indicators: '/' dirs, '*' executables, '@' symlinks, '|' FIFOs, '=' sockets
    #[arg(short = 'F', long = "classify")]
    classify: bool,

    /// Do not descend directories with more than # files
    #[arg(long = "filelimit")]
    filelimit: Option<usize>,

    /// Prune empty directories from the output
    #[arg(long = "prune")]
    prune: bool,

    /// For each directory report its size as accumulation of all contained file sizes
    #[arg(long = "du")]
    du: bool,

    /// Turn off the file/directory count report at the end
    #[arg(long = "noreport")]
    noreport: bool,

    /// Output to filename instead of stdout (plain text, no ANSI codes)
    #[arg(short = 'o', long = "output")]
    output_file: Option<String>,

    /// Turn off ANSI colors
    #[arg(short = 'n', long = "nocolor")]
    nocolor: bool,

    /// Print non-printable characters as '?'
    #[arg(short = 'q', long = "quote-chars")]
    quote_chars: bool,

    /// Quote filenames with double quotes
    #[arg(short = 'Q', long = "quote")]
    quote: bool,

    /// Stay on the current filesystem only (do not cross mount points)
    #[arg(short = 'x', long = "xdev")]
    xdev: bool,

    /// Print the inode number for each file
    #[arg(long = "inodes")]
    inodes: bool,

    /// Print the device number for each file
    #[arg(long = "device")]
    device: bool,
}

// ── Counting state ────────────────────────────────────────────────────────────

struct Counts {
    dirs: u64,
    files: u64,
}

// ── Config (shared traversal settings) ───────────────────────────────────────

struct Config {
    excludes: HashSet<String>,
    root_excludes: HashSet<String>,
    exception_patterns: Vec<Pattern>,

    // linux-tree compat
    dirs_only: bool,
    full_path: bool,
    follow_links: bool,
    level: Option<usize>,
    pattern: Option<WildPattern>,
    ignore_pattern: Option<WildPattern>,
    ignore_case: bool,
    dirsfirst: bool,
    sort_time: bool,
    reverse: bool,
    protections: bool,
    owner: bool,
    group: bool,
    size_bytes: bool,
    human_readable: bool,
    si_units: bool,
    date: bool,
    classify: bool,
    filelimit: Option<usize>,
    prune: bool,
    du: bool,
    quote_chars: bool,
    quote: bool,
    xdev: bool,
    inodes: bool,
    device: bool,

    // root device id (for xdev)
    root_dev: Option<u64>,
}

// ── Pattern helpers ───────────────────────────────────────────────────────────

struct WildPattern {
    raw: String,
    ignore_case: bool,
}

impl WildPattern {
    fn new(s: &str, ignore_case: bool) -> Self {
        WildPattern { raw: s.to_string(), ignore_case }
    }

    fn matches(&self, text: &str) -> bool {
        let (pattern, haystack) = if self.ignore_case {
            (self.raw.to_lowercase(), text.to_lowercase())
        } else {
            (self.raw.clone(), text.to_string())
        };
        wildcard_match(&pattern, &haystack)
    }
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
        if let Some(pattern) = s.strip_prefix("regex:") {
            match Regex::new(pattern) {
                Ok(re) => Ok(Pattern::Regex(re)),
                Err(e) => Err(format!("Invalid regex '{}': {}", pattern, e)),
            }
        } else if s.contains('*') || s.contains('?') {
            Ok(Pattern::Wildcard(s.to_string()))
        } else {
            Ok(Pattern::Exact(s.to_string()))
        }
    }
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pc: Vec<char> = pattern.chars().collect();
    let tc: Vec<char> = text.chars().collect();
    wm_rec(&pc, &tc, 0, 0)
}

fn wm_rec(pattern: &[char], text: &[char], pi: usize, ti: usize) -> bool {
    if pi == pattern.len() { return ti == text.len(); }
    match pattern[pi] {
        '*' => (ti..=text.len()).any(|i| wm_rec(pattern, text, pi + 1, i)),
        '?' => ti < text.len() && wm_rec(pattern, text, pi + 1, ti + 1),
        c   => ti < text.len() && text[ti] == c && wm_rec(pattern, text, pi + 1, ti + 1),
    }
}

// ── Size formatting ───────────────────────────────────────────────────────────

fn human_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut s = size as f64;
    for unit in &units {
        if s < 1024.0 { return format!("{:.2} {}", s, unit); }
        s /= 1024.0;
    }
    format!("{:.2} PB", s)
}

fn human_size_si(size: u64) -> String {
    let units = ["B", "kB", "MB", "GB", "TB"];
    let mut s = size as f64;
    for unit in &units {
        if s < 1000.0 { return format!("{:.2} {}", s, unit); }
        s /= 1000.0;
    }
    format!("{:.2} PB", s)
}

// ── Ignore file loading ───────────────────────────────────────────────────────

fn load_ignore_file(path: &Path, filename: &str) -> HashSet<String> {
    let ignore_path = path.join(filename);
    if !ignore_path.exists() { return HashSet::new(); }
    match fs::read_to_string(ignore_path) {
        Ok(content) => content.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| l.trim_end_matches('/').to_string())
            .collect(),
        Err(_) => HashSet::new(),
    }
}

fn load_all_ignore_files(path: &Path, specific_files: Option<&[String]>, show_all: bool) -> HashSet<String> {
    let mut all = HashSet::new();
    if !show_all {
        for s in &[".git",".svn",".hg",".bzr","_darcs","CVS",".DS_Store","Thumbs.db","desktop.ini"] {
            all.insert(s.to_string());
        }
    }
    let default_files = [
        ".gitignore",".dockerignore",".npmignore",".eslintignore",".prettierignore",
        ".hgignore",".terraformignore",".helmignore",".gcloudignore",".cfignore",
        ".slugignore",".pt",
    ];
    if let Some(files) = specific_files {
        for f in files { all.extend(load_ignore_file(path, f)); }
    } else {
        for f in &default_files { all.extend(load_ignore_file(path, f)); }
    }
    all
}

fn should_exclude(
    entry: &str,
    excludes: &HashSet<String>,
    root_excludes: &HashSet<String>,
    exception_patterns: &[Pattern],
) -> bool {
    for p in exception_patterns { if p.matches(entry) { return false; } }
    if excludes.contains(entry) { return true; }
    for pattern in root_excludes {
        if pattern.contains('*') || pattern.contains('?') {
            if wildcard_match(pattern, entry) { return true; }
        } else if entry == pattern { return true; }
    }
    false
}

// ── Permission string (like ls -l) ───────────────────────────────────────────

fn permission_string(meta: &fs::Metadata) -> String {
    let mode = meta.permissions().mode();
    let file_type = if meta.is_dir() { 'd' }
        else if meta.file_type().is_symlink() { 'l' }
        else { '-' };
    let bits = [
        (0o400,'r'),(0o200,'w'),(0o100,'x'),
        (0o040,'r'),(0o020,'w'),(0o010,'x'),
        (0o004,'r'),(0o002,'w'),(0o001,'x'),
    ];
    let perm: String = bits.iter().map(|(b, c)| if mode & b != 0 { *c } else { '-' }).collect();
    format!("[{}{}]", file_type, perm)
}

// ── Owner / group ─────────────────────────────────────────────────────────────

fn owner_name(uid: u32) -> String {
    // Try to resolve via /etc/passwd; fall back to UID number
    if let Ok(content) = fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(u) = parts[2].parse::<u32>() {
                    if u == uid { return parts[0].to_string(); }
                }
            }
        }
    }
    uid.to_string()
}

fn group_name(gid: u32) -> String {
    if let Ok(content) = fs::read_to_string("/etc/group") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(g) = parts[2].parse::<u32>() {
                    if g == gid { return parts[0].to_string(); }
                }
            }
        }
    }
    gid.to_string()
}

// ── Date formatting ───────────────────────────────────────────────────────────

fn format_mtime(meta: &fs::Metadata) -> String {
    use std::time::UNIX_EPOCH;
    if let Ok(mtime) = meta.modified() {
        if let Ok(dur) = mtime.duration_since(UNIX_EPOCH) {
            let secs = dur.as_secs();
            // Simple RFC-like format: YYYY-MM-DD HH:MM
            let (y, mo, d, h, mi) = secs_to_ymd_hm(secs);
            return format!("[{:04}-{:02}-{:02} {:02}:{:02}]", y, mo, d, h, mi);
        }
    }
    "[----/--/-- --:--]".to_string()
}

fn secs_to_ymd_hm(secs: u64) -> (u64, u64, u64, u64, u64) {
    let mins  = secs / 60;
    let hours = mins / 60;
    let days  = hours / 24;
    let mi    = mins % 60;
    let h     = hours % 24;

    // Gregorian calendar calculation
    let mut y: u64 = 1970;
    let mut remaining = days;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        let days_in_year = if leap { 366 } else { 365 };
        if remaining < days_in_year { break; }
        remaining -= days_in_year;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let month_days = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut mo: u64 = 1;
    for &md in &month_days {
        if remaining < md { break; }
        remaining -= md;
        mo += 1;
    }
    let d = remaining + 1;
    (y, mo, d, h, mi)
}

// ── Classify indicator ────────────────────────────────────────────────────────

fn classify_indicator(meta: &fs::Metadata) -> &'static str {
    if meta.is_dir() { return "/"; }
    let ft = meta.file_type();
    if ft.is_symlink() { return "@"; }
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileTypeExt;
        if ft.is_fifo()   { return "|"; }
        if ft.is_socket() { return "="; }
        let mode = meta.permissions().mode();
        if mode & 0o111 != 0 { return "*"; }
    }
    ""
}

// ── Sanitize filename ─────────────────────────────────────────────────────────

fn sanitize_name(name: &str, quote_chars: bool, quote: bool) -> String {
    let s = if quote_chars {
        name.chars().map(|c| if c.is_control() { '?' } else { c }).collect()
    } else {
        name.to_string()
    };
    if quote { format!("\"{}\"", s) } else { s }
}

// ── Accumulate dir size (--du) ────────────────────────────────────────────────

fn accumulate_size(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    total += accumulate_size(&entry.path());
                } else {
                    total += meta.len();
                }
            }
        }
    }
    total
}

// ── Meta prefix builder ───────────────────────────────────────────────────────

fn build_meta_prefix(meta: &fs::Metadata, config: &Config) -> String {
    let mut parts = Vec::new();

    if config.inodes  { parts.push(format!("{}", meta.ino())); }
    if config.device  { parts.push(format!("{}", meta.dev())); }
    if config.protections { parts.push(permission_string(meta)); }
    if config.owner   { parts.push(owner_name(meta.uid())); }
    if config.group   { parts.push(group_name(meta.gid())); }
    if config.size_bytes {
        parts.push(format!("{}", meta.len()));
    } else if config.si_units {
        let sz = human_size_si(meta.len());
        parts.push(format!("[{}]", sz));
    } else if config.human_readable {
        let sz = human_size(meta.len());
        parts.push(format!("[{}]", sz));
    }
    if config.date { parts.push(format_mtime(meta)); }

    if parts.is_empty() { String::new() } else { format!("{} ", parts.join(" ")) }
}

// ── Core tree traversal ───────────────────────────────────────────────────────

fn print_tree(
    path: &Path,
    prefix: &str,
    config: &Config,
    output: &mut String,
    use_colors: bool,
    depth: usize,
    counts: &mut Counts,
    root_path: &Path,
) {
    // Depth limit
    if let Some(max) = config.level {
        if depth > max { return; }
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => {
            let mut v: Vec<_> = e.collect::<Result<Vec<_>, _>>().unwrap_or_default();

            // Sort
            if config.sort_time {
                v.sort_by(|a, b| {
                    let ta = a.metadata().and_then(|m| m.modified()).ok();
                    let tb = b.metadata().and_then(|m| m.modified()).ok();
                    ta.cmp(&tb)
                });
            } else {
                v.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            }

            if config.reverse { v.reverse(); }

            // dirsfirst
            if config.dirsfirst {
                v.sort_by(|a, b| {
                    let ad = a.metadata().map(|m| m.is_dir()).unwrap_or(false);
                    let bd = b.metadata().map(|m| m.is_dir()).unwrap_or(false);
                    bd.cmp(&ad)
                });
            }

            v
        }
        Err(_) => {
            let txt = format!("{}└── 🔒 [Permission Denied]\n", prefix);
            emit(&txt, &txt, output, use_colors, COLOR_WHITE_ON_RED);
            return;
        }
    };

    // filelimit check
    if let Some(limit) = config.filelimit {
        if entries.len() > limit { return; }
    }

    // Filter entries
    let filtered: Vec<_> = entries.iter().filter(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        if should_exclude(&name, &config.excludes, &config.root_excludes, &config.exception_patterns) {
            return false;
        }
        let meta = match e.metadata() { Ok(m) => m, Err(_) => return true };

        // xdev: stay on same filesystem
        if config.xdev {
            if let Some(root_dev) = config.root_dev {
                if meta.dev() != root_dev { return false; }
            }
        }

        // -I pattern: exclude matching
        if let Some(ref pat) = config.ignore_pattern {
            if pat.matches(&name) { return false; }
        }

        // -d: only dirs
        if config.dirs_only && !meta.is_dir() { return false; }

        // -P pattern: only show matching files (dirs always pass so we can recurse)
        if let Some(ref pat) = config.pattern {
            if !meta.is_dir() && !pat.matches(&name) { return false; }
        }

        true
    }).collect();

    // --prune: if all children would be empty dirs, skip (simple one-level check)
    // We do a two-pass for prune: collect valid entries then emit
    for (idx, entry) in filtered.iter().enumerate() {
        let name = entry.file_name().to_string_lossy().to_string();
        let display_name = sanitize_name(&name, config.quote_chars, config.quote);
        let connector = if idx == filtered.len() - 1 { "└── " } else { "├── " };

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let full = if config.full_path {
            entry.path().to_string_lossy().to_string()
        } else {
            display_name.clone()
        };

        if meta.is_dir() {
            // --prune: skip empty dirs
            if config.prune {
                let child_count = fs::read_dir(entry.path())
                    .map(|rd| rd.count())
                    .unwrap_or(0);
                if child_count == 0 { continue; }
            }

            counts.dirs += 1;

            let meta_str = if config.du {
                let sz = accumulate_size(&entry.path());
                let s = if config.si_units { human_size_si(sz) } else { human_size(sz) };
                format!("[{}] ", s)
            } else {
                build_meta_prefix(&meta, config)
            };

            let indicator = if config.classify { "/" } else { "" };
            let plain = format!("{}{}{}📁 {}{}/\n", prefix, connector, meta_str, full, indicator);

            if use_colors {
                // meta in gray, folder name in bright yellow
                let colored = format!(
                    "{}{}{}{}{}{}📁 {}{}/{}{}",
                    prefix, connector,
                    COLOR_GRAY, meta_str, COLOR_RESET,
                    COLOR_BRIGHT_YELLOW, full, indicator,
                    COLOR_RESET, "\n"
                );
                print!("{}", colored);
                output.push_str(&plain);
            } else {
                print!("{}", plain);
                output.push_str(&plain);
            }

            let new_prefix = if idx == filtered.len() - 1 {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree(&entry.path(), &new_prefix, config, output, use_colors, depth + 1, counts, root_path);

        } else {
            counts.files += 1;

            // Check if it's a symlink
            let is_symlink = meta.file_type().is_symlink();

            let meta_str = build_meta_prefix(&meta, config);
            let sz = meta.len();

            let indicator = if config.classify { classify_indicator(&meta) } else { "" };

            // Default: always show human-readable size (original tree2 behavior)
            // unless user asked for raw bytes with -s
            let size_display = if config.size_bytes {
                format!("{} B", sz)
            } else if config.si_units {
                human_size_si(sz)
            } else {
                // Always show human-readable (original tree2 default)
                human_size(sz)
            };

            let parts: Vec<&str> = size_display.split_whitespace().collect();
            let (size_val, size_unit) = if parts.len() >= 2 {
                (parts[0], parts[1])
            } else {
                (parts[0], "")
            };

            // symlink target
            let symlink_info = if is_symlink {
                if let Ok(target) = fs::read_link(entry.path()) {
                    format!(" -> {}", target.display())
                } else { String::new() }
            } else { String::new() };

            let plain = format!(
                "{}{}{}📄 {}{} ({} {}){}\n",
                prefix, connector, meta_str, full, indicator, size_val, size_unit, symlink_info
            );

            if use_colors {
                let name_color = if is_symlink { COLOR_BRIGHT_GREEN } else { COLOR_BRIGHT_CYAN };
                // meta prefix
                print!("{}{}", prefix, connector);
                if !meta_str.is_empty() {
                    print!("{}{}{}", COLOR_GRAY, meta_str, COLOR_RESET);
                }
                print!("{}📄 {}{}{}", name_color, full, indicator, COLOR_RESET);
                if !symlink_info.is_empty() {
                    print!("{}{}{}", COLOR_BRIGHT_GREEN, symlink_info, COLOR_RESET);
                }
                print!(" (");
                if sz == 0 {
                    print!("{}{}", COLOR_WHITE_ON_RED, size_val);
                } else {
                    print!("{}{}", COLOR_LIGHT_MAGENTA_TRUE, size_val);
                }
                print!("{} ", COLOR_RESET);
                print!("{}{}", COLOR_ORANGE, size_unit);
                println!("{})", COLOR_RESET);
                output.push_str(&plain);
            } else {
                print!("{}", plain);
                output.push_str(&plain);
            }
        }
    }
}

// ── Small emit helper ─────────────────────────────────────────────────────────

fn emit(plain: &str, _colored_hint: &str, output: &mut String, use_colors: bool, color: &str) {
    if use_colors {
        print!("{}{}{}", color, plain, COLOR_RESET);
    } else {
        print!("{}", plain);
    }
    output.push_str(plain);
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "-V" || args[1] == "--version") {
        let version = colorful_version!();
        version.print_and_exit();
    }

    let cli = Cli::parse();
    let version_str = colorful_version!();

    if cli.version {
        println!("{}", version_str);
        std::process::exit(0);
    }

    let path = PathBuf::from(&cli.path);
    let abs_path = match canonicalize(&path) {
        Ok(p) => p,
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    };

    // Load ignore files
    let ignore_file_excludes = if cli.ignore_file.is_empty() {
        load_all_ignore_files(&abs_path, None, cli.show_all)
    } else {
        load_all_ignore_files(&abs_path, Some(&cli.ignore_file), cli.show_all)
    };

    // Parse exception patterns
    let mut exception_patterns = Vec::new();
    for exc in &cli.exceptions {
        match Pattern::from_string(exc) {
            Ok(p) => exception_patterns.push(p),
            Err(e) => eprintln!("Warning: {}", e),
        }
    }

    // Parse -P / -I wildcard patterns
    let pattern = cli.pattern.as_deref().map(|s| WildPattern::new(s, cli.ignore_case));
    let ignore_pattern = cli.ignore_pattern.as_deref().map(|s| WildPattern::new(s, cli.ignore_case));

    // root device id for --xdev
    let root_dev = if cli.xdev {
        fs::metadata(&abs_path).ok().map(|m| m.dev())
    } else {
        None
    };

    let config = Config {
        excludes: cli.exclude.into_iter().collect(),
        root_excludes: ignore_file_excludes,
        exception_patterns,
        dirs_only:      cli.dirs_only,
        full_path:      cli.full_path,
        follow_links:   cli.follow_links,
        level:          cli.level,
        pattern,
        ignore_pattern,
        ignore_case:    cli.ignore_case,
        dirsfirst:      cli.dirsfirst,
        sort_time:      cli.sort_time,
        reverse:        cli.reverse,
        protections:    cli.protections,
        owner:          cli.owner,
        group:          cli.group,
        size_bytes:     cli.size_bytes,
        human_readable: cli.human_readable,
        si_units:       cli.si_units,
        date:           cli.date,
        classify:       cli.classify,
        filelimit:      cli.filelimit,
        prune:          cli.prune,
        du:             cli.du,
        quote_chars:    cli.quote_chars,
        quote:          cli.quote,
        xdev:           cli.xdev,
        inodes:         cli.inodes,
        device:         cli.device,
        root_dev,
    };

    let mut output = String::new();
    // Colors: off if --nocolor, clipboard, or output file
    let use_colors = !cli.clipboard && !cli.nocolor && cli.output_file.is_none();

    // Root line
    let root_text = format!("📂 {}/\n", abs_path.display());
    if use_colors {
        print!("{}{}{}", COLOR_BRIGHT_YELLOW, root_text, COLOR_RESET);
    } else {
        print!("{}", root_text);
    }
    output.push_str(&root_text);

    let mut counts = Counts { dirs: 0, files: 0 };
    print_tree(&abs_path, "", &config, &mut output, use_colors, 1, &mut counts, &abs_path);

    // Summary report (linux tree default: "X directories, Y files")
    if !cli.noreport {
        let report = format!("\n{} {}, {} {}\n",
            counts.dirs,
            if counts.dirs == 1 { "directory" } else { "directories" },
            counts.files,
            if counts.files == 1 { "file" } else { "files" },
        );
        if use_colors {
            print!("{}{}{}", COLOR_BRIGHT_WHITE, report, COLOR_RESET);
        } else {
            print!("{}", report);
        }
        output.push_str(&report);
    }

    // -o: output to file (no ANSI codes — output string is already plain)
    if let Some(ref outfile) = cli.output_file {
        match fs::write(outfile, &output) {
            Ok(_)  => eprintln!("✅ Output written to '{}'", outfile),
            Err(e) => eprintln!("❌ Failed to write '{}': {}", outfile, e),
        }
    }

    // -c: copy to clipboard
    if cli.clipboard {
        match ClipboardContext::new() {
            Ok(mut ctx) => match ctx.set_contents(output.clone()) {
                Ok(_)  => eprintln!("\n✅ Tree output copied to clipboard!"),
                Err(e) => eprintln!("\n❌ Failed to copy to clipboard: {}", e),
            },
            Err(e) => eprintln!("\n❌ Failed to access clipboard: {}", e),
        }
    }
}
