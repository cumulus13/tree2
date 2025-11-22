# Tree2

A high-performance directory tree visualization tool written in Rust with colors, emojis, and gitignore support. Available as both CLI tool and library crate.

![Tree2 Rust Example](https://via.placeholder.com/800x400.png?text=Tree2+Rust+Directory+Listing+Example)

## Features

- 🎨 **Colorful Output**: Beautiful colored output with emojis for better visualization
- 📊 **File Sizes**: Human-readable file sizes with color-coded values
- 🔒 **Permission Handling**: Gracefully handles permission denied errors
- 📋 **Exclusion Support**: Supports `.gitignore` files and custom exclude patterns
- ⚡ **Blazing Fast**: Optimized Rust implementation for maximum performance
- 📦 **Library & CLI**: Available as both command-line tool and Rust library
- 🦀 **Memory Safe**: Rust's safety guarantees ensure reliability
- 💻 **Cross-Platform**: Works on Windows, macOS, and Linux

## Color Scheme

- **Folders**: Yellow (#FFFF00) with 📁 emoji
- **Files**: Cyan (#00FFFF) with 📄 emoji  
- **Size Values**: Light magenta (red if size is 0)
- **Size Units**: Orange suffix
- **Permission Denied**: Red with 🔒 emoji

## Installation

### Method 1: Install from crates.io
```bash
cargo install tree2
```

### Method 2: Install from source
```bash
git clone https://github.com/cumulus13/tree2
cd tree2
cargo install --path .
```

### Method 3: Build and run directly
```bash
git clone https://github.com/cumulus13/tree2
cd tree2
cargo run --release -- [options]
```

## Usage

### Basic Usage

```bash
# Show current directory tree
tree2

# Show specific directory
tree2 /path/to/directory
```

### With Exclusions

```bash
# Exclude patterns
tree2 -e node_modules,.git,target,dist

# Using long form with space-separated patterns
tree2 --exclude __pycache__ *.tmp temp
```

### Examples

```bash
# Typical Rust project directory
tree2 -e target,.git,node_modules

# With multiple exclude patterns
tree2 -e target .git node_modules __pycache__ dist build

# Specific path with exclusions
tree2 /path/to/project -e target,.git,*.log
```

## Command Line Options

```
Usage: tree2 [OPTIONS] [PATH]

Arguments:
  [PATH]  Target directory [default: .]

Options:
  -e, --exclude <EXCLUDE>...    Exclude patterns or directory names
  -h, --help                   Print help
  -V, --version                Print version
```

## Output Example

```
📂 /home/user/project/
├── 📁 src/
│   ├── 📄 main.rs (12.45 KB)
│   └── 📄 lib.rs (0.00 B)
├── 📁 tests/
│   └── 📄 integration_test.rs (2.10 KB)
├── 📄 Cargo.toml (1.20 KB)
├── 📄 README.md (4.50 KB)
└── 🔒 [Permission Denied]
```

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
tree2 = "0.1.0"
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
        excludes: vec!["target".into(), ".git".into(), "node_modules".into()],
        show_hidden: false,
        max_depth: Some(5),
    };
    
    let tree = TreeBuilder::from_config(config).build();
    tree.print();
    
    // Or get the output as string
    let output = tree.to_string();
    println!("{}", output);
}
```

## Project Structure

```
tree2/
├── Cargo.toml          # Rust package manifest
├── LICENSE             # MIT License
├── README.md           # This file
└── src/
    ├── lib.rs          # Library crate
    ├── main.rs         # Binary crate
    ├── config.rs       # Configuration structures
    ├── tree.rs         # Tree generation logic
    └── colors.rs       # Color and styling utilities
```

## Cargo.toml (for crates.io)

```toml
[package]
name = "tree2"
version = "0.1.0"
edition = "2021"
description = "A beautiful and feature-rich directory tree visualization tool with colors and emojis"
authors = ["Hadi Cahyadi <cumulus13@gmail.com>"]
license = "MIT"
repository = "https://github.com/cumulus13/tree2"
documentation = "https://docs.rs/tree2"
homepage = "https://github.com/cumulus13/tree2"
readme = "README.md"
keywords = ["tree", "directory", "filesystem", "visualization", "cli"]
categories = ["command-line-utilities", "filesystem"]

[dependencies]
clap = { version = "4.0", features = ["derive"] }

[features]
default = ["cli"]
cli = ["clap"]

[[bin]]
name = "tree2"
path = "src/main.rs"

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]
```

## Building from Source

### Prerequisites
- Rust 1.60 or higher
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

# Run benchmarks (if any)
cargo bench
```

## Performance

The Rust version is optimized for performance:
- Zero-cost abstractions
- Minimal memory allocations
- Efficient directory traversal
- Lazy evaluation where possible

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

**Hadi Cahyadi**
- Email: cumulus13@gmail.com
- GitHub: [cumulus13](https://github.com/cumulus13)

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/cumulus13/tree2/issues) page
2. Create a new issue with detailed description
3. Contact: cumulus13@gmail.com
4. Documentation: [docs.rs/tree2](https://docs.rs/tree2)

---

**Enjoy blazing-fast directory visualization with Tree2 Rust!** 🚀🦀