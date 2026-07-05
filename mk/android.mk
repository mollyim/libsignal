# Copyright 2026 Molly Instant Messenger
# SPDX-License-Identifier: AGPL-3.0-only

## Extra arguments forwarded to Gradle
GRADLE_ARGS ?=

GRADLEW := ./java/gradlew -p java

do_assemble = $(strip $(GRADLEW) publishToMavenLocal $(GRADLE_ARGS))
do_test     = $(strip $(GRADLEW) build $(GRADLE_ARGS))
do_publish  = $(strip $(GRADLEW) publish $(GRADLE_ARGS))
do_clean    = $(strip $(GRADLEW) clean $(GRADLE_ARGS))
