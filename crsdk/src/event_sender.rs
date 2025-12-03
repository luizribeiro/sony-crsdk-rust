//! Event sender for C++ callback to Rust channel bridge
//!
//! This module provides the FFI functions that C++ callbacks call to send events
//! to Rust. The EventSender wraps a tokio::sync::mpsc::UnboundedSender and is
//! passed to C++ as a void pointer.
//!
//! # Safety
//!
//! The C++ code must ensure it only calls these functions with a valid EventSender
//! pointer obtained from `EventSender::into_raw()`, and must not use the pointer
//! after calling `EventSender::from_raw()` to reclaim it.

use crate::event::CameraEvent;
use crate::property::PropertyCode;
use std::ffi::c_void;
use tokio::sync::mpsc::UnboundedSender;

/// Wrapper around a channel sender for passing to C++
///
/// This is heap-allocated and passed to C++ as a raw pointer.
/// C++ callback functions will call back into Rust with this pointer.
pub struct EventSender {
    sender: UnboundedSender<CameraEvent>,
}

impl EventSender {
    /// Create a new EventSender wrapping the given channel sender
    pub fn new(sender: UnboundedSender<CameraEvent>) -> Self {
        Self { sender }
    }

    /// Convert to a raw pointer for passing to C++
    ///
    /// The caller is responsible for eventually calling `from_raw` to reclaim
    /// the memory, or the EventSender will leak.
    pub fn into_raw(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut c_void
    }

    /// Reclaim an EventSender from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must have been created by `into_raw` and must not have been
    /// previously reclaimed.
    pub unsafe fn from_raw(ptr: *mut c_void) -> Self {
        // SAFETY: Caller guarantees ptr was created by into_raw and not yet reclaimed
        *unsafe { Box::from_raw(ptr as *mut Self) }
    }

    /// Send an event to the channel
    ///
    /// This is non-blocking and will never fail (unbounded channel).
    /// If the receiver is dropped, the event is silently discarded.
    fn send(&self, event: CameraEvent) {
        let _ = self.sender.send(event);
    }
}

// =============================================================================
// FFI functions called by C++ callback
//
// These functions are called from SDK threads when events occur.
// They must be fast and non-blocking (just send to channel).
//
// SAFETY: All functions assume `ctx` is a valid pointer to an EventSender
// that was created via EventSender::into_raw().
// =============================================================================

#[no_mangle]
pub extern "C" fn crsdk_event_connected(ctx: *mut c_void, version: u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::Connected { version });
}

#[no_mangle]
pub extern "C" fn crsdk_event_disconnected(ctx: *mut c_void, error: u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::Disconnected { error });
}

#[no_mangle]
pub extern "C" fn crsdk_event_property_changed(ctx: *mut c_void, num: u32, codes: *const u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };

    let codes = if codes.is_null() || num == 0 {
        Vec::new()
    } else {
        // SAFETY: C++ guarantees codes points to `num` valid u32 values
        let slice = unsafe { std::slice::from_raw_parts(codes, num as usize) };
        slice
            .iter()
            .filter_map(|&code| PropertyCode::from_raw(code))
            .collect()
    };

    sender.send(CameraEvent::PropertyChanged { codes });
}

#[no_mangle]
pub extern "C" fn crsdk_event_lv_property_changed(ctx: *mut c_void, num: u32, codes: *const u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };

    let codes = if codes.is_null() || num == 0 {
        Vec::new()
    } else {
        // SAFETY: C++ guarantees codes points to `num` valid u32 values
        unsafe { std::slice::from_raw_parts(codes, num as usize).to_vec() }
    };

    sender.send(CameraEvent::LiveViewPropertyChanged { codes });
}

#[no_mangle]
pub extern "C" fn crsdk_event_download_complete(ctx: *mut c_void, filename: *const i8) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };

    let filename = if filename.is_null() {
        String::new()
    } else {
        // SAFETY: C++ guarantees filename is null-terminated
        unsafe {
            std::ffi::CStr::from_ptr(filename)
                .to_string_lossy()
                .into_owned()
        }
    };

    sender.send(CameraEvent::DownloadComplete { filename });
}

#[no_mangle]
pub extern "C" fn crsdk_event_contents_transfer(
    ctx: *mut c_void,
    notify: u32,
    handle: u64,
    filename: *const i8,
) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };

    let filename = if filename.is_null() {
        None
    } else {
        // SAFETY: C++ guarantees filename is null-terminated if not null
        Some(unsafe {
            std::ffi::CStr::from_ptr(filename)
                .to_string_lossy()
                .into_owned()
        })
    };

    sender.send(CameraEvent::ContentsTransfer {
        notify,
        handle,
        filename,
    });
}

#[no_mangle]
pub extern "C" fn crsdk_event_warning(ctx: *mut c_void, warning: u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::Warning {
        code: warning,
        params: None,
    });
}

#[no_mangle]
pub extern "C" fn crsdk_event_warning_ext(
    ctx: *mut c_void,
    warning: u32,
    p1: i32,
    p2: i32,
    p3: i32,
) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::Warning {
        code: warning,
        params: Some((p1, p2, p3)),
    });
}

#[no_mangle]
pub extern "C" fn crsdk_event_error(ctx: *mut c_void, error: u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::Error { code: error });
}

#[no_mangle]
pub extern "C" fn crsdk_event_remote_transfer_progress(
    ctx: *mut c_void,
    notify: u32,
    percent: u32,
    filename: *const i8,
) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };

    let filename = if filename.is_null() {
        None
    } else {
        Some(unsafe {
            std::ffi::CStr::from_ptr(filename)
                .to_string_lossy()
                .into_owned()
        })
    };

    sender.send(CameraEvent::RemoteTransferProgress {
        notify,
        percent,
        filename,
    });
}

#[no_mangle]
pub extern "C" fn crsdk_event_remote_transfer_data(
    ctx: *mut c_void,
    notify: u32,
    percent: u32,
    data: *const u8,
    size: u64,
) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };

    let data = if data.is_null() || size == 0 {
        Vec::new()
    } else {
        // SAFETY: C++ guarantees data points to `size` valid bytes
        unsafe { std::slice::from_raw_parts(data, size as usize).to_vec() }
    };

    sender.send(CameraEvent::RemoteTransferData {
        notify,
        percent,
        data,
    });
}

#[no_mangle]
pub extern "C" fn crsdk_event_contents_list_changed(
    ctx: *mut c_void,
    notify: u32,
    slot: u32,
    added: u32,
) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::ContentsListChanged {
        notify,
        slot,
        added,
    });
}

#[no_mangle]
pub extern "C" fn crsdk_event_firmware_update(ctx: *mut c_void, notify: u32) {
    if ctx.is_null() {
        return;
    }
    // SAFETY: C++ guarantees ctx is a valid EventSender pointer
    let sender = unsafe { &*(ctx as *const EventSender) };
    sender.send(CameraEvent::FirmwareUpdateProgress { notify });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_sender_connected() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let sender = EventSender::new(tx);
        let ptr = sender.into_raw();

        crsdk_event_connected(ptr, 42);

        let event = rx.try_recv().unwrap();
        if let CameraEvent::Connected { version } = event {
            assert_eq!(version, 42);
        } else {
            panic!("Expected Connected event");
        }

        // Clean up
        let _ = unsafe { EventSender::from_raw(ptr) };
    }

    #[test]
    fn test_event_sender_disconnected() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let sender = EventSender::new(tx);
        let ptr = sender.into_raw();

        crsdk_event_disconnected(ptr, 0x8200);

        let event = rx.try_recv().unwrap();
        if let CameraEvent::Disconnected { error } = event {
            assert_eq!(error, 0x8200);
        } else {
            panic!("Expected Disconnected event");
        }

        let _ = unsafe { EventSender::from_raw(ptr) };
    }

    #[test]
    fn test_event_sender_property_changed() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let sender = EventSender::new(tx);
        let ptr = sender.into_raw();

        // Use valid SDK property codes (FNumber=0x0016, IsoSensitivity=0x0017)
        let codes: [u32; 2] = [
            crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FNumber,
            crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_IsoSensitivity,
        ];
        crsdk_event_property_changed(ptr, 2, codes.as_ptr());

        let event = rx.try_recv().unwrap();
        if let CameraEvent::PropertyChanged { codes } = event {
            assert_eq!(codes.len(), 2);
            assert_eq!(codes[0], PropertyCode::FNumber);
            assert_eq!(codes[1], PropertyCode::IsoSensitivity);
        } else {
            panic!("Expected PropertyChanged event");
        }

        let _ = unsafe { EventSender::from_raw(ptr) };
    }

    #[test]
    fn test_event_sender_null_ctx_no_panic() {
        crsdk_event_connected(std::ptr::null_mut(), 1);
        crsdk_event_disconnected(std::ptr::null_mut(), 0);
        crsdk_event_property_changed(std::ptr::null_mut(), 0, std::ptr::null());
        crsdk_event_warning(std::ptr::null_mut(), 0);
        crsdk_event_error(std::ptr::null_mut(), 0);
    }

    #[test]
    fn test_event_sender_multiple_events() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let sender = EventSender::new(tx);
        let ptr = sender.into_raw();

        crsdk_event_connected(ptr, 1);
        crsdk_event_warning(ptr, 100);
        crsdk_event_error(ptr, 200);

        assert!(matches!(
            rx.try_recv().unwrap(),
            CameraEvent::Connected { .. }
        ));
        assert!(matches!(
            rx.try_recv().unwrap(),
            CameraEvent::Warning { .. }
        ));
        assert!(matches!(rx.try_recv().unwrap(), CameraEvent::Error { .. }));
        assert!(rx.try_recv().is_err()); // No more events

        let _ = unsafe { EventSender::from_raw(ptr) };
    }
}
