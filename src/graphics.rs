pub use sdl2::pixels::Color;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;

use std::collections::HashMap;
use std::ops::{Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vector2 {
	pub x: i32,
	pub y: i32,
}

impl Vector2 {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn square(w: i32) -> Self {
		Self {
			x: w,
			y: w,
		}
	}

	pub fn origin() -> Self {
		Self {
			x: 0,
			y: 0,
		}
	}

	pub fn collides(&self, bounds: Rect) -> bool {
		self.x > bounds.x &&
		self.x < bounds.x + bounds.w &&
		self.y > bounds.y &&
		self.y < bounds.y + bounds.h
	}
}

impl SubAssign for Vector2 {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl Sub for Vector2 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		Vector2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

pub struct GraphicsHandler {
	pub canvas: WindowCanvas,
	sprite_cache: HashMap<String, Texture>,
	text_cache: HashMap<Text, Texture>,
	ttf: Sdl2TtfContext,
}

impl GraphicsHandler {
	pub fn new(canvas: WindowCanvas) -> Self {
		Self {
			canvas,
			sprite_cache: HashMap::new(),
			text_cache: HashMap::new(),
			ttf: sdl2::ttf::init().unwrap(),
		}
	}

	pub fn get_bounds(texture: &Texture, pos: Vector2) -> Rect {
		Rect::new(pos.x, pos.y, texture.query().width, texture.query().height)
	}

	pub fn render(&mut self, image: &Image, pos: Vector2) {
		let texture = self.sprite_cache.get(&image.render()).unwrap();
		self.render_scaled(image, Self::get_bounds(texture, pos))
	}

	pub fn render_scaled(&mut self, image: &Image, bound: Rect) {
		let path = image.render();
		if !self.sprite_cache.contains_key(&path) {
			self.sprite_cache.insert(path.clone(), self.canvas.texture_creator().load_texture(path.clone()).unwrap());
		}

		let texture = self.sprite_cache.get(&path).unwrap();
		self.canvas.copy(texture, None, bound).unwrap();
	}

	pub fn render_text(&mut self, text: &Text, pos: Vector2) {
		if !self.text_cache.contains_key(text) {
			let font = self.ttf.load_font(text.font_path.clone(), text.size).unwrap();
			self.text_cache.insert(
				text.clone(),
				self.canvas.create_texture_from_surface(font.render(&text.text.to_string()).blended_wrapped(text.color, self.canvas.output_size().unwrap().0).unwrap()).unwrap()
			);
		}

		let texture = self.text_cache.get(text).unwrap();
		self.canvas.copy(texture, None, Self::get_bounds(texture, pos)).unwrap();
	}
}

pub enum Image {
	None,
	Sprite(String),
}

impl Image {
	pub fn render(&self) -> String {
		match self {
			Image::None => "".to_owned(),
			Image::Sprite(path) => path.to_owned(),
		}
	}
}

#[derive(Eq, Hash, PartialEq)]
pub struct Text {
    pub text: String,
    pub font_path: String,
    pub size: u16,
    pub color: Color,
}

impl Clone for Text {
	fn clone(&self) -> Self {
		Self {
			text: self.text.clone(),
			font_path: self.font_path.clone(),
			size: self.size,
			color: self.color,
		}
	}
}