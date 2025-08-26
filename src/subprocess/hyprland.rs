use anyhow::{Context, Result};
use hyprland::{data::Monitors, shared::HyprData};

use crate::geometry::Geometry;

/// Get coordinates of the top left corner of the monitor arrangement
pub fn get_global_nw_coordinates() -> Result<Geometry> {
  let monitors = Monitors::get().context("Failed getting hyprland's monitors")?;
  let min_x = monitors.iter().map(|m| m.x).min().context("Monitor coordinates not found")?;
  let min_y = monitors.iter().map(|m| m.y).min().context("Monitor coordinates not found")?;

  Ok(Geometry {
    x: min_x,
    y: min_y,
    w: 0,
    h: 0,
  })
}
