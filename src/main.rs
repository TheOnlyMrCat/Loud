use cpal::traits::*;

use crate::graphics::{GraphicsHandler, Image, Vector2};
use crate::input::InputHandler;
use crate::node::Node;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowBuilder;

mod audio;
mod graphics;
mod input;
mod node;

const TOOL_SIZE: u32 = 32;

fn main() {
	// Initialising IO
	let sdl_context = sdl2::init().unwrap();
	let sdl_video = sdl_context.video().unwrap();
	let sdl_window = WindowBuilder::new(&sdl_video, "Loud", 1000, 700).resizable().build().unwrap();
	let mut graphics = GraphicsHandler::new(sdl_window.into_canvas().accelerated().present_vsync().build().unwrap());
	let mut input = InputHandler::new();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut running = true;

	// let host = cpal::default_host();
	// let default_device = host.default_output_device().unwrap();
	// let mut audio_stream = audio::AudioStream::with_device(default_device).unwrap();

	let mut nodes = vec![
		Node {
			pos: Vector2::new(0, 0),
			parents: Vec::new(),
		},
	];

	while running {
		graphics.canvas.set_draw_color(Color::RGB(0x0b, 0x43, 0x78));
		graphics.canvas.clear();

		let canvas_size = graphics.canvas.output_size().unwrap();

		// Render nodes
		for node in nodes.iter() {
			graphics.canvas.set_draw_color(Color::RGB(200, 200, 200));
			graphics.canvas.fill_rect(Rect::new(node.pos.x, node.pos.y + TOOL_SIZE as i32, 256, 256)).unwrap();
		}

		// Render tool bar
		graphics.canvas.set_draw_color(Color::WHITE);
		graphics.canvas.fill_rect(Rect::new(0, 0, canvas_size.0, 32)).unwrap();
		graphics.render_scaled(&Image::Sprite("res/toolbar/add_project.png".to_owned()), Rect::new(0, 0, TOOL_SIZE, TOOL_SIZE));
		graphics.render_scaled(&Image::Sprite("res/toolbar/add_mic.png".to_owned()), Rect::new(32, 0, TOOL_SIZE, TOOL_SIZE));

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
