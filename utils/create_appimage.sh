#!/usr/bin/env bash

set -ex

# Check current directory.
[[ $PWD == */monero-vanity ]] && cd utils
[[ $PWD == */utils/monero-vanity/utils ]]

# Set variables.
APP_DIR="${PWD}/monero-vanity.AppDir"
VERSION="v$(grep -m1 "version" ../Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"

# Remove old AppImage.
[[ -f "monero-vanity-x86_64.AppImage" ]] && rm "monero-vanity-x86_64.AppImage"

# Update icon/binary.
rm "${APP_DIR}/icon.png"
rm "${APP_DIR}/usr/bin/monero-vanity"
cp ../src/icon.png "${APP_DIR}/icon.png"
cp ../target/release/monero-vanity "${APP_DIR}/usr/bin/monero-vanity"

# Create AppImage.
if ARCH=x86_64 appimagetool --sign --sign-key "31C5145AAFA5A8DF1C1DB2A6D47CE05FA175A499" "$APP_DIR"; then
	mv "monero-vanity-x86_64.AppImage" "monero-vanity-${VERSION}-x86_64.AppImage"
fi

# Wipe icon/binary.
echo > "${APP_DIR}/icon.png"
echo > "${APP_DIR}/usr/bin/monero-vanity"
