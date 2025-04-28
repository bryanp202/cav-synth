use std::collections::VecDeque;

use crate::audio::module::{Module, ModuleMessage};

#[derive(Clone, Copy, Debug)]
pub enum AllpassUpdate {
    Gain(f32),
    DelayTime(usize),
}

#[derive(Default)]
struct Inputs {
    value: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct Allpass {
    id: usize,
    gain: f32,
    delay_time: usize,
    input: Inputs,
    output: Outputs,
    // State
    buffer: VecDeque<(f32, f32)>,
}

impl Allpass {
    pub fn new(id: usize, gain: f32, delay_time: usize) -> Self {
        Self {
            id,
            gain,
            delay_time,
            input: Inputs::default(),
            output: Outputs::default(),
            buffer: VecDeque::with_capacity(delay_time),
        }
    }
}

impl Module for Allpass {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        let x;
        if let Some((delay, feedback)) = self.buffer.remove(self.delay_time) {
            x = -self.gain * self.input.value + delay + self.gain * feedback;
        } else {
            x = 0.0;
        }

        self.buffer.push_front((self.input.value, x));
        self.output.value = x;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.allpass} {
                AllpassUpdate::Gain(gain) => self.gain = gain,
                AllpassUpdate::DelayTime(delay_time) => self.delay_time = delay_time,
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
            _ => unreachable!(),
        }
    }
}