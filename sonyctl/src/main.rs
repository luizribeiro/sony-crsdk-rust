//! sonyctl - CLI and TUI tool for controlling Sony cameras
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
//! # Launch interactive TUI (with discovery)
//! sonyctl tui
//!
//! # Launch TUI with direct connection
//! sonyctl --ip 192.168.1.100 --mac 00:00:00:00:00:00 tui
//!
//! # List all properties
//! sonyctl --ip 192.168.1.100 --mac 00:00:00:00:00:00 props list
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

mod tui;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use crsdk::{
    property_category, property_description, property_display_name, property_value_type,
    CameraDevice, CameraModel, DeviceProperty, EnableFlag, Result, TypedValue, ValueConstraint,
};
use crsdk_sys::DevicePropertyCode;
use dialoguer::Confirm;

#[derive(Parser)]
#[command(name = "sonyctl")]
#[command(about = "CLI and TUI tool for controlling Sony cameras via the Camera Remote SDK")]
#[command(version)]
pub struct Cli {
    /// Camera IP address (required for CLI commands, optional for TUI)
    #[arg(long, env = "SONY_CAMERA_IP", global = true)]
    ip: Option<String>,

    /// Camera MAC address (required for CLI commands, optional for TUI)
    #[arg(long, env = "SONY_CAMERA_MAC", global = true)]
    mac: Option<String>,

    /// SSH username (enables SSH mode)
    #[arg(long, env = "SONY_SSH_USER", global = true)]
    user: Option<String>,

    /// SSH password
    #[arg(long, env = "SONY_SSH_PASSWORD", global = true)]
    password: Option<String>,

    /// Skip SSH fingerprint confirmation
    #[arg(long, env = "SONY_SSH_TRUST", global = true)]
    trust: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Launch interactive TUI
    Tui(TuiArgs),
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

#[derive(Args)]
pub struct TuiArgs {
    /// Log file path
    #[arg(long, default_value = "sonyctl.log")]
    pub log_file: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "debug")]
    pub log_level: String,
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
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Tui(args) => {
            tui::run(&cli, args).await?;
        }
        _ => {
            let device = connect(&cli).await?;

            match &cli.command {
                Command::Tui(_) => unreachable!(),
                Command::Props { action } => match action {
                    PropsAction::List {
                        filter,
                        writable,
                        raw,
                    } => {
                        list_properties(&device, filter.as_deref(), *writable, *raw)?;
                    }
                    PropsAction::Get { name } => {
                        get_property(&device, name)?;
                    }
                    PropsAction::Set { name, value } => {
                        set_property(&device, name, *value)?;
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
        }
    }

    Ok(())
}

async fn connect(cli: &Cli) -> Result<crsdk::blocking::CameraDevice> {
    let ip = cli
        .ip
        .as_ref()
        .ok_or_else(|| crsdk::Error::InvalidParameter("--ip is required".into()))?;
    let mac = cli
        .mac
        .as_ref()
        .ok_or_else(|| crsdk::Error::InvalidParameter("--mac is required".into()))?;

    eprintln!("Connecting to {}...", ip);

    let mut builder = CameraDevice::builder()
        .ip_address(ip.parse().expect("Invalid IP address"))
        .mac_address(mac.parse().expect("Invalid MAC address"))
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
            // Unknown codes sort after known codes
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            // Both unknown: sort by raw code
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
    TypedValue::from_raw(code, raw).to_string()
}
