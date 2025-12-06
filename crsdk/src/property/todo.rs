//! TODO lists for property metadata coverage.
//!
//! This module contains lists of properties that still need display names,
//! descriptions, and value types. Remove items from these lists as you add the metadata.

use crsdk_sys::DevicePropertyCode;

/// Properties that need display names.
/// Remove items as you add display names to the subsystem modules.
///
/// Note: Some properties below have explicit entries that happen to match the
/// auto-generated format. The test can't distinguish these from fallthrough cases.
pub const NEEDS_DISPLAY_NAME: &[DevicePropertyCode] = &[];

/// Properties that need descriptions.
/// Remove items as you add descriptions to the subsystem modules.
pub const NEEDS_DESCRIPTION: &[DevicePropertyCode] = &[];

/// Properties that need value type mappings.
/// Remove items as you add value types to the subsystem modules.
pub const NEEDS_VALUE_TYPE: &[DevicePropertyCode] = &[DevicePropertyCode::Undefined];
