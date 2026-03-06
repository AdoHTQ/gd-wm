use wayland_server::DisplayHandle;

mod wl_compositor;
mod xdg_wm_base;
mod wl_shm;

pub struct State
{
	pub display_handle: DisplayHandle,
}

