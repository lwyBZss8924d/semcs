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
cargo install ck-search
```

That's it! You're ready to search.

## Installation Methods

### From crates.io (Recommended)

```bash
# Install latest stable version
cargo install ck-search

# Update to latest version
cargo install ck-search --force

# Install specific version
cargo install ck-search --version 0.5.3
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
git clone https://github.com/BeaconBay/ck
cd ck

# Install from source
cargo install --path ck-cli

# Or just build (doesn't install)
cargo build --release
./target/release/ck --version
```

**Benefits:**
- Latest features before official release
- Can modify and customize
- Contribute improvements

### Package Managers

**Coming Soon:**

```bash
# Homebrew (macOS/Linux) - In development
brew install ck-search

# APT (Debian/Ubuntu) - In development
apt install ck-search

# DNF/YUM (Fedora/RHEL) - In development
dnf install ck-search

# Scoop (Windows) - In development
scoop install ck-search

# Chocolatey (Windows) - In development
choco install ck-search
```

For now, use `cargo install ck-search` on all platforms.

## Platform-Specific Instructions

### macOS

#### Using Cargo (Recommended)

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install ck
cargo install ck-search

# Verify installation
ck --version
```

#### Using Homebrew (Coming Soon)

```bash
# Not yet available - use cargo for now
brew install ck-search
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

# Install ck
cargo install ck-search

# Verify installation
ck --version
```

#### Debian/Ubuntu (Coming Soon)

```bash
# Not yet available - use cargo for now
sudo apt update
sudo apt install ck-search
```

#### Fedora/RHEL (Coming Soon)

```bash
# Not yet available - use cargo for now
sudo dnf install ck-search
```

#### Arch Linux (Coming Soon)

```bash
# Not yet available - use cargo for now
yay -S ck-search
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

# Install ck
cargo install ck-search

# Verify installation
ck --version
```

#### Using Scoop (Coming Soon)

```powershell
# Not yet available - use cargo for now
scoop install ck-search
```

#### Using Chocolatey (Coming Soon)

```powershell
# Not yet available - use cargo for now
choco install ck-search
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
ck --version

# Test basic search
ck "test" .

# Test semantic search
ck --sem "error handling" .
```

If these work, you're all set!

## Updating

### Update from crates.io

```bash
# Update to latest version
cargo install ck-search --force

# Check new version
ck --version
```

### Update from source

```bash
# Pull latest changes
cd /path/to/ck
git pull

# Rebuild and install
cargo install --path ck-cli --force
```

## Uninstalling

```bash
# Remove ck binary
cargo uninstall ck-search

# Remove index data (optional)
rm -rf .ck/  # In each repository where you used ck
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

Typical `.ck/` index sizes:

| Repository Size | Index Size | Indexing Time |
|-----------------|------------|---------------|
| Small (1k files) | 10-50MB | 1-2 seconds |
| Medium (10k files) | 100-500MB | 5-10 seconds |
| Large (100k files) | 1-5GB | 30-60 seconds |

**Note:** Indexes are created once and incrementally updated.

## Configuration

### Default Settings

ck works out of the box with sensible defaults. No configuration required!

### Optional Configuration

#### Shell Completions

Generate completions for your shell:

```bash
# Bash
ck --generate-completions bash > ~/.local/share/bash-completion/completions/ck

# Zsh
ck --generate-completions zsh > ~/.zfunc/_ck

# Fish
ck --generate-completions fish > ~/.config/fish/completions/ck.fish

# PowerShell
ck --generate-completions powershell > ck.ps1
```

#### Environment Variables

```bash
# Change embedding model
export CK_MODEL=large

# Custom index location
export CK_INDEX_PATH=/custom/path

# Adjust worker threads
export CK_WORKERS=8
```

See [Advanced Usage](advanced-usage.html) for details.

#### .ckignore Files

Customize what files ck indexes:

```bash
# Create .ckignore in repository root
cat > .ckignore <<EOF
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

See [Advanced Usage](advanced-usage.html#ckignore-patterns) for details.

## Troubleshooting

### Command not found

**Problem:** `ck: command not found`

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
cargo install ck-search  # Without sudo

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

**Problem:** Multiple ck versions installed

**Solution:**
```bash
# Check which ck is being used
which ck

# Uninstall all versions
cargo uninstall ck-search

# Reinstall latest
cargo install ck-search

# Verify
ck --version
```

## Next Steps

✅ **Installation complete!** Here's what to do next:

1. **Quick test:** `ck --sem "error handling" .`
2. **Try TUI mode:** `ck --tui .`
3. **Learn search modes:** [Search Modes Guide](search-modes.html)
4. **Setup AI integration:** [AI Integration Guide](ai-integration.html)
5. **Explore advanced features:** [Advanced Usage](advanced-usage.html)

## Getting Help

- **Documentation:** [beaconbay.github.io/ck](https://beaconbay.github.io/ck) (you are here!)
- **GitHub Issues:** [Report bugs / request features](https://github.com/BeaconBay/ck/issues)
- **Examples:** [EXAMPLES.md](https://github.com/BeaconBay/ck/blob/main/EXAMPLES.md)
- **Changelog:** [CHANGELOG.md](https://github.com/BeaconBay/ck/blob/main/CHANGELOG.md)
