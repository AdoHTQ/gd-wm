use std::os::fd::OwnedFd;

use wayland_server::{Dispatch, GlobalDispatch, protocol::{wl_buffer::{self, WlBuffer}, wl_shm::{self, Format, WlShm}, wl_shm_pool::{self, WlShmPool}}};

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
		request: <WlShm as wayland_server::Resource>::Request,
		_data: &(),
		_dhandle: &wayland_server::DisplayHandle,
		data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		match request
		{
			wl_shm::Request::CreatePool { id, fd, size } =>
			{
				data_init.init(id, ShmPoolData {fd, size});
			}
			_ => {}
		}
	}
}

struct ShmPoolData
{
	fd: OwnedFd,
	size: i32,
}

impl Dispatch<WlShmPool, ShmPoolData> for State
{
	fn request(
		_state: &mut Self,
		_client: &wayland_server::Client,
		_resource: &WlShmPool,
		request: <WlShmPool as wayland_server::Resource>::Request,
		_data: &ShmPoolData,
		_dhandle: &wayland_server::DisplayHandle,
		data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		match request
		{
			wl_shm_pool::Request::CreateBuffer { id, offset, width, height, stride, format } =>
			{
				let buffer_format: Format = format.into_result().expect("Invalid format");
				data_init.init(id, BufferData {offset, width, height, stride, format: buffer_format});
			}
			wl_shm_pool::Request::Destroy => {}
			wl_shm_pool::Request::Resize { size: _size } =>
			{
				
			}
			_ => {}
		}
	}
}

struct BufferData
{
	offset: i32,
	width: i32,
	height: i32,
	stride: i32,
	format: Format,
}

impl Dispatch<WlBuffer, BufferData> for State
{
	fn request(
		_state: &mut Self,
		_client: &wayland_server::Client,
		resource: &WlBuffer,
		request: <WlBuffer as wayland_server::Resource>::Request,
		_data: &BufferData,
		_dhandle: &wayland_server::DisplayHandle,
		_data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		match request
		{
			wl_buffer::Request::Destroy =>
			{
				resource.release();
			}
			_ => {}
		}
	}
}