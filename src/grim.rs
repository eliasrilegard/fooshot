use std::process::Command;

pub fn fullscreen() -> Vec<u8> {
  let output = Command::new("grim")
    .arg("-") // stdout
    .output()
    .expect("Failed to run grim");

  if !output.status.success() {
    let why = String::from_utf8_lossy(&output.stderr);
    panic!("grim failed with exit code {}: {}", output.status, why.trim());
  }

  return output.stdout;
}
