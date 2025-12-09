use clap::Subcommand;
use crsdk::{
    property_category, property_description, property_display_name, property_value_type,
    EnableFlag, Result, ValueConstraint,
};
use crsdk_sys::DevicePropertyCode;

use super::{find_property_code, format_value};

#[derive(Subcommand)]
pub enum Args {
    /// List all properties
    List {
        /// Filter properties by name (case-insensitive substring match)
        #[arg(short, long)]
        filter: Option<String>,

        /// Show only writable properties
        #[arg(short, long)]
        writable: bool,

        /// Show raw hex values instead of constraints
        #[arg(long)]
        raw: bool,
    },
    /// Get detailed information about a property
    Get {
        /// Property name (e.g., IsoSensitivity, FNumber, ShutterSpeed)
        name: String,
    },
    /// Set a property value
    Set {
        /// Property name
        name: String,
        /// Value to set (raw numeric value)
        value: u64,
    },
}

pub fn run(device: &crsdk::blocking::CameraDevice, args: &Args) -> Result<()> {
    match args {
        Args::List {
            filter,
            writable,
            raw,
        } => list(device, filter.as_deref(), *writable, *raw),
        Args::Get { name } => get(device, name),
        Args::Set { name, value } => set(device, name, *value),
    }
}

fn list(
    device: &crsdk::blocking::CameraDevice,
    filter: Option<&str>,
    writable_only: bool,
    show_raw: bool,
) -> Result<()> {
    let properties = device.get_all_properties()?;

    let filter_lower = filter.map(|f| f.to_lowercase());

    let mut matched: Vec<_> = properties
        .iter()
        .filter(|p| {
            if writable_only && !p.enable_flag.is_writable() {
                return false;
            }

            if let Some(ref filter) = filter_lower {
                let Some(code) = DevicePropertyCode::from_raw(p.code) else {
                    return false;
                };
                let display = property_display_name(code).to_lowercase();
                let code_name = code.name().to_lowercase();
                if !display.contains(filter) && !code_name.contains(filter) {
                    return false;
                }
            }

            true
        })
        .collect();

    matched.sort_by(|a, b| {
        let code_a = DevicePropertyCode::from_raw(a.code);
        let code_b = DevicePropertyCode::from_raw(b.code);
        match (code_a, code_b) {
            (Some(ca), Some(cb)) => {
                let cat_a = format!("{:?}", property_category(ca));
                let cat_b = format!("{:?}", property_category(cb));
                cat_a.cmp(&cat_b).then_with(|| ca.name().cmp(cb.name()))
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.code.cmp(&b.code),
        }
    });

    println!(
        "{:<14} {:<35} {:<6} {:<20} RAW/CONSTRAINT",
        "CATEGORY", "PROPERTY", "RW", "VALUE"
    );
    println!("{}", "-".repeat(100));

    for prop in &matched {
        let Some(code) = DevicePropertyCode::from_raw(prop.code) else {
            continue;
        };
        let category = property_category(code);
        let display_name = property_display_name(code);
        let formatted = format_value(code, prop.current_value);

        let rw = match prop.enable_flag {
            EnableFlag::ReadWrite => "RW",
            EnableFlag::ReadOnly => "R-",
            EnableFlag::WriteOnly => "-W",
            EnableFlag::Disabled => "--",
            EnableFlag::NotSupported => "NS",
        };

        let constraint_info = if show_raw {
            format!("0x{:X}", prop.current_value)
        } else {
            match &prop.constraint {
                ValueConstraint::Range { min, max, step } => {
                    format!("range: {}..{} step {}", min, max, step)
                }
                ValueConstraint::Discrete(values) if values.len() <= 5 => {
                    format!("values: {:?}", values)
                }
                ValueConstraint::Discrete(values) => {
                    format!("{} values", values.len())
                }
                ValueConstraint::None => String::new(),
            }
        };

        let cat_str = format!("{:?}", category);
        let cat_display = if cat_str.len() > 14 {
            format!("{}…", &cat_str[..13])
        } else {
            cat_str
        };

        let name_display = if display_name.len() > 35 {
            format!("{}…", &display_name[..34])
        } else {
            display_name.to_string()
        };

        println!(
            "{:<14} {:<35} {:<6} {:<20} {}",
            cat_display, name_display, rw, formatted, constraint_info
        );
    }

    println!("\nTotal: {} properties", matched.len());

    Ok(())
}

pub fn get(device: &crsdk::blocking::CameraDevice, name: &str) -> Result<()> {
    let properties = device.get_all_properties()?;

    let code = find_property_code(&properties, name).ok_or_else(|| {
        crsdk::Error::InvalidParameter(format!(
            "Unknown property: '{}'. Use 'props list' to see available properties.",
            name
        ))
    })?;

    let prop = device.get_property(code)?;

    println!("Property: {}", property_display_name(code));
    println!("Code:     {} (0x{:X})", code.name(), prop.code);
    println!("Category: {:?}", property_category(code));
    println!();
    println!("Current Value:");
    println!("  Formatted: {}", format_value(code, prop.current_value));
    println!(
        "  Raw:       {} (0x{:X})",
        prop.current_value, prop.current_value
    );
    println!();
    println!("Status:   {:?}", prop.enable_flag);
    println!("DataType: {:?}", prop.data_type);
    println!("ValueType: {:?}", property_value_type(code));
    println!();

    match &prop.constraint {
        ValueConstraint::None => {
            println!("Constraint: None");
        }
        ValueConstraint::Range { min, max, step } => {
            let count = if *step > 0 { (max - min) / step + 1 } else { 0 };
            println!("Constraint: Range");
            println!("  Min:   {} (0x{:X})", min, *min as u64);
            println!("  Max:   {} (0x{:X})", max, *max as u64);
            println!("  Step:  {}", step);
            println!("  Count: {} values", count);
        }
        ValueConstraint::Discrete(values) => {
            println!("Constraint: Discrete ({} values)", values.len());
            if values.len() <= 20 {
                for val in values {
                    let formatted = format_value(code, *val);
                    println!("  {} (0x{:X}) = {}", val, val, formatted);
                }
            } else {
                println!("  (showing first 10 and last 5)");
                for val in values.iter().take(10) {
                    let formatted = format_value(code, *val);
                    println!("  {} (0x{:X}) = {}", val, val, formatted);
                }
                println!("  ...");
                for val in values
                    .iter()
                    .rev()
                    .take(5)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                {
                    let formatted = format_value(code, *val);
                    println!("  {} (0x{:X}) = {}", val, val, formatted);
                }
            }
        }
    }

    println!();
    println!("Description:");
    println!("  {}", property_description(code));

    Ok(())
}

pub fn set(device: &crsdk::blocking::CameraDevice, name: &str, value: u64) -> Result<()> {
    let properties = device.get_all_properties()?;

    let code = find_property_code(&properties, name)
        .ok_or_else(|| crsdk::Error::InvalidParameter(format!("Unknown property: '{}'", name)))?;

    let prop = device.get_property(code)?;

    println!("Property: {}", property_display_name(code));
    println!(
        "Current:  {} (raw: {})",
        format_value(code, prop.current_value),
        prop.current_value
    );
    println!("Setting:  {} (raw: {})", format_value(code, value), value);
    println!();

    if !prop.enable_flag.is_writable() {
        return Err(crsdk::Error::InvalidParameter(format!(
            "Property is not writable (status: {:?})",
            prop.enable_flag
        )));
    }

    if !prop.constraint.is_valid(value) {
        eprintln!(
            "Warning: Value {} may not be valid for this property",
            value
        );
        eprintln!("Constraint: {:?}", prop.constraint);
    }

    device.set_property(code, value)?;

    let updated = device.get_property(code)?;
    println!(
        "Result:   {} (raw: {})",
        format_value(code, updated.current_value),
        updated.current_value
    );

    if updated.current_value == value {
        println!("✓ Property set successfully");
    } else {
        println!("⚠ Value changed but may differ from requested");
    }

    Ok(())
}
