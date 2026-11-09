#!/usr/bin/env bash
# ci-cd-fix: llvm-tools-preview fuer rust-ci / system-ci / test-suite
# Grund: bootloader 0.11 build.rs benoetigt llvm-tools (objcopy) fuer die
# BIOS-Stages. Ohne das Component panic: "failed to get llvm tools: NotFound"
# => cargo check/test --workspace schlagen fehl, seit boot im Workspace ist.
# Ausfuehren vom Repo-Root:  bash ci-cd-fix/apply-fix.sh && git commit + push
set -euo pipefail
cp ci-cd-fix/rust-ci.yml .github/workflows/rust-ci.yml
cp ci-cd-fix/system-ci.yml .github/workflows/system-ci.yml
cp ci-cd-fix/test-suite.yml .github/workflows/test-suite.yml
echo "OK — jetzt committen und pushen (needs workflow-scope token)."
