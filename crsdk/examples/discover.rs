//! Camera discovery example
//!
//! Scans for Sony cameras connected via network or USB.
//!
//! Usage:
//!   cargo run --example discover
//!   cargo run --example discover -- --timeout 5

use clap::Parser;
use crsdk::{discover_cameras, Result};

#[derive(Parser)]
#[command(name = "discover")]
#[command(about = "Discover Sony cameras on the network and USB")]
struct Args {
    /// Scan timeout in seconds
    #[arg(long, default_value = "3")]
    timeout: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    println!("Sony Camera Remote SDK - Discovery");
    println!("===================================\n");

    println!("Scanning for cameras ({} seconds)...\n", args.timeout);

    let cameras = discover_cameras(args.timeout).await?;

    if cameras.is_empty() {
        println!("No cameras found.");
        println!("\nTips:");
        println!("  - Ensure the camera is powered on");
        println!("  - Check network connection (for IP cameras)");
        println!("  - Check USB connection (for USB cameras)");
        println!("  - Enable remote control in camera settings");
        return Ok(());
    }

    println!("Found {} camera(s):\n", cameras.len());

    for (i, camera) in cameras.iter().enumerate() {
        println!("{}. {}", i + 1, camera);
        println!("   Model: {}", camera.model);
        println!("   Name: {}", camera.name);
        println!("   Type: {}", camera.connection_type);

        if let Some(ip) = &camera.ip_address {
            println!("   IP: {}", ip);
        }
        if let Some(mac) = &camera.mac_address {
            println!("   MAC: {}", mac);
        }
        if camera.ssh_supported {
            println!("   SSH: supported");
        }
        if let Some(pid) = camera.usb_pid {
            println!("   USB PID: {:04x}", pid);
        }
        println!();
    }

    Ok(())
}
