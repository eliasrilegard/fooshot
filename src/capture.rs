use anyhow::Result;

use crate::image;
use crate::options::Cli;
use crate::subprocess::{grim, slurp};

pub fn fullscreen(args: &Cli) -> Result<()> {
  let data = grim::fullscreen()?;
  image::save(&data, args.save_to)
}

pub fn region(args: &Cli) -> Result<()> {
  let Some(geometry) = slurp::select_region()? else {
    return Ok(());
  };

  let data = grim::region(geometry)?;
  image::save(&data, args.save_to)
}

pub fn window(_args: &Cli) -> Result<()> {
  todo!()
}

pub fn monitor(_args: &Cli) -> Result<()> {
  todo!()
}
