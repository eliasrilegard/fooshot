use clap::Parser;

use crate::options::{CaptureMode, Cli};

mod capture;
mod image;
mod options;
mod subprocess;

fn main() {
  let args = Cli::parse();

  match args.mode {
    CaptureMode::Fullscreen => capture::fullscreen(&args),
    CaptureMode::Region => capture::region(&args),
    CaptureMode::Window => capture::window(&args),
    CaptureMode::Monitor => capture::monitor(&args),
  }
}
