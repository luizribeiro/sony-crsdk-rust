//! Basic camera connection example
//!
//! Usage:
//!   cargo run --example connect -- --ip 10.1.3.105 --mac 10:32:2c:7d:c7:b3 --ssh --user kbJB7B --password 4Sa8rc9B

use clap::Parser;
use crsdk::{CameraDevice, CameraModel, Result};
use dialoguer::Confirm;

#[derive(Parser)]
#[command(name = "connect")]
#[command(about = "Connect to a Sony camera via IP address")]
struct Args {
    /// Camera IP address
    #[arg(long)]
    ip: String,

    /// Camera MAC address (find with: arp -n <camera-ip>)
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

    println!("Sony Camera Remote SDK - Rust");
    println!("==============================\n");

    println!("Connecting to camera...");
    println!("  IP:  {}", args.ip);
    println!("  MAC: {}", args.mac);

    let mut builder = CameraDevice::builder()
        .ip_address(args.ip.parse().expect("Invalid IP address"))
        .mac_address(args.mac.parse().expect("Invalid MAC address"))
        .model(CameraModel::Fx3);

    if args.ssh {
        let user = args.user.expect("--user required with --ssh");
        let password = args.password.expect("--password required with --ssh");
        println!("  SSH: enabled (user: {})", user);

        builder = builder.ssh_enabled(true);

        println!("\nFetching SSH fingerprint from camera...");
        let fingerprint = builder.fetch_ssh_fingerprint().await?;

        println!("\nSSH Fingerprint:");
        println!("{}", fingerprint);

        let confirmed = Confirm::new()
            .with_prompt("Do you trust this fingerprint?")
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirmed {
            println!("Connection cancelled.");
            std::process::exit(1);
        }

        builder = builder
            .ssh_credentials(&user, &password)
            .ssh_fingerprint(fingerprint);
    }

    let camera = builder.connect().await?;

    println!("\nConnected!");
    println!("  Model: {}", camera.model().await);

    println!("\nCamera is ready. Press Ctrl+C to disconnect.\n");

    tokio::signal::ctrl_c().await.ok();
    println!("\nDisconnecting...");

    Ok(())
}
