use std::default;

use crate::audio::module::{Module, ModuleMessage};

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
    level: f32,
    frequency: f32,
    phase: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct AnalogOscillator {
    id: usize,
    sample_rate: usize,
    shape: WaveShape,
    level: f32,
    frequency: f32,
    phase: f32,
    index: usize,
    input: Inputs,
    output: Outputs,
}

impl AnalogOscillator {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            sample_rate,
            level: 0.0,
            index: 0,
            frequency: 0.0,
            shape: WaveShape::Sine,
            phase: 0.0,
            input: Inputs::default(),
            output: Outputs::default(),
        }
    }
}

impl Module for AnalogOscillator {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        let phase_input = self.input.phase;
        let frequency_input = self.input.frequency;
        let level_input = self.input.level;

        let level = (self.level + level_input).min(0.02);
        let frequency = (self.frequency + frequency_input).min(1.0);
        let frequency = 2.0_f32.powf(127.0 / 12.0 * frequency) * 8.176; // C-1 (midi note 0)
        let phase = (frequency * self.index as f32 / self.sample_rate as f32 + phase_input) % 1.0;

        let raw = match self.shape {
            WaveShape::Saw => 1.0 - 2.0 * phase,
            WaveShape::Sine => (2.0 * std::f32::consts::PI * phase).sin(),
            WaveShape::Square => if phase >= 0.5 {1.0} else {-1.0},
            WaveShape::Triangle => 1.0 - 4.0 * (phase - (phase + 0.5).floor()).abs(),
        };

        self.index = (self.index + 1) % self.sample_rate;

        let scaled_raw = raw * level;
        //println!("level {}", self.level);
        self.output.value = f32::from(scaled_raw);
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

    fn get_output(&self, target_output: usize) -> f32 {
        match target_output {
            0 => self.output.value,
            _ => unreachable!(),
        }
    }

    fn modulate(&mut self, component: usize, value: f32) {
        match component {
            0 => self.input.level = value,
            1 => self.input.frequency = value,
            2 => self.input.phase = value,
            _ => unreachable!(),
        }
    }
}