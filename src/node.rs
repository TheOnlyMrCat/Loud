use crate::graphics::Vector2;

pub struct WorkbenchNode {
	pub pos: Vector2,
	pub parents: Vec<usize>,
	pub node: Box<dyn Node>,
}

pub struct NodeConnection {
	node: usize,
	output_index: usize,
}

pub trait Node {
	fn inputs(&self) -> Vec<String>;
	fn outputs(&self) -> Vec<String>;

	fn tick(&mut self, parents: &[NodeConnection], tick: i32);
	fn poll(&self, output_index: usize) -> Box<[f32; 512]>;
}

pub struct EmptyNode;
impl Node for EmptyNode {
	fn inputs(&self) -> Vec<String> {
		Vec::new()
	}

	fn outputs(&self) -> Vec<String> {
		Vec::new()
	}

	fn tick(&mut self, parents: &[NodeConnection], tick: i32) {
		panic!()
	}

	fn poll(&self, output_index: usize) -> Box<[f32; 512]> {
		panic!()
	}
}