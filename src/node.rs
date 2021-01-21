use crate::graphics::Vector2;

mod input;
mod basic_mod;

const TICK_SIZE: usize = 512;

pub struct WorkbenchNode {
	pub pos: Vector2,
	pub parents: Vec<NodeConnection>,
	pub node: Box<dyn Node>,
}

pub struct NodeConnection {
	node: usize,
	output_index: usize,
}

pub trait Node {
	fn inputs(&self) -> Vec<String>;
	fn outputs(&self) -> Vec<String>;

	fn tick(&mut self, parents: &[NodeConnection], tick: u32, node_list: &[WorkbenchNode]);
	fn poll(&self, output_index: usize) -> Box<[f32; TICK_SIZE]>;
}

pub struct EmptyNode;
impl Node for EmptyNode {
	fn inputs(&self) -> Vec<String> {
		Vec::new()
	}

	fn outputs(&self) -> Vec<String> {
		Vec::new()
	}

	fn tick(&mut self, _parents: &[NodeConnection], _tick: u32, _nodes: &[WorkbenchNode]) {
		panic!()
	}

	fn poll(&self, output_index: usize) -> Box<[f32; TICK_SIZE]> {
		panic!()
	}
}