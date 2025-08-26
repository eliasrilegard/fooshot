use std::io::Cursor;

use ::image as image_ext;
use anyhow::Result;

use crate::geometry::Geometry;
use crate::image;
use crate::options::{Cli, SaveMethod};
use crate::subprocess::{grim, hyprland, hyprpicker, slurp};

/// Capture an image of all screens, arrangement included
pub fn capture_fullscreen(args: &Cli) -> Result<()> {
  let bytes = grim::fullscreen()?;
  image::save(&bytes, args.save_to, args.quiet)
}

/// Capture an image of a region specified with the mouse
pub fn capture_region(args: &Cli) -> Result<()> {
  capture_selection(slurp::select_region, args.freeze, args.save_to, args.quiet)
}

/// Capture an image of a specific window
pub fn capture_window(args: &Cli) -> Result<()> {
  if args.active {
    let geometry = hyprland::get_active_window()?;
    let bytes = grim::region(geometry)?;
    image::save(&bytes, args.save_to, args.quiet)
  } else {
    capture_selection(slurp::select_window, args.freeze, args.save_to, args.quiet)
  }
}

/// Capture an image of a single (whole) monitor
pub fn capture_monitor(args: &Cli) -> Result<()> {
  if args.active {
    let geometry = hyprland::get_active_monitor()?;
    let bytes = grim::region(geometry)?;
    image::save(&bytes, args.save_to, args.quiet)
  } else {
    capture_selection(slurp::select_monitor, args.freeze, args.save_to, args.quiet)
  }
}

fn capture_selection<F>(select_area: F, freeze_select: bool, save_method: SaveMethod, quiet: bool) -> Result<()>
where
  F: Fn() -> Result<Option<Geometry>>,
{
  let bytes = if freeze_select {
    let data = grim::fullscreen()?;

    let mut geometry = {
      let _f = hyprpicker::freeze_screen()?;

      let Some(geometry) = select_area()? else {
        return Ok(());
      };

      geometry
    };

    // Translate from monitor space to image space
    let nw_corner = hyprland::get_global_nw_coordinates()?;
    geometry.translate_by(nw_corner.x, nw_corner.y);

    // Decode + crop + re-encode
    let img =
      image_ext::load_from_memory(&data)?.crop_imm(geometry.x as u32, geometry.y as u32, geometry.w, geometry.h);

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image_ext::ImageFormat::Png)?;
    buf
  } else {
    let Some(geometry) = select_area()? else {
      return Ok(());
    };

    grim::region(geometry)?
  };

  image::save(&bytes, save_method, quiet)
}
