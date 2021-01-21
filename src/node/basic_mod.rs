use crate::node::{TICK_SIZE, NodeConnection, Node, WorkbenchNode};

pub struct Amplifier {
	pub factor: f32,
	buffer: Box<[f32; TICK_SIZE]>
}

impl Node for Amplifier {
	fn inputs(&self) -> Vec<String> {
		vec!["".to_owned()]
	}

	fn outputs(&self) -> Vec<String> {
		vec!["".to_owned()]
	}

	fn tick(&mut self, parents: &[NodeConnection], _tick: u32, node_list: &[WorkbenchNode]) {
		let parent = &node_list[parents[0].node];
		self.buffer = parent.node.poll(parents[0].output_index);
		for f in self.buffer.iter_mut() {
			*f *= self.factor
		}
	}

	fn poll(&self, output_index: usize) -> Box<[f32; TICK_SIZE]> {
		self.buffer.clone()
	}
}