use std::{env, fs};

use anyhow::{Context, Result};
use chrono::{Datelike, Local, Timelike};

use crate::options::SaveMethod;
use crate::subprocess::{notify, wl_copy};

/// Saves data to intended location
pub fn save(data: &[u8], method: SaveMethod, quiet: bool) -> Result<()> {
  match method {
    SaveMethod::Clipboard => save_to_clipboard(data, quiet),
    SaveMethod::Filesystem => save_to_filesystem(data, quiet),
    SaveMethod::Both => {
      save_to_clipboard(data, quiet)?;
      save_to_filesystem(data, quiet)?;
      Ok(())
    }
    SaveMethod::Nothing => {
      println!("Nothing saved");
      Ok(())
    }
  }
}

fn save_to_clipboard(data: &[u8], quiet: bool) -> Result<()> {
  let result = wl_copy::copy_png_to_clipboard(data);

  if !quiet {
    match result {
      Ok(_) => notify::saved_to_clipboard(data),
      Err(why) => notify::save_failed(why),
    }?;
  }

  Ok(())
}

fn save_to_filesystem(data: &[u8], quiet: bool) -> Result<()> {
  let screenshot_dir = env::var("FOOSHOT_DIR")
    .or_else(|_| env::var("XDG_PICTURES_DIR"))
    .map(|x| x.into())
    .unwrap_or(env::home_dir().unwrap_or("/".into()).join("Pictures"));
  if !screenshot_dir.exists() {
    fs::create_dir_all(&screenshot_dir).context("Failed to create screenshot directory")?;
  }

  let filename = generate_name();
  let target_location = screenshot_dir.join(filename);

  let result = fs::write(&target_location, data).context("Failed to write to file");

  if !quiet {
    match result {
      Ok(_) => notify::saved_to_filesystem(&target_location),
      Err(why) => notify::save_failed(why),
    }?;
  }

  Ok(())
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
