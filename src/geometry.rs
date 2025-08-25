use std::fmt;

pub struct Geometry {
  pub x: i32,
  pub y: i32,
  pub w: u32,
  pub h: u32,
}

impl fmt::Display for Geometry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{},{} {}x{}", self.x, self.y, self.w, self.h)
  }
}
