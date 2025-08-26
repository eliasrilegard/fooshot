use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};

pub struct FreezeHandle {
  child: Child,
}

impl Drop for FreezeHandle {
  fn drop(&mut self) {
    let _ = self.child.kill();
    let _ = self.child.wait();
  }
}

/// Freeze the screen
pub fn freeze_screen() -> Result<FreezeHandle> {
  let child = Command::new("hyprpicker")
    .arg("--render-inactive") // Also freeze inactive displays
    .arg("--no-zoom") // Disable zoom lens
    .spawn()
    .context("Failed to spawn hyprpicker")?;

  thread::sleep(Duration::from_millis(100));

  Ok(FreezeHandle { child })
}
