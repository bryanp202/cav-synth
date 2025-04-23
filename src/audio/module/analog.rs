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
    current_phase: f32,
    input: Inputs,
    output: Outputs,
}

impl AnalogOscillator {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            sample_rate,
            level: 0.0,
            current_phase: 0.0,
            frequency: 0.0,
            shape: WaveShape::Sine,
            phase: 0.0,
            input: Inputs::default(),
            output: Outputs::default(),
        }
    }

    fn poly_blep(phase: f32, phase_increment: f32) -> f32 {
        if phase < phase_increment {
            let t = phase / phase_increment;
            t+t - t*t - 1.0
        } else if phase > 1.0 - phase_increment {
            let t = (phase - 1.0) / phase_increment;
            t*t + t+t + 1.0
        } else {
            0.0
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

        let level = (self.level + level_input).min(1.0).max(0.0);
        let frequency = (self.frequency + frequency_input).min(1.0).max(0.0);
        let frequency = 2.0_f32.powf(127.0 / 12.0 * frequency) * 8.176; // C-1 (midi note 0)
        let phase = (self.current_phase + phase_input) % 1.0;

        let phase_increment = frequency / self.sample_rate as f32;

        let raw = match self.shape {
            WaveShape::Saw => 2.0 * phase - 1.0 - AnalogOscillator::poly_blep(phase, phase_increment),
            WaveShape::Sine => (2.0 * std::f32::consts::PI * phase).sin(),
            WaveShape::Square => {
                let raw = if phase < 0.5 {1.0} else {-1.0};
                raw + AnalogOscillator::poly_blep(phase, phase_increment) - AnalogOscillator::poly_blep((phase + 0.5) % 1.0, phase_increment)
            },
            WaveShape::Triangle => 1.0 - 4.0 * (phase - (phase + 0.5).floor()).abs(),
        };

        self.current_phase = (self.current_phase + phase_increment) % 1.0;

        let scaled_raw = raw * level;
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