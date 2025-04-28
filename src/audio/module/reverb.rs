use crate::audio::module::{Module, ModuleMessage};

use super::{allpass::Allpass, butterworth::Butterworth, comb::Comb, lfo::Lfo};

#[derive(Clone, Copy, Debug)]
pub enum ReverbUpdate {
    Wet(f32),
}

#[derive(Default)]
struct Inputs {
    value: f32,
}

#[derive(Default)]
struct Outputs {
    value: f32,
}

pub struct Reverb {
    id: usize,
    input: Inputs,
    output: Outputs,
    wet: f32,
    // State
    allpass: [Allpass; 3],
    lp: Butterworth,
    lfo: Lfo,
    combs: [Comb; 4],
}

impl Reverb {
    pub fn new(id: usize, sample_rate: usize) -> Self {
        Self {
            id,
            input: Inputs::default(),
            output: Outputs::default(),
            wet: 0.5,
            allpass: [
                Allpass::new(id, 0.7, 400),
                Allpass::new(id, 0.7, 200),
                Allpass::new(id, 0.7, 80),
            ],
            lp: Butterworth::new(id, sample_rate).cutoff(10000.0),
            lfo: Lfo::new(id, sample_rate).frequency(0.06),
            combs: [
                Comb::new(id, 0.783, 1835),
                Comb::new(id, 0.771, 2133),
                Comb::new(id, 0.763, 1478),
                Comb::new(id, 0.743, 1911),
            ],
        }
    }
}

impl Module for Reverb {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        self.output.value = self.input.value * (1.0 - self.wet);

        for ap in &mut self.allpass {
            ap.process();
        }
        self.allpass[0].modulate(0, self.input.value);
        self.allpass[1].modulate(0, self.allpass[0].get_output(0));
        self.allpass[2].modulate(0, self.allpass[1].get_output(0));

        let allpass_out = self.allpass[2].get_output(0);
        self.lp.modulate(0, allpass_out);
        self.lp.process();

        let lp_out = self.lp.get_output(0);

        self.lfo.process();
        let lfo_out = self.lfo.get_output(0);

        let mut wet_total = 0.0;
        for comb in &mut self.combs {
            comb.process();
            comb.modulate(0, lp_out);
            comb.modulate(1, lfo_out);
            wet_total += comb.get_output(0) * 0.25;
        }

        self.output.value += wet_total * self.wet;
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.reverb} {
                ReverbUpdate::Wet(wet) => self.wet = wet,
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