use crate::audio::module::{Module, ModuleMessage};

#[derive(Clone, Copy, Debug)]
pub enum MidiUpdate {
    KeyPress(u8, u8),
    KeyRelease(u8),
}

pub struct Midi {
    id: usize,
    gate: f32,
    note: f32,
    velocity: f32,
}

impl Module for Midi {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {}

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.midi} {
                MidiUpdate::KeyPress(note, velocity) => {
                    self.gate = 1.0;
                    self.note = note as f32 / 127.0;
                    self.velocity = velocity as f32 / 127.0;
                },
                MidiUpdate::KeyRelease(note) => {
                    if self.note == note as f32 / 127.0 {
                        self.gate = 0.0;
                    }
                }
            }
        }
    }

    fn get_output(&self, target_output: usize) -> f32 {
        match target_output {
            0 => self.gate,
            1 => self.note,
            2 => self.velocity,
            _ => unreachable!(),
        }
    }

    fn modulate(&mut self, component: usize, value: f32) {}
}

impl Midi {
    pub fn new(id: usize) -> Midi {
        Self {
            id,
            gate: 0.0,
            note: 0.0,
            velocity: 0.0,
        }
    }
}