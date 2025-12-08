//! Property metadata: descriptions, display names, and value type mappings.
//!
//! This module contains human-readable metadata for camera properties:
//! - `description(code)` - Detailed explanation of what the property does
//! - `display_name(code)` - Short human-readable name for UI display
//! - `value_type(code)` - Which value type this property uses

pub mod audio;
pub mod display;
pub mod drive;
pub mod exposure;
pub mod flash;
pub mod focus;
pub mod image;
pub mod lens;
pub mod media;
pub mod movie;
pub mod nd_filter;
pub mod other;
pub mod picture_profile;
pub mod power;
pub mod silent;
pub mod stabilization;
pub mod white_balance;
pub mod zoom;
