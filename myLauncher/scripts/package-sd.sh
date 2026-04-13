#!/bin/zsh
set -euo pipefail

SCRIPT_DIR=${0:a:h}
ROOT_DIR=${SCRIPT_DIR:h}
DIST_DIR=${ROOT_DIR}/dist-sd
TARGET_DIR=${ROOT_DIR}/target/armv7-unknown-linux-gnueabihf/release
BASE_DIR=${DIST_DIR}/.allium
PAK_DIR=${DIST_DIR}/Apps/MyLauncher.pak

if [[ ! -x "${TARGET_DIR}/allium-launcher" || ! -x "${TARGET_DIR}/allium-menu" ]]; then
  echo "missing Miyoo binaries in ${TARGET_DIR}" >&2
  echo "run ./scripts/build-miyoo.sh first" >&2
  exit 1
fi

rm -rf "$DIST_DIR"
mkdir -p \
  "$BASE_DIR/bin" \
  "$DIST_DIR/Roms" \
  "$DIST_DIR/Saves/CurrentProfile" \
  "$PAK_DIR"

cp -R "${ROOT_DIR}/static/.allium/config" "$BASE_DIR/"
cp -R "${ROOT_DIR}/static/.allium/cores" "$BASE_DIR/"
cp -R "${ROOT_DIR}/static/.allium/fonts" "$BASE_DIR/"
cp -R "${ROOT_DIR}/static/.allium/locales" "$BASE_DIR/"
cp -R "${ROOT_DIR}/static/.allium/migrations" "$BASE_DIR/"
cp -R "${ROOT_DIR}/static/.allium/scripts" "$BASE_DIR/"
cp -R "${ROOT_DIR}/static/.allium/state" "$BASE_DIR/"
cp "${ROOT_DIR}/static/.allium/version.txt" "$BASE_DIR/version.txt"
cp "${TARGET_DIR}/allium-launcher" "$BASE_DIR/bin/"
cp "${TARGET_DIR}/allium-menu" "$BASE_DIR/bin/"
cp "${ROOT_DIR}/package/Apps/MyLauncher.pak/config.json" "$PAK_DIR/"
cp "${ROOT_DIR}/package/Apps/MyLauncher.pak/launch.sh" "$PAK_DIR/"
chmod +x "$PAK_DIR/launch.sh"

echo "created ${DIST_DIR}"
