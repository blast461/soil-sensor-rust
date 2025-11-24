# ESP32 Soil Humidity Sensor (Rust Reference)

Reference-only firmware targeting `xtensa-esp32-espidf`. The purpose of this repo is to exercise CI/CD for building ESP32 Rust firmware artifacts (ELF + `.bin` files); it is **not** concerned with device flashing.

For production deployments and full device-flash workflows, use the C++ project in `../soil-sensor-cpp/` instead.

## Status

- Builds a deployable ESP32 firmware artifact with ESP-IDF bindings (`esp-idf-sys` / `esp-idf-svc`)
- Toolchain pinned to the `esp` channel (via `rust-toolchain.toml`)
- CI uses the xtensa toolchain and runs `cargo build --release --target xtensa-esp32-espidf`; hardware flashing is intentionally out of scope

## Prerequisites (for local builds)

- `espup`-installed toolchain (or the `esp` toolchain from `rustup`)
- ESP-IDF v5.3.3 (set via `.cargo/config.toml`)
- `ldproxy` on PATH (used as linker)
- Python 3 and Git (required by ESP-IDF)

## Local build (equivalent to CI)

```powershell
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features --target xtensa-esp32-espidf -- -D warnings

# Build (release)
cargo build --release --target xtensa-esp32-espidf
```

The release artifacts (with the default `target` directory) will be under:

- `target/xtensa-esp32-espidf/release/soil-sensor-rust` (ELF firmware image)
- `target/xtensa-esp32-espidf/release/bootloader.bin`
- `target/xtensa-esp32-espidf/release/partition-table.bin`

Flashing these artifacts to real hardware is left to downstream tooling and is **not** covered by this project.

### WSL note: short target dir

When building from WSL with the repo on a mounted Windows drive (e.g. `/mnt/e/...`), `esp-idf-sys` requires a short build path. Keep sources where they are and redirect Cargo’s target to a short path on the Linux filesystem:

```bash
# In WSL shell
mkdir -p ~/t/soil-target
export CARGO_TARGET_DIR=~/t/soil-target
export CARGO_WORKSPACE_DIR=$(pwd)   # so esp-idf-sys finds Cargo.toml

cd /mnt/e/pico_dev/soil-sensor-rust
cargo build --release --target xtensa-esp32-espidf
```

Add the two `export` lines to your shell profile if you always build this way.

## Project Notes

- Target is set in `.cargo/config.toml` to `xtensa-esp32-espidf`
- `esp-idf-sys` enables `binstart`; `build.rs` uses `embuild` to propagate ESP-IDF cfg/link args
- Logging uses `EspLogger` (ESP-IDF backend)
- `Cargo.lock` is tracked for reproducible builds
- Release profile favors size (`opt-level = "s"`); dev uses `opt-level = "z"`

## Layout

- `src/main.rs` – reference application (simulated sensor loop)
- `.cargo/config.toml` – target/runner/IDF settings
- `build.rs` – ESP-IDF cfg/link propagation
- `Cargo.toml` – crate metadata and ESP-IDF dependencies
- `.github/workflows/rust_ci.yml` – xtensa build/fmt/clippy
- `rust-toolchain.toml` – pins `esp` toolchain
