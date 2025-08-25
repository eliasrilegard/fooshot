use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy_png_to_clipboard(data: &[u8]) {
  let mut wlcopy = Command::new("wl-copy")
    .args(["--type", "image/png"])
    .stdin(Stdio::piped())
    .spawn()
    .expect("Failed to spawn wl-copy");

  if let Some(mut input) = wlcopy.stdin.take() {
    input
      .write_all(data)
      .expect("Failed to write data to wl-copy");
  } else {
    panic!("Failed to open stdin for wl-copy");
  }

  let status = wlcopy.wait().expect("Failed to wait for wl-copy");
  if !status.success() {
    panic!("wl-copy exited with an error");
  }
}
