use std::collections::VecDeque;

use crate::audio::module::{Module, ModuleMessage};

use super::delay;

#[derive(Clone, Copy, Debug)]
pub enum CombUpdate {
    Gain(f32),
    DelayTime(usize),
}

#[derive(Default)]
struct Inputs {
    value: f32,
    delay: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct Comb {
    id: usize,
    gain: f32,
    delay_time: usize,
    input: Inputs,
    output: Outputs,
    // State
    buffer: VecDeque<f32>,
}

impl Comb {
    pub fn new(id: usize, gain: f32, delay_time: usize) -> Self {
        Self {
            id,
            gain,
            delay_time,
            input: Inputs::default(),
            output: Outputs::default(),
            buffer: VecDeque::with_capacity(delay_time + 40),
        }
    }
}

impl Module for Comb {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        let x;
        if let Some(feedback) = self.buffer.get(self.delay_time + (20.0 + self.input.delay * 20.0) as usize) {
            x = self.input.value + self.gain * feedback;
        } else {
            x = 0.0;
        }
        self.buffer.remove(self.delay_time + 40);

        self.buffer.push_front(x);
        self.output.value = x;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.comb} {
                CombUpdate::DelayTime(delay_time) => self.delay_time = delay_time,
                CombUpdate::Gain(gain) => self.gain = gain,
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
            0 => self.input.value = value,
            1 => self.input.delay = value,
            _ => unreachable!(),
        }
    }
}