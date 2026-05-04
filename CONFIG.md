# Configuration File Guide - Tree2

## 📁 Config File Locations

Tree2 searches for configuration files in the following locations (in order):

### Windows
1. `%USERPROFILE%\.tree2\.env`
2. `%APPDATA%\.tree2\tree2.toml`
3. `%USERPROFILE%\.tree2\tree2.toml`
4. `%APPDATA%\.tree2\tree2.json`
5. `%USERPROFILE%\.tree2\tree2.json`
6. `%APPDATA%\.tree2\tree2.yml`
7. `%USERPROFILE%\.tree2\tree2.yml`
8. `%APPDATA%\.tree2\tree2.ini`
9. `%USERPROFILE%\.tree2\tree2.ini`

### Linux / macOS
1. `~/.tree2/.env`
2. `~/.config/.tree2/.env`
3. `~/.config/.env`
4. `~/.tree2/tree2.toml`
5. `~/.config/.tree2/tree2.toml`
6. `~/.config/tree2.toml`
7. `~/.tree2/tree2.json`
8. `~/.config/.tree2/tree2.json`
9. `~/.config/tree2.json`
10. `~/.tree2/tree2.yml`
11. `~/.config/.tree2/tree2.yml`
12. `~/.config/tree2.yml`
13. `~/.tree2/tree2.ini`
14. `~/.config/.tree2/tree2.ini`
15. `~/.config/tree2.ini`

**Note:** Tree2 uses the FIRST config file it finds.

---

## 📝 Config File Formats

### TOML (Recommended)

**File:** `tree2.toml`

```toml
# Default folders/files to exclude
default_excludes = [
    "target",
    "build",
    "node_modules",
    ".venv",
    "__pycache__",
    ".idea",
    ".vscode",
    "*.sublime-project",
]

# Ignore files to load
ignore_files = [
    ".gitignore",
    ".dockerignore",
    ".pt",
]

# Exception patterns
exceptions = [
    "*.md",
    "README*",
    "LICENSE*",
]
```

### JSON

**File:** `tree2.json`

```json
{
  "default_excludes": [
    "target",
    "build",
    "node_modules"
  ],
  "ignore_files": [
    ".gitignore",
    ".pt"
  ],
  "exceptions": [
    "*.md",
    "README*"
  ]
}
```

### YAML

**File:** `tree2.yml`

```yaml
default_excludes:
  - target
  - build
  - node_modules

ignore_files:
  - .gitignore
  - .pt

exceptions:
  - "*.md"
  - "README*"
```

---

## 🔧 Configuration Options

### 1. `default_excludes`

Folders and files to **always exclude** (in addition to built-in system folders like `.git`).

**Type:** Array of strings  
**Supports:** Exact match and wildcards (`*`, `?`)

**Examples:**
```toml
default_excludes = [
    # Exact folder names
    "target",
    "node_modules",
    
    # Wildcard patterns
    "*.pyc",
    "*.tmp",
    "build_*",
    
    # IDE files
    ".idea",
    ".vscode",
    "*.sublime-workspace",
]
```

**Built-in Excludes (always applied unless `-a` flag):**
- Version control: `.git`, `.svn`, `.hg`, `.bzr`, `CVS`
- OS files: `.DS_Store`, `Thumbs.db`, `desktop.ini`
- IDE files: `.vscode`, `.idea`, `*.sublime-project`, etc.

### 2. `ignore_files`

List of ignore files to automatically load.

**Type:** Array of strings  
**Default:** `[".gitignore", ".dockerignore", ".pt", ...]`

**Examples:**
```toml
ignore_files = [
    ".gitignore",
    ".dockerignore",
    ".pt",
    ".myignore",        # Custom ignore file
]
```

**Note:** If not specified, uses all default ignore files.

### 3. `exceptions`

Patterns that should NEVER be excluded, even if matched by ignore rules.

**Type:** Array of strings  
**Supports:** Exact match, wildcards, and regex (with `regex:` prefix)

**Examples:**
```toml
exceptions = [
    # Keep all markdown files
    "*.md",
    
    # Keep README files
    "README*",
    
    # Keep specific file
    "important.log",
    
    # Regex: keep all test files
    "regex:^test.*\\.rs$",
]
```

---

## 🎯 Usage Examples

### Example 1: Simple Config

**File:** `~/.tree2/tree2.toml`

```toml
default_excludes = ["target", "node_modules"]
exceptions = ["README.md"]
```

**Effect:**
- Always excludes `target` and `node_modules`
- Keeps `README.md` even if ignored elsewhere

### Example 2: Project-Specific Setup

**File:** `~/.tree2/tree2.toml`

```toml
default_excludes = [
    # Rust
    "target",
    "Cargo.lock",
    
    # Python
    "__pycache__",
    "*.pyc",
    ".venv",
    
    # Node.js
    "node_modules",
    "package-lock.json",
]

ignore_files = [".gitignore", ".dockerignore"]

exceptions = [
    "*.md",
    "*.toml",
    "Cargo.toml",  # Keep this even if excluded
]
```

### Example 3: Minimal Exclusions

**File:** `~/.tree2/tree2.json`

```json
{
  "default_excludes": [],
  "ignore_files": [".gitignore"],
  "exceptions": []
}
```

**Effect:**
- Only uses `.gitignore` rules
- No additional default excludes
- Useful for reviewing everything

---

## 🔄 Priority Order

When determining if a file/folder should be excluded:

1. **Exception patterns** (highest priority) → If matches, ALWAYS show
2. **CLI `--exclude`** → Exact match exclusion
3. **Config `default_excludes`** → Pattern matching
4. **Ignore files** (`.gitignore`, `.pt`, etc.) → Pattern matching
5. **Built-in system excludes** (`.git`, etc.) → Unless `-a` flag

**Example:**
```toml
# Config file
default_excludes = ["*.log"]
exceptions = ["important.log"]
```

```bash
# .gitignore contains: *.log
tree2
```

**Result:**
- `error.log` → Excluded (matches `*.log`)
- `important.log` → **Shown** (exception pattern)
- `debug.log` → Excluded (matches `*.log`)

---

## 🛠️ Creating Config Files

### Quick Setup

```bash
# Linux/macOS
mkdir -p ~/.tree2
cat > ~/.tree2/tree2.toml << 'EOF'
default_excludes = ["target", "node_modules", ".venv"]
ignore_files = [".gitignore", ".pt"]
exceptions = ["*.md", "README*"]
EOF

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.tree2
@"
default_excludes = ["target", "node_modules", ".venv"]
ignore_files = [".gitignore", ".pt"]
exceptions = ["*.md", "README*"]
"@ | Out-File -FilePath $env:USERPROFILE\.tree2\tree2.toml -Encoding UTF8
```

### Verify Config

```bash
# Tree2 will use the config automatically
tree2

# Use -a to see what's being excluded
tree2 -a

# Override config with CLI flags
tree2 -i .gitignore  # Use only .gitignore, ignore config
```

---

## 📊 Config File Examples

### For Rust Projects

```toml
default_excludes = [
    "target",
    "*.lock",
]

ignore_files = [".gitignore"]

exceptions = [
    "Cargo.toml",
    "Cargo.lock",  # Keep lock file visible
    "*.md",
]
```

### For Node.js Projects

```toml
default_excludes = [
    "node_modules",
    "dist",
    "build",
    ".next",
]

ignore_files = [
    ".gitignore",
    ".npmignore",
    ".eslintignore",
]

exceptions = [
    "package.json",
    "package-lock.json",
    "README.md",
]
```

### For Python Projects

```toml
default_excludes = [
    "__pycache__",
    "*.pyc",
    ".venv",
    "venv",
    ".pytest_cache",
    "*.egg-info",
]

ignore_files = [".gitignore"]

exceptions = [
    "requirements.txt",
    "setup.py",
    "README.md",
]
```

### For Multi-Language Projects

```toml
default_excludes = [
    # Rust
    "target",
    
    # Node
    "node_modules",
    "dist",
    
    # Python
    "__pycache__",
    ".venv",
    
    # General
    "build",
    "*.tmp",
]

ignore_files = [
    ".gitignore",
    ".dockerignore",
    ".pt",
]

exceptions = [
    "*.md",
    "*.toml",
    "*.json",
    "Dockerfile",
]
```

---

## 🔍 Troubleshooting

### Config Not Loading?

**Check:**
1. File location is correct
2. File extension matches format (`.toml`, `.json`, `.yml`)
3. File syntax is valid (use online validators)
4. File permissions are readable

**Debug:**
```bash
# Check which config would be used (if any)
# Currently no built-in command, but you can:

# Linux/macOS
ls -la ~/.tree2/
ls -la ~/.config/.tree2/

# Windows
dir %USERPROFILE%\.tree2\
dir %APPDATA%\.tree2\
```

### Config Being Ignored?

**CLI flags override config:**
```bash
# This ignores config ignore_files
tree2 -i .gitignore

# This ignores config exceptions
tree2 -e "*.md"
```

**Solution:** Don't use CLI flags if you want config to apply.

### Invalid Config Format?

**Validate your config:**

**TOML:**
```bash
# Use online validator: https://www.toml-lint.com/
```

**JSON:**
```bash
# Use jq or online validator
cat tree2.json | jq .
```

**YAML:**
```bash
# Use online validator: https://www.yamllint.com/
```

---

## 💡 Best Practices

1. **Use TOML** - Most readable and recommended
2. **Start Simple** - Add excludes as needed
3. **Use Comments** - Document why you exclude things
4. **Test First** - Try patterns with `-e` flag before adding to config
5. **Project-Specific** - Keep project-specific rules in `.pt` file, not config
6. **Version Control** - Add example config to your project's docs

---

## 🔗 Related Documentation

- [QUICKSTART.md](QUICKSTART.md) - Getting started guide
- [EXAMPLES.md](EXAMPLES.md) - Usage examples
- [README.md](README.md) - Main documentation

---

**Config files make Tree2 work perfectly for YOUR workflow!** ⚙️
