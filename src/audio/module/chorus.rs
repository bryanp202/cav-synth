use std::collections::VecDeque;

use crate::audio::module::{Module, ModuleMessage};

use crate::audio::module::lfo::Lfo;

const MAX_CHORUS_SAMPLES: usize = 512;

#[derive(Clone, Copy, Debug)]
pub enum ChorusUpdate {
    Time(f32),
    Ratio(f32),
}

#[derive(Default)]
struct Inputs {
    value: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct Chorus {
    id: usize,
    ratio: f32,
    max_delay: f32,
    input: Inputs,
    output: Outputs,
    // State
    buffer: VecDeque<f32>,
    lfo: Lfo,
}

impl Chorus {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            ratio: 0.5,
            max_delay: 128.0,
            input: Inputs::default(),
            output: Outputs::default(),
            buffer: VecDeque::with_capacity(MAX_CHORUS_SAMPLES),
            lfo: Lfo::new(id, sample_rate).frequency(0.05)
        }
    }
}

impl Module for Chorus {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        self.lfo.process();

        let chorus_index1 = (self.lfo.get_output(0) * self.max_delay + self.max_delay) as usize;
        let chorus_amp1 = self.buffer.get(chorus_index1).unwrap_or(&0.0);

        let chorus_index2 = (self.lfo.get_output(1) * self.max_delay + self.max_delay) as usize;
        let chorus_amp2 = self.buffer.get(chorus_index2).unwrap_or(&0.0);

        self.output.value = (self.ratio - 1.0) * self.input.value + self.ratio * (chorus_amp1 + chorus_amp2) / 2.0;

        self.buffer.remove(MAX_CHORUS_SAMPLES - 1);
        self.buffer.push_front(self.output.value);

        self.input.value = 0.0;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.chorus} {
                ChorusUpdate::Ratio(ratio) => self.ratio = ratio.min(0.99),
                ChorusUpdate::Time(delay) => {
                    self.max_delay = delay;

                    if delay == 0.0 {
                        self.ratio = 0.0;
                    } else {
                        self.ratio = 0.5;
                    }
                },
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
            _ => unreachable!(),
        }
    }
}