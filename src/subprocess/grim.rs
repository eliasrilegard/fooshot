use std::process::Command;

use anyhow::{Context, Result};

use crate::geometry::Geometry;

/// Capture a PNG image of the entire screen
pub fn fullscreen() -> Result<Vec<u8>> {
  let output = Command::new("grim")
    .arg("-") // Output to stdout
    .output()
    .context("Failed to run grim")?;

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("grim failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(output.stdout)
}

/// Capture a PNG image of a region with specific position and size
pub fn region(geometry: Geometry) -> Result<Vec<u8>> {
  let output = Command::new("grim")
    .arg("-g") // Use specific geometry
    .arg(geometry.to_string())
    .arg("-")
    .output()
    .context("Failed to run grim")?;

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("grim failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(output.stdout)
}
