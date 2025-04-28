use crate::audio::module::{Module, ModuleMessage};

#[derive(Clone, Copy, Debug)]
pub enum ButterworthUpdate {
    SampleRate(usize),
    Frequency(f32),
}

#[derive(Default)]
struct Inputs {
    value: f32,
    frequency: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct Butterworth {
    id: usize,
    sample_rate: usize,
    frequency: f32,
    input: Inputs,
    output: Outputs,
    // State
    x_minus: f32,
    x_minus2: f32,
    y_minus: f32,
    y_minus2: f32,
}

impl Butterworth {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            sample_rate,
            frequency: 1.0,
            input: Inputs::default(),
            output: Outputs::default(),
            x_minus: 0.0,
            x_minus2: 0.0,
            y_minus: 0.0,
            y_minus2: 0.0,
        }
    }

    pub fn cutoff(mut self, freq: f32) -> Self {
        self.frequency = freq.log2() / 14.55;
        self
    }
}

impl Module for Butterworth {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        let frequency = (self.frequency + self.input.frequency).min(1.0).max(0.0);
        let frequency = 2.0_f32.powf(127.0 / 12.0 * frequency) * 8.176; // C-1 (midi note 0)

        let c;
        let a0;
        let a1;
        let a2;
        let b1;
        let b2;

        c = 1.0 / (std::f32::consts::PI * frequency / self.sample_rate as f32).tan();
        a0 = 1.0 / (1.0 + 2.0_f32.sqrt() * c + c * c);
        a1 = 2.0 * a0;
        a2 = a0;
        b1 = 2.0 * a0 * (1.0 - c * c);
        b2 = a0 * (1.0 - 2.0_f32.sqrt() * c + c * c);

        self.output.value = self.input.value * a0 + self.x_minus * a1 + self.x_minus2 * a2 - self.y_minus * b1 - self.y_minus2 * b2;

        self.x_minus2 = self.x_minus;
        self.x_minus = self.input.value;
        self.y_minus2 = self.y_minus;
        self.y_minus = self.output.value;

        self.input.value = 0.0;
        self.input.frequency = 0.0;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.butterworth} {
                ButterworthUpdate::Frequency(frequency) => self.frequency = frequency,
                ButterworthUpdate::SampleRate(sample_rate) => self.sample_rate = sample_rate,
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
            0 => self.input.value += value,
            1 => self.input.frequency += value,
            _ => unreachable!(),
        }
    }
}