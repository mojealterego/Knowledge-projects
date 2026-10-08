plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "org.mojealterego.sovereignlauncher"
    compileSdk = 35

    defaultConfig {
        applicationId = "org.mojealterego.sovereignlauncher"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }
}

kotlin {
    jvmToolchain(17)
}
