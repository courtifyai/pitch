#!/usr/bin/env bash
set -e

REPO="courtifyai/pitch"
BINARY_NAME="pitch"

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux)
        OS_TAG="linux"
        ;;
    Darwin)
        OS_TAG="macos"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH_TAG="x64"
        ;;
    arm64|aarch64)
        ARCH_TAG="arm64"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

ASSET="${BINARY_NAME}-${OS_TAG}-${ARCH_TAG}.zip"

echo "➡️ Detected platform: ${OS_TAG}-${ARCH_TAG}"
echo "➡️ Fetching latest release info..."

LATEST=$(curl -fsSL https://api.github.com/repos/${REPO}/releases/latest | grep "tag_name" | cut -d '"' -f 4)

if [ -z "$LATEST" ]; then
    echo "❌ Could not find latest release tag."
    exit 1
fi

echo "➡️ Latest version: $LATEST"

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST}/${ASSET}"

echo "➡️ Downloading: $DOWNLOAD_URL"

TMP_DIR=$(mktemp -d)
cd $TMP_DIR

curl -fSL -o "$ASSET" "$DOWNLOAD_URL"

echo "➡️ Unzipping..."
unzip -q "$ASSET"

if [ "$OS_TAG" = "linux" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
fi

echo "➡️ Installing to $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
mv "${BINARY_NAME}-${OS_TAG}-${ARCH_TAG}" "$INSTALL_DIR/${BINARY_NAME}"
chmod +x "$INSTALL_DIR/${BINARY_NAME}"

echo "✅ Installation complete!"
echo ""
echo "Run:"
echo "    $BINARY_NAME --help"
echo ""
