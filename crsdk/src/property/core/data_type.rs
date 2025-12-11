//! SDK data type classification.

use crate::types::FromCrsdk;

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
    /// Returns true if this is a signed integer type.
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Int8 | Self::Int16 | Self::Int32 | Self::Int64)
    }

    /// Format a raw u64 value as a signed integer string if this is a signed type.
    /// For unsigned types, returns the value as-is.
    pub fn format_raw(&self, raw: u64) -> String {
        match self {
            Self::Int8 => format!("{}", raw as i8),
            Self::Int16 => format!("{}", raw as i16),
            Self::Int32 => format!("{}", raw as i32),
            Self::Int64 => format!("{}", raw as i64),
            _ => format!("{}", raw),
        }
    }

    /// Format a raw u64 value as a hex string, using signed representation for signed types.
    pub fn format_raw_hex(&self, raw: u64) -> String {
        match self {
            Self::Int8 => {
                let v = raw as i8;
                if v < 0 {
                    format!("-0x{:X}", -(v as i16))
                } else {
                    format!("0x{:X}", v)
                }
            }
            Self::Int16 => {
                let v = raw as i16;
                if v < 0 {
                    format!("-0x{:X}", -(v as i32))
                } else {
                    format!("0x{:X}", v)
                }
            }
            Self::Int32 => {
                let v = raw as i32;
                if v < 0 {
                    format!("-0x{:X}", -(v as i64))
                } else {
                    format!("0x{:X}", v)
                }
            }
            Self::Int64 => {
                let v = raw as i64;
                if v < 0 {
                    format!("-0x{:X}", v.wrapping_neg() as u64)
                } else {
                    format!("0x{:X}", v)
                }
            }
            _ => format!("0x{:X}", raw),
        }
    }
}

impl FromCrsdk<u32> for DataType {
    fn from_crsdk(value: u32) -> crate::error::Result<Self> {
        use crsdk_sys::SCRSDK::*;

        const ARRAY_BIT: u32 = 0x2000;
        const RANGE_BIT: u32 = 0x4000;

        let base_type = value & !(ARRAY_BIT | RANGE_BIT);

        Ok(match base_type {
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
        })
    }
}
