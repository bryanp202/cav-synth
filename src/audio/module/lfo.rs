use crate::audio::module::{Module, ModuleMessage};

#[derive(Clone, Copy, Debug)]
pub enum WaveShape {
    Saw,
    Sine,
    Square,
    Triangle,
}

#[derive(Clone, Copy, Debug)]
pub enum LFOUpdate {
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
    zero_phase: f32,
    quarter_phase: f32,
}

pub struct LFO {
    id: usize,
    sample_rate: usize,
    shape: WaveShape,
    level: f32,
    frequency: f32,
    phase: f32,
    current_phase: f32,
    input: Inputs,
    output: Outputs,
}

impl LFO {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            sample_rate,
            level: 1.0,
            current_phase: 0.0,
            frequency: 0.0,
            shape: WaveShape::Triangle,
            phase: 0.0,
            input: Inputs::default(),
            output: Outputs::default(),
        }
    }

    pub fn frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }
}

impl Module for LFO {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        let phase_input = self.input.phase;
        let frequency_input = self.input.frequency;
        let level_input = self.input.level;

        let level = (self.level + level_input).min(1.0).max(0.0);
        let frequency = (self.frequency + frequency_input).min(1.0).max(0.0);
        let frequency = 2.0_f32.powf(127.0 / 12.0 * frequency) * 0.5; // C-1 (midi note 0)

        let phase = (self.current_phase + phase_input) % 1.0;
        let quarter_phase = (phase + 0.25) % 1.0;

        let phase_increment = frequency / self.sample_rate as f32;

        let raw = match self.shape {
            WaveShape::Saw => 2.0 * phase - 1.0,
            WaveShape::Sine => (2.0 * std::f32::consts::PI * phase).sin(),
            WaveShape::Square => if phase < 0.5 {1.0} else {-1.0},
            WaveShape::Triangle => 1.0 - 4.0 * (phase - (phase + 0.5).floor()).abs(),
        };

        let raw_quarter_phase = match self.shape {
            WaveShape::Saw => 2.0 * quarter_phase - 1.0,
            WaveShape::Sine => (2.0 * std::f32::consts::PI * quarter_phase).sin(),
            WaveShape::Square => if quarter_phase < 0.5 {1.0} else {-1.0},
            WaveShape::Triangle => 1.0 - 4.0 * (quarter_phase - (quarter_phase + 0.5).floor()).abs(),
        };

        self.current_phase = (self.current_phase + phase_increment) % 1.0;

        let scaled_raw = raw * level;
        self.output.zero_phase = scaled_raw;

        let scaled_quarter_phase = raw_quarter_phase * level;
        self.output.quarter_phase = scaled_quarter_phase;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.lfo} {
                LFOUpdate::SampleRate(sample_rate) => self.sample_rate = sample_rate,
                LFOUpdate::Frequency(frequency) => self.frequency = frequency,
                LFOUpdate::Phase(phase) => self.phase = phase,
                LFOUpdate::Shape(shape) => self.shape = shape,
            }
        }
    }

    fn get_output(&self, target_output: usize) -> f32 {
        match target_output {
            0 => self.output.zero_phase,
            1 => self.output.quarter_phase,
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