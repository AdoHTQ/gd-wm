use wayland_protocols::xdg::shell::server::{xdg_surface, xdg_toplevel};
use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, protocol::{wl_callback::WlCallback, wl_compositor::{self, WlCompositor}, wl_region::WlRegion, wl_surface::{self, WlSurface}}};

use crate::state::State;
use crate::state::xdg_wm_base::XdgTopLevelData;


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
			wl_compositor::Request::CreateRegion { id } =>
			{
				data_init.init(id, ());
			}
			_ => {}
		}
	}
}



impl Dispatch<WlSurface, ()> for State {
	fn request(
		state: &mut Self,
		_client: &Client,
		resource: &WlSurface,
		request: <WlSurface as wayland_server::Resource>::Request,
		_data: &(),
		_dhandle: &DisplayHandle,
		data_init: &mut DataInit<'_, Self>,
	) {
		match request
		{
			wl_surface::Request::Commit => 
			{
				//Lotta stuff to do here once we get to rendering
				//Double buffering and other things.

				//Get xdg toplevel of surface and configure it
				let toplevel_data = state.toplevels.get(resource).expect("Could not get toplevel from state hashmap.");
				let toplevel = toplevel_data.toplevel.clone();
				toplevel.send_event(
					//Junk parameters for now
					xdg_toplevel::Event::Configure { 
						width: 500, 
						height: 500, 
						states: Vec::<u8>::new() 
					}
				).expect("Could not send configure event to xdg toplevel.");

				//Get xdg surface from toplevel and configure it
				let xdg_surface = toplevel.data::<XdgTopLevelData>().unwrap().xdg_surface.clone();
				xdg_surface.send_event(
					xdg_surface::Event::Configure { 
						serial: state.serial 
					}
				).expect("Could not send configure event to xdg surface.");
			}
			wl_surface::Request::Frame { callback } =>
			{
				data_init.init(callback, ());
			}
			_ => {}
		}
	}
}


impl Dispatch<WlCallback, ()> for State {
	fn request(
		_state: &mut Self,
		_client: &Client,
		_resource: &WlCallback,
		_request: <WlCallback as Resource>::Request,
		_data: &(),
		_dhandle: &DisplayHandle,
		_data_init: &mut DataInit<'_, Self>,
	) {
		
	}
}


impl Dispatch<WlRegion, ()> for State {
	fn request(
		_state: &mut Self,
		_client: &Client,
		_resource: &WlRegion,
		_request: <WlRegion as Resource>::Request,
		_data: &(),
		_dhandle: &DisplayHandle,
		_data_init: &mut DataInit<'_, Self>,
	) {
		
	}
}