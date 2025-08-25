use crate::options::Cli;

pub fn fullscreen(_args: &Cli) {
  println!("Fullscreen capture");
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
