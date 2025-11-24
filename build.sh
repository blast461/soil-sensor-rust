# cargo fmt --all -- --check --color always
# cargo clippy --all-targets --all-features --workspace --target xtensa-esp32-espidf -- -D warnings

#!/usr/bin/env bash
set -euo pipefail

# Default values
RUN_CHECKS=true
RUN_FLASH=true
BUILD_RELEASE=true
CI=${CI:-false}

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --no-check|--skip-check) RUN_CHECKS=false; shift ;;
    --no-flash|--skip-flash) RUN_FLASH=false; shift ;;
    --debug) BUILD_RELEASE=false; shift ;;
    --ci) CI=true; RUN_CHECKS=true; RUN_FLASH=false; shift ;;
    -h|--help)
      echo "Usage: $0 [--no-check] [--no-flash] [--debug] [--ci]"
      exit 0
      ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
  esac
done

# Now use the flags
$RUN_CHECKS && {
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets --all-features --target xtensa-esp32-espidf -- -D warnings
}

cargo build $( [[ $BUILD_RELEASE == true ]] && echo "--release" ) \
  --target xtensa-esp32-espidf --workspace

# Flash only if not in CI and user didn't disable it
if [[ $RUN_FLASH == true && $CI == false ]]; then
  # ... same espflash logic as before
fi