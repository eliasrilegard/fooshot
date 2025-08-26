use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::geometry::Geometry;
use crate::subprocess::hyprland;

// Flags (taken from slurp -h)
// -d Display dimensions of selection
// -w Set border weight
// -f Set output format
// -o Select a display output
// -r Restrict selection to predefined boxes

/// Use mouse to select a region on the screen
pub fn select_region() -> Result<Option<Geometry>> {
  run_slurp(&["-d", "-f", "%x %y %w %h", "-w", "0"], None)
}

/// Use mouse to select a visible window
pub fn select_window() -> Result<Option<Geometry>> {
  let active_windows: Vec<_> = hyprland::get_visible_windows()?
    .iter()
    .map(|geometry| geometry.to_string())
    .collect();

  run_slurp(
    &["-r", "-f", "%x %y %w %h", "-w", "0"],
    Some(active_windows.join("\n").as_bytes()),
  )
}

/// Use mouse to select a monitor
pub fn select_monitor() -> Result<Option<Geometry>> {
  run_slurp(&["-or", "-f", "%x %y %w %h", "-w", "0"], None)
}

fn run_slurp(flags: &[&str], to_stdin: Option<&[u8]>) -> Result<Option<Geometry>> {
  let mut cmd = Command::new("slurp");
  cmd.args(flags).stdout(Stdio::piped());

  if to_stdin.is_some() {
    cmd.stdin(Stdio::piped());
  }

  let mut slurp = cmd.spawn().context("Failed to spawn slurp")?;

  if let Some(data) = to_stdin {
    slurp
      .stdin
      .take()
      .context("Failed to open stdin for slurp")?
      .write_all(data)
      .context("Failed to write data to slurp")?;
  }

  let output = slurp.wait_with_output().context("Failed to run slurp")?;

  if !output.status.success() {
    // Selection cancelled
    return Ok(None);
  }

  let numbers = String::from_utf8_lossy(&output.stdout)
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
