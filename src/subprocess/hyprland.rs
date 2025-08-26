use anyhow::{Context, Result, anyhow};
use hyprland::data::{Client, Clients, Monitor, Monitors, Workspaces};
use hyprland::shared::{HyprData, HyprDataActive, HyprDataActiveOptional};

use crate::geometry::Geometry;

/// Get coordinates of the top left corner of the monitor arrangement
pub fn get_global_nw_coordinates() -> Result<Geometry> {
  let monitors = Monitors::get().context("Failed getting hyprland's monitors")?;
  let min_x = monitors
    .iter()
    .map(|m| m.x)
    .min()
    .context("Monitor coordinates not found")?;
  let min_y = monitors
    .iter()
    .map(|m| m.y)
    .min()
    .context("Monitor coordinates not found")?;

  Ok(Geometry {
    x: min_x,
    y: min_y,
    w: 0,
    h: 0,
  })
}

/// Get geometries of all visible windows
pub fn get_visible_windows() -> Result<Vec<Geometry>> {
  let active_workspace_ids: Vec<_> = Workspaces::get()
    .context("Failed getting hyprland's workspaces")?
    .iter()
    .map(|workspace| workspace.id)
    .collect();

  let visible_clients: Vec<_> = Clients::get()
    .context("Failed getting hyprland's clients")?
    .into_iter()
    .filter(|client| active_workspace_ids.contains(&client.workspace.id))
    .collect();

  let geometries: Vec<_> = visible_clients
    .iter()
    .map(|client| Geometry {
      x: client.at.0 as i32,
      y: client.at.1 as i32,
      w: client.size.0 as u32,
      h: client.size.1 as u32,
    })
    .collect();

  Ok(geometries)
}

/// Get geometry of active (focused) window
pub fn get_active_window() -> Result<Geometry> {
  let window = Client::get_active()
    .context("Failed getting hyprland's active client")?
    .ok_or_else(|| anyhow!("No active window found"))?;

  Ok(Geometry {
    x: window.at.0 as i32,
    y: window.at.1 as i32,
    w: window.size.0 as u32,
    h: window.size.1 as u32,
  })
}

/// Get geometry of the active (focused) monitor
pub fn get_active_monitor() -> Result<Geometry> {
  let monitor = Monitor::get_active().context("Failed getting hyprland's active monitor")?;

  Ok(Geometry {
    x: monitor.x,
    y: monitor.y,
    w: monitor.width as u32,
    h: monitor.height as u32,
  })
}
