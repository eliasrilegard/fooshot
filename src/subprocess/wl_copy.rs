use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

/// Copy data of a PNG into the clipboard
pub fn copy_png_to_clipboard(data: &[u8]) -> Result<()> {
  let mut wlcopy = Command::new("wl-copy")
    .args(["--type", "image/png"])
    .stdin(Stdio::piped())
    .spawn()
    .context("Failed to spawn wl-copy")?;

  wlcopy
    .stdin
    .take()
    .context("Failed to open stdin for wl-copy")?
    .write_all(data)
    .context("Failed to write data to wl-copy")?;

  let output = wlcopy
    .wait_with_output()
    .context("Failed to wait for wl-copy")?;
  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("wl-copy failed with exit code {}: {}", output.status, why.trim());
  }

  Ok(())
}
