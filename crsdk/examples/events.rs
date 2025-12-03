//! Event monitoring example
//!
//! Connects to a camera and displays all events received from the SDK.
//!
//! Usage:
//!   cargo run --example events -- --ip 10.1.3.105 --mac 10:32:2c:7d:c7:b3 --ssh --user kbJB7B --password 4Sa8rc9B

use clap::Parser;
use crsdk::{CameraDevice, CameraEvent, CameraModel, Result};
use dialoguer::Confirm;
use tokio::select;

#[derive(Parser)]
#[command(name = "events")]
#[command(about = "Monitor camera events")]
struct Args {
    /// Camera IP address
    #[arg(long)]
    ip: String,

    /// Camera MAC address
    #[arg(long)]
    mac: String,

    /// Enable SSH tunnel
    #[arg(long)]
    ssh: bool,

    /// SSH username
    #[arg(long)]
    user: Option<String>,

    /// SSH password
    #[arg(long)]
    password: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    println!("Sony Camera Remote SDK - Event Monitor");
    println!("=======================================\n");

    let mut builder = CameraDevice::builder()
        .ip_address(args.ip.parse().expect("Invalid IP address"))
        .mac_address(args.mac.parse().expect("Invalid MAC address"))
        .model(CameraModel::Fx3);

    if args.ssh {
        let user = args.user.expect("--user required with --ssh");
        let password = args.password.expect("--password required with --ssh");

        builder = builder.ssh_enabled(true);

        println!("Fetching SSH fingerprint...");
        let fingerprint = builder.fetch_ssh_fingerprint().await?;
        println!("Fingerprint: {}", fingerprint);

        let confirmed = Confirm::new()
            .with_prompt("Trust this fingerprint?")
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirmed {
            println!("Cancelled.");
            std::process::exit(1);
        }

        builder = builder
            .ssh_credentials(&user, &password)
            .ssh_fingerprint(fingerprint);
    }

    println!("\nConnecting...");
    let mut camera = builder.connect().await?;
    println!("Connected to {}\n", camera.model().await);

    println!("Listening for events (Ctrl+C to stop)...\n");

    // Take the event receiver for manual control
    let mut receiver = camera
        .take_event_receiver()
        .expect("Event receiver already taken");

    loop {
        select! {
            event = receiver.recv() => {
                match event {
                    Some(event) => print_event(&event),
                    None => {
                        println!("Event channel closed");
                        break;
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("\nShutting down...");
                break;
            }
        }
    }

    Ok(())
}

fn print_event(event: &CameraEvent) {
    match event {
        CameraEvent::Connected { version } => {
            println!("[CONNECTED] Protocol version: {}", version);
        }
        CameraEvent::Disconnected { error } => {
            if *error == 0 {
                println!("[DISCONNECTED] Normal disconnect");
            } else {
                println!("[DISCONNECTED] Error: 0x{:08X}", error);
            }
        }
        CameraEvent::PropertyChanged { codes } => {
            println!("[PROPERTY] {} properties changed:", codes.len());
            for code in codes {
                println!("  - {:?}", code);
            }
        }
        CameraEvent::LiveViewPropertyChanged { codes } => {
            println!("[LV PROPERTY] {} live view properties changed", codes.len());
        }
        CameraEvent::DownloadComplete { filename } => {
            println!("[DOWNLOAD] Complete: {}", filename);
        }
        CameraEvent::ContentsTransfer {
            notify,
            handle,
            filename,
        } => {
            println!(
                "[TRANSFER] notify={}, handle={}, file={:?}",
                notify, handle, filename
            );
        }
        CameraEvent::Warning { code, params } => {
            print!("[WARNING] 0x{:08X}", code);
            if let Some((p1, p2, p3)) = params {
                print!(" (params: {}, {}, {})", p1, p2, p3);
            }
            println!();
        }
        CameraEvent::Error { code } => {
            println!("[ERROR] 0x{:08X}", code);
        }
        CameraEvent::RemoteTransferProgress {
            percent, filename, ..
        } => {
            println!("[PROGRESS] {}% - {:?}", percent, filename);
        }
        CameraEvent::RemoteTransferData { percent, data, .. } => {
            println!("[DATA] {}% - {} bytes received", percent, data.len());
        }
        CameraEvent::ContentsListChanged { slot, added, .. } => {
            println!("[CONTENTS] Slot {}: {} items added", slot, added);
        }
        CameraEvent::FirmwareUpdateProgress { notify } => {
            println!("[FIRMWARE] Progress notification: {}", notify);
        }
        _ => {
            println!("[EVENT] {:?}", event);
        }
    }
}
