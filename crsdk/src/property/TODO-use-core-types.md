# TODO: Use core/ types in mod.rs

This document describes the remaining refactoring work to complete the property module restructure.

## Current State

The `core/` directory was created with infrastructure types:
- `core/data_type.rs` - `DataType` enum
- `core/enable_flag.rs` - `EnableFlag` enum
- `core/constraint.rs` - `ValueConstraint` enum
- `core/device_property.rs` - `DeviceProperty` struct + parsing utilities

However, `mod.rs` still defines these same types (lines 40-215 for enums, lines 1010+ for DeviceProperty). The core/ types are currently unused.

## Goal

Replace the duplicate type definitions in `mod.rs` with re-exports from `core/`, reducing `mod.rs` from ~1,674 lines to ~1,000 lines.

## Steps

### 1. Add core module declaration

In `mod.rs`, add:
```rust
mod core;
```

### 2. Update re-exports

Replace the type definitions with re-exports:
```rust
// Re-export core infrastructure types
pub use core::{DataType, DeviceProperty, EnableFlag, ValueConstraint};
```

### 3. Delete duplicate definitions from mod.rs

Remove these sections from `mod.rs`:
- `DataType` enum and impl (lines ~40-85)
- `EnableFlag` enum and impl (lines ~87-124)
- `ValueConstraint` enum and impl (lines ~126-215)
- `DeviceProperty` struct and impl (lines ~1010-1340)
- Related parsing functions (`parse_raw_values`, `parse_range_constraint`, etc.)

### 4. Update internal references

In `mod.rs`, the `property_value_type` function and tests use these types. After removing the definitions, they'll use the re-exported versions automatically.

### 5. Fix visibility

The `core/` types use `pub(crate)` for some methods like `from_sdk()`. Ensure these are accessible where needed:
- `DataType::from_sdk()` - used in property parsing
- `EnableFlag::from_sdk()` - used in property parsing
- Parsing functions in `device_property.rs`

You may need to change some `pub(crate)` to `pub` or adjust the module structure.

### 6. Move tests

The tests at the end of `mod.rs` that test `EnableFlag`, `ValueConstraint`, and `DeviceProperty` should be moved to their respective files in `core/`.

### 7. Run tests

```bash
cargo test -p crsdk
cargo clippy --all-features
```

## Expected Result

After this refactoring:
- `mod.rs` will be ~1,000 lines (only PropertyValueType, dispatch functions, re-exports)
- `core/` will contain all infrastructure types with their tests
- No duplicate code

## Notes

- This is a breaking change if any external code imports these types from specific paths
- The public API (`crsdk::property::DataType`, etc.) will remain unchanged since we're using re-exports
- Consider doing this in a separate PR for easier review
