#!/usr/bin/env bash

set -e

[[ $PWD = */monero-vanity ]]

echo "=== Removing old /tmp/ build folders ==="
rm -rf /tmp/monero-vanity-${VER}*

VER="v$(grep "version" Cargo.toml | awk '{print $3}' | tr -d '"')"
TMP=$(mktemp -d /tmp/monero-vanity-${VER}.XXXX)
echo "=== \$TMP: $TMP ==="
echo "=== \$VER: $VER ==="

echo "=== Building Linux x86_64 ==="
cargo build --profile optimized
mv target/optimized/monero-vanity $TMP/monero-vanity-${VER}-linux-x64

echo "=== Building Windows x86_64 ==="
cargo build --profile optimized --target x86_64-pc-windows-gnu
mv target/x86_64-pc-windows-gnu/optimized/monero-vanity.exe $TMP/monero-vanity-${VER}-windows-x64.exe

cd $TMP
echo "=== Hashing/Signing ==="
sha256sum monero-vanity-${VER}-linux-x64 monero-vanity-${VER}-windows-x64.exe | gpg --clearsign > $TMP/SHA256SUM
echo "Done."
