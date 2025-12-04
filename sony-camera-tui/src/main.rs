use std::io;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod action;
mod app;
mod camera_service;
mod event;
mod property;
mod ui;

use app::App;
use camera_service::{CameraCommand, CameraService};
use crsdk::MacAddr;
use event::EventHandler;

#[derive(Parser)]
#[command(name = "sony-camera-tui")]
#[command(about = "Terminal UI for Sony Camera Remote SDK")]
struct Args {
    /// Camera IP address (skip discovery if provided)
    #[arg(long)]
    ip: Option<String>,

    /// Camera MAC address (required with --ip)
    #[arg(long)]
    mac: Option<String>,

    /// Enable SSH tunnel
    #[arg(long)]
    ssh: bool,

    /// SSH username
    #[arg(long)]
    user: Option<String>,

    /// SSH password
    #[arg(long)]
    password: Option<String>,

    /// Log file path (default: sony-camera-tui.log in current directory)
    #[arg(long, default_value = "sony-camera-tui.log")]
    log_file: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "debug")]
    log_level: String,
}

fn setup_logging(args: &Args) -> Result<WorkerGuard> {
    let file_appender = tracing_appender::rolling::never(".", &args.log_file);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_new(&args.log_level).unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    tracing::info!("Sony Camera TUI starting");
    tracing::info!("Log level: {}", args.log_level);

    Ok(guard)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let _guard = setup_logging(&args)?;

    let terminal = setup_terminal()?;
    let result = run(terminal, args).await;
    restore_terminal()?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

async fn run(mut terminal: Terminal<CrosstermBackend<io::Stdout>>, args: Args) -> Result<()> {
    let camera_handle = CameraService::spawn();

    let mut app = App::new(camera_handle);
    let mut events = EventHandler::new();

    // If CLI args provided, skip discovery and connect directly
    if let (Some(ip_str), Some(mac_str)) = (&args.ip, &args.mac) {
        tracing::info!("Connecting via CLI args: ip={}, mac={}", ip_str, mac_str);

        let ip: std::net::Ipv4Addr = ip_str.parse().expect("Invalid IP address");
        let mac: MacAddr = mac_str.parse().expect("Invalid MAC address");

        if args.ssh {
            let user = args.user.expect("--user required with --ssh");
            let password = args.password.expect("--password required with --ssh");
            tracing::info!("SSH enabled, fetching fingerprint...");

            let _ = app
                .camera_service_cmd()
                .send(CameraCommand::FetchSshFingerprint {
                    ip,
                    mac,
                    ssh_user: user,
                    ssh_pass: password,
                })
                .await;
        } else {
            let _ = app
                .camera_service_cmd()
                .send(CameraCommand::Connect { ip, mac })
                .await;
        }
    }

    loop {
        app.poll_camera_updates();

        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Some(action) = events.next(&app).await {
            app.update(action).await;
        }

        if app.should_quit {
            break;
        }
    }

    tracing::info!("Sony Camera TUI exiting");
    Ok(())
}
