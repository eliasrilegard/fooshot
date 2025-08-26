use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::geometry::Geometry;

/// Use mouse to select a region on the screen
pub fn select_region() -> Result<Option<Geometry>> {
  let output = Command::new("slurp")
    .arg("-d") // Display dimensions of selection
    .args(["-f", "%x %y %w %h"]) // Output format
    .args(["-w", "0"]) // Zero border width
    .stdout(Stdio::piped())
    .output()
    .context("Failed to run slurp")?;

  if !output.status.success() {
    // Selection cancelled
    return Ok(None);
  }

  let data = str::from_utf8(&output.stdout).context("Unexpected output: Not UTF-8")?;

  let numbers = data
    .split_whitespace()
    .map(|x| {
      x.parse::<i64>()
        .with_context(|| format!("Failed to parse output from slurp: {}", x))
    })
    .collect::<Result<Vec<_>>>()?;

  Ok(Some(Geometry {
    x: numbers[0] as i32,
    y: numbers[1] as i32,
    w: numbers[2] as u32,
    h: numbers[3] as u32,
  }))
}
