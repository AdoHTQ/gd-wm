use wayland_server::{Dispatch, GlobalDispatch, protocol::wl_shm::WlShm};

use crate::state::State;



impl GlobalDispatch<WlShm, ()> for State
{
	fn bind(
		_state: &mut Self,
		_handle: &wayland_server::DisplayHandle,
		_client: &wayland_server::Client,
		resource: wayland_server::New<WlShm>,
		_global_data: &(),
		data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		data_init.init(resource, ());
		// // Tell the client which pixel formats we support
        // shm.format(wl_shm::Format::Argb8888);
        // shm.format(wl_shm::Format::Xrgb8888);
	}
}

impl Dispatch<WlShm, ()> for State
{
	fn request(
		_state: &mut Self,
		_client: &wayland_server::Client,
		_resource: &WlShm,
		_request: <WlShm as wayland_server::Resource>::Request,
		_data: &(),
		_dhandle: &wayland_server::DisplayHandle,
		_data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		
	}
}