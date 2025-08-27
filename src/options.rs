use clap::{Parser, ValueEnum};

/// A screenshot tool built for Hyprland.
///
/// Fooshot allows taking screenshots of regions, specific windows or entire
/// monitors, which are saved to the clipboard and/or a folder of your choosing.
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

  /// Check availability of all dependencies, then exit
  #[arg(long)]
  pub check: bool,
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
}
