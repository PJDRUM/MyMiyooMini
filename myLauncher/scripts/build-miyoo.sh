#!/bin/zsh
set -euo pipefail

SCRIPT_DIR=${0:a:h}
ROOT_DIR=${SCRIPT_DIR:h}

export RUSTUP_HOME=${RUSTUP_HOME:-/tmp/mylauncher-rustup}
export CARGO_HOME=${CARGO_HOME:-/tmp/mylauncher-cargo}
export XDG_CACHE_HOME=${XDG_CACHE_HOME:-/tmp/mylauncher-cache}
export HOME=/tmp/mylauncher-home
export PATH="$CARGO_HOME/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin"

if [[ -x "$RUSTUP_HOME/toolchains/stable-aarch64-apple-darwin/bin/rustc" ]]; then
  export RUSTC="$RUSTUP_HOME/toolchains/stable-aarch64-apple-darwin/bin/rustc"
fi

mkdir -p "$HOME" "$XDG_CACHE_HOME"

cd "$ROOT_DIR"
exec cargo-zigbuild build \
  --release \
  --target armv7-unknown-linux-gnueabihf \
  -p allium-launcher \
  -p allium-menu \
  --features miyoo
