#!/bin/bash
set -e

# PMSynapse CLI Installer for Linux/macOS
# https://github.com/HelixoidLLC/pmsynapse

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO="HelixoidLLC/pmsynapse"
BINARY_NAME="snps"
INSTALL_DIR="/usr/local/bin"

echo -e "${BLUE}"
echo "╔═══════════════════════════════════════╗"
echo "║   PMSynapse CLI Installer             ║"
echo "╚═══════════════════════════════════════╝"
echo -e "${NC}"

# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
case "$OS" in
  linux) OS="linux" ;;
  darwin) OS="macos" ;;
  *)
    echo -e "${RED}Error: Unsupported OS: $OS${NC}"
    echo "Supported: Linux, macOS"
    exit 1
    ;;
esac

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  *)
    echo -e "${RED}Error: Unsupported architecture: $ARCH${NC}"
    echo "Supported: x86_64, ARM64"
    exit 1
    ;;
esac

ASSET_NAME="snps-${OS}-${ARCH}"
ARCHIVE="${ASSET_NAME}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ARCHIVE}"
CHECKSUM_URL="${DOWNLOAD_URL}.sha256"

echo -e "${GREEN}Detected: ${OS}-${ARCH}${NC}"
echo -e "Installing ${BINARY_NAME} to ${INSTALL_DIR}...\n"

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf ${TEMP_DIR}" EXIT

cd "${TEMP_DIR}"

# Download archive
echo -e "${BLUE}[1/4] Downloading ${ARCHIVE}...${NC}"
if command -v curl &> /dev/null; then
  curl -fsSL "${DOWNLOAD_URL}" -o "${ARCHIVE}"
elif command -v wget &> /dev/null; then
  wget -q "${DOWNLOAD_URL}" -O "${ARCHIVE}"
else
  echo -e "${RED}Error: curl or wget required${NC}"
  exit 1
fi

# Download checksum
echo -e "${BLUE}[2/4] Verifying checksum...${NC}"
if command -v curl &> /dev/null; then
  curl -fsSL "${CHECKSUM_URL}" -o "${ARCHIVE}.sha256"
else
  wget -q "${CHECKSUM_URL}" -O "${ARCHIVE}.sha256"
fi

# Verify checksum
if command -v sha256sum &> /dev/null; then
  sha256sum -c "${ARCHIVE}.sha256" || {
    echo -e "${RED}Error: Checksum verification failed${NC}"
    exit 1
  }
elif command -v shasum &> /dev/null; then
  shasum -a 256 -c "${ARCHIVE}.sha256" || {
    echo -e "${RED}Error: Checksum verification failed${NC}"
    exit 1
  }
else
  echo -e "${YELLOW}Warning: sha256sum not found, skipping verification${NC}"
fi

# Extract
echo -e "${BLUE}[3/4] Extracting...${NC}"
tar -xzf "${ARCHIVE}"

# Install
echo -e "${BLUE}[4/4] Installing to ${INSTALL_DIR}...${NC}"
if [ -w "${INSTALL_DIR}" ]; then
  mv "${BINARY_NAME}" "${INSTALL_DIR}/"
  chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
else
  echo -e "${YELLOW}Requires sudo to install to ${INSTALL_DIR}${NC}"
  sudo mv "${BINARY_NAME}" "${INSTALL_DIR}/"
  sudo chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
fi

# Verify installation
echo ""
if command -v ${BINARY_NAME} &> /dev/null; then
  echo -e "${GREEN}✓ Installation successful!${NC}\n"
  ${BINARY_NAME} --version
  echo ""
  echo -e "${GREEN}Get started:${NC}"
  echo "  ${BINARY_NAME} --help"
  echo "  ${BINARY_NAME} thoughts init"
  echo "  ${BINARY_NAME} daemon start"
else
  echo -e "${RED}Error: Installation failed${NC}"
  echo "Please ensure ${INSTALL_DIR} is in your PATH"
  exit 1
fi
