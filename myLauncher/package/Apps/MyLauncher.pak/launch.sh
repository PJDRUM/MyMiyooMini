#!/bin/sh
set -eu

PAK_DIR=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
SDCARD_DIR=$(CDPATH= cd -- "$PAK_DIR/../.." && pwd)

export ALLIUM_SD_ROOT="$SDCARD_DIR"
export ALLIUM_BASE_DIR="$SDCARD_DIR/.allium"

exec "$ALLIUM_BASE_DIR/bin/allium-launcher"
