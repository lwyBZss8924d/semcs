#!/bin/bash

# ck installation script
# This script builds and installs ck to your system

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "========================================="
echo "       ck Installation Script"
echo "========================================="
echo ""

# Default installation directory
DEFAULT_INSTALL_DIR="$HOME/.local/bin"
INSTALL_DIR="${CK_INSTALL_DIR:-$DEFAULT_INSTALL_DIR}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed${NC}"
    echo "Please install Rust first: https://rustup.rs"
    exit 1
fi

# Build release version
echo "Building ck in release mode..."
cargo build --release

if [ ! -f "target/release/ck" ]; then
    echo -e "${RED}Error: Build failed${NC}"
    exit 1
fi

# Get binary size
BINARY_SIZE=$(ls -lh target/release/ck | awk '{print $5}')
echo -e "${GREEN}✓${NC} Build successful (binary size: $BINARY_SIZE)"
echo ""

# Create install directory if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Creating installation directory: $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"
fi

# Copy binary to installation directory
echo "Installing ck to $INSTALL_DIR/ck"
cp target/release/ck "$INSTALL_DIR/ck"
chmod +x "$INSTALL_DIR/ck"

echo -e "${GREEN}✓${NC} ck installed successfully!"
echo ""

# Check if installation directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}⚠${NC}  $INSTALL_DIR is not in your PATH"
    echo ""
    echo "To add it to your PATH, add one of these lines to your shell configuration:"
    echo ""
    
    # Detect shell and provide appropriate instructions
    if [ -n "$BASH_VERSION" ]; then
        echo "For bash (~/.bashrc or ~/.bash_profile):"
        echo -e "${GREEN}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    fi
    
    if [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ]; then
        echo "For zsh (~/.zshrc):"
        echo -e "${GREEN}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    fi
    
    if [ "$SHELL" = "/bin/fish" ] || [ "$SHELL" = "/usr/bin/fish" ]; then
        echo "For fish (~/.config/fish/config.fish):"
        echo -e "${GREEN}set -gx PATH \$PATH $INSTALL_DIR${NC}"
    fi
    
    echo ""
    echo "After adding the line, reload your shell configuration:"
    echo "  source ~/.bashrc    (for bash)"
    echo "  source ~/.zshrc     (for zsh)"
    echo "  source ~/.config/fish/config.fish (for fish)"
    echo ""
    echo "Or simply start a new terminal session."
else
    echo -e "${GREEN}✓${NC} $INSTALL_DIR is already in your PATH"
    echo ""
    echo "You can now use ck from anywhere:"
    echo "  ck --help"
fi

echo ""
echo "Alternative installation locations:"
echo "  • System-wide: sudo cp target/release/ck /usr/local/bin/"
echo "  • Cargo install: cargo install --path ck-cli"
echo "  • Custom location: CK_INSTALL_DIR=/custom/path ./install.sh"
echo ""
echo "To uninstall: rm $INSTALL_DIR/ck"