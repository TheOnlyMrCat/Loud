use cpal::traits::StreamTrait;

const DEFAULT_SAMPLE_RATE: cpal::SampleRate = cpal::SampleRate(44100);

pub struct AudioStream<D: cpal::traits::DeviceTrait> {
	device: D,
	stream: D::Stream,
	buffer: ringbuf::Producer<f32>,
}

#[derive(Debug)]
pub enum BuildAudioStreamError {
	UnsupportedDevice,
	StreamConfigsError(cpal::SupportedStreamConfigsError),
	BuildStreamError(cpal::BuildStreamError),
}

impl<D> AudioStream<D>
where
	D: cpal::traits::DeviceTrait,
{
	pub fn with_device(device: D) -> Result<AudioStream<D>, BuildAudioStreamError> {
		let supported_audio_config = device.supported_output_configs().map_err(|e| BuildAudioStreamError::StreamConfigsError(e))?
			.filter_map(|supported_config|
				if (supported_config.min_sample_rate()..=supported_config.max_sample_rate()).contains(&DEFAULT_SAMPLE_RATE) {
					Some(supported_config.with_sample_rate(DEFAULT_SAMPLE_RATE))
				} else {
					None
				}
			)
			.next().ok_or(BuildAudioStreamError::UnsupportedDevice)?;

		let buffer = ringbuf::RingBuffer::new(44100);
		let (producer, mut consumer) = buffer.split();

		Ok(AudioStream {
			stream: device.build_output_stream_raw(
				&supported_audio_config.config(),
				supported_audio_config.sample_format(),
				move |data, _| {
					for sample in data.as_slice_mut().unwrap() {
						*sample = match consumer.pop() {
							Some(s) => s,
							None => {
								eprintln!("Input fell behind");
								0.0
							}
						};
					}
				},
				|err| {
					eprintln!("An error occurred on a stream: {}", err);
				}
			).map_err(|e| BuildAudioStreamError::BuildStreamError(e))?,
			device,
			buffer: producer,
		})
	}

	pub fn play(&self) {
		self.stream.play().unwrap();
	}

	pub fn push_audio(&mut self, data: &[f32]) {
		self.buffer.push_slice(data);
	}

	pub fn buffer_remaining(&self) -> usize {
		self.buffer.remaining()
	}

	pub fn buffer_full(&self) -> bool {
		self.buffer.is_full()
	}
}

pub struct WaveGenerator {

}

impl WaveGenerator {
	pub fn play_sine_freq_to<D: cpal::traits::DeviceTrait>(&self, frequency: f64, audio_stream: &mut AudioStream<D>) {
		let wavelength = 44100.0 / (frequency / 2.0);
		let mut phase = 0.0;
		let phase_step = std::f64::consts::TAU / wavelength;
		loop {
			if !audio_stream.buffer_full() {
				audio_stream.push_audio(&[f32::sin(phase as f32)]);
				phase = (phase_step + phase) % std::f64::consts::TAU;
			}
		}
	}

	pub fn play_tri_freq_to<D: cpal::traits::DeviceTrait>(&self, frequency: f64, audio_stream: &mut AudioStream<D>) {
		let wavelength = 44100.0 / (frequency / 2.0);
		let mut phase = 0.0;
		let phase_step = 1.0 / wavelength;
		loop {
			if !audio_stream.buffer_full() {
				audio_stream.push_audio(&[(f64::abs(phase - 0.5) * 4.0 - 1.0) as f32]);
				phase = (phase + phase_step) % 1.0;
			}
		}
	}

	pub fn play_saw_freq_to<D: cpal::traits::DeviceTrait>(&self, frequency: f64, audio_stream: &mut AudioStream<D>) {
		let wavelength = 44100.0 / (frequency / 2.0);
		let mut phase = 0.0;
		let phase_step = 1.0 / wavelength;
		loop {
			if !audio_stream.buffer_full() {
				audio_stream.push_audio(&[(phase * 2.0 - 1.0) as f32]);
				phase = (phase + phase_step) % 1.0;
			}
		}
	}

	pub fn play_square_freq_to<D: cpal::traits::DeviceTrait>(&self, frequency: f64, audio_stream: &mut AudioStream<D>) {
		let wavelength = (44100.0 / frequency).round() as usize;
		loop {
			if audio_stream.buffer_remaining() >= wavelength * 2 {
				audio_stream.push_audio(&[-1.0][..].repeat(wavelength));
				audio_stream.push_audio(&[1.0][..].repeat(wavelength));
			}
		}
	}
}

/// Converts a semitone (with 0.0 being A0) to a frequency (with A0 being 27.5 Hz)
pub fn semitone_to_frequency(semitone: f64) -> f64 {
	2f64.powf(semitone / 12.0) * 27.5
}