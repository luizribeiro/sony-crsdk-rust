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
#include "ICrCameraObjectInfo.h"

// C shim functions for ICrEnumCameraObjectInfo virtual methods
extern "C" {
    CrInt32u crsdk_enum_camera_get_count(const SCRSDK::ICrEnumCameraObjectInfo* enumInfo) {
        if (!enumInfo) return 0;
        return enumInfo->GetCount();
    }

    const SCRSDK::ICrCameraObjectInfo* crsdk_enum_camera_get_info(
        const SCRSDK::ICrEnumCameraObjectInfo* enumInfo,
        CrInt32u index
    ) {
        if (!enumInfo) return nullptr;
        return enumInfo->GetCameraObjectInfo(index);
    }

    void crsdk_enum_camera_release(SCRSDK::ICrEnumCameraObjectInfo* enumInfo) {
        if (enumInfo) enumInfo->Release();
    }
}

// C shim functions for ICrCameraObjectInfo virtual methods
extern "C" {
    const CrChar* crsdk_camera_info_get_model(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return nullptr;
        return info->GetModel();
    }

    CrInt32u crsdk_camera_info_get_model_size(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetModelSize();
    }

    const CrChar* crsdk_camera_info_get_name(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return nullptr;
        return info->GetName();
    }

    CrInt32u crsdk_camera_info_get_name_size(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetNameSize();
    }

    CrInt32u crsdk_camera_info_get_connection_status(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetConnectionStatus();
    }

    const CrChar* crsdk_camera_info_get_connection_type(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return nullptr;
        return info->GetConnectionTypeName();
    }

    CrInt32u crsdk_camera_info_get_ip_address(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetIPAddress();
    }

    const CrChar* crsdk_camera_info_get_ip_address_str(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return nullptr;
        return info->GetIPAddressChar();
    }

    const CrInt8u* crsdk_camera_info_get_mac_address(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return nullptr;
        return info->GetMACAddress();
    }

    CrInt32u crsdk_camera_info_get_mac_address_size(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetMACAddressSize();
    }

    CrInt32u crsdk_camera_info_get_ssh_support(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetSSHsupport();
    }

    CrInt16 crsdk_camera_info_get_usb_pid(const SCRSDK::ICrCameraObjectInfo* info) {
        if (!info) return 0;
        return info->GetUsbPid();
    }

    void crsdk_camera_info_release(SCRSDK::ICrCameraObjectInfo* info) {
        if (info) info->Release();
    }
}

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
