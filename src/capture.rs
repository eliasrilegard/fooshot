use crate::image;
use crate::options::Cli;
use crate::subprocess::{grim, slurp};

pub fn fullscreen(args: &Cli) {
  let data = grim::fullscreen();
  image::save(&data, args.save_to);
}

pub fn region(args: &Cli) {
  let Some(geometry) = slurp::select_region() else {
    return;
  };

  let data = grim::region(geometry);
  image::save(&data, args.save_to);
}

pub fn window(_args: &Cli) {
  println!("Window capture");
}

pub fn monitor(_args: &Cli) {
  println!("Monitor capture");
}
