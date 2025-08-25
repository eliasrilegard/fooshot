use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
  /// What to capture
  #[arg(short, long, default_value = "fullscreen")]
  pub mode: CaptureMode,

  /// Automatically select active/focused target
  #[arg(short, long)]
  pub active: bool,

  /// Where to save the image
  #[arg(short, long = "save", value_name = "WHERE", default_value = "clipboard")]
  pub save_to: SaveMethod,

  /// Freeze the screen during selection
  #[arg(short, long)]
  pub freeze: bool,

  /// Suppress notifications
  #[arg(short, long)]
  pub quiet: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum CaptureMode {
  Fullscreen,
  Region,
  Window,
  Monitor,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SaveMethod {
  Clipboard,
  Filesystem,
  Both,
  Nothing,
}
