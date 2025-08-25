use clap::Parser;

use crate::options::{CaptureMode, Cli};

mod options;

fn main() {
  let args = Cli::parse();

  match args.mode {
    CaptureMode::Fullscreen => println!("Fullscreen capture"),
    CaptureMode::Region => println!("Region capture"),
    CaptureMode::Window => println!("Window capture"),
    CaptureMode::Monitor => println!("Monitor capture"),
  }
}
