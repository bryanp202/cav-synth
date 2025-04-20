use std::default;

use iced::Length::Shrink;

use crate::audio::module::{Module, ModuleId, ModuleMessage, Sample};

#[derive(Clone, Copy, Debug)]
pub enum WaveShape {
    Saw,
    Sine,
    Square,
    Triangle,
}

#[derive(Clone, Copy, Debug)]
pub enum AnalogOscillatorUpdate {
    SampleRate(usize),
    Frequency(f32),
    Phase(f32),
    Shape(WaveShape),
}

#[derive(Default)]
struct Inputs {
    frequency: Sample,
    phase: Sample,
    level: Sample,
}
pub struct AnalogOscillator {
    id: ModuleId,
    sample_rate: usize,
    shape: WaveShape,
    level: f32,
    frequency: f32,
    phase: f32,
    index: usize,
    input: Inputs,
}

impl AnalogOscillator {
    fn new(id: ModuleId, sample_rate: usize) -> Self {
        Self {
            id,
            sample_rate,
            level: 1.0,
            index: 0,
            frequency: 0.0,
            shape: WaveShape::Sine,
            phase: 0.0,
            input: Inputs::default(),
        }
    }
}

impl Module for AnalogOscillator {
    fn id(&self) -> &ModuleId {
        &self.id
    }

    fn process(&mut self) -> Box<[Sample]> {
        let phase_input: f32 = self.input.phase.into();
        let frequency_input: f32 = self.input.frequency.into();
        let level_input: f32 = self.input.level.into();

        let level = (self.level + level_input) % 1.0;
        let frequency = (self.frequency + frequency_input) % 1.0;
        let frequency = 2.0_f32.powf(frequency * 8.0) + 13.75;
        let phase = (frequency * self.index as f32 / self.sample_rate as f32 + phase_input) % 1.0;

        let raw = match self.shape {
            WaveShape::Saw => 1.0 - 2.0 * phase,
            WaveShape::Sine => (2.0 * std::f32::consts::PI * phase).sin(),
            WaveShape::Square => if phase >= 0.5 {1.0} else {-1.0},
            WaveShape::Triangle => 1.0 - 4.0 * (phase - (phase + 0.5).floor()).abs(),
        };

        self.index = (self.index + 1) % self.sample_rate;

        let scaled_raw = raw * level;
        Box::new([Sample::from(scaled_raw)])
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.analog} {
                AnalogOscillatorUpdate::SampleRate(sample_rate) => self.sample_rate = sample_rate,
                AnalogOscillatorUpdate::Frequency(frequency) => self.frequency = frequency,
                AnalogOscillatorUpdate::Phase(phase) => self.phase = phase,
                AnalogOscillatorUpdate::Shape(shape) => self.shape = shape,
            }
        }
    }

    fn modulate(&mut self, component: usize, value: Sample) {
        match component {
            0 => self.input.level = value,
            1 => self.input.frequency = value,
            2 => self.input.phase = value,
            _ => unreachable!(),
        }
    }
}