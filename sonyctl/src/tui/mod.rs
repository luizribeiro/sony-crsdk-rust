use std::io;
use std::path::PathBuf;

use anyhow::Result;
use clap::Args as ClapArgs;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub mod action;
pub mod app;
pub mod camera_service;
pub mod event;
pub mod property;
pub mod ui;

use app::App;
use camera_service::{CameraCommand, CameraService};
use crsdk::MacAddr;
use event::EventHandler;

use crate::Cli;

#[derive(ClapArgs)]
pub struct Args {
    /// Log file path
    #[arg(long, default_value = "sonyctl.log")]
    pub log_file: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "debug")]
    pub log_level: String,
}

fn setup_logging(args: &Args) -> Result<WorkerGuard> {
    let file_appender = tracing_appender::rolling::never(".", &args.log_file);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_new(&args.log_level).unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    tracing::info!("sonyctl TUI starting");
    tracing::info!("Log level: {}", args.log_level);

    Ok(guard)
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

pub async fn run(cli: &Cli, args: &Args) -> Result<()> {
    let _guard = setup_logging(args)?;

    let terminal = setup_terminal()?;
    let result = run_app(terminal, cli).await;
    restore_terminal()?;
    result
}

async fn run_app(mut terminal: Terminal<CrosstermBackend<io::Stdout>>, cli: &Cli) -> Result<()> {
    let camera_handle = CameraService::spawn();

    let mut app = App::new(camera_handle, cli.trust);
    let mut events = EventHandler::new();

    // If CLI args provided, skip discovery and connect directly
    if let (Some(ip_str), Some(mac_str)) = (&cli.ip, &cli.mac) {
        tracing::info!("Connecting via CLI args: ip={}, mac={}", ip_str, mac_str);

        let ip: std::net::Ipv4Addr = ip_str.parse().expect("Invalid IP address");
        let mac: MacAddr = mac_str.parse().expect("Invalid MAC address");

        // Skip discovery screen - go straight to dashboard while connecting
        app.set_connecting();

        if cli.user.is_some() && cli.password.is_some() {
            let user = cli.user.clone().unwrap();
            let password = cli.password.clone().unwrap();
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

    tracing::info!("sonyctl TUI exiting");
    Ok(())
}
