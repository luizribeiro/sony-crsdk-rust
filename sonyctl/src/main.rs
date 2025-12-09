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

mod commands;
mod tui;

use clap::Parser;

#[derive(Parser)]
#[command(name = "sonyctl")]
#[command(about = "CLI and TUI tool for controlling Sony cameras via the Camera Remote SDK")]
#[command(version)]
pub struct Cli {
    /// Camera IP address (required for CLI commands, optional for TUI)
    #[arg(long, env = "SONY_CAMERA_IP", global = true)]
    pub ip: Option<String>,

    /// Camera MAC address (required for CLI commands, optional for TUI)
    #[arg(long, env = "SONY_CAMERA_MAC", global = true)]
    pub mac: Option<String>,

    /// SSH username (enables SSH mode)
    #[arg(long, env = "SONY_SSH_USER", global = true)]
    pub user: Option<String>,

    /// SSH password
    #[arg(long, env = "SONY_SSH_PASSWORD", global = true)]
    pub password: Option<String>,

    /// Skip SSH fingerprint confirmation
    #[arg(long, env = "SONY_SSH_TRUST", global = true)]
    pub trust: bool,

    #[command(subcommand)]
    pub command: commands::Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    commands::run(&Cli::parse()).await
}
