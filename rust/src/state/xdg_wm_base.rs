

use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, protocol::wl_surface::WlSurface};
use wayland_protocols::xdg::shell::server::{xdg_surface::{self, XdgSurface}, xdg_toplevel::{self, XdgToplevel}, xdg_wm_base::{self, XdgWmBase}};

use crate::state::{State, ToplevelState};


impl GlobalDispatch<XdgWmBase, ()> for State {
	fn bind(
		_state: &mut State,
		_handle: &DisplayHandle,
		_client: &Client,
		resource: New<XdgWmBase>,
		_global_data: &(),
		data_init: &mut DataInit<'_, State>,
	) {
		data_init.init(resource, ());
	}
}

impl Dispatch<XdgWmBase, ()> for State {
	fn request(
		_state: &mut State,
		_client: &Client,
		_resource: &XdgWmBase,
		request: xdg_wm_base::Request,
		_data: &(),
		_handle: &DisplayHandle,
		data_init: &mut DataInit<'_, State>,
	) {
		match request
		{
			xdg_wm_base::Request::GetXdgSurface { id, surface } =>
			{
				data_init.init(id, XdgSurfaceData{wl_surface: surface});
			}
			_ => {}
		}
	}
}

pub struct XdgSurfaceData
{
	pub wl_surface: WlSurface,
}

impl Dispatch<XdgSurface, XdgSurfaceData> for State {
	fn request(
		state: &mut Self,
		_client: &Client,
		xdg_surface: &XdgSurface,
		request: <XdgSurface as wayland_server::Resource>::Request,
		data: &XdgSurfaceData,
		_dhandle: &DisplayHandle,
		data_init: &mut DataInit<'_, Self>,
	) {
		match request
		{
			xdg_surface::Request::GetToplevel { id } =>
			{
				let toplevel = data_init.init(id, XdgTopLevelData {wl_surface: data.wl_surface.clone(), xdg_surface: xdg_surface.clone()});
				
				// Initialize toplevel state in the global state
				state.toplevels.insert(data.wl_surface.clone(), ToplevelState::new(toplevel));
			}
			_ => {}
		}
	}
}

pub struct XdgTopLevelData
{
	pub wl_surface: WlSurface,
	pub xdg_surface: XdgSurface,
}

impl Dispatch<XdgToplevel, XdgTopLevelData> for State {
	fn request(
		state: &mut Self,
		_client: &Client,
		_resource: &XdgToplevel,
		request: <XdgToplevel as wayland_server::Resource>::Request,
		data: &XdgTopLevelData,
		_dhandle: &DisplayHandle,
		_data_init: &mut DataInit<'_, Self>,
	) {
		match request
		{
			xdg_toplevel::Request::SetTitle { title } =>
			{
				state.toplevels.get_mut(&data.wl_surface).unwrap().title = title;
			}
			xdg_toplevel::Request::SetAppId { app_id } =>
			{
				state.toplevels.get_mut(&data.wl_surface).unwrap().app_id = app_id;
			}
			_ => {}
		}
	}
}