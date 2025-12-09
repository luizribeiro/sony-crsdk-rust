use crsdk::Result;

pub fn run(device: &crsdk::blocking::CameraDevice) -> Result<()> {
    println!("Capturing...");
    device.capture()?;
    println!("✓ Capture complete");
    Ok(())
}
