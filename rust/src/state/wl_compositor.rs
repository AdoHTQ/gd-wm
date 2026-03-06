use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, protocol::wl_compositor::{self, WlCompositor}};

use crate::state::State;



impl GlobalDispatch<WlCompositor, ()> for State
{
	fn bind(
		_state: &mut Self,
		_handle: &DisplayHandle,
		_client: &Client,
		resource: New<WlCompositor>,
		_global_data: &(),
		data_init: &mut DataInit<'_, Self>,
	) 
	{
		// Binds must initialize a new instance
		data_init.init(resource, ());
	}
}

impl Dispatch<WlCompositor, ()> for State {
    fn request(
        _state: &mut State,
        _client: &Client,
        _resource: &WlCompositor,
        _request: wl_compositor::Request,
        _data: &(),
        _handle: &DisplayHandle,
        _data_init: &mut DataInit<'_, State>,
    ) {
		eprintln!("WlCompositor Request");
        // match request {
        //     wl_compositor::Request::CreateSurface { id } => {
        //         data_init.init(id, ());
        //     }
        //     wl_compositor::Request::CreateRegion { id } => {
        //         data_init.init(id, ());
        //     }
        //     _ => {}
        // }
    }
}