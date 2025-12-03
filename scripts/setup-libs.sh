#!/usr/bin/env bash
set -euo pipefail

# Sony Camera Remote SDK - Library Setup Script
#
# This script extracts and prepares the Sony SDK libraries for use with the Rust bindings.
# Download the SDK from Sony's developer portal and provide the path to the zip file.
#
# Usage: ./scripts/setup-libs.sh /path/to/CrSDK_v2.00.00_20251030a_Mac.zip

EXPECTED_SHA256="3da7b862cabae5fd83d6111a7a25661d8937fcc82480f45ea17cee53836ec2eb"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LIBS_DIR="$PROJECT_ROOT/libs"

usage() {
    echo "Usage: $0 <path-to-sdk-zip>"
    echo ""
    echo "Download the Sony Camera Remote SDK v2.00.00 from Sony's developer portal,"
    echo "then run this script with the path to the downloaded zip file."
    echo ""
    echo "Expected file: CrSDK_v2.00.00_20251030a_Mac.zip"
    echo "Expected SHA256: $EXPECTED_SHA256"
    exit 1
}

if [[ $# -ne 1 ]]; then
    usage
fi

SDK_ZIP="$1"

if [[ ! -f "$SDK_ZIP" ]]; then
    echo "Error: File not found: $SDK_ZIP"
    exit 1
fi

echo "==> Verifying SDK zip file..."
ACTUAL_SHA256=$(shasum -a 256 "$SDK_ZIP" | cut -d' ' -f1)

if [[ "$ACTUAL_SHA256" != "$EXPECTED_SHA256" ]]; then
    echo "Error: SHA256 mismatch!"
    echo "  Expected: $EXPECTED_SHA256"
    echo "  Got:      $ACTUAL_SHA256"
    echo ""
    echo "Please ensure you have the correct SDK version (v2.00.00_20251030a)."
    exit 1
fi

echo "    SHA256 verified."

echo "==> Extracting SDK..."
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

unzip -q "$SDK_ZIP" -d "$TEMP_DIR"

# The zip extracts directly without a top-level directory
SDK_ROOT="$TEMP_DIR"

if [[ ! -d "$SDK_ROOT/external/crsdk" ]]; then
    echo "Error: Expected SDK structure not found in zip."
    echo "       Looking for: external/crsdk/"
    exit 1
fi

echo "==> Setting up libs directory..."
rm -rf "$LIBS_DIR"
mkdir -p "$LIBS_DIR/crsdk" "$LIBS_DIR/adapters" "$LIBS_DIR/opencv"

echo "==> Copying libraries..."

# Core SDK libraries
cp "$SDK_ROOT/external/crsdk/libCr_Core.dylib" "$LIBS_DIR/crsdk/"
cp "$SDK_ROOT/external/crsdk/libmonitor_protocol.dylib" "$LIBS_DIR/crsdk/"
cp "$SDK_ROOT/external/crsdk/libmonitor_protocol_pf.dylib" "$LIBS_DIR/crsdk/"

# Adapter libraries (PTP/IP, USB, SSH)
cp "$SDK_ROOT/external/crsdk/CrAdapter/libCr_PTP_IP.dylib" "$LIBS_DIR/adapters/"
cp "$SDK_ROOT/external/crsdk/CrAdapter/libCr_PTP_USB.dylib" "$LIBS_DIR/adapters/"
cp "$SDK_ROOT/external/crsdk/CrAdapter/libssh2.dylib" "$LIBS_DIR/adapters/"
cp "$SDK_ROOT/external/crsdk/CrAdapter/libusb-1.0.0.dylib" "$LIBS_DIR/adapters/"

# OpenCV libraries (if needed for image processing)
if [[ -d "$SDK_ROOT/external/opencv" ]]; then
    cp "$SDK_ROOT/external/opencv/"*.dylib "$LIBS_DIR/opencv/" 2>/dev/null || true
fi

echo "==> Fixing library install names..."

# Sony shipped these dylibs with hardcoded absolute paths from their build machine
# (e.g., /Users/muramatsu/Projects/sony/...). This breaks loading on any other system.
# We fix them to use @rpath, which allows the binary to specify where to find them.

for lib in "$LIBS_DIR/crsdk/"*.dylib "$LIBS_DIR/adapters/"*.dylib; do
    if [[ -f "$lib" ]]; then
        name=$(basename "$lib")
        echo "    Fixing: $name"
        # Redirect stderr to hide "invalidates code signature" warnings - we re-sign below
        install_name_tool -id "@rpath/$name" "$lib" 2>/dev/null
    fi
done

echo "==> Re-signing libraries..."

# Modifying install names invalidates Apple's code signature.
# We re-sign with an ad-hoc signature so macOS will load them.

codesign --force --sign - "$LIBS_DIR/crsdk/"*.dylib "$LIBS_DIR/adapters/"*.dylib 2>/dev/null

echo "==> Removing quarantine attributes..."

# Files downloaded from the internet have a quarantine attribute that causes
# Gatekeeper to block them. We remove this attribute to allow loading.

xattr -dr com.apple.quarantine "$LIBS_DIR" 2>/dev/null || true

echo "==> Creating adapter symlinks for cargo build targets..."

# The SDK looks for adapters in Contents/Frameworks/CrAdapter/ relative to the executable.
# For Rust builds, we need to create this structure in the target directories.

for target_type in debug release; do
    for binary_dir in "$PROJECT_ROOT/target/$target_type" "$PROJECT_ROOT/target/$target_type/examples"; do
        mkdir -p "$binary_dir/Contents/Frameworks"
        rm -rf "$binary_dir/Contents/Frameworks/CrAdapter"
        ln -s "$LIBS_DIR/adapters" "$binary_dir/Contents/Frameworks/CrAdapter"
        echo "    Created: $binary_dir/Contents/Frameworks/CrAdapter"
    done
done

echo ""
echo "Done! Libraries are ready in: $LIBS_DIR"
echo ""
echo "You can now build and run examples:"
echo "  cargo build --example connect"
echo "  cargo run --example connect -- --ip <ip> --mac <mac> --ssh --user <user> --password <pass>"
