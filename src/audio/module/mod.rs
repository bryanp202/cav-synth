pub mod analog;
pub mod midi;

use crate::audio::sample::Sample;

pub trait Module {
    fn id(&self) -> &ModuleId;

    fn process(&mut self) -> Box<[Sample]>;

    fn update(&mut self, msg: ModuleMessage);

    fn modulate(&mut self, component: usize, value: Sample);
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleId {
    num: usize,
}

impl From<usize> for ModuleId {
    fn from(value: usize) -> Self {
        Self {
            num: value,
        }
    }
}

impl Into<usize> for ModuleId {
    fn into(self) -> usize {
        self.num
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ModuleMessage {
    ComponentChange(ModuleMessageUnion),
}

#[derive(Copy, Clone)]
pub union ModuleMessageUnion {
    analog: analog::AnalogOscillatorUpdate,
    midi: midi::MidiUpdate,
}

impl std::fmt::Debug for ModuleMessageUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleMessageUnion")
    }
}