use cpal::traits::*;

use crate::graphics::GraphicsHandler;
use crate::input::InputHandler;

use sdl2::event::Event;
use sdl2::video::WindowBuilder;

mod audio;
mod graphics;
mod input;

fn main() {
	// Initialising IO
	let sdl_context = sdl2::init().unwrap();
	let sdl_video = sdl_context.video().unwrap();
	let sdl_window = WindowBuilder::new(&sdl_video, "Loud", 640, 480).resizable().build().unwrap();
	let mut graphics = GraphicsHandler::new(sdl_window.into_canvas().accelerated().present_vsync().build().unwrap());
	let mut input = InputHandler::new();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut running = true;

	// let host = cpal::default_host();
	// let default_device = host.default_output_device().unwrap();
	// let mut audio_stream = audio::AudioStream::with_device(default_device).unwrap();

	while running {
		graphics.canvas.clear();

		for event in event_pump.poll_iter() {
			match event {
				Event::KeyDown { .. } | Event::KeyUp { .. } |
				Event::MouseButtonDown { .. } | Event::MouseButtonUp { .. } |
				Event::MouseMotion { .. } => { input.event(event) }
				Event::Quit { .. } => { running = false },
				_ => {},
			}
		}

		graphics.canvas.present();
	}
}
