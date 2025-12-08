//! SDK data type classification.

/// SDK data type classification for property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// 8-bit unsigned integer
    UInt8,
    /// 16-bit unsigned integer
    UInt16,
    /// 32-bit unsigned integer
    UInt32,
    /// 64-bit unsigned integer
    UInt64,
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
    /// String value
    String,
    /// Unknown data type (raw SDK value)
    Unknown(u32),
}

impl DataType {
    pub(crate) fn from_sdk(value: u32) -> Self {
        use crsdk_sys::SCRSDK::*;

        const ARRAY_BIT: u32 = 0x2000;
        const RANGE_BIT: u32 = 0x4000;

        let base_type = value & !(ARRAY_BIT | RANGE_BIT);

        match base_type {
            x if x == CrDataType_CrDataType_UInt8 => Self::UInt8,
            x if x == CrDataType_CrDataType_UInt16 => Self::UInt16,
            x if x == CrDataType_CrDataType_UInt32 => Self::UInt32,
            x if x == CrDataType_CrDataType_UInt64 => Self::UInt64,
            x if x == CrDataType_CrDataType_Int8 => Self::Int8,
            x if x == CrDataType_CrDataType_Int16 => Self::Int16,
            x if x == CrDataType_CrDataType_Int32 => Self::Int32,
            x if x == CrDataType_CrDataType_Int64 => Self::Int64,
            x if x == CrDataType_CrDataType_STR => Self::String,
            _ => Self::Unknown(value),
        }
    }
}
