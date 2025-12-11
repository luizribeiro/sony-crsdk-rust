# Sony Camera Remote SDK - Rust Wrapper

Safe, idiomatic Rust bindings for the Sony Camera Remote SDK, enabling programmatic control of Sony cameras via network or USB.

## Supported Cameras

- Sony FX3, FX6, FX30
- Sony α (Alpha) series: α1, α7 IV, α7R V, α7S III, α9 II, α9 III
- See [Sony's official compatibility list](https://support.d-imaging.sony.co.jp/app/sdk/en/index.html)

## Supported Platforms

- **macOS** (12.1+): Monterey, Ventura, Sonoma
- **Linux**: Ubuntu 20.04 LTS or compatible

## Setup

### 1. Download Sony Camera Remote SDK

The SDK libraries are **not included** due to licensing restrictions.

1. Visit [Sony Camera Remote SDK](https://support.d-imaging.sony.co.jp/app/sdk/en/index.html)
2. Accept the license agreement
3. Download the SDK for your platform:
   - macOS: `CrSDK_v2.00.00_20251030a_Mac.zip`
   - Linux: `CrSDK_v2.00.00_20251030a_Linux.zip`

### 2. Set Up Libraries

```bash
./scripts/setup-libs.sh ~/Downloads/CrSDK_v2.00.00_20251030a_Mac.zip
```

This extracts SDK libraries and fixes their install names for proper loading.

### 3. Build

```bash
cargo build
```

The build process automatically creates the necessary symlinks for the SDK to find its adapter libraries. These symlinks are recreated on every build, so `cargo clean` won't break the SDK.

## Usage

### sonyctl - CLI/TUI Tool

The easiest way to interact with your camera is through `sonyctl`:

```bash
# Launch interactive TUI with camera discovery
cargo run -p sonyctl -- tui

# Or connect directly to a known camera
cargo run -p sonyctl -- --ip 192.168.1.100 --mac 00:00:00:00:00:00 tui
```

#### CLI Commands

```bash
# Show camera info
sonyctl --ip 192.168.1.100 --mac 00:00:00:00:00:00 info

# List all camera properties
sonyctl --ip 192.168.1.100 --mac 00:00:00:00:00:00 props list

# Filter properties by name
sonyctl props list --filter iso

# Get detailed property info
sonyctl props get IsoSensitivity

# Set a property value
sonyctl props set IsoSensitivity 800

# Capture a photo
sonyctl capture

# Video recording
sonyctl record start
sonyctl record stop
```

#### Environment Variables

Connection parameters can be set via environment variables:

```bash
export SONY_CAMERA_IP=192.168.1.100
export SONY_CAMERA_MAC=00:00:00:00:00:00
export SONY_SSH_USER=your_username
export SONY_SSH_PASSWORD=your_password

# Then simply:
sonyctl tui
sonyctl capture
```

### Examples

```bash
# Discover cameras on network/USB
cargo run --example discover

# Connect to a camera
cargo run --example connect -- --ip 192.168.1.100 --mac 00:00:00:00:00:00

# Monitor camera events
cargo run --example events -- --ip 192.168.1.100 --mac 00:00:00:00:00:00

# Inspect and modify properties
cargo run --example props -- --ip 192.168.1.100 --mac 00:00:00:00:00:00 list
```

For SSH connections, add `--ssh --user <USER> --password <PASS>`.

Find your camera's MAC address with:
```bash
arp -n <camera-ip>
```

### As a Library

```rust
use crsdk::{CameraDevice, CameraModel, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let camera = CameraDevice::builder()
        .ip_address("192.168.1.100".parse()?)
        .mac_address("00:00:00:00:00:00".parse()?)
        .model(CameraModel::Fx3)
        .ssh_credentials("your_username", "yourpassword")
        .connect()
        .await?;

    println!("Connected to {}", camera.model().await);
    Ok(())
}
```

For blocking (non-async) usage:

```rust
use crsdk::blocking::CameraDevice;
```

## Project Structure

```
sony-crsdk-rust/
├── crsdk-sys/           # Unsafe FFI bindings (auto-generated via bindgen)
├── crsdk/               # Safe Rust wrapper library
│   ├── src/
│   │   ├── device.rs    # Async camera connection
│   │   ├── blocking/    # Blocking (sync) API
│   │   ├── property/    # Property system (ISO, aperture, etc.)
│   │   ├── command.rs   # Shooting commands
│   │   ├── event.rs     # Camera event types
│   │   ├── types.rs     # MacAddr, CameraModel, etc.
│   │   ├── error.rs     # Error types
│   │   └── sdk.rs       # SDK lifecycle
│   └── examples/
│       ├── connect.rs   # Basic connection
│       ├── discover.rs  # Camera discovery
│       ├── events.rs    # Event monitoring
│       └── props.rs     # Property inspector
├── sonyctl/             # CLI/TUI application
│   └── src/
│       ├── commands/    # CLI command handlers
│       └── tui/         # Terminal UI
├── libs/                # SDK libraries (created by setup script)
└── scripts/
    └── setup-libs.sh    # Library setup script
```

## Features

### Implemented

- SDK initialization and lifecycle
- Camera discovery (network and USB enumeration)
- Network connection (IP + MAC + SSH)
- Property system (ISO, aperture, shutter speed, focus mode, white balance, etc.)
- Shooting operations (capture, autofocus, movie recording)
- Event callbacks (property changes, warnings, errors, transfer progress)
- Interactive TUI with camera discovery

### Planned

- Live view streaming
- Content transfer (download images/videos)
- Advanced features (firmware update, settings management)

## Troubleshooting

### Camera connection issues

1. Enable **Network Remote Control** on the camera
2. Verify IP connectivity: `ping <camera-ip>`
3. Get MAC address: `arp -n <camera-ip>`
4. Check SSH credentials match camera settings

### Bindgen errors

Ensure LLVM/Clang is available. With devenv:
```bash
direnv allow
```

## License

MIT OR Apache-2.0

**Note:** The Sony Camera Remote SDK is proprietary. Obtain it from Sony and accept their license.
