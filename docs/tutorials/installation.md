# (cs) Installation

## Quick Install

The fastest way to get started:

```shell
cargo install cs-search
```

That's it! You're ready to search.

## Installation Methods

### From crates.io (Recommended)

```shell
# Install latest stable version
cargo install cs-search

# Update to latest version
cargo install cs-search --force

# Install specific version
cargo install cs-search --version 0.5.3
```

**Requirements:**

- Rust 1.70 or later
- Cargo (comes with Rust)

**Install Rust** (if needed):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### From Source

For latest features or development:

```shell
# Clone repository
git clone https://github.com/lwyBZss8924d/semcs
cd cs

# Install from source
cargo install --path cs-cli

# Or just build (doesn't install)
cargo build --release
./target/release/cs --version
```

**Benefits:**

- Latest features before official release
- Can modify and customize
- Contribute improvements

### Package Managers

**Coming Soon:**

```shell
# Homebrew (macOS/Linux) - In development
brew install cs-search

# APT (Debian/Ubuntu) - In development
apt install cs-search

# DNF/YUM (Fedora/RHEL) - In development
dnf install cs-search

# Scoop (Windows) - In development
scoop install cs-search

# Chocolatey (Windows) - In development
choco install cs-search
```

For now, use `cargo install cs-search` on all platforms.

## Platform-Specific Instructions

### macOS

#### Using Cargo (Recommended)

```shell
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install cc
cargo install cs-search

# Verify installation
cs --version
```

#### Using Homebrew (Coming Soon)

```shell
# Not yet available - use cargo for now
brew install cs-search
```

**Troubleshooting:**

If you get permission errors:

```shell
# Ensure cargo bin is in PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Linux

#### Using Cargo (Works on all distros)

```shell
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install cc
cargo install cs-search

# Verify installation
cc --version
```

#### Debian/Ubuntu (Coming Soon)

```shell
# Not yet available - use cargo for now
sudo apt update
sudo apt install cs-search
```

#### Fedora/RHEL (Coming Soon)

```shell
# Not yet available - use cargo for now
sudo dnf install cs-search
```

#### Arch Linux (Coming Soon)

```shell
# Not yet available - use cargo for now
yay -S cs-search
```

**Troubleshooting:**

If you get linker errors:

```shell
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

### Windows

#### Windows Using Cargo (Recommended)

```powershell
# Install Rust if needed
# Download and run: https://rustup.rs/

# Install cc
cargo install cs-search

# Verify installation
cc --version
```

#### Windows Using Scoop (Coming Soon)

```powershell
# Not yet available - use cargo for now
scoop install cs-search
```

#### Windows Using Chocolatey (Coming Soon)

```powershell
# Not yet available - use cargo for now
choco install cs-search
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
cs --version

# Test basic search
cs "test" .

# Test semantic search
cs --sem "error handling" .
```

If these work, you're all set!

## Updating

### Update from crates.io

```bash
# Update to latest version
cargo install cs-search --force

# Check new version
cs --version
```

### Update from source

```bash
# Pull latest changes
cd /path/to/cs
git pull

# Rebuild and install
cargo install --path cs-cli --force
```

## Uninstalling

```bash
# Remove cc binary
cargo uninstall cs-search

# Remove index data (optional)
rm -rf .cs/  # In each repository where you used cs
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

Typical `.cs/` index sizes:

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
cs --generate-completions bash > ~/.local/share/bash-completion/completions/cs

# Zsh
cs --generate-completions zsh > ~/.zfunc/_cs

# Fish
cs --generate-completions fish > ~/.config/fish/completions/cs.fish

# PowerShell
cs --generate-completions powershell > cs.ps1
```

#### Environment Variables

```bash
# Change embedding model
export CS_MODEL=large

# Custom index location
export CS_INDEX_PATH=/custom/path

# Adjust worker threads
export CS_WORKERS=8
```

See [Advanced Usage](advanced-usage.html) for details.

#### .ccignore Files

Customize what files cc indexes:

```bash
# Create .ccignore in repository root
cat > .csignore <<EOF
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

See [Advanced Usage](advanced-usage.html#csignore-patterns) for details.

## Troubleshooting

### Command not found

**Problem:** `cs: command not found`

**Solution:**

```shell
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

```shell
# Don't use sudo with cargo
cargo install cs-search  # Without sudo

# If you accidentally used sudo, fix ownership
sudo chown -R $USER:$USER ~/.cargo
```

### Linker errors

**Problem:** Cannot find linker / compiler errors

**Solution:**

```shell
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

```shell
# Check which cc is being used
which cs

# Uninstall all versions
cargo uninstall cs-search

# Reinstall latest
cargo install cs-search

# Verify
cs --version
```

## Next Steps

✅ **Installation complete!** Here's what to do next:

1. **Quick test:** `cs --sem "error handling" .`
2. **Try TUI mode:** `cs --tui .`
3. **Learn search modes:** [Search Modes Guide](search-modes.html)
4. **Setup AI integration:** [AI Integration Guide](ai-integration.html)
5. **Explore advanced features:** [Advanced Usage](advanced-usage.html)

## Getting Help

- **sorce fork from cs:** [BeaconBay/ck](https://github.com/BeaconBay/ck)
- **Documentation:** [lwyBZss8924d.github.io/cc](https://github.com/lwyBZss8924d/semcs) (you are here!)
- **GitHub Issues:** [Report bugs / request features](https://github.com/lwyBZss8924d/semcs/issues)
- **Examples:** [EXAMPLES.md](https://github.com/lwyBZss8924d/semcs/blob/main/EXAMPLES.md)
- **Changelog:** [CHANGELOG.md](https://github.com/lwyBZss8924d/semcs/blob/main/CHANGELOG.md)
