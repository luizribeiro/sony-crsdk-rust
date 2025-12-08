//! sonyctl - CLI tool for controlling Sony cameras
//!
//! # Environment Variables
//!
//! Connection parameters can be set via environment variables:
//!
//! ```bash
//! export SONY_CAMERA_IP=192.168.1.100
//! export SONY_CAMERA_MAC=00:00:00:00:00:00
//! export SONY_SSH_USER=your_username
//! export SONY_SSH_PASSWORD=your_password
//! ```
//!
//! # Usage
//!
//! ```bash
//! # List all properties
//! sonyctl props list
//!
//! # Filter properties
//! sonyctl props list --filter iso
//!
//! # Get property details
//! sonyctl props get IsoSensitivity
//!
//! # Set property value
//! sonyctl props set IsoSensitivity 800
//!
//! # Capture a photo
//! sonyctl capture
//!
//! # Start/stop recording
//! sonyctl record start
//! sonyctl record stop
//! ```

use clap::{Parser, Subcommand};
use crsdk::{
    format::{format_aperture, format_exposure_comp, format_iso, format_shutter_speed},
    property_category, property_description, property_display_name, property_value_type,
    CameraDevice, CameraModel, DeviceProperty, EnableFlag, PropertyValueType, Result,
    ValueConstraint,
};
use crsdk_sys::DevicePropertyCode;
use dialoguer::Confirm;

#[derive(Parser)]
#[command(name = "sonyctl")]
#[command(about = "CLI tool for controlling Sony cameras via the Camera Remote SDK")]
#[command(version)]
struct Cli {
    /// Camera IP address
    #[arg(long, env = "SONY_CAMERA_IP")]
    ip: String,

    /// Camera MAC address
    #[arg(long, env = "SONY_CAMERA_MAC")]
    mac: String,

    /// SSH username (enables SSH mode)
    #[arg(long, env = "SONY_SSH_USER")]
    user: Option<String>,

    /// SSH password
    #[arg(long, env = "SONY_SSH_PASSWORD")]
    password: Option<String>,

    /// Skip SSH fingerprint confirmation
    #[arg(long, env = "SONY_SSH_TRUST")]
    trust: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Property operations
    Props {
        #[command(subcommand)]
        action: PropsAction,
    },
    /// Capture a photo
    Capture,
    /// Video recording control
    Record {
        #[command(subcommand)]
        action: RecordAction,
    },
    /// Show camera info
    Info,
}

#[derive(Subcommand)]
enum PropsAction {
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

#[derive(Subcommand)]
enum RecordAction {
    /// Start video recording
    Start,
    /// Stop video recording
    Stop,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let device = connect(&cli).await?;

    match cli.command {
        Command::Props { action } => match action {
            PropsAction::List {
                filter,
                writable,
                raw,
            } => {
                list_properties(&device, filter.as_deref(), writable, raw)?;
            }
            PropsAction::Get { name } => {
                get_property(&device, &name)?;
            }
            PropsAction::Set { name, value } => {
                set_property(&device, &name, value)?;
            }
        },
        Command::Capture => {
            capture(&device)?;
        }
        Command::Record { action } => match action {
            RecordAction::Start => {
                device.start_recording()?;
                println!("Recording started");
            }
            RecordAction::Stop => {
                device.stop_recording()?;
                println!("Recording stopped");
            }
        },
        Command::Info => {
            show_info(&device)?;
        }
    }

    Ok(())
}

async fn connect(cli: &Cli) -> Result<crsdk::blocking::CameraDevice> {
    eprintln!("Connecting to {}...", cli.ip);

    let mut builder = CameraDevice::builder()
        .ip_address(cli.ip.parse().expect("Invalid IP address"))
        .mac_address(cli.mac.parse().expect("Invalid MAC address"))
        .model(CameraModel::Fx3);

    // Enable SSH if credentials provided
    if let (Some(user), Some(password)) = (&cli.user, &cli.password) {
        builder = builder.ssh_enabled(true);

        eprintln!("Fetching SSH fingerprint...");
        let fingerprint = builder.fetch_ssh_fingerprint().await?;

        if !cli.trust {
            eprintln!("SSH Fingerprint: {}", fingerprint);

            let confirmed = Confirm::new()
                .with_prompt("Trust this fingerprint?")
                .default(false)
                .interact()
                .unwrap_or(false);

            if !confirmed {
                eprintln!("Connection cancelled.");
                std::process::exit(1);
            }
        }

        builder = builder
            .ssh_credentials(user, password)
            .ssh_fingerprint(fingerprint);
    }

    let camera = builder.connect().await?;
    eprintln!("Connected to {}", camera.model().await);

    // Wait for camera to populate properties
    eprintln!("Loading properties...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    eprintln!();

    Ok(camera.into_inner())
}

fn find_property_code(properties: &[DeviceProperty], name: &str) -> Option<DevicePropertyCode> {
    let name_lower = name.to_lowercase();

    properties.iter().find_map(|prop| {
        let code = DevicePropertyCode::from_raw(prop.code)?;
        let display = property_display_name(code).to_lowercase();
        let code_name = code.name().to_lowercase();

        if display == name_lower || code_name == name_lower {
            Some(code)
        } else {
            None
        }
    })
}

fn list_properties(
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
            _ => a.code.cmp(&b.code),
        }
    });

    println!(
        "{:<14} {:<35} {:<6} {:<20} {}",
        "CATEGORY", "PROPERTY", "RW", "VALUE", "RAW/CONSTRAINT"
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

fn get_property(device: &crsdk::blocking::CameraDevice, name: &str) -> Result<()> {
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

fn set_property(device: &crsdk::blocking::CameraDevice, name: &str, value: u64) -> Result<()> {
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

fn capture(device: &crsdk::blocking::CameraDevice) -> Result<()> {
    println!("Capturing...");
    device.capture()?;
    println!("✓ Capture complete");
    Ok(())
}

fn show_info(device: &crsdk::blocking::CameraDevice) -> Result<()> {
    println!("Camera Model: {:?}", device.model());

    let properties = device.get_all_properties()?;
    let writable = properties
        .iter()
        .filter(|p| p.enable_flag.is_writable())
        .count();
    let readonly = properties
        .iter()
        .filter(|p| matches!(p.enable_flag, EnableFlag::ReadOnly))
        .count();

    println!(
        "Properties:   {} total ({} writable, {} read-only)",
        properties.len(),
        writable,
        readonly
    );

    // Show some key properties if available
    for code in [
        DevicePropertyCode::ExposureProgramMode,
        DevicePropertyCode::IsoSensitivity,
        DevicePropertyCode::FNumber,
        DevicePropertyCode::ShutterSpeed,
        DevicePropertyCode::FocusMode,
        DevicePropertyCode::WhiteBalance,
    ] {
        if let Ok(prop) = device.get_property(code) {
            println!(
                "  {}: {}",
                property_display_name(code),
                format_value(code, prop.current_value)
            );
        }
    }

    Ok(())
}

fn format_value(code: DevicePropertyCode, raw: u64) -> String {
    match property_value_type(code) {
        PropertyValueType::Aperture => format_aperture(raw),
        PropertyValueType::ShutterSpeed => format_shutter_speed(raw),
        PropertyValueType::Iso => format_iso(raw),
        PropertyValueType::ExposureCompensation => format_exposure_comp(raw as i64),
        PropertyValueType::ColorTemperature => format!("{}K", raw),
        PropertyValueType::ExposureProgram => crsdk::ExposureProgram::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::MeteringMode => crsdk::MeteringMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::FocusMode => crsdk::FocusMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::FocusArea => crsdk::FocusArea::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::WhiteBalance => crsdk::WhiteBalance::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::DriveMode => crsdk::DriveMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::FlashMode => crsdk::FlashMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::FileType => crsdk::FileType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::ImageQuality => crsdk::ImageQuality::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::AspectRatio => crsdk::AspectRatio::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::ImageSize => crsdk::ImageSize::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::MovieFileFormat => crsdk::MovieFileFormat::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::MovieQuality => crsdk::format_movie_quality(raw),
        PropertyValueType::ShutterModeStatus => crsdk::ShutterModeStatus::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::ShutterMode => crsdk::ShutterMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::SubjectRecognitionAF => crsdk::SubjectRecognitionAF::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::PrioritySetInAF => crsdk::PrioritySetInAF::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::FocusTrackingStatus => crsdk::FocusTrackingStatus::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::PrioritySetInAWB => crsdk::PrioritySetInAWB::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::IntervalRecShutterType => crsdk::IntervalRecShutterType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::ExposureCtrlType => crsdk::ExposureCtrlType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::LiveViewDisplayEffect => crsdk::LiveViewDisplayEffect::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::SilentModeApertureDrive => crsdk::SilentModeApertureDrive::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PropertyValueType::OnOff => crsdk::OnOff::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        PropertyValueType::Switch => crsdk::Switch::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        PropertyValueType::AutoManual => crsdk::AutoManual::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        PropertyValueType::LockIndicator => crsdk::LockIndicator::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        PropertyValueType::Percentage => format!("{}%", raw),
        PropertyValueType::Integer => format!("{}", raw),
        PropertyValueType::Unknown => format!("{}", raw),
    }
}
