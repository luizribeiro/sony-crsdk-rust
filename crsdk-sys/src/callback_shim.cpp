// Minimal IDeviceCallback implementation for Rust FFI
// All methods have empty default implementations in the SDK header
//
// TODO: Implement proper event forwarding to Rust
//
// The SDK calls these methods to notify about camera events:
//   - OnConnected(version)        -> connection established
//   - OnDisconnected(error)       -> connection lost
//   - OnPropertyChanged()         -> camera settings changed (ISO, shutter, etc.)
//   - OnPropertyChangedCodes()    -> which specific properties changed
//   - OnError(error)              -> camera error occurred
//   - OnWarning(warning)          -> camera warning
//   - OnCompleteDownload()        -> file download finished
//   - OnNotifyContentsTransfer()  -> transfer progress updates
//
// To implement:
// 1. Add Rust function pointers for each event type
// 2. Store them in a struct accessible from C++
// 3. Override each virtual method to call the Rust function pointer
// 4. On Rust side: send events through tokio::sync::mpsc channel
// 5. Expose as async Stream<Item=CameraEvent> on CameraDevice
//
// Example Rust API:
//   while let Some(event) = camera.events().next().await {
//       match event {
//           CameraEvent::PropertyChanged(codes) => { ... }
//           CameraEvent::Disconnected(error) => break,
//           _ => {}
//       }
//   }

#include "IDeviceCallback.h"

namespace {
    class MinimalCallback : public SCRSDK::IDeviceCallback {
    public:
        // All methods use default empty implementations from base class
    };

    static MinimalCallback g_callback;
}

extern "C" {
    SCRSDK::IDeviceCallback* crsdk_get_minimal_callback() {
        return &g_callback;
    }
}
