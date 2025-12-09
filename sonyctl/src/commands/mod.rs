pub mod capture;
pub mod info;
pub mod props;
pub mod record;

use clap::Subcommand;
use crsdk::{CameraDevice, CameraModel, DeviceProperty, Result, TypedValue};
use crsdk_sys::DevicePropertyCode;
use dialoguer::Confirm;

use crate::{tui, Cli};

#[derive(Subcommand)]
pub enum Command {
    /// Launch interactive TUI
    Tui(tui::Args),
    /// Property operations
    Props {
        #[command(subcommand)]
        action: props::Args,
    },
    /// Capture a photo
    Capture,
    /// Video recording control
    Record {
        #[command(subcommand)]
        action: record::Args,
    },
    /// Show camera info
    Info,
}

pub async fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Command::Tui(args) => {
            tui::run(cli, args).await?;
        }
        _ => {
            let device = connect(cli).await?;

            match &cli.command {
                Command::Tui(_) => unreachable!(),
                Command::Props { action } => {
                    props::run(&device, action)?;
                }
                Command::Capture => {
                    capture::run(&device)?;
                }
                Command::Record { action } => {
                    record::run(&device, action)?;
                }
                Command::Info => {
                    info::run(&device)?;
                }
            }
        }
    }
    Ok(())
}

pub async fn connect(cli: &Cli) -> Result<crsdk::blocking::CameraDevice> {
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

    eprintln!("Loading properties...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    eprintln!();

    Ok(camera.into_inner())
}

pub fn find_property_code(properties: &[DeviceProperty], name: &str) -> Option<DevicePropertyCode> {
    use crsdk::property_display_name;

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

pub fn format_value(code: DevicePropertyCode, raw: u64) -> String {
    TypedValue::from_raw(code, raw).to_string()
}
