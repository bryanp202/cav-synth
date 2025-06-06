pub mod allpass;
pub mod analog;
pub mod butterworth;
pub mod comb;
pub mod chorus;
pub mod delay;
pub mod envelope;
pub mod lfo;
pub mod midi;
pub mod reverb;

pub trait Module {
    fn id(&self) -> usize;

    fn process(&mut self);

    fn update(&mut self, msg: ModuleMessage);

    fn get_output(&self, target_output: usize) -> f32;

    fn modulate(&mut self, component: usize, value: f32);
}

#[derive(Copy, Clone, Debug)]
pub enum ModuleMessage {
    ComponentChange(ModuleMessageUnion),
}

#[derive(Copy, Clone)]
pub union ModuleMessageUnion {
    pub allpass: allpass::AllpassUpdate,
    pub analog: analog::AnalogOscillatorUpdate,
    pub butterworth: butterworth::ButterworthUpdate,
    pub comb: comb::CombUpdate,
    pub chorus: chorus::ChorusUpdate,
    pub delay: delay::DelayUpdate,
    pub envelope: envelope::EnvelopeUpdate,
    pub midi: midi::MidiUpdate,
    pub lfo: lfo::LfoUpdate,
    pub reverb: reverb::ReverbUpdate,
}

impl std::fmt::Debug for ModuleMessageUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleMessageUnion")
    }
}