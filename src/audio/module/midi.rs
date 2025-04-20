use crate::audio::module::{Module, ModuleId, ModuleMessage, ModuleMessageUnion};
use crate::audio::sample::Sample;

#[derive(Clone, Copy, Debug)]
pub enum MidiUpdate {
    KeyPress(u8, u8),
    KeyRelease(u8),
}

pub struct Midi {
    id: ModuleId,
    gate: bool,
    note: u8,
    velocity: u8,
}

impl Module for Midi {
    fn id(&self) -> &ModuleId {
        &self.id
    }

    fn process(&mut self) -> Box<[Sample]> {
        Box::new([
            Sample::from(self.gate as u8 as f32),
            Sample::from(self.note as f32 / 127.0),
            Sample::from(self.velocity as f32 / 127.0),
        ])
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.midi} {
                MidiUpdate::KeyPress(note, velocity) => {
                  self.gate = true;
                  self.note = note;
                  self.velocity = velocity;
                },
                MidiUpdate::KeyRelease(_) => {
                    self.gate = false;
                }
            }
        }
    }

    fn modulate(&mut self, component: usize, value: Sample) {
        
    }
}

impl Midi {
    pub fn new(id: ModuleId) -> Midi {
        Self {
            id,
            gate: false,
            note: 0,
            velocity: 0,
        }
    }
}