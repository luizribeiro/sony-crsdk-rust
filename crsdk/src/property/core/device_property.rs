//! DeviceProperty struct and SDK parsing utilities.

use super::{DataType, EnableFlag, ValueConstraint};

/// A camera property with its current value and metadata.
#[derive(Debug, Clone)]
pub struct DeviceProperty {
    /// Property code (raw SDK value)
    pub code: u32,
    /// Data type
    pub data_type: DataType,
    /// Enable/writable status
    pub enable_flag: EnableFlag,
    /// Current value as u64 (for numeric properties)
    pub current_value: u64,
    /// Current value as string (for string properties)
    pub current_string: Option<String>,
    /// Constraint on what values this property can be set to
    pub constraint: ValueConstraint,
}

impl DeviceProperty {
    /// Check if this property can be read
    pub fn is_readable(&self) -> bool {
        self.enable_flag.is_readable()
    }

    /// Check if this property can be written
    pub fn is_writable(&self) -> bool {
        self.enable_flag.is_writable()
    }

    /// Check if a value is valid for this property
    pub fn is_valid_value(&self, value: u64) -> bool {
        self.constraint.is_valid(value)
    }

    /// Get discrete possible values (for backward compatibility and UI)
    ///
    /// Returns `Some(&[u64])` if this property has discrete values,
    /// `None` if it's a range or has no constraint.
    pub fn possible_values(&self) -> Option<&[u64]> {
        self.constraint.discrete_values()
    }

    /// Check if this property is range-constrained
    pub fn is_range(&self) -> bool {
        matches!(self.constraint, ValueConstraint::Range { .. })
    }

    /// Get range parameters if this is a range-constrained property
    ///
    /// Returns `Some((min, max, step))` if this property has a range constraint.
    pub fn range_params(&self) -> Option<(i64, i64, i64)> {
        self.constraint.range_params()
    }
}

const RANGE_BIT: u32 = 0x4000;

/// Parse raw values from SDK property data as u64 (for discrete values)
pub(crate) fn parse_raw_values(
    data_type: DataType,
    values_ptr: *mut u8,
    values_size: u32,
) -> Vec<u64> {
    if values_ptr.is_null() || values_size == 0 {
        return Vec::new();
    }

    let element_size = match data_type {
        DataType::UInt8 | DataType::Int8 => 1,
        DataType::UInt16 | DataType::Int16 => 2,
        DataType::UInt32 | DataType::Int32 => 4,
        DataType::UInt64 | DataType::Int64 => 8,
        _ => return Vec::new(),
    };

    let count = values_size as usize / element_size;
    let mut result = Vec::with_capacity(count);

    unsafe {
        for i in 0..count {
            let offset = i * element_size;
            let value = match data_type {
                DataType::UInt8 => *values_ptr.add(offset) as u64,
                DataType::Int8 => *values_ptr.add(offset) as i8 as u64,
                DataType::UInt16 => {
                    u16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as u64
                }
                DataType::Int16 => {
                    i16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as u64
                }
                DataType::UInt32 => u32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as u64,
                DataType::Int32 => i32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as u64,
                DataType::UInt64 => u64::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                    *values_ptr.add(offset + 4),
                    *values_ptr.add(offset + 5),
                    *values_ptr.add(offset + 6),
                    *values_ptr.add(offset + 7),
                ]),
                DataType::Int64 => i64::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                    *values_ptr.add(offset + 4),
                    *values_ptr.add(offset + 5),
                    *values_ptr.add(offset + 6),
                    *values_ptr.add(offset + 7),
                ]) as u64,
                _ => 0,
            };
            result.push(value);
        }
    }

    result
}

/// Parse raw values as signed i64 (for range constraints with signed types)
fn parse_raw_values_signed(data_type: DataType, values_ptr: *mut u8, values_size: u32) -> Vec<i64> {
    if values_ptr.is_null() || values_size == 0 {
        return Vec::new();
    }

    let element_size = match data_type {
        DataType::UInt8 | DataType::Int8 => 1,
        DataType::UInt16 | DataType::Int16 => 2,
        DataType::UInt32 | DataType::Int32 => 4,
        DataType::UInt64 | DataType::Int64 => 8,
        _ => return Vec::new(),
    };

    let count = values_size as usize / element_size;
    let mut result = Vec::with_capacity(count);

    unsafe {
        for i in 0..count {
            let offset = i * element_size;
            let value: i64 = match data_type {
                DataType::UInt8 => *values_ptr.add(offset) as i64,
                DataType::Int8 => (*values_ptr.add(offset) as i8) as i64,
                DataType::UInt16 => {
                    u16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as i64
                }
                DataType::Int16 => {
                    i16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as i64
                }
                DataType::UInt32 => u32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as i64,
                DataType::Int32 => i32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as i64,
                DataType::UInt64 => u64::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                    *values_ptr.add(offset + 4),
                    *values_ptr.add(offset + 5),
                    *values_ptr.add(offset + 6),
                    *values_ptr.add(offset + 7),
                ]) as i64,
                DataType::Int64 => i64::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                    *values_ptr.add(offset + 4),
                    *values_ptr.add(offset + 5),
                    *values_ptr.add(offset + 6),
                    *values_ptr.add(offset + 7),
                ]),
                _ => continue,
            };
            result.push(value);
        }
    }

    result
}

/// Parse a ValueConstraint from SDK property data
pub(crate) fn parse_constraint(
    raw_value_type: u32,
    data_type: DataType,
    values_ptr: *mut u8,
    values_size: u32,
) -> ValueConstraint {
    if values_ptr.is_null() || values_size == 0 {
        return ValueConstraint::None;
    }

    let is_range = (raw_value_type & RANGE_BIT) != 0;

    if is_range {
        let signed_values = parse_raw_values_signed(data_type, values_ptr, values_size);
        if signed_values.len() >= 3 {
            return ValueConstraint::Range {
                min: signed_values[0],
                max: signed_values[1],
                step: signed_values[2],
            };
        } else if signed_values.len() == 2 {
            return ValueConstraint::Range {
                min: signed_values[0],
                max: signed_values[1],
                step: 1,
            };
        }
    }

    let values = parse_raw_values(data_type, values_ptr, values_size);
    if values.is_empty() {
        ValueConstraint::None
    } else {
        ValueConstraint::Discrete(values)
    }
}

/// Parse UTF-16 string from SDK's currentStr pointer
unsafe fn parse_current_string(str_ptr: *const u16) -> Option<String> {
    if str_ptr.is_null() {
        return None;
    }

    let len = unsafe { *str_ptr } as usize;
    if len == 0 || len > 1024 {
        return None;
    }

    let char_count = len.saturating_sub(1);
    if char_count == 0 {
        return None;
    }

    let slice = unsafe { std::slice::from_raw_parts(str_ptr.add(1), char_count) };
    String::from_utf16(slice).ok()
}

/// Convert SDK CrDeviceProperty to our DeviceProperty
pub(crate) unsafe fn device_property_from_sdk(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> DeviceProperty {
    let data_type = DataType::from_sdk(prop.valueType);

    let constraint = if prop.getSetValuesSize > 0 && !prop.getSetValues.is_null() {
        parse_constraint(
            prop.valueType,
            data_type,
            prop.getSetValues,
            prop.getSetValuesSize,
        )
    } else {
        parse_constraint(prop.valueType, data_type, prop.values, prop.valuesSize)
    };

    let current_string = unsafe { parse_current_string(prop.currentStr) };

    DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        current_string,
        constraint,
    }
}

/// Convert SDK CrDeviceProperty to our DeviceProperty with debug info
pub(crate) unsafe fn device_property_from_sdk_debug(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> (DeviceProperty, String) {
    let data_type = DataType::from_sdk(prop.valueType);

    let values_from_sdk = parse_raw_values(data_type, prop.values, prop.valuesSize);
    let get_set_values = parse_raw_values(data_type, prop.getSetValues, prop.getSetValuesSize);

    let is_range = (prop.valueType & RANGE_BIT) != 0;

    let constraint = if prop.getSetValuesSize > 0 && !prop.getSetValues.is_null() {
        parse_constraint(
            prop.valueType,
            data_type,
            prop.getSetValues,
            prop.getSetValuesSize,
        )
    } else {
        parse_constraint(prop.valueType, data_type, prop.values, prop.valuesSize)
    };

    let current_string = unsafe { parse_current_string(prop.currentStr) };

    let debug_info = format!(
        "dataType={:?}(raw={:#06x}) is_range={} valuesSize={} values_ptr={:?} getSetValuesSize={} getSetValues_ptr={:?} values={:?} getSetValues={:?} constraint={:?} currentStr={:?}",
        data_type,
        prop.valueType,
        is_range,
        prop.valuesSize,
        prop.values,
        prop.getSetValuesSize,
        prop.getSetValues,
        values_from_sdk,
        get_set_values,
        constraint,
        current_string,
    );

    let device_prop = DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        current_string,
        constraint,
    };

    (device_prop, debug_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_property_is_valid_value_discrete() {
        let prop = DeviceProperty {
            code: 0,
            data_type: DataType::UInt32,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 100,
            current_string: None,
            constraint: ValueConstraint::Discrete(vec![100, 200, 400, 800]),
        };
        assert!(prop.is_valid_value(100));
        assert!(prop.is_valid_value(400));
        assert!(!prop.is_valid_value(300));
        assert!(prop.possible_values().is_some());
        assert_eq!(prop.possible_values().unwrap(), &[100, 200, 400, 800]);
    }

    #[test]
    fn test_device_property_is_valid_value_none() {
        let prop_empty = DeviceProperty {
            code: 0,
            data_type: DataType::UInt32,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 100,
            current_string: None,
            constraint: ValueConstraint::None,
        };
        assert!(prop_empty.is_valid_value(999));
        assert!(prop_empty.possible_values().is_none());
    }

    #[test]
    fn test_device_property_is_valid_value_range() {
        let prop_range = DeviceProperty {
            code: 0,
            data_type: DataType::UInt8,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 3,
            current_string: None,
            constraint: ValueConstraint::Range {
                min: 1,
                max: 7,
                step: 1,
            },
        };
        assert!(prop_range.is_valid_value(1));
        assert!(prop_range.is_valid_value(4));
        assert!(prop_range.is_valid_value(7));
        assert!(!prop_range.is_valid_value(0));
        assert!(!prop_range.is_valid_value(8));
        assert!(prop_range.is_range());
        assert_eq!(prop_range.range_params(), Some((1, 7, 1)));
        assert!(prop_range.possible_values().is_none());
    }
}
