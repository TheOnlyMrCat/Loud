use crate::graphics::Vector2;

use sdl2::event::Event;
use sdl2::rect::Rect;

use std::collections::HashMap;

pub use sdl2::{
	keyboard::Scancode,
	mouse::MouseButton,
};

pub struct InputHandler {
	keys_down: HashMap<Scancode, bool>,
	keys_pressed: HashMap<Scancode, bool>,

	buttons_down: HashMap<MouseButton, bool>,
	buttons_pressed: HashMap<MouseButton, bool>,

	pub mouse_pos: Vector2,
}

impl InputHandler {
	pub fn new() -> Self {
		Self {
			keys_down: HashMap::new(),
			keys_pressed: HashMap::new(),

			buttons_down: HashMap::new(),
			buttons_pressed: HashMap::new(),

			mouse_pos: Vector2::origin(),
		}
	}
	
	pub fn event(&mut self, event: Event) {
		match event {
			Event::KeyDown { scancode, .. } => {
				self.keys_down.insert(scancode.unwrap(), true);
				self.keys_pressed.insert(scancode.unwrap(), true);
			},
			Event::KeyUp { scancode, .. } => {
				self.keys_down.insert(scancode.unwrap(), false);
			},
			Event::MouseButtonDown { mouse_btn, .. } => {
				self.buttons_down.insert(mouse_btn, true);
				self.buttons_pressed.insert(mouse_btn, true);
			}
			Event::MouseButtonUp { mouse_btn, .. } => {
				self.buttons_down.insert(mouse_btn, false);
			}
			Event::MouseMotion { x, y, .. } => {
				self.mouse_pos = Vector2::new(x, y);
			}
			_ => unreachable!()			// Other functions will never be passed
		}
	}

	pub fn update(&mut self) {
		self.keys_pressed.clear();
		self.buttons_pressed.clear();
	}

	pub fn key_is(&self, key: Scancode, state: InputState) -> bool {
		match state {
			InputState::Down => *self.keys_down.get(&key).unwrap_or(&false),
			InputState::Pressed => *self.keys_pressed.get(&key).unwrap_or(&false),
			InputState::Up => !self.keys_down.get(&key).unwrap_or(&false),
		}
	}

	pub fn button_is(&self, button: MouseButton, state: InputState) -> bool {
		match state {
			InputState::Down => *self.buttons_down.get(&button).unwrap_or(&false),
			InputState::Pressed => *self.buttons_pressed.get(&button).unwrap_or(&false),
			InputState::Up => !self.buttons_down.get(&button).unwrap_or(&false),
		}
	}

	pub fn clicked_in_bounds(&self, bounds: Rect) -> bool {
		self.button_is(MouseButton::Left, InputState::Pressed) && self.mouse_pos.collides(bounds)
	}
}

pub enum InputState {
	Down,
	Pressed,
	Up,
}