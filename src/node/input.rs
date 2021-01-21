use crate::node::{TICK_SIZE, NodeConnection, Node, WorkbenchNode};

pub struct Wave {
	pub form: Waveform,
	pub freq: f32,
	phase: u32,
	buffer: Box<[f32; TICK_SIZE]>
}

pub enum Waveform {
	Sine,
	Tri,
	Saw,
	Square,
}

impl Node for Wave {
	fn title(&self) -> String {
		"Wave Generator".to_owned()
	}

	fn inputs(&self) -> Vec<String> {
		vec![]
	}

	fn outputs(&self) -> Vec<String> {
		vec!["Wave".to_owned()]
	}

	fn tick(&mut self, parents: &[NodeConnection], tick: u32, node_list: &[WorkbenchNode]) {
		self.phase = tick * TICK_SIZE as u32;
		//TODO: Generate waves
	}

	fn poll(&self, output_index: usize) -> Box<[f32; TICK_SIZE]> {
		self.buffer.clone()
	}
}