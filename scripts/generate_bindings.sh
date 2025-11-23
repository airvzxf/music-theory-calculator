#!/bin/bash
set -e

# Default to release
BUILD_TYPE="release"
CARGO_FLAGS="--release"

# Parse arguments
for arg in "$@"
do
    case $arg in
        --debug)
        BUILD_TYPE="debug"
        CARGO_FLAGS=""
        ;;
    esac
done

# Define paths
PROJECT_ROOT=$(git rev-parse --show-toplevel)
TARGET_DIR="$PROJECT_ROOT/target"
BINDINGS_DIR="$PROJECT_ROOT/bindings"
# Adjust FFI lib path based on build type
if [ "$BUILD_TYPE" == "debug" ]; then
    FFI_LIB_PATH="$TARGET_DIR/debug/libtonic_music_ffi.so"
else
    FFI_LIB_PATH="$TARGET_DIR/release/libtonic_music_ffi.so"
fi

# App Directories (Monorepo structure)
ANDROID_APP_DIR="$PROJECT_ROOT/apps/android"
IOS_APP_DIR="$PROJECT_ROOT/apps/ios"

# Ensure we are at the root
cd "$PROJECT_ROOT"

echo "=== Building FFI Crate ($BUILD_TYPE) ==="
cargo build -p tonic-music-ffi $CARGO_FLAGS

echo ""
echo "=== Generating Bindings ==="

# Kotlin
echo "-> Generating Kotlin..."
cargo run -p tonic-music-ffi --bin uniffi-bindgen -- generate \
  --library "$FFI_LIB_PATH" \
  --language kotlin \
  --out-dir "$BINDINGS_DIR/kotlin"

# Swift
echo "-> Generating Swift..."
cargo run -p tonic-music-ffi --bin uniffi-bindgen -- generate \
  --library "$FFI_LIB_PATH" \
  --language swift \
  --out-dir "$BINDINGS_DIR/swift"

echo ""
echo "=== Syncing to Apps (if present) ==="

# Sync Android
# Look for a standard Android package path. This finds the first main/java folder.
if [ -d "$ANDROID_APP_DIR" ]; then
  JAVA_SRC=$(find "$ANDROID_APP_DIR" -type d -name "java" -path "*/src/main/java" | head -n 1)
  JNI_LIBS=$(find "$ANDROID_APP_DIR" -type d -name "jniLibs" -path "*/src/main/jniLibs" | head -n 1)

  if [ -n "$JAVA_SRC" ]; then
    DEST_DIR="$JAVA_SRC/net/rovisoft/tonicmusic"
    mkdir -p "$DEST_DIR"
    echo "-> Copying Kotlin bindings to $DEST_DIR"

    # Copy and patch the package name
    cp "$BINDINGS_DIR/kotlin/uniffi/tonic_music_ffi/tonic_music_ffi.kt" "$DEST_DIR/"

    # Patch the package name to match our app structure
    if [[ "$OSTYPE" == "darwin"* ]]; then
      sed -i '' 's/package uniffi.tonic_music_ffi/package net.rovisoft.tonicmusic/g' "$DEST_DIR/tonic_music_ffi.kt"
    else
      sed -i 's/package uniffi.tonic_music_ffi/package net.rovisoft.tonicmusic/g' "$DEST_DIR/tonic_music_ffi.kt"
    fi
  else
    echo "-> Android project detected but 'src/main/java' not found. Skipping Kotlin copy."
  fi

  # Check for cargo-ndk and build native libraries
  if command -v cargo-ndk &>/dev/null; then
    echo "-> Detected cargo-ndk. Building and syncing native libraries ($BUILD_TYPE)..."

    # Determine target directory for jniLibs
    JNI_LIBS_TARGET=""
    if [ -n "$JNI_LIBS" ]; then
      JNI_LIBS_TARGET="$JNI_LIBS"
    else
      # Fallback to standard path if not found
      JNI_LIBS_TARGET="$ANDROID_APP_DIR/app/src/main/jniLibs"
    fi

    # Clean the target directory to ensure no stale libs
    echo "   Cleaning $JNI_LIBS_TARGET..."
    rm -rf "$JNI_LIBS_TARGET"
    mkdir -p "$JNI_LIBS_TARGET"

    echo "   Outputting .so files to: $JNI_LIBS_TARGET"

    # Build for common architectures (arm64, armv7, x86_64 for emulator)
    cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 \
      -o "$JNI_LIBS_TARGET" \
      build $CARGO_FLAGS -p tonic-music-ffi
  else
    echo "-> WARNING: cargo-ndk not found. Skipping native library build."
    echo "   Please install: cargo install cargo-ndk"
  fi
else
  echo "-> No Android app found in apps/android. Skipping sync."
fi

# Sync iOS
if [ -d "$IOS_APP_DIR" ]; then
  # Simple check for an Xcode project structure
  if ls "$IOS_APP_DIR"/*.xcodeproj >/dev/null 2>&1 || ls "$IOS_APP_DIR"/*.swift >/dev/null 2>&1; then
    echo "-> Copying Swift bindings to $IOS_APP_DIR"
    cp "$BINDINGS_DIR/swift/tonic_music_ffi.swift" "$IOS_APP_DIR/"
    cp "$BINDINGS_DIR/swift/tonic_music_ffiFFI.h" "$IOS_APP_DIR/"
    cp "$BINDINGS_DIR/swift/tonic_music_ffiFFI.modulemap" "$IOS_APP_DIR/"
  else
    echo "-> iOS folder exists but no project files found. Skipping Swift copy."
  fi
else
  echo "-> No iOS app found in apps/ios. Skipping sync."
fi

echo ""
echo "Done! Bindings generated in $BINDINGS_DIR"
