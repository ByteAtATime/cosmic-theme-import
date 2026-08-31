#!/bin/sh
# Installer for cosmic-theme-import.
# Usage: curl -sSL <raw-url>/install.sh | sh
set -eu

REPO="ByteAtATime/cosmic-theme-import"
DEST="${COSMIC_THEME_IMPORT_INSTALL_DIR:-$HOME/.local/bin}"

command -v curl >/dev/null 2>&1 || command -v wget >/dev/null 2>&1 || {
    echo "error: curl or wget is required" >&2
    exit 1
}

case "$(uname -m)" in
    x86_64) ARCH="x86_64" ;;
    aarch64 | arm64) ARCH="aarch64" ;;
    *)
        echo "error: unsupported architecture: $(uname -m)" >&2
        exit 1
        ;;
esac

mkdir -p "$DEST"

fetch() {
    url="$1"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url"
    else
        wget -qO- "$url"
    fi
}

download_release() {
    target="cosmic-theme-import-$ARCH-unknown-linux-musl.tar.gz"
    url="https://github.com/$REPO/releases/latest/download/$target"
    echo "Downloading $url"
    fetch "$url" | tar -xz -C "$TMPDIR" cosmic-theme-import 2>/dev/null || return 1
}

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

if ! download_release; then
    echo "No prebuilt release found, falling back to building from source (requires cargo)"
    command -v cargo >/dev/null 2>&1 || {
        echo "error: cargo not found. Install Rust from https://rustup.rs and retry" >&2
        exit 1
    }
    cargo install --git "https://github.com/$REPO.git" --root "$TMPDIR/cargo"
    mv "$TMPDIR/cargo/bin/cosmic-theme-import" "$DEST/"
else
    mv "$TMPDIR/cosmic-theme-import" "$DEST/"
fi

chmod +x "$DEST/cosmic-theme-import"

case ":$PATH:" in
    *":$DEST:"*) ;;
    *)
        echo "note: add $DEST to your PATH to use 'cosmic-theme-import' directly"
        ;;
esac

echo "Installed to $DEST/cosmic-theme-import"
"$DEST/cosmic-theme-import" --version
