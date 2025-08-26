use std::io::Cursor;

use anyhow::Result;
use ::image as image_ext;

use crate::image;
use crate::options::Cli;
use crate::subprocess::{grim, hyprland, hyprpicker, slurp};

pub fn fullscreen(args: &Cli) -> Result<()> {
  let bytes = grim::fullscreen()?;
  image::save(&bytes, args.save_to, args.quiet)
}

pub fn region(args: &Cli) -> Result<()> {
  let bytes = if args.freeze {
    let data = grim::fullscreen()?;

    let mut geometry = {
      let _f = hyprpicker::freeze_screen()?;

      let Some(geometry) = slurp::select_region()? else {
        return Ok(());
      };

      geometry
    };

    // Translate from monitor space to image space
    let nw_corner = hyprland::get_global_nw_coordinates()?;
    geometry.translate_by(nw_corner.x, nw_corner.y);

    // Decode + crop + re-encode
    let img = image_ext::load_from_memory(&data)?
      .crop_imm(geometry.x as u32, geometry.y as u32, geometry.w, geometry.h);

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image_ext::ImageFormat::Png)?;
    buf
  } else {
    let Some(geometry) = slurp::select_region()? else {
      return Ok(());
    };

    grim::region(geometry)?
  };

  image::save(&bytes, args.save_to, args.quiet)
}

pub fn window(_args: &Cli) -> Result<()> {
  todo!()
}

pub fn monitor(_args: &Cli) -> Result<()> {
  todo!()
}
