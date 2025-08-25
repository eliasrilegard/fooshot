use std::process::Command;

use anyhow::{Context, Result};

use crate::geometry::Geometry;

pub fn fullscreen() -> Result<Vec<u8>> {
  let output = Command::new("grim")
    .arg("-") // stdout
    .output()
    .context("Failed to run grim")?;

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("grim failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(output.stdout)
}

pub fn region(geometry: Geometry) -> Result<Vec<u8>> {
  let output = Command::new("grim")
    .arg("-g")
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
