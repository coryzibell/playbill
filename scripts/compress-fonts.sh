#!/usr/bin/env bash
# Compress all .flf fonts in fonts/ into fonts/compressed/.
# Requires the zstd CLI (dev-time only; not a build or consumer dependency).
# Run after adding new .flf files, then commit both the .flf and .zst files.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
FONTS_DIR="$REPO_ROOT/fonts"
COMPRESSED_DIR="$REPO_ROOT/fonts/compressed"

mkdir -p "$COMPRESSED_DIR"
rm -f "$COMPRESSED_DIR"/*.zst

count=0
for f in "$FONTS_DIR"/*.flf; do
    stem="$(basename "${f%.flf}")"
    zstd -q -f -19 "$f" -o "$COMPRESSED_DIR/${stem}.zst"
    count=$((count + 1))
done

echo "Compressed $count font(s) into $COMPRESSED_DIR"
