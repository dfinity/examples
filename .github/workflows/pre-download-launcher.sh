#!/bin/bash
# Pre-download the icp-cli network launcher to avoid GitHub API rate limits.
# icp-cli v0.1.0 doesn't support ICP_CLI_GITHUB_TOKEN yet, so `icp network start`
# hits the unauthenticated GitHub API rate limit (60 req/hr) when fetching the
# latest launcher version. This script uses the authenticated `gh` CLI to get
# the version, downloads the binary directly, and sets ICP_CLI_NETWORK_LAUNCHER_PATH
# so icp-cli skips the API call entirely.
#
# Requires: GH_TOKEN env var (for authenticated GitHub API access via gh CLI)
#
# This workaround can be removed once icp-cli supports ICP_CLI_GITHUB_TOKEN.
set -ex

VERSION=$(gh release view --repo dfinity/icp-cli-network-launcher --json tagName -q .tagName)

ARCH=$(uname -m)
case "$ARCH" in
  arm64|aarch64) ARCH="arm64" ;;
  x86_64)        ARCH="x86_64" ;;
  *)             echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

OS=$(uname -s)
case "$OS" in
  Darwin) OS="darwin" ;;
  Linux)  OS="linux" ;;
  *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

TARBALL="icp-cli-network-launcher-${ARCH}-${OS}-${VERSION}"
URL="https://github.com/dfinity/icp-cli-network-launcher/releases/download/${VERSION}/${TARBALL}.tar.gz"

LAUNCHER_DIR="$HOME/.icp-cli-launcher"
mkdir -p "$LAUNCHER_DIR"
curl -sL "$URL" | tar xz -C "$LAUNCHER_DIR"

LAUNCHER_PATH="${LAUNCHER_DIR}/${TARBALL}/icp-cli-network-launcher"
chmod +x "$LAUNCHER_PATH"

echo "ICP_CLI_NETWORK_LAUNCHER_PATH=${LAUNCHER_PATH}" >> "$GITHUB_ENV"
echo "Network launcher ${VERSION} downloaded to ${LAUNCHER_PATH}"
