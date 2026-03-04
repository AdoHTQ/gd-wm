use godot::{classes::Engine, prelude::*};

struct GDWMExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GDWMExtension 
{
	fn on_level_init(level: InitLevel) 
	{
		match level 
		{
			// Runs before the renderer initializes
			InitLevel::Core => 
			{
				// Don't start a wayland server in the editor obviously
				if Engine::singleton().is_editor_hint() { return; }
				
				godot_print!("GD-WM init");
			}
			_ => {}
		}
	}

	fn on_level_deinit(level: InitLevel)
	{
		if level == InitLevel::Core
		{
			if Engine::singleton().is_editor_hint() { return; }
			
			godot_print!("Godot app has exited");
		}
	}
}
