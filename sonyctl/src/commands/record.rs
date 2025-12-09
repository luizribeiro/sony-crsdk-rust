use clap::Subcommand;
use crsdk::Result;

#[derive(Subcommand)]
pub enum Args {
    /// Start video recording
    Start,
    /// Stop video recording
    Stop,
}

pub fn run(device: &crsdk::blocking::CameraDevice, args: &Args) -> Result<()> {
    match args {
        Args::Start => {
            device.start_recording()?;
            println!("Recording started");
        }
        Args::Stop => {
            device.stop_recording()?;
            println!("Recording stopped");
        }
    }
    Ok(())
}
