# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build the workspace
cargo build

# Build a specific example
cargo build --example connect

# Run the connect example
cargo run --example connect -- --ip <IP> --mac <MAC> --ssh --user <USER> --password <PASS>

# Run tests
cargo test

# Run tests for a specific crate
cargo test -p crsdk

# Run a single test
cargo test -p crsdk test_mac_addr_parse

# Run clippy linter
cargo clippy --all-features

# Format code
cargo fmt --all
```

## Initial Setup

The Sony SDK libraries are not included due to licensing. Before building:

```bash
./scripts/setup-libs.sh ~/Downloads/CrSDK_v2.00.00_20251030a_Mac.zip
```

The SDK must be downloaded from Sony's official site after accepting their license.

## Architecture

This is a Rust workspace wrapping the Sony Camera Remote SDK (C++) for controlling Sony cameras over network/USB.

### Two-Crate Design

- **crsdk-sys**: Unsafe FFI bindings auto-generated via bindgen. Contains `wrapper.h` and `callback_shim.cpp` for bridging C++ SDK to Rust.
- **crsdk**: Safe, idiomatic Rust API built on top of crsdk-sys. Provides async (tokio) and blocking APIs.

### Key Patterns

**Async wraps Blocking**: The async API in `crsdk/src/device.rs` wraps the blocking API from `crsdk/src/blocking/device.rs` using `tokio::task::spawn_blocking`. For sync use cases, import from `crsdk::blocking`.

**Builder Pattern**: Camera connections use `CameraDevice::builder()` for ergonomic configuration of IP, MAC, SSH credentials, etc.

**SDK Lifecycle**: The SDK is lazily initialized once via `ensure_sdk_initialized()` and kept alive for program lifetime (intentionally leaked).

**FFI Safety**: All unsafe SDK calls are encapsulated in the blocking module. The `CameraDevice` handle is managed via RAII (`Drop` impl calls `Disconnect` + `ReleaseDevice`).

### Module Layout (crsdk)

- `device.rs` - Async camera connection (wraps blocking)
- `blocking/device.rs` - Blocking camera connection (raw FFI calls)
- `types.rs` - `MacAddr`, `CameraModel`, `ConnectionInfo`
- `error.rs` - Error types with SDK error code mapping
- `sdk.rs` - SDK initialization

## Pre-commit Hooks

Pre-commit hooks run automatically: cargo-check, clippy, and rustfmt. These are configured via Nix.
