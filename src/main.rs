use anyhow::Result;
use clap::Parser;

use crate::options::{CaptureMode, Cli};

mod capture;
mod geometry;
mod image;
mod options;
mod subprocess;

fn main() -> Result<()> {
  let args = Cli::parse();

  match args.mode {
    CaptureMode::Fullscreen => capture::capture_fullscreen(&args),
    CaptureMode::Region => capture::capture_region(&args),
    CaptureMode::Window => capture::capture_window(&args),
    CaptureMode::Monitor => capture::capture_monitor(&args),
  }
}
