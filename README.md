# Sony Camera Remote SDK - Rust Wrapper

Safe, idiomatic Rust bindings for the Sony Camera Remote SDK, enabling programmatic control of Sony cameras via network connection.

## Supported Cameras

- Sony FX3, FX6, FX30
- Sony α (Alpha) series: α1, α7 IV, α7R V, α7S III, α9 II, α9 III
- See [Sony's official compatibility list](https://support.d-imaging.sony.co.jp/app/sdk/en/index.html)

## Setup

### 1. Download Sony Camera Remote SDK

The SDK libraries are **not included** due to licensing restrictions.

1. Visit [Sony Camera Remote SDK](https://support.d-imaging.sony.co.jp/app/sdk/en/index.html)
2. Accept the license agreement
3. Download `CrSDK_v2.00.00_20251030a_Mac.zip`

### 2. Set Up Libraries

```bash
./scripts/setup-libs.sh ~/Downloads/CrSDK_v2.00.00_20251030a_Mac.zip
```

### 3. Build

```bash
cargo build --example connect
```

## Usage

### Connect to a Camera

```bash
cargo run --example connect -- \
  --ip 192.168.1.100 \
  --mac 00:00:00:00:00:00 \
  --ssh \
  --user your_username \
  --password yourpassword
```

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

    println!("Connected to {}", camera.model());
    Ok(())
}
```

## Project Structure

```
sony-crsdk-rust/
├── crsdk-sys/          # Unsafe FFI bindings (auto-generated via bindgen)
├── crsdk/              # Safe Rust wrapper
│   ├── src/
│   │   ├── device.rs   # Camera connection
│   │   ├── types.rs    # MacAddr, CameraModel, etc.
│   │   ├── error.rs    # Error types
│   │   └── sdk.rs      # SDK lifecycle
│   └── examples/
│       └── connect.rs  # Connection example
├── libs/               # SDK libraries (not in git, created by setup script)
└── scripts/
    └── setup-libs.sh   # Library setup script
```

## Development Status

**Phase 1: Foundation** (current)
- SDK initialization and lifecycle
- Network connection (IP + MAC + SSH)
- Error handling

Future phases:
- Property system (ISO, aperture, shutter speed, etc.)
- Shooting operations
- Live view streaming
- Event callbacks

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
