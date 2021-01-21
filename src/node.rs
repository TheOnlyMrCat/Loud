use crate::graphics::Vector2;

pub struct Node {
	pub pos: Vector2,
	pub parents: Vec<usize>,
}