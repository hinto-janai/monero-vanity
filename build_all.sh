#!/usr/bin/env bash

set -e

VER="v$(grep "version" Cargo.toml | awk '{print $3}' | tr -d '"')"
TMP=$(mktemp -d /tmp/monero-vanity-${VER}.XXXX)
echo "=== \$TMP: $TMP ==="
echo "=== \$VER: $VER ==="

[[ $PWD = */monero-vanity ]]

echo "=== Building Optimized Linux x86_64 ==="
cargo build --profile optimized
mv target/optimized/monero-vanity $TMP/monero-vanity-${VER}-linux-x64

echo "=== Building Optimized Windows x86_64 ==="
cargo build --profile optimized --target x86_64-pc-windows-gnu
mv target/x86_64-pc-windows-gnu/optimized/monero-vanity.exe $TMP/monero-vanity-${VER}-windows-x64.exe

RUSTFLAGS="-C target-cpu=native"
echo "=== Building CPU Optimized Linux x86_64 ==="
cargo build --profile optimized
mv target/optimized/monero-vanity $TMP/monero-vanity-${VER}-linux-x64-5950x

echo "=== Building CPU Optimized Windows x86_64 ==="
cargo build --profile optimized --target x86_64-pc-windows-gnu
mv target/x86_64-pc-windows-gnu/optimized/monero-vanity.exe $TMP/monero-vanity-${VER}-windows-x64-5950x.exe

echo "Sign? (Y/n) "
read yn
case $yn in
	""|y|Y|yes|Yes|YES)
		cd $TMP
		echo "=== Hashing ==="
		sha256sum \
			monero-vanity-${VER}-linux-x64 \
			monero-vanity-${VER}-linux-x64-5950x \
			monero-vanity-${VER}-windows-x64.exe \
			monero-vanity-${VER}-windows-x64-5950x.exe > $TMP/SHA256SUM
		echo "=== Signing ==="
		gpg --clearsign SHA256SUM
		mv SHA256SUM.asc SHA256SUM
		echo "Done."
		exit 0
		;;
	*) echo "Done."; exit 0;;
esac
