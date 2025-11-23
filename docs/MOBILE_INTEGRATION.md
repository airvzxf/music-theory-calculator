# Mobile Integration Guide (Monorepo)

This project is set up as a Monorepo. The core logic lives in `crates/` and mobile apps live in `apps/`.

## Structure
```text
music-theory-calculator/
├── apps/
│   ├── android/      <-- Your Android Studio Project goes here
│   └── ios/          <-- Your Xcode Project goes here
├── scripts/
│   └── generate_bindings.sh  <-- Run this to update apps
```

## 1. Initial Setup

### Android
1.  Open Android Studio.
2.  **New Project** -> **Empty Activity**.
3.  **Save Location**: Navigate to `/home/wolf/workspace/projects/music-theory-calculator/apps/android`.
    *   *Important:* Ensure the project root is `apps/android`. If Android Studio creates a subfolder (e.g., `apps/android/MyMusicApp`), move the contents up so `build.gradle` is directly inside `apps/android`.
4.  **Language**: Kotlin.
5.  **Build Configuration Language**: Kotlin DSL (Recommended).

### iOS
1.  Open Xcode.
2.  **Create a new Xcode project** -> **App**.
3.  **Save Location**: `/home/wolf/workspace/projects/music-theory-calculator/apps/ios`.

### Prerequisites

-   Rust and Cargo
-   `cargo-ndk`: `cargo install cargo-ndk` (Required for Android builds)

## 2. Linking the Rust Core

We use a script to generate bindings and automatically build/copy the native libraries to your app folders.

**For Release (Default):**
```bash
./scripts/generate_bindings.sh
```

**For Debug (Faster compilation, debug symbols):**
```bash
./scripts/generate_bindings.sh --debug
```

The script will:
1.  Build the `tonic-music-ffi` crate.
2.  Generate Kotlin and Swift bindings.
3.  Copy the Kotlin file to your Android project (adjusting the package name).
4.  **Automatically** build and copy the `.so` native libraries (arm64, armv7, x86_64) to `apps/android/app/src/main/jniLibs`.

### Manual Steps

If the script fails to find your specific project structure:

1.  **Move the Kotlin File:**
    The script tries to copy `tonic_music_ffi.kt` to a standard path. If your package structure differs, move it manually:

    ```bash
    mv apps/android/app/src/main/java/tonic_music_ffi.kt apps/android/app/src/main/java/com/your/package/
    ```

2.  **Sync Gradle** in Android Studio.

## 3. Workflow

When you update Rust code (`crates/tonic-music-core`):

1.  Make your changes in Rust.
2.  Run tests: `cargo test`.
3.  Run `./scripts/generate_bindings.sh`.
    *   This single command updates both the Kotlin code and the `.so` binary libraries.
4.  Build and run your Android/iOS app.
