use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, protocol::{wl_compositor::{self, WlCompositor}, wl_surface::{self, WlSurface}}};

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
        request: wl_compositor::Request,
        _data: &(),
        _handle: &DisplayHandle,
        data_init: &mut DataInit<'_, State>,
    ) {
        match request 
        {
            wl_compositor::Request::CreateSurface { id } => 
            {
                data_init.init(id, ());
            }
            // wl_compositor::Request::CreateRegion { id } => {
            //     data_init.init(id, ());
            // }
            _ => {}
        }
    }
}

impl Dispatch<WlSurface, ()> for State {
    fn request(
        _state: &mut Self,
        _client: &Client,
        _resource: &WlSurface,
        request: <WlSurface as wayland_server::Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        _data_init: &mut DataInit<'_, Self>,
    ) {
        match request
        {
            wl_surface::Request::Commit => 
            {
                //Lotta stuff to do here once we get to rendering
                //Double buffering and other things. 
                
            }
            _ => {}
        }
    }
}