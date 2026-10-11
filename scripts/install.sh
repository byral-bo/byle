#!/bin/bash
set -eu

REPO="kyal-tech/byle"
VERSION="${BY_VERSION:-v0.8.9}"

usage() {
    echo "Usage: curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/install.sh | sh"
    echo ""
    echo "Environment:"
    echo "  BY_VERSION=v0.8.9     Version (default: latest)"
    echo "  BY_PREFIX=/custom     Install dir (default: ~/.by or /usr/local)"
    echo ""
    echo "  install.sh uninstall   Remove Byle"
    exit 0
}

detect_platform() {
    local os arch
    case "$(uname -s)" in
        Darwin) os="macos" ;;
        Linux)  os="linux" ;;
        *)
            echo "Error: Windows? Use PowerShell:"
            echo "  iwr -Uri \"https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.ps1\" | iex"
            exit 1 ;;
    esac
    case "$(uname -m)" in
        arm64|aarch64) arch="arm64" ;;
        x86_64|amd64)  arch="x64" ;;
        *) echo "Error: unsupported arch ($(uname -m))"; exit 1 ;;
    esac
    echo "${os}-${arch}"
}

cleanup() { rm -rf "/tmp/by-install" 2>/dev/null || true; }

# Uninstall
if [ "${1:-}" = "uninstall" ]; then
    echo "Removing Byle..."
    for f in /usr/local/bin/by "$HOME/.by/bin/by" "$HOME/.by/bin/by"; do
        [ -f "$f" ] && rm -f "$f" && echo "  Removed $f"
    done
    for f in /usr/local/lib/libbyc_runtime.a /usr/local/lib/by/libbyc_runtime.a "$HOME/.by/lib/libbyc_runtime.a" "$HOME/.by/lib/libbyc_runtime.a"; do
        [ -f "$f" ] && rm -f "$f" && echo "  Removed $f"
    done
    for d in "$HOME/.by" "$HOME/.by" /usr/local/lib/by; do
        [ -d "$d" ] && rm -rf "$d" && echo "  Removed $d"
    done
    # Remove from /etc/paths.d (current installer)
    sudo rm -f /etc/paths.d/by 2>/dev/null || true
    # Remove from dotfiles (legacy installer)
    for rc in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile"; do
        [ -f "$rc" ] && sed -i '' '/# Byle/d' "$rc" 2>/dev/null || true
        [ -f "$rc" ] && sed -i '' '/\.by\/bin/d' "$rc" 2>/dev/null || true
        [ -f "$rc" ] && sed -i '/# Byle/d' "$rc" 2>/dev/null || true
        [ -f "$rc" ] && sed -i '/\.by\/bin/d' "$rc" 2>/dev/null || true
    done
    echo "by uninstalled. Close and reopen terminal."
    exit 0
fi

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then usage; fi

# Download
PLATFORM=$(detect_platform)
echo "Detected: $PLATFORM"
BUNDLE="by-${PLATFORM}.tar.gz"
BUNDLE_URL="https://github.com/$REPO/releases/download/$VERSION/$BUNDLE"
SHA256_URL="$BUNDLE_URL.sha256"

cleanup
mkdir -p /tmp/by-install
cd /tmp/by-install

echo "Downloading Byle $VERSION for $PLATFORM..."
curl -fsSL "$BUNDLE_URL" -o "$BUNDLE" || {
    echo "Error: failed to download $BUNDLE"
    echo "Check release at: https://github.com/$REPO/releases"
    exit 1
}

if ! curl -fsSL "$SHA256_URL" -o "${BUNDLE}.sha256" 2>/dev/null; then
    echo "Error: checksum file unavailable; refusing unverified installation"
    exit 1
fi
echo "Verifying checksum..."
if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 -c "${BUNDLE}.sha256" || { echo "Checksum failed"; exit 1; }
elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum -c "${BUNDLE}.sha256" || { echo "Checksum failed"; exit 1; }
else
    echo "Error: no SHA-256 verifier (shasum or sha256sum) found"
    exit 1
fi

# Pre-install cleanup
[ -f /usr/local/lib/by/libbyc_runtime.a ] && rm -f /usr/local/lib/by/libbyc_runtime.a && rmdir /usr/local/lib/by 2>/dev/null || true

# Extract
tar xzf "$BUNDLE"

if [ ! -f "by" ]; then
    echo "Error: 'by' not found in archive"
    ls -la; exit 1
fi

# macOS: sign binary + bundled dylibs (ad-hoc, no Team ID)
if [ "$(uname -s)" = "Darwin" ]; then
    echo "  Signing binary for macOS..."
    xattr -d com.apple.quarantine by 2>/dev/null || true
    if [ -d "lib" ]; then
        for f in lib/*.dylib; do
            [ -f "$f" ] || continue
            codesign -f -s - "$f"
        done
    fi
    codesign -f -s - by
    echo "  Code signature applied"
fi

chmod +x by

# Determine install dir
INSTALL_TO_USR=false
if [ "${BY_PREFIX:-}" = "" ]; then
    if [ -w /usr/local/bin ]; then
        INSTALL_TO_USR=true
    elif command -v sudo >/dev/null 2>&1 && sudo -n true 2>/dev/null; then
        INSTALL_TO_USR=true
    fi
fi

if [ "$INSTALL_TO_USR" = true ] && [ "${BY_PREFIX:-}" = "" ]; then
    echo "Installing to /usr/local..."
    if [ "$(id -u)" -eq 0 ]; then
        mkdir -p /usr/local/bin /usr/local/lib
        cp by /usr/local/bin/by
        [ -f libbyc_runtime.a ] && cp libbyc_runtime.a /usr/local/lib/libbyc_runtime.a
        [ -d lib ] && cp -f lib/*.dylib /usr/local/lib/ 2>/dev/null || true
    else
        sudo mkdir -p /usr/local/bin /usr/local/lib
        sudo cp by /usr/local/bin/by
        [ -f libbyc_runtime.a ] && sudo cp libbyc_runtime.a /usr/local/lib/libbyc_runtime.a
        [ -d lib ] && sudo cp -f lib/*.dylib /usr/local/lib/ 2>/dev/null || true
    fi
    INSTALL_DIR="/usr/local/bin"
    BY_PREFIX="/usr/local"
elif [ -n "${BY_PREFIX:-}" ]; then
    echo "Installing to $BY_PREFIX..."
    mkdir -p "$BY_PREFIX/bin" "$BY_PREFIX/lib"
    cp by "$BY_PREFIX/bin/by"
    [ -f libbyc_runtime.a ] && cp libbyc_runtime.a "$BY_PREFIX/lib/libbyc_runtime.a"
    [ -d lib ] && cp -f lib/*.dylib "$BY_PREFIX/lib/" 2>/dev/null || true
    INSTALL_DIR="$BY_PREFIX/bin"
else
    echo "Installing to $HOME/.by..."
    mkdir -p "$HOME/.by/bin" "$HOME/.by/lib"
    cp by "$HOME/.by/bin/by"
    [ -f libbyc_runtime.a ] && cp libbyc_runtime.a "$HOME/.by/lib/libbyc_runtime.a"
    [ -d lib ] && cp -f lib/*.dylib "$HOME/.by/lib/" 2>/dev/null || true
    INSTALL_DIR="$HOME/.by/bin"
    BY_PREFIX="$HOME/.by"
fi

# Fix dylib paths for macOS (resolve @executable_path relative to installed binary)
if [ "$(uname -s)" = "Darwin" ] && command -v install_name_tool >/dev/null 2>&1; then
    BY_BIN="${INSTALL_DIR}/by"
    for dylib in "$BY_PREFIX"/lib/lib*.dylib; do
        [ -f "$dylib" ] || continue
        libname=$(basename "$dylib")
        # Rewrite any @executable_path/lib/X.dylib → @executable_path/../lib/X.dylib
        install_name_tool -change "@executable_path/lib/$libname" "@executable_path/../lib/$libname" "$BY_BIN"
    done
    # Re-sign after install_name_tool invalidates the code signature
    codesign -f -s - "$BY_BIN"
fi

# Add to PATH — try /etc/paths.d/ first, fall back to shell config
ADDED_PATH=false
PATHS_D="/etc/paths.d"
if [ -d "$PATHS_D" ] || mkdir "$PATHS_D" 2>/dev/null || sudo mkdir "$PATHS_D" 2>/dev/null; then
    if [ -w "$PATHS_D" ]; then
        if [ ! -f "$PATHS_D/by" ] || [ "$(cat "$PATHS_D/by" 2>/dev/null)" != "$INSTALL_DIR" ]; then
            echo "$INSTALL_DIR" | sudo tee "$PATHS_D/by" > /dev/null
            echo "  Added $INSTALL_DIR to PATH via $PATHS_D/by"
            ADDED_PATH=true
        else
            echo "  PATH already configured in $PATHS_D/by"
            ADDED_PATH=true
        fi
    fi
fi

if [ "$ADDED_PATH" = false ]; then
    SHELL_NAME=$(basename "${SHELL:-}")
    case "$SHELL_NAME" in
        zsh)  SHELL_CONFIG="$HOME/.zshrc" ;;
        bash) SHELL_CONFIG="$HOME/.bashrc" ;;
        *)    SHELL_CONFIG="" ;;
    esac
    if [ -n "$SHELL_CONFIG" ]; then
        touch "$SHELL_CONFIG" 2>/dev/null || true
        if [ -w "$SHELL_CONFIG" ]; then
            if ! grep -q "$INSTALL_DIR" "$SHELL_CONFIG" 2>/dev/null; then
                echo "" >> "$SHELL_CONFIG"
                echo "# Byle" >> "$SHELL_CONFIG"
                echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_CONFIG"
                echo "  Added $INSTALL_DIR to PATH in $SHELL_CONFIG"
            else
                echo "  PATH already configured in $(basename "$SHELL_CONFIG")"
            fi
        else
            echo "  Warning: $SHELL_CONFIG not writable."
            echo "  Run: echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> $SHELL_CONFIG"
        fi
    fi
fi

export PATH="$INSTALL_DIR:$PATH"

# Verify
echo ""
if by --version 2>/dev/null; then
    echo ""
    echo "✅ Byle $VERSION installed successfully!"
    echo ""
    echo "  Binary:  $INSTALL_DIR/by"
    [ -f "$BY_PREFIX/lib/libbyc_runtime.a" ] && echo "  Runtime: installed"
    echo ""
    echo "  Use: Open a new terminal and run: by -v"
    echo "  Or:  export PATH=\"$INSTALL_DIR:\$PATH\" && by -v"
    echo "  Try: by run examples/hello.by"
else
    echo "⚠️  Installation completed but 'by --version' failed."
    echo "   Check PATH: $INSTALL_DIR"
fi

cleanup
