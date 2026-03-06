use std::sync::OnceLock;

use godot::{classes::Engine, prelude::*};

use crate::server::WaylandServer;

mod server;
mod state;
mod clientdata;


static WAYLAND_SERVER: OnceLock<InstanceId> = OnceLock::new();

struct GDWMExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GDWMExtension 
{
	fn on_level_init(level: InitLevel) 
	{
		if level != InitLevel::Core || Engine::singleton().is_editor_hint() {return;}
		
		godot_print!("GD-WM init");

		let mut instance = WaylandServer::new_alloc();
		WAYLAND_SERVER.set(instance.instance_id()).expect("Tried to instance second wayland server.");

		if let Err(e) = instance.bind_mut().start_socket()
		{
			godot_print!("Wayland server failed to start: {}", e);
		}
	}

	fn on_level_deinit(level: InitLevel)
	{
		if level != InitLevel::Core || Engine::singleton().is_editor_hint() {return;}

		godot_print!("Godot app has exited");

		if let Some(id) = WAYLAND_SERVER.get() {
			if let Ok(mut instance) = Gd::<WaylandServer>::try_from_instance_id(*id) {
				// Stop socket and free the object
				if let Err(e) = instance.bind_mut().stop_socket() { godot_print!("Wayland server failed to stop: {}", e); }
				instance.free();
			}
		}
	}
}
