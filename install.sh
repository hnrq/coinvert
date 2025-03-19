#!/bin/bash
set -e

# Detect platform and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Get the latest release information from GitHub API
echo "Fetching latest release..."
LATEST_RELEASE=$(curl -s https://api.github.com/repos/hnrq/coinvert/releases/latest)
VERSION=$(echo "$LATEST_RELEASE" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)

# Construct binary name
BINARY_NAME="coinvert-${VERSION}-${ARCH}-${OS}"

# Download URL
DOWNLOAD_URL="https://github.com/hnrq/coinvert/releases/download/${VERSION}/${BINARY_NAME}"

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "Downloading Coinvert ${VERSION}..."
curl -LO "$DOWNLOAD_URL"

# Make binary executable
chmod +x "$BINARY_NAME"

# Move to /usr/local/bin
echo "Installing Coinvert..."
sudo mv "$BINARY_NAME" /usr/local/bin/coinvert

# Clean up
cd - > /dev/null
rm -rf "$TMP_DIR"

echo "Coinvert ${VERSION} has been installed successfully!"

