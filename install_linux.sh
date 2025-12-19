#!/usr/bin/env bash
set -euo pipefail

APP_NAME="nero"
BUILD_MODE="release"
TARGET_DIR="target/${BUILD_MODE}"
INSTALL_DIR="/usr/local/bin"

log() {
  echo -e "\033[1;34m[INFO]\033[0m $1"
}

error() {
  echo -e "\033[1;31m[ERROR]\033[0m $1" >&2
  exit 1
}

check_command() {
  command -v "$1" >/dev/null 2>&1 || error "$1 not installed"
}

check_command cargo
check_command cp

log "Building ${APP_NAME} (${BUILD_MODE})..."
cargo build --${BUILD_MODE}

BIN_PATH="${TARGET_DIR}/${APP_NAME}"

[ -f "$BIN_PATH" ] || error "Binary not found: $BIN_PATH"

log "Installing to ${INSTALL_DIR}..."

if [ ! -w "$INSTALL_DIR" ]; then
  sudo cp "$BIN_PATH" "$INSTALL_DIR"
else
  cp "$BIN_PATH" "$INSTALL_DIR"
fi

log "Done ✔"
