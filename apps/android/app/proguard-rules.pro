# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.
#
# For more details, see
#   http://developer.android.com/guide/developing/tools/proguard.html

# --- JNA (Java Native Access) Rules ---
# Required because UniFFI uses JNA to communicate with the Rust library.
-dontwarn java.awt.*
-dontwarn java.nio.file.*
-dontwarn javax.swing.*
-keep class com.sun.jna.** { *; }
-keepclassmembers class * extends com.sun.jna.** {
    <init>(...);
    public *;
}

# --- UniFFI / Project Specific ---
# Keep the generated Kotlin bindings and the JNA interface/classes
# ensuring R8 doesn't rename the methods that map to the native symbols.
-keep class net.rovisoft.tonicmusic.** { *; }
-keep interface net.rovisoft.tonicmusic.** { *; }

# Preserve Line Numbers for easier debugging in Google Play Console
-keepattributes SourceFile,LineNumberTable
