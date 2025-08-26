#!/usr/bin/env sh
set -e

INSTALLER_VERSION="1.0.0"

# --- Colors ---
RED=$(printf '\033[31m')
GREEN=$(printf '\033[32m')
YELLOW=$(printf '\033[33m')
BLUE=$(printf '\033[34m')
BOLD=$(printf '\033[1m')
RESET=$(printf '\033[0m')

# --- Banner ---
echo "${BLUE}${BOLD}"
echo "============================================="
echo "   eiipm Installer (v$INSTALLER_VERSION)"
echo "============================================="
echo "${RESET}"

# --- Ask OS ---
echo "${YELLOW}Which OS are you installing on?${RESET}"
echo "  [1] Linux"
echo "  [2] macOS"
printf "Enter choice [1/2]: "
read choice

if [ "$choice" = "2" ]; then
    echo "${RED}Sorry, macOS is not yet supported by this installer.${RESET}"
    echo
    echo "${YELLOW}Build from source instructions:${RESET}"
    echo "  1. Install Rust (https://www.rust-lang.org/)"
    echo "  2. Clone the repo:"
    echo "     git clone https://github.com/Ewwii-sh/eiipm.git"
    echo "  3. Build:"
    echo "     cd eiipm && cargo build --release"
    echo "  4. Move binary into PATH:"
    echo "     mv ./target/release/eiipm /usr/local/bin/"
    exit 1
fi

# --- Continue for Linux ---
echo "${GREEN}Great! Proceeding with Linux installation...${RESET}"

REPO="Ewwii-sh/eiipm"
LATEST_URL="https://github.com/$REPO/releases/latest"
DOWNLOAD_URL=$(curl -sL -o /dev/null -w '%{url_effective}' "$LATEST_URL" | sed 's/tag/download/')
BIN_NAME="eiipm"

# Create temp dir
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "${BLUE}Downloading latest $BIN_NAME release...${RESET}"
curl -sL "$DOWNLOAD_URL/eiipm" -o "$BIN_NAME"

echo "${BLUE}Granting eiipm executable permission...${RESET}"
chmod +x "$BIN_NAME"

echo "${BLUE}Installing to /usr/local/bin (requires sudo)...${RESET}"
sudo mv "$BIN_NAME" /usr/local/bin/

echo ""
echo "${GREEN}${BOLD}⭐✨ Installation complete! ✨⭐${RESET}"
echo ""
echo "Run '${BOLD}eiipm${RESET}' to get started."
