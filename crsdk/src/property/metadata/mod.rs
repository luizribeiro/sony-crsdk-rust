//! Property metadata: descriptions, display names, and value type mappings.
//!
//! This module contains human-readable metadata for camera properties:
//! - `description(code)` - Detailed explanation of what the property does
//! - `display_name(code)` - Short human-readable name for UI display
//! - `value_type(code)` - Which value type this property uses

pub mod drive;
pub mod exposure;
pub mod flash;
pub mod focus;
pub mod image;
pub mod movie;
pub mod white_balance;
