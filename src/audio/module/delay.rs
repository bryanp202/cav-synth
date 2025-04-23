use std::collections::VecDeque;

use crate::audio::module::{Module, ModuleMessage};

const MAX_DELAY_SAMPLES: usize = 48000 * 6;

#[derive(Clone, Copy, Debug)]
pub enum DelayUpdate {
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

pub struct Delay {
    id: usize,
    sample_rate: usize,
    ratio: f32,
    time: f32,
    input: Inputs,
    output: Outputs,
    // State
    buffer: VecDeque<f32>,
}

impl Delay {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            sample_rate,
            ratio: 0.5,
            time: 1.0,
            input: Inputs::default(),
            output: Outputs::default(),
            buffer: VecDeque::with_capacity(MAX_DELAY_SAMPLES),
        }
    }
}

impl Module for Delay {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        let delay_index = (self.time * self.sample_rate as f32) as usize;

        let delay_amp = self.buffer.remove(delay_index);

        self.output.value = self.input.value + self.ratio * delay_amp.unwrap_or_default();
        self.buffer.push_front(self.output.value);

        self.input.value = 0.0;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.delay} {
                DelayUpdate::Ratio(ratio) => self.ratio = ratio.min(0.99),
                DelayUpdate::Time(time) => {
                    self.time = time.min(MAX_DELAY_SAMPLES as f32 / self.sample_rate as f32);

                    let buffer_len = (self.time * self.sample_rate as f32) as usize;

                    if buffer_len < self.buffer.len() {
                        self.buffer.drain(buffer_len..self.buffer.len());
                    }

                    if time == 0.0 {
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