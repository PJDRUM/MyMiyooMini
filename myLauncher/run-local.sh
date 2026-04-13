#!/bin/zsh

set -euo pipefail

SCRIPT_DIR=${0:a:h}

export ALLIUM_SD_ROOT="$SCRIPT_DIR/static"
export ALLIUM_BASE_DIR="$SCRIPT_DIR/static/.allium"
export ALLIUM_DATABASE="${ALLIUM_DATABASE:-/tmp/mylauncher-sim.db}"

cd "$SCRIPT_DIR"
exec /opt/homebrew/bin/cargo run --bin allium-launcher --features simulator
