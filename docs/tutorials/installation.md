---
layout: default
title: Installation Guide
parent: Tutorials
nav_order: 3
---

# Installation

## Quick Install

The fastest way to get started:

```bash
cargo install cc-search
```

That's it! You're ready to search.

## Installation Methods

### From crates.io (Recommended)

```bash
# Install latest stable version
cargo install cc-search

# Update to latest version
cargo install cc-search --force

# Install specific version
cargo install cc-search --version 0.5.3
```

**Requirements:**
- Rust 1.70 or later
- Cargo (comes with Rust)

**Install Rust** (if needed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### From Source

For latest features or development:

```bash
# Clone repository
git clone https://github.com/BeaconBay/cc
cd cc

# Install from source
cargo install --path cc-cli

# Or just build (doesn't install)
cargo build --release
./target/release/cc --version
```

**Benefits:**
- Latest features before official release
- Can modify and customize
- Contribute improvements

### Package Managers

**Coming Soon:**

```bash
# Homebrew (macOS/Linux) - In development
brew install cc-search

# APT (Debian/Ubuntu) - In development
apt install cc-search

# DNF/YUM (Fedora/RHEL) - In development
dnf install cc-search

# Scoop (Windows) - In development
scoop install cc-search

# Chocolatey (Windows) - In development
choco install cc-search
```

For now, use `cargo install cc-search` on all platforms.

## Platform-Specific Instructions

### macOS

#### Using Cargo (Recommended)

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install cc
cargo install cc-search

# Verify installation
cc --version
```

#### Using Homebrew (Coming Soon)

```bash
# Not yet available - use cargo for now
brew install cc-search
```

**Troubleshooting:**

If you get permission errors:
```bash
# Ensure cargo bin is in PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Linux

#### Using Cargo (Works on all distros)

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install cc
cargo install cc-search

# Verify installation
cc --version
```

#### Debian/Ubuntu (Coming Soon)

```bash
# Not yet available - use cargo for now
sudo apt update
sudo apt install cc-search
```

#### Fedora/RHEL (Coming Soon)

```bash
# Not yet available - use cargo for now
sudo dnf install cc-search
```

#### Arch Linux (Coming Soon)

```bash
# Not yet available - use cargo for now
yay -S cc-search
```

**Troubleshooting:**

If you get linker errors:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

### Windows

#### Using Cargo (Recommended)

```powershell
# Install Rust if needed
# Download and run: https://rustup.rs/

# Install cc
cargo install cc-search

# Verify installation
cc --version
```

#### Using Scoop (Coming Soon)

```powershell
# Not yet available - use cargo for now
scoop install cc-search
```

#### Using Chocolatey (Coming Soon)

```powershell
# Not yet available - use cargo for now
choco install cc-search
```

**Troubleshooting:**

If you get Visual Studio errors:
1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Or install full Visual Studio with C++ support
3. Restart terminal after installation

**PATH Issues:**
```powershell
# Add cargo bin to PATH manually
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Make permanent in PowerShell profile
Add-Content $PROFILE "`n`$env:PATH += `";`$env:USERPROFILE\.cargo\bin`""
```

## Verify Installation

```bash
# Check version
cc --version

# Test basic search
cc "test" .

# Test semantic search
cc --sem "error handling" .
```

If these work, you're all set!

## Updating

### Update from crates.io

```bash
# Update to latest version
cargo install cc-search --force

# Check new version
cc --version
```

### Update from source

```bash
# Pull latest changes
cd /path/to/cc
git pull

# Rebuild and install
cargo install --path cc-cli --force
```

## Uninstalling

```bash
# Remove cc binary
cargo uninstall cc-search

# Remove index data (optional)
rm -rf .cc/  # In each repository where you used cc
```

## System Requirements

### Minimum

- **OS**: macOS 10.15+, Linux (any recent distro), Windows 10+
- **RAM**: 512MB available
- **Disk**: 50MB for binary + index space (varies by repo size)
- **CPU**: Any modern CPU (64-bit)

### Recommended

- **OS**: macOS 12+, Ubuntu 20.04+, Windows 11
- **RAM**: 2GB+ available (for large repos)
- **Disk**: SSD recommended for fast indexing
- **CPU**: Multi-core for parallel indexing

### Index Size Estimates

Typical `.cc/` index sizes:

| Repository Size | Index Size | Indexing Time |
|-----------------|------------|---------------|
| Small (1k files) | 10-50MB | 1-2 seconds |
| Medium (10k files) | 100-500MB | 5-10 seconds |
| Large (100k files) | 1-5GB | 30-60 seconds |

**Note:** Indexes are created once and incrementally updated.

## Configuration

### Default Settings

cc works out of the box with sensible defaults. No configuration required!

### Optional Configuration

#### Shell Completions

Generate completions for your shell:

```bash
# Bash
cc --generate-completions bash > ~/.local/share/bash-completion/completions/cc

# Zsh
cc --generate-completions zsh > ~/.zfunc/_ck

# Fish
cc --generate-completions fish > ~/.config/fish/completions/cc.fish

# PowerShell
cc --generate-completions powershell > cc.ps1
```

#### Environment Variables

```bash
# Change embedding model
export CC_MODEL=large

# Custom index location
export CC_INDEX_PATH=/custom/path

# Adjust worker threads
export CC_WORKERS=8
```

See [Advanced Usage](advanced-usage.html) for details.

#### .ccignore Files

Customize what files cc indexes:

```bash
# Create .ccignore in repository root
cat > .ccignore <<EOF
# Exclude generated files
*.generated.ts
dist/
build/

# Exclude data files
*.json
*.yaml
*.csv

# Keep specific files
!important-config.json
EOF
```

Syntax matches `.gitignore` (glob patterns, `!` for negation).

See [Advanced Usage](advanced-usage.html#ccignore-patterns) for details.

## Troubleshooting

### Command not found

**Problem:** `cc: command not found`

**Solution:**
```bash
# Check cargo bin is in PATH
echo $PATH | grep cargo

# Add to PATH if missing (Linux/macOS)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Windows: Add %USERPROFILE%\.cargo\bin to PATH
```

### Permission denied

**Problem:** Permission errors during installation

**Solution:**
```bash
# Don't use sudo with cargo
cargo install cc-search  # Without sudo

# If you accidentally used sudo, fix ownership
sudo chown -R $USER:$USER ~/.cargo
```

### Linker errors

**Problem:** Cannot find linker / compiler errors

**Solution:**
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# macOS: Install Xcode Command Line Tools
xcode-select --install
```

### Slow installation

**Problem:** Cargo install takes a long time

**Explanation:** First install compiles from source (~2-5 minutes).

**Tips:**
- Subsequent updates are faster
- Use `--locked` flag to avoid dependency resolution
- Pre-built binaries coming soon

### Version conflicts

**Problem:** Multiple cc versions installed

**Solution:**
```bash
# Check which cc is being used
which cc

# Uninstall all versions
cargo uninstall cc-search

# Reinstall latest
cargo install cc-search

# Verify
cc --version
```

## Next Steps

✅ **Installation complete!** Here's what to do next:

1. **Quick test:** `cc --sem "error handling" .`
2. **Try TUI mode:** `cc --tui .`
3. **Learn search modes:** [Search Modes Guide](search-modes.html)
4. **Setup AI integration:** [AI Integration Guide](ai-integration.html)
5. **Explore advanced features:** [Advanced Usage](advanced-usage.html)

## Getting Help

- **Documentation:** [beaconbay.github.io/cc](https://beaconbay.github.io/cc) (you are here!)
- **GitHub Issues:** [Report bugs / request features](https://github.com/BeaconBay/cc/issues)
- **Examples:** [EXAMPLES.md](https://github.com/BeaconBay/cc/blob/main/EXAMPLES.md)
- **Changelog:** [CHANGELOG.md](https://github.com/BeaconBay/cc/blob/main/CHANGELOG.md)
