use std::collections::HashMap;

use wayland_server::{DisplayHandle, backend::ObjectId};

mod wl_compositor;
mod xdg_wm_base;
mod wl_shm;

struct ToplevelState
{
	title: String,
	app_id: String,
}

impl ToplevelState
{
	pub fn new() -> Self
	{
		Self
		{
			title: "".to_string(),
			app_id: "".to_string(),
		}
	}
}

pub struct State
{
	pub display_handle: DisplayHandle,

	pub toplevels: HashMap<ObjectId, ToplevelState>,
}

impl State
{
	pub fn new(display_handle: DisplayHandle) -> Self
	{
		Self
		{
			display_handle,
			toplevels: HashMap::new(),
		}
	}
}
