#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_NAME="fry-tftp-server"
APPDIR="${ROOT_DIR}/AppDir"
BUILD_TARGET="${ROOT_DIR}/target/release/${APP_NAME}"
OUTPUT_NAME="${APP_NAME}-linux-x86_64.AppImage"
OUTPUT_PATH="${ROOT_DIR}/${OUTPUT_NAME}"
LINUXDEPLOY="${ROOT_DIR}/linuxdeploy-x86_64.AppImage"
APPIMAGETOOL="${ROOT_DIR}/appimagetool-x86_64.AppImage"

require_tool() {
    command -v "$1" >/dev/null 2>&1 || {
        printf 'Missing required tool: %s\n' "$1" >&2
        exit 1
    }
}

download_if_missing() {
    local url="$1"
    local dest="$2"
    if [ ! -f "$dest" ]; then
        curl -L "$url" -o "$dest"
        chmod +x "$dest"
    fi
}

require_tool cargo
require_tool curl

cd "$ROOT_DIR"
rm -rf "$APPDIR" "$OUTPUT_PATH"

cargo build --release --all-features

if [ ! -x "$BUILD_TARGET" ]; then
    printf 'Missing built binary: %s\n' "$BUILD_TARGET" >&2
    exit 1
fi

mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/256x256/apps"
cp "$BUILD_TARGET" "$APPDIR/usr/bin/${APP_NAME}"
cp "deploy/appimage/fry-tftp-server.desktop" "$APPDIR/usr/share/applications/${APP_NAME}.desktop"
cp "src/gui/app_icon_256.png" "$APPDIR/usr/share/icons/hicolor/256x256/apps/${APP_NAME}.png"
cp "src/gui/app_icon_256.png" "$APPDIR/${APP_NAME}.png"

download_if_missing "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" "$LINUXDEPLOY"
download_if_missing "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage" "$APPIMAGETOOL"

export ARCH=x86_64
export OUTPUT="$OUTPUT_NAME"
export NO_STRIP=1

"$LINUXDEPLOY" --appdir "$APPDIR" --desktop-file "$APPDIR/usr/share/applications/${APP_NAME}.desktop" --icon-file "$APPDIR/usr/share/icons/hicolor/256x256/apps/${APP_NAME}.png"
"$APPIMAGETOOL" "$APPDIR" "$OUTPUT_PATH"

printf 'Created %s\n' "$OUTPUT_PATH"
