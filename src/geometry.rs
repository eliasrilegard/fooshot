use std::fmt;

pub struct Geometry {
  pub x: i32,
  pub y: i32,
  pub w: u32,
  pub h: u32,
}

impl Geometry {
  /// Translates the coordinate system
  pub fn translate_by(&mut self, x: i32, y: i32) {
    self.x -= x;
    self.y -= y;
  }
}

impl fmt::Display for Geometry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{},{} {}x{}", self.x, self.y, self.w, self.h)
  }
}
