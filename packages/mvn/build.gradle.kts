plugins {
    kotlin("jvm") version "1.9.0"
    kotlin("plugin.serialization") version "1.9.0"
    id("maven-publish")
    `java-library`
}

group = "app.lacabra"
description = "Official Kotlin library for LA CABRA Sailor anti-phishing api."
version = "1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.0")

    implementation("com.github.kittinunf.fuel:fuel:2.3.1")
    implementation("com.github.kittinunf.fuel:fuel-kotlinx-serialization:2.3.1")
    implementation("com.github.kittinunf.fuel:fuel-coroutines:2.3.1")
}

kotlin {
    jvmToolchain(18)
}

publishing {
    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/holasoyender/ragdrop")
            credentials {
                username = "holasoyender"
                password = project.findProperty("gpr.key") as String? ?: System.getenv("TOKEN")
            }
        }
    }
    publications {
        register<MavenPublication>("gpr") {
            from(components["java"])
        }
    }
}


java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(18))
    }
}
