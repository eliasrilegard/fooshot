use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Error, Result};
use tempfile::NamedTempFile;

pub fn saved_to_clipboard(data: &[u8]) -> Result<()> {
  let mut tmp = NamedTempFile::new()?;
  tmp
    .write_all(data)
    .context("Could not write temporary file")?;

  let output = Command::new("notify-send")
    .args(["-a", "fooshot"]) // App name
    .arg("Screenshot saved") // Title
    .arg("Successfully saved screenshot to clipboard") // Description
    .arg("-i") // Icon
    .arg(tmp.path())
    .output()
    .context("Failed to run notify-send")?;

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("notify-send failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(())
}

pub fn saved_to_filesystem(location: &Path) -> Result<()> {
  let output = Command::new("notify-send")
    .args(["-a", "fooshot"])
    .arg("Screenshot saved")
    .arg(format!(
      "Successfully saved screenshot to {}",
      location.to_string_lossy()
    ))
    .arg("-i")
    .arg(location)
    .output()
    .context("Failed to run notify-send")?;

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("notify-send failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(())
}

pub fn save_failed(err: Error) -> Result<()> {
  let output = Command::new("notify-send")
    .args(["-a", "fooshot"])
    .arg("Save failed")
    .arg(format!("Failed to save screenshot: {}", err))
    .output()
    .context("Failed to run notify-send")?;

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("notify-send failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(())
}
