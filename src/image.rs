use std::{env, fs};

use chrono::{Datelike, Local, Timelike};

use crate::options::SaveMethod;

pub fn save(data: &[u8], method: SaveMethod) {
  match method {
    SaveMethod::Clipboard => save_to_clipboard(data),
    SaveMethod::Filesystem => save_to_filesystem(data),
    SaveMethod::Both => {
      save_to_clipboard(data);
      save_to_filesystem(data);
    }
  }
}

fn save_to_clipboard(_data: &[u8]) {
  todo!()
}

fn save_to_filesystem(data: &[u8]) {
  let screenshot_dir = env::home_dir().unwrap_or("/".into()).join("Pictures");
  if !screenshot_dir.exists() {
    fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");
  }

  let filename = generate_name();
  let target_location = screenshot_dir.join(filename);

  fs::write(target_location, data).expect("Failed to write to file");
}

fn generate_name() -> String {
  let now = Local::now();
  format!(
    "{year:04}-{month:02}-{day:02}_{hour:02}{minute:02}{second:02}_fooshot.png",
    year = now.year(),
    month = now.month(),
    day = now.day(),
    hour = now.hour(),
    minute = now.minute(),
    second = now.second(),
  )
}
