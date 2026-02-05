# Tree2

A high-performance directory tree visualization tool written in Rust with colors, emojis, and comprehensive ignore file support. Available as both CLI tool and library crate.

<p align="center">
  <img src="https://raw.githubusercontent.com/cumulus13/tree2/master/tree2.png" alt="Tree2">
</p>

## 🎦 Demo

<div align="center">
  <a href="https://youtu.be/1BrUiQWi47Y">
    <img src="https://i.ytimg.com/an_webp/1BrUiQWi47Y/mqdefault_6s.webp?du=3000&sqp=CLzY-skG&rs=AOn4CLAfcZjbkZ_1Zw1Yn-fAFJuaUKa88w" alt="How to use mks - tree2 -pt" style="width:100%;">
  </a>
  <br>
  <a href="https://youtu.be/1BrUiQWi47Y">Demo</a>
</div>

## ✨ Features

- 🎨 **Colorful Output**: Beautiful colored output with emojis for better visualization
- 📊 **File Sizes**: Human-readable file sizes with color-coded values
- 🚫 **Multiple Ignore Files**: Automatic support for `.gitignore`, `.dockerignore`, `.pt`, and more
- 🔒 **Default System Exclusions**: `.git`, `.svn`, and other system folders hidden by default
- 🎯 **Exception Patterns**: Wildcard and regex support to override ignore rules
- 📋 **Clipboard Support**: Copy tree output directly to clipboard with `-c` flag
- ⚡ **Blazing Fast**: Optimized Rust implementation for maximum performance
- 🌐 **Cross-Platform**: Works on Windows, macOS, Linux (x86_64, ARM64, ARMv7, i686)
- 📦 **Library & CLI**: Available as both command-line tool and Rust library
- 🦀 **Memory Safe**: Rust's safety guarantees ensure reliability

## 🎨 Color Scheme

- **Folders**: Bright Yellow (#FFFF00) with 📁 emoji
- **Files**: Bright Cyan (#00FFFF) with 📄 emoji  
- **Size Values**: Light magenta (#FF80FF) - White on red background if size is 0
- **Size Units**: Orange (#FFB380)
- **Permission Denied**: White on red background with 🔒 emoji

## 🛠️ Installation

### Using Pre-built Binaries (Recommended)

Download the appropriate binary for your platform from [GitHub Releases](https://github.com/cumulus13/tree2/releases):

#### Linux
```bash
# x86_64 (Standard)
wget https://github.com/cumulus13/tree2/releases/latest/download/tree2-linux-amd64.tar.gz
tar xzf tree2-linux-amd64.tar.gz
sudo mv tree2 /usr/local/bin/

# ARM64 (Raspberry Pi 4+, AWS Graviton)
wget https://github.com/cumulus13/tree2/releases/latest/download/tree2-linux-arm64.tar.gz
tar xzf tree2-linux-arm64.tar.gz
sudo mv tree2 /usr/local/bin/

# ARMv7 (Raspberry Pi 2/3)
wget https://github.com/cumulus13/tree2/releases/latest/download/tree2-linux-armv7.tar.gz
tar xzf tree2-linux-armv7.tar.gz
sudo mv tree2 /usr/local/bin/
```

#### Windows
Download from [releases page](https://github.com/cumulus13/tree2/releases) and extract to a folder in your PATH.

#### macOS
```bash
# Intel Macs
wget https://github.com/cumulus13/tree2/releases/latest/download/tree2-macos-amd64.tar.gz
tar xzf tree2-macos-amd64.tar.gz
sudo mv tree2 /usr/local/bin/

# Apple Silicon (M1/M2/M3)
wget https://github.com/cumulus13/tree2/releases/latest/download/tree2-macos-arm64.tar.gz
tar xzf tree2-macos-arm64.tar.gz
sudo mv tree2 /usr/local/bin/
```

### From crates.io
```bash
cargo install tree2
```

### From Source
```bash
git clone https://github.com/cumulus13/tree2
cd tree2
cargo install --path .
```

## 🌐 Platform Support

Tree2 supports **11 different platforms**:

### Operating Systems
- 🐧 **Linux**: x86_64, ARM64, ARMv7, i686 (+ musl variants)
- 🪟 **Windows**: x86_64, ARM64, i686
- 🍎 **macOS**: Intel (x86_64) and Apple Silicon (ARM64)

For detailed platform information, see [PLATFORM_SUPPORT.md](PLATFORM_SUPPORT.md).

## 📖 Usage

### Basic Usage

```bash
# Show current directory tree
tree2

# Show specific directory
tree2 /path/to/directory

# Copy output to clipboard
tree2 -c

# Show system folders (.git, etc.)
tree2 -a

# Show version info
tree2 -V
```

### Command Line Options

```
Usage: tree2 [OPTIONS] [PATH]

Arguments:
  [PATH]  Target directory [default: .]

Options:
  -a, --all                         Show hidden system folders (.git, .svn, etc.)
  -e, --exception <PATTERNS>...     Exception patterns (wildcards, regex, exact match)
  -i, --ignore-file <FILES>...      Specific ignore files to use
  -c, --clipboard                   Copy result to clipboard
  -x, --exclude <NAMES>...          Exclude directories/files (exact match)
  -V, --version                     Print version information
  -h, --help                        Print help
```

### New Features in v2.0

#### 🌟 Multiple Ignore Files Support

Tree2 automatically loads ALL common ignore files:
- `.gitignore`, `.dockerignore`, `.npmignore`
- `.eslintignore`, `.prettierignore`, `.hgignore`
- `.terraformignore`, `.helmignore`, `.gcloudignore`
- `.cfignore`, `.slugignore`
- **`.pt`** (custom ignore file for tree2)

**Important:** All patterns in ignore files support wildcards (`*` and `?`). For example, `*.exe` in `.gitignore` will match all `.exe` files.

#### 🛡️ Default System Folder Exclusion

Tree2 **automatically hides** these system folders and files by default:
- `.git`, `.svn`, `.hg`, `.bzr`, `_darcs`, `CVS`
- `.DS_Store`, `Thumbs.db`, `desktop.ini`

**To show these folders:**
```bash
tree2 -a           # Show all including system folders
tree2 --all        # Same as above
```

#### 🎯 Exception Patterns (New!)

Exclude patterns from being ignored using the `-e` or `--exception` flag. Supports three types:

**Wildcard Patterns:**
```bash
# Keep all .log files even if ignored
tree2 -e "*.log"

# Keep specific pattern
tree2 -e "important_*.txt"

# Multiple patterns
tree2 -e "*.log" "*.tmp" "debug_*"
```

**Exact Match:**
```bash
# Keep specific file/folder
tree2 -e "node_modules"
tree2 -e ".env"
```

**Regex Patterns:**
Use `regex:` prefix for complex patterns:
```bash
# Keep all files ending with numbers
tree2 -e "regex:.*\d+$"

# Keep files matching pattern
tree2 -e "regex:^(test|spec)_.*\.rs$"
```

#### 📝 Selective Ignore File Usage

You can specify which ignore files to use:

```bash
# Use only .gitignore
tree2 -i .gitignore

# Use both .gitignore and .dockerignore
tree2 -i .gitignore .dockerignore

# Use custom ignore file
tree2 -i .myignore
```

### Advanced Examples

```bash
# Use only .gitignore but keep all .log files
tree2 -i .gitignore -e "*.log"

# Use multiple ignore files with exceptions
tree2 -i .gitignore .dockerignore -e "*.log" "*.md" "important_*"

# Exclude specific folders but keep exceptions
tree2 --exclude target build -e "target/debug/important_file.txt"

# Complex regex exception
tree2 -e "regex:^(test|spec).*\.(rs|py)$"

# Show all including .git
tree2 -a

# Combined flags
tree2 -a -i .gitignore -e "*.log"
```

### 📄 Creating a .pt File

The `.pt` file works like `.gitignore` but is specific to tree2:

```bash
# Create .pt file
cat > .pt << EOF
# Custom ignores for tree2
*.tmp
*.cache
local_settings.json
dev_notes/
EOF

# tree2 will automatically use it
tree2
```

## 💡 Pattern Matching Examples

### Wildcard Examples:
- `*.log` - matches any file ending with .log
- `test_*` - matches any file starting with test_
- `*_backup.*` - matches any file with _backup before extension
- `?.txt` - matches single character followed by .txt

### Regex Examples:
- `regex:.*\d+$` - matches files ending with numbers
- `regex:^test.*\.rs$` - matches Rust test files
- `regex:(debug|test)_.*` - matches files starting with debug_ or test_

## 📋 Output Example

```
📂 /home/user/project/
├── 📁 src/
│   ├── 📄 main.rs (12.45 KB)
│   └── 📄 lib.rs (2.34 KB)
├── 📁 tests/
│   └── 📄 integration_test.rs (2.10 KB)
├── 📄 Cargo.toml (1.20 KB)
├── 📄 README.md (4.50 KB)
└── 🔒 [Permission Denied]

# .git folder is hidden by default ✓
```

### Clipboard Output

When using `-c` flag, the output copied to clipboard is **plain text without ANSI color codes**, making it perfect for:
- Pasting into documentation
- Sharing in emails or chat
- Including in Markdown files
- Code reviews and discussions

## 🚀 Performance

The Rust version is optimized for performance:
- Zero-cost abstractions
- Minimal memory allocations
- Efficient directory traversal
- Fast pattern matching with HashSet
- Optimized wildcard matching algorithm
- Concurrent ignore file loading

## 📚 Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
tree2 = "2.0"
```

### Basic Example

```rust
use tree2::TreeBuilder;

fn main() {
    let tree = TreeBuilder::new()
        .path(".")
        .excludes(vec!["target", ".git"])
        .build();
    
    tree.print();
}
```

### Advanced Example

```rust
use tree2::{TreeBuilder, TreeConfig};

fn main() {
    let config = TreeConfig {
        path: ".".into(),
        excludes: vec!["target".into(), ".git".into()],
        ignore_files: vec![".gitignore".into(), ".dockerignore".into()],
        exceptions: vec!["*.log".into()],
        show_all: false,
        max_depth: Some(5),
    };
    
    let tree = TreeBuilder::from_config(config).build();
    tree.print();
    
    // Or get the output as string
    let output = tree.to_string();
    println!("{}", output);
}
```

## 🔧 Dependencies

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
dunce = "1.0.4"
cli-clipboard = "0.4"
clap-version-flag = "1.0.5"
regex = "1.10"
```

## 🏗️ Building from Source

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Build Steps
```bash
git clone https://github.com/cumulus13/tree2
cd tree2

# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with arguments
cargo run --release -- -e target -c
```

### Cross-Compilation

To build for a different platform:

```bash
# Install target
rustup target add aarch64-unknown-linux-gnu

# Build
cargo build --release --target aarch64-unknown-linux-gnu

# Or use cross for easier cross-compilation
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

## 🤝 Contributing

We welcome contributions!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 👤 Author

**Hadi Cahyadi**
- Email: cumulus13@gmail.com
- GitHub: [cumulus13](https://github.com/cumulus13)

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

## 💬 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/cumulus13/tree2/issues) page
2. Create a new issue with detailed description
3. Read the [Documentation](PLATFORM_SUPPORT.md)
4. Contact: cumulus13@gmail.com

## 📝 Changelog

### v2.0.12 (Latest)
- 🐛 **Fixed**: Wildcard pattern matching in ignore files (`.gitignore`, `.pt`, etc.)
- 🐛 **Fixed**: Tree connector logic for correct display
- ✨ **Added**: Default system folder exclusion (`.git`, `.svn`, etc.)
- ✨ **Added**: `-a/--all` flag to show hidden system folders
- 📚 **Improved**: Documentation and troubleshooting guides

### v1.0.10
- ✨ **Added**: Multiple ignore file support (`.gitignore`, `.dockerignore`, `.pt`, etc.)
- ✨ **Added**: Exception patterns with wildcard, regex, and exact match
- ✨ **Added**: Selective ignore file usage with `-i` flag
- ✨ **Added**: Custom `.pt` ignore file support
- 🚀 **Improved**: Pattern matching algorithm
- 📚 **Added**: Comprehensive documentation (QUICKSTART, EXAMPLES, TROUBLESHOOTING)

### v1.0.9
- ✅ Fixed exact match exclusion (`.git` no longer excludes `.github`)
- ✨ Added clipboard support with `-c` flag
- 📝 Improved version info with `-V` flag
- 🎨 Better color scheme with true color support
- ⚡ Performance optimizations

## 🔗 Links

- [QUICKSTART Guide](QUICKSTART.md) - Get started in 5 minutes
- [Examples](EXAMPLES.md) - 48+ practical usage examples
- [Platform Support](PLATFORM_SUPPORT.md) - Detailed platform information
- [Changelog](CHANGELOG.md) - Full version history

---

**Enjoy blazing-fast directory visualization with Tree2!** 🚀🦀
