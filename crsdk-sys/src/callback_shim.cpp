// IDeviceCallback implementation for Rust FFI
//
// This file provides two callback implementations:
// 1. MinimalCallback - does nothing, used when events aren't needed
// 2. RustCallback - forwards all events to Rust via function pointers
//
// The RustCallback stores a context pointer (the Rust channel sender) and
// calls Rust FFI functions for each event. These functions are non-blocking
// and simply send to a tokio::sync::mpsc channel.

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

// Rust FFI function declarations - these are implemented in Rust
// and send events to a tokio::sync::mpsc channel
extern "C" {
    void crsdk_event_connected(void* ctx, CrInt32u version);
    void crsdk_event_disconnected(void* ctx, CrInt32u error);
    void crsdk_event_property_changed(void* ctx, CrInt32u num, const CrInt32u* codes);
    void crsdk_event_lv_property_changed(void* ctx, CrInt32u num, const CrInt32u* codes);
    void crsdk_event_download_complete(void* ctx, const CrChar* filename);
    void crsdk_event_contents_transfer(void* ctx, CrInt32u notify, CrInt64u handle, const CrChar* filename);
    void crsdk_event_warning(void* ctx, CrInt32u warning);
    void crsdk_event_warning_ext(void* ctx, CrInt32u warning, CrInt32 p1, CrInt32 p2, CrInt32 p3);
    void crsdk_event_error(void* ctx, CrInt32u error);
    void crsdk_event_remote_transfer_progress(void* ctx, CrInt32u notify, CrInt32u percent, const CrChar* filename);
    void crsdk_event_remote_transfer_data(void* ctx, CrInt32u notify, CrInt32u percent, const CrInt8u* data, CrInt64u size);
    void crsdk_event_contents_list_changed(void* ctx, CrInt32u notify, CrInt32u slot, CrInt32u added);
    void crsdk_event_firmware_update(void* ctx, CrInt32u notify);
}

// Callback class that forwards events to Rust
class RustCallback : public SCRSDK::IDeviceCallback {
public:
    explicit RustCallback(void* ctx) : ctx_(ctx) {}

    void OnConnected(SCRSDK::DeviceConnectionVersioin version) override {
        if (ctx_) crsdk_event_connected(ctx_, static_cast<CrInt32u>(version));
    }

    void OnDisconnected(CrInt32u error) override {
        if (ctx_) crsdk_event_disconnected(ctx_, error);
    }

    void OnPropertyChanged() override {
        // Use the codes version instead for more detail
    }

    void OnPropertyChangedCodes(CrInt32u num, CrInt32u* codes) override {
        if (ctx_) crsdk_event_property_changed(ctx_, num, codes);
    }

    void OnLvPropertyChanged() override {
        // Use the codes version instead for more detail
    }

    void OnLvPropertyChangedCodes(CrInt32u num, CrInt32u* codes) override {
        if (ctx_) crsdk_event_lv_property_changed(ctx_, num, codes);
    }

    void OnCompleteDownload(CrChar* filename, CrInt32u /*type*/) override {
        if (ctx_) crsdk_event_download_complete(ctx_, filename);
    }

    void OnNotifyContentsTransfer(CrInt32u notify, SCRSDK::CrContentHandle handle, CrChar* filename) override {
        if (ctx_) crsdk_event_contents_transfer(ctx_, notify, handle, filename);
    }

    void OnWarning(CrInt32u warning) override {
        if (ctx_) crsdk_event_warning(ctx_, warning);
    }

    void OnWarningExt(CrInt32u warning, CrInt32 p1, CrInt32 p2, CrInt32 p3) override {
        if (ctx_) crsdk_event_warning_ext(ctx_, warning, p1, p2, p3);
    }

    void OnError(CrInt32u error) override {
        if (ctx_) crsdk_event_error(ctx_, error);
    }

    void OnNotifyRemoteTransferResult(CrInt32u notify, CrInt32u percent, CrChar* filename) override {
        if (ctx_) crsdk_event_remote_transfer_progress(ctx_, notify, percent, filename);
    }

    void OnNotifyRemoteTransferResult(CrInt32u notify, CrInt32u percent, CrInt8u* data, CrInt64u size) override {
        if (ctx_) crsdk_event_remote_transfer_data(ctx_, notify, percent, data, size);
    }

    void OnNotifyRemoteTransferContentsListChanged(CrInt32u notify, CrInt32u slot, CrInt32u added) override {
        if (ctx_) crsdk_event_contents_list_changed(ctx_, notify, slot, added);
    }

    void OnNotifyRemoteFirmwareUpdateResult(CrInt32u notify, const void* /*param*/) override {
        if (ctx_) crsdk_event_firmware_update(ctx_, notify);
    }

    // These are less commonly needed - use defaults for now
    // void OnNotifyFTPTransferResult(...) override { }
    // void OnReceivePlaybackTimeCode(...) override { }
    // void OnReceivePlaybackData(...) override { }
    // void OnNotifyMonitorUpdated(...) override { }

private:
    void* ctx_;
};

extern "C" {
    SCRSDK::IDeviceCallback* crsdk_get_minimal_callback() {
        return &g_callback;
    }

    // Create a new RustCallback with the given context (Rust channel sender pointer)
    SCRSDK::IDeviceCallback* crsdk_create_rust_callback(void* ctx) {
        return new RustCallback(ctx);
    }

    // Destroy a RustCallback
    void crsdk_destroy_rust_callback(SCRSDK::IDeviceCallback* callback) {
        delete callback;
    }
}
