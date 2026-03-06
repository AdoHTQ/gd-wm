use std::collections::HashMap;

use wayland_protocols::xdg::shell::server::xdg_toplevel::XdgToplevel;
use wayland_server::{DisplayHandle, protocol::wl_surface::WlSurface};

mod wl_compositor;
mod xdg_wm_base;
mod wl_shm;

pub struct ToplevelState
{
	toplevel: XdgToplevel,
	title: String,
	app_id: String,
}

impl ToplevelState
{
	pub fn new(toplevel: XdgToplevel) -> Self
	{
		Self
		{
			toplevel,
			title: "".to_string(),
			app_id: "".to_string(),
		}
	}
}

pub struct State
{
	pub serial: u32,
	pub display_handle: DisplayHandle,

	pub toplevels: HashMap<WlSurface, ToplevelState>,
}

impl State
{
	pub fn new(display_handle: DisplayHandle) -> Self
	{
		Self
		{
			serial: 0,
			display_handle,
			toplevels: HashMap::new(),
		}
	}
}
