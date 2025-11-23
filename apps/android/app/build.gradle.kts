plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
}

kotlin {
    compilerOptions {
        jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_11)
    }
}

android {
    namespace = "net.rovisoft.tonicmusic"
    compileSdk {
        version = release(36)
    }

    defaultConfig {
        applicationId = "net.rovisoft.tonicmusic"
        minSdk = 23
        targetSdk = 36
        versionCode = 5
        versionName = "3.0.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        ndk {
            debugSymbolLevel = "FULL"
        }
    }

    signingConfigs {
        create("release") {
            val envKeystorePath = System.getenv("ANDROID_KEYSTORE_FILE")
            val envKeystorePassword = System.getenv("ANDROID_KEYSTORE_PASSWORD")
            val envKeyAlias = System.getenv("ANDROID_KEY_ALIAS")
            val envKeyPassword = System.getenv("ANDROID_KEY_PASSWORD")

            if (envKeystorePath != null && File(envKeystorePath).exists()) {
                storeFile = File(envKeystorePath)
                storePassword = envKeystorePassword
                keyAlias = envKeyAlias
                keyPassword = envKeyPassword
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            signingConfig = signingConfigs.getByName("release")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    buildFeatures {
        compose = true
    }
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.activity.compose)
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.compose.ui)
    implementation(libs.androidx.compose.ui.graphics)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.material.icons.extended)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.compose.ui.test.junit4)
    implementation(libs.androidx.compose.ui.test.manifest)

    // Splash Screen
    implementation("androidx.core:core-splashscreen:1.2.0")

    // UniFFI depends on JNA - Explicitly using AAR to include native libraries
    implementation("net.java.dev.jna:jna:5.18.1@aar")
}

// --- Custom Task: Package Native Debug Symbols ---
// Automatically creates a ZIP file containing the unstripped .so libraries
// required by Google Play Console to symbolicate native crashes.
tasks.register<Zip>("packageNativeSymbols") {
    description = "Package native debug symbols for Google Play Console"
    group = "publishing"

    // Source: The unstripped libraries in jniLibs
    // We preserve the architecture folder structure (e.g. arm64-v8a/lib.so)
    from("src/main/jniLibs") {
        include("**/*.so")
    }

    // Destination: Inside the build output folder, next to the APK/AAB
    archiveFileName.set("native-debug-symbols.zip")
    destinationDirectory.set(layout.buildDirectory.dir("outputs/native-debug-symbols"))
}

// Hook into the build process: Run this task whenever we assemble a release
tasks.whenTaskAdded {
    if (name == "bundleRelease") {
        finalizedBy("packageNativeSymbols")
    }
}
