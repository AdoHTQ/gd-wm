use std::{os::fd::OwnedFd, ptr::NonNull};

use wayland_server::{Dispatch, GlobalDispatch, Resource, protocol::{wl_buffer::{self, WlBuffer}, wl_shm::{self, Format, WlShm}, wl_shm_pool::{self, WlShmPool}}};

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
		let shm = data_init.init(resource, ());
		// Tell the client which pixel formats we support
		shm.format(wl_shm::Format::Argb8888);
		shm.format(wl_shm::Format::Xrgb8888);
	}
}

impl Dispatch<WlShm, ()> for State
{
	fn request(
		_state: &mut Self,
		_client: &wayland_server::Client,
		resource: &WlShm,
		request: <WlShm as wayland_server::Resource>::Request,
		_data: &(),
		_dhandle: &wayland_server::DisplayHandle,
		data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		match request
		{
			wl_shm::Request::CreatePool { id, fd, size } =>
			{
				// Check that the memory size is positive
				if size <= 0
				{
					resource.post_error(
						wl_shm::Error::InvalidStride, 
						"Pool size must be positive".to_string()
					);
					return;
				}

				// mmap the fd. This is safe from the compositor's side:
                // we map it read-only so the client can't trick us into
                // writing somewhere unexpected. SHARED means we see
                // whatever the client writes.
                let mapping = match ShmMapping::new(fd, size as usize) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Failed to mmap shm pool: {e}");
                        return;
                    }
                };

				// Create our internal pool record and attach it as
                // UserData to the WlShmPool object.
                let pool = ShmPoolData {
                    mapping,
                    size: size as usize,
                };

				// Init the pool object — this activates it and attaches
                // `pool` as the UserData we'll get back in Dispatch<WlShmPool>.
                data_init.init(id, pool);
			}
			_ => {}
		}
	}
}


pub struct ShmMapping {
	ptr: NonNull<u8>,
	size: usize,
}

// SAFETY: The pointer is to an anonymous shared mapping.
// We only read from it (with appropriate care about client writes).
unsafe impl Send for ShmMapping {}
unsafe impl Sync for ShmMapping {}

impl ShmMapping
{
	pub fn new(fd: OwnedFd, size: usize) -> Result<Self, nix::Error>
	{
		use nix::sys::mman::{mmap, MapFlags, ProtFlags};
		use std::os::fd::AsFd;
		use std::num::NonZeroUsize;

		let len = NonZeroUsize::new(size).ok_or(nix::Error::EINVAL)?;

		let ptr: NonNull<u8> = unsafe {
			mmap(
				None,					//Let kernel pick address
				len,
				ProtFlags::PROT_READ,
				MapFlags::MAP_SHARED,
				fd.as_fd(),
				0,					//Offset into fd
			)?
		}.cast::<u8>();

		Ok(Self {
			ptr: ptr,
			size,
		})
	}
}

impl Drop for ShmMapping
{
	fn drop(&mut self) {
		use nix::sys::mman::munmap;
		use std::num::NonZeroUsize;

		if let Ok(len) = NonZeroUsize::try_from(self.size) {
			unsafe {
				let _ = munmap(self.ptr.as_ptr() as *mut _, len);
			}
		}
	}
}


struct ShmPoolData
{
	mapping: ShmMapping,
	size: usize,
}

impl Dispatch<WlShmPool, ShmPoolData> for State
{
	fn request(
		_state: &mut Self,
		_client: &wayland_server::Client,
		resource: &WlShmPool,
		request: <WlShmPool as wayland_server::Resource>::Request,
		data: &ShmPoolData,
		_dhandle: &wayland_server::DisplayHandle,
		data_init: &mut wayland_server::DataInit<'_, Self>,
	) {
		match request
		{
			wl_shm_pool::Request::CreateBuffer { id, offset, width, height, stride, format } =>
			{
				let buffer_format: Format = format.into_result().expect("Invalid format");
				//data_init.init(id, BufferData {_offset: offset, _width: width, _height:height, _stride:stride, _format: buffer_format});

				let buffer_size = (stride as usize)
					.checked_mul(height as usize)
					.and_then(|s| (offset as usize).checked_add(s));	

				let valid = buffer_size
					.map(|end| end <= data.size)
					.unwrap_or(false);

				if !valid || offset < 0 || width <= 0 || height <= 0 || stride < width * 4 {
					// Post a protocol error back to the client
					resource.post_error(
						wl_shm::Error::InvalidStride,
						"invalid buffer parameters".to_string(),
					);
					return;
				}

				// Build our internal buffer descriptor.
				// We DON'T copy pixels here — we just record where in
				// the pool this buffer lives.
				let buffer_user_data = BufferData {
					offset: offset as usize,
					width: width as u32,
					height: height as u32,
					stride: stride as u32,
					format: buffer_format,
				};

				// Activate the WlBuffer object with our metadata as UserData.
                data_init.init(id, buffer_user_data);
			}
			wl_shm_pool::Request::Destroy => {}
			wl_shm_pool::Request::Resize { size } =>
			{
				if (size as usize) < data.size {
                    resource.post_error(
                        wl_shm::Error::InvalidFd,
                        "cannot shrink shm pool".to_string(),
                    );
                }
			}
			_ => {}
		}
	}
}

struct BufferData
{
	offset: usize,
	width: u32,
	height: u32,
	stride: u32,
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
				//resource.release();
			}
			_ => {}
		}
	}
}