FROM docker.io/eclipse-temurin:17.0.16_8-jdk-noble@sha256:63360a3f1ac5e0acf9c33dabd1ac1d6c5afaa24659536d57910a32ddada41a6c AS builder
LABEL authors="mollyim"

# Install essential tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      git=1:2.43.0-1ubuntu7.3 \
      python3=3.12.3-0ubuntu2 \
      unzip=6.0-28ubuntu4.1 \
      rustup=1.26.0-5build1 \
    && rm -rf /var/lib/apt/lists/*

# Install Android SDK
ARG ANDROID_SDK_DIST="commandlinetools-linux-13114758_latest.zip"
ARG ANDROID_SDK_SHA256="7ec965280a073311c339e571cd5de778b9975026cfcbe79f2b1cdcb1e15317ee"
ENV ANDROID_HOME=/opt/android-sdk
ENV PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"
RUN set -eux; \
    curl -fS "https://dl.google.com/android/repository/$ANDROID_SDK_DIST" -o sdk.zip; \
    echo "$ANDROID_SDK_SHA256" sdk.zip | sha256sum --check; \
    mkdir -p "$ANDROID_HOME"; \
    unzip -q -d "$ANDROID_HOME/cmdline-tools/" sdk.zip; \
    mv "$ANDROID_HOME/cmdline-tools/cmdline-tools" "$ANDROID_HOME/cmdline-tools/latest"; \
    rm sdk.zip; \
    yes | sdkmanager --licenses; \
    sdkmanager "platform-tools"

# Install Android NDK
ARG NDK_VERSION="28.0.13004108"
ENV ANDROID_NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
RUN sdkmanager "ndk;$NDK_VERSION"

## Cache gradle wrapper
COPY java/gradlew /libsignal/java/gradlew
COPY java/gradle /libsignal/java/gradle/
RUN /libsignal/java/gradlew --version

# Set optional read-only Gradle cache (mount at runtime)
ENV GRADLE_RO_DEP_CACHE=/.gradle-ro-cache

# Install Rust toolchain and Android targets
COPY rust-toolchain /libsignal/
RUN rustup toolchain install "$(cat /libsignal/rust-toolchain)" --profile minimal \
      --target aarch64-linux-android \
      --target armv7-linux-androideabi \
      --target x86_64-linux-android \
      --target aarch64-unknown-linux-gnu \
      --no-self-update

# Copy project files
COPY . /libsignal/
WORKDIR /libsignal
RUN git clean -ffdx

# Install compiler and build tools for the library, pinned via snapshot service
RUN SNAPSHOT_ID=$(git show -s --format=%cd --date=format:%Y%m%dT%H%M%SZ HEAD) && \
    apt-get update --snapshot "$SNAPSHOT_ID" && \
    apt-get install -y --no-install-recommends \
      clang \
      protobuf-compiler \
      cmake \
      make \
      crossbuild-essential-arm64 \
    && rm -rf /var/lib/apt/lists/*

# Create build script
RUN cat <<'EOF' > build.sh
#!/bin/sh
set -eu
export OVERRIDE_VERSION=$(git describe --tags --always)
./java/gradlew -p java "$@"
EOF
RUN chmod +x build.sh

# Set entrypoint to the build script
ENTRYPOINT ["/libsignal/build.sh"]
CMD ["--help"]
