use cpal::traits::*;

mod audio;
mod project;
mod window;

fn main() {
	let host = cpal::default_host();
	let default_device = host.default_output_device().unwrap();
	let mut audio_stream = audio::AudioStream::with_device(default_device).unwrap();
	
	let main_window = druid::WindowDesc::new(window::build_root);
}
