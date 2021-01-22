use relm::{connect, Relm, Update, Widget};
use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};
use relm_derive::Msg;

use std::ops::{AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
}

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

impl AddAssign for Vector2 {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl Div for Vector2 {
	type Output = Self;
	fn div(self, rhs: Self) -> Self {
		Vector2 {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl Mul for Vector2 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Vector2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
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

pub struct Model {

}

#[derive(Msg)]
pub enum Msg {
	Quit,
}

pub struct Win {
	model: Model,
	window: Window,
}

impl Update for Win {
	// Specify the model used for this widget.
	type Model = Model;
	// Specify the model parameter used to init the model.
	type ModelParam = ();
	// Specify the type of the messages sent to the update function.
	type Msg = Msg;

	// Return the initial model.
	fn model(_: &Relm<Self>, _: ()) -> Model {
		Model {
		}
	}

	// The model may be updated when a message is received.
	// Widgets may also be updated in this function.
	fn update(&mut self, event: Msg) {
		match event {
			Msg::Quit => gtk::main_quit(),
		}
	}
}

impl Widget for Win {
	// Specify the type of the root widget.
	type Root = Window;

	// Return the root widget.
	fn root(&self) -> Self::Root {
		self.window.clone()
	}

	// Create the widgets.
	fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
		// GTK+ widgets are used normally within a `Widget`.
		let window = Window::new(WindowType::Toplevel);
		window.resize(1000, 700);
		window.set_title("Loud");

		let screen_box = gtk::BoxBuilder::new().orientation(gtk::Orientation::Vertical).build();
		window.add(&screen_box);

		let toolbar = gtk::ButtonBoxBuilder::new().build();
		screen_box.add(&toolbar);

		let workspace = gtk::DrawingAreaBuilder::new().build();
		screen_box.add(&workspace);

		// Connect the signal `delete_event` to send the `Quit` message.
		connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
		// There is also a `connect!()` macro for GTK+ events that do not need a
		// value to be returned in the callback.

		window.show_all();

		Win {
			model,
			window,
		}
	}
}