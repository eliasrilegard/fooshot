use anyhow::Result;
use clap::Parser;

use crate::options::{CaptureMode, Cli};

mod capture;
mod check;
mod geometry;
mod image;
mod options;
mod subprocess;

fn main() -> Result<()> {
  let args = Cli::parse();

  if args.check {
    check::verify_dependencies();
    return Ok(());
  }

  match args.mode {
    CaptureMode::Fullscreen => capture::capture_fullscreen(&args),
    CaptureMode::Region => capture::capture_region(&args),
    CaptureMode::Window => capture::capture_window(&args),
    CaptureMode::Monitor => capture::capture_monitor(&args),
  }
}
