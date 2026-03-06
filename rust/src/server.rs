
use std::{sync::{Arc, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}};

use calloop::{EventLoop, Interest, LoopSignal, Mode, PostAction, generic::Generic};
use godot::prelude::*;
use wayland_protocols::xdg::shell::server::xdg_wm_base::XdgWmBase;
use wayland_server::{Display, ListeningSocket, protocol::{wl_compositor::WlCompositor, wl_shm::WlShm}};
use crate::{clientdata::CustomClientData, state::State};





#[derive(GodotClass)]
#[class(base=Object)]
pub struct WaylandServer
{
	base: Base<Object>,
	loop_signal: Option<LoopSignal>,
	thread_handle: Option<JoinHandle<()>>,
}

#[godot_api]
impl IObject for WaylandServer
{
	fn init(base: Base<Object>) -> Self
	{
		Self {
			base,
			loop_signal: None,
			thread_handle: None,
		}
	}
}

#[godot_api]
impl WaylandServer
{
	/// Starts the server on a separate thread
	pub fn start_socket(&mut self) -> Result<(), Box<dyn std::error::Error>>
	{
		let (tx, rx): (Sender<LoopSignal>, Receiver<LoopSignal>) = mpsc::channel();

		// Run server on a separate thread so godot can launch
		self.thread_handle = Some(thread::spawn(move ||
		{
			// Create wl_display singleton
			let display: Display<State> = Display::new().expect("Failed to initialize wayland display.");
			let display_handle = display.handle();

			let socket: ListeningSocket = ListeningSocket::bind_auto("wayland", 1..10).expect("Failed to create wayland socket");
			eprintln!("Socket created on {:?}", socket.socket_name().unwrap());

			let mut event_loop = EventLoop::<State>::try_new().expect("Could not initialize event loop.");
			let loop_handle = event_loop.handle();
			let loop_signal = event_loop.get_signal();

			// Create display globals
			display_handle.create_global::<State, WlCompositor, _>(6, ());
			display_handle.create_global::<State, XdgWmBase, _>(7, ());
			display_handle.create_global::<State, WlShm, _>(2, ());

			let mut state: State = State{display_handle};

			// Poll socket for ready connections and accept new ones
			loop_handle.insert_source(
				Generic::new(socket, Interest::READ, Mode::Level), 
				|_, socket, state| 
				{
					// Accept stream and insert new client
					let stream = socket.accept().expect("Could not connect client").unwrap();
					state.display_handle.insert_client(stream, Arc::new(CustomClientData {})).unwrap();
					
					eprintln!("Connection accepted");
					Ok(PostAction::Continue)
				}
			).unwrap();

			// This will poll requests from the wayland socket and dispatch them to their functions
			loop_handle.insert_source(
				Generic::new(display, Interest::READ, Mode::Level), 
				|_, display, state| 
				{
					eprintln!("Dispatch clients");
					unsafe {
						display.get_mut().dispatch_clients(state).unwrap();
					}
					Ok(PostAction::Continue)
				}
			).unwrap();
			
			//Send signal that initialization has finished
			tx.send(loop_signal).unwrap();

			event_loop.run(
				std::time::Duration::from_millis(20), 
				&mut state, 
				|state|
				{
					state.display_handle.flush_clients().unwrap();
				}
			).expect("Error during event loop");
		}));
		
		//Wait for message from thread to start godot
		self.loop_signal = Some(rx.recv().unwrap());

		Ok(())
	}

	pub fn stop_socket(&mut self) -> Result<(), Box<dyn std::error::Error>>
	{
		self.loop_signal
			.as_ref()
			.unwrap()
			.stop();
		
		// Wait for the thread to finish before returning
		if let Some(handle) = self.thread_handle.take() {
			let _ = handle.join();
		}
		
		godot_print!("Wayland server has stopped");

		Ok(())
	}
}