use crate::options::Cli;
use crate::image;
use crate::subprocess::grim;

pub fn fullscreen(args: &Cli) {
  let data = grim::fullscreen();
  image::save(&data, args.save_to);
}

pub fn region(_args: &Cli) {
  println!("Region capture");
}

pub fn window(_args: &Cli) {
  println!("Window capture");
}

pub fn monitor(_args: &Cli) {
  println!("Monitor capture");
}
