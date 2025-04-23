use std::time::Instant;

use crate::audio::module::{Module, ModuleMessage};

#[derive(Clone, Copy, Debug)]
pub enum EnvelopeUpdate {
    Attack(f32),
    Decay(f32),
    Release(f32),
    Sustain(f32),
}

#[derive(Default)]
struct Inputs {
    gate: f32,
    velocity: f32,
    attack: f32,
    decay: f32,
    release: f32,
    sustain: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct Envelope {
    id: usize,
    start: Option<Instant>,
    start_value: f32,
    released: Option<Instant>,
    release_start_value: f32,
    attack: f32,
    decay: f32,
    release: f32,
    sustain: f32,
    input: Inputs,
    output: Outputs,
}

impl Envelope {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            start: None,
            start_value: 0.0,
            released: None,
            release_start_value: 0.0,
            attack: 0.006,
            decay: 1.2,
            release: 0.1,
            sustain: 0.0,
            input: Inputs::default(),
            output: Outputs::default(),
        }
    }
}

impl Module for Envelope {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        if let Some(start_time) = self.start {
            let elapsed = start_time.elapsed().as_secs_f32();
            if elapsed < self.attack {
                self.output.value = self.start_value + (1.0 - self.start_value) * elapsed / self.attack;
            } else {
                let since_decay = elapsed - self.attack;
                let peak_sustain_delta = 1.0 - self.sustain;

                let raw = 1.0 - peak_sustain_delta * since_decay / self.decay;

                self.output.value = raw.max(self.sustain);
            }
        } else if let Some(released_time) = self.released {
            let elapsed = released_time.elapsed().as_secs_f32();

            let raw = self.release_start_value * (1.0 - elapsed / self.release);
            self.output.value = raw.max(0.0);
        }
    }

    fn get_output(&self, target_output: usize) -> f32 {
        match target_output {
            0 => self.output.value * self.input.velocity,
            _ => unreachable!(),
        }
    }

    fn modulate(&mut self, component: usize, value: f32) {
        match component {
            0 => {
                self.input.gate = value;
                if self.input.gate != 0.0 {
                    if let None = self.start {
                        self.start = Some(Instant::now());
                        self.released = None;
                        self.start_value = self.output.value;
                    }
                } else {
                    if let None = self.released {
                        self.start = None;
                        self.released = Some(Instant::now());
                        self.release_start_value = self.output.value;
                    }
                }
            },
            1 => self.input.velocity = value,
            2 => self.input.attack = value,
            3 => self.input.decay = value,
            4 => self.input.release = value,
            5 => self.input.sustain = value,
            _ => unreachable!(),
        }
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.envelope} {
                EnvelopeUpdate::Attack(attack) => self.attack = attack,
                EnvelopeUpdate::Decay(decay) => self.decay = decay,
                EnvelopeUpdate::Release(release) => self.release = release,
                EnvelopeUpdate::Sustain(sustain) => self.sustain = sustain,
            }
        }
    }
}