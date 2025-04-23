use crate::audio::module::Module;
use crate::audio::module::midi::Midi;

use super::module::analog::AnalogOscillator;
use super::module::butterworth::Butterworth;
use super::module::delay::Delay;
use super::module::envelope::Envelope;
use super::module::{ModuleMessage, ModuleMessageUnion};

struct Cable {
    source_module: usize,
    source_output: usize,
    target_module: usize,
    target_input: usize,
}

impl Cable {
    fn new(
        source_module: usize,
        source_output: usize,
        target_module: usize,
        target_input: usize,
    ) -> Self {
        Self {
            source_module,
            source_output,
            target_input,
            target_module,
        }
    }
}

pub struct ModTable {
    modules: Vec<Box<dyn Module>>,
    cables: Vec<Cable>,
}

impl ModTable {
    pub fn new() -> Self {
        let osc1 = Box::new(AnalogOscillator::new(1, 48000));
        let mut osc2 = Box::new(AnalogOscillator::new(2, 48000));
        let mut osc3 = Box::new(AnalogOscillator::new(3, 48000));
        let mut osc4 = Box::new(AnalogOscillator::new(4, 48000));
        let mut osc5 = Box::new(AnalogOscillator::new(5, 48000));
        let mut osc6 = Box::new(AnalogOscillator::new(6, 48000));
        let mut osc7 = Box::new(AnalogOscillator::new(7, 48000));
        let mut osc8 = Box::new(AnalogOscillator::new(8, 48000));
        let mut osc9 = Box::new(AnalogOscillator::new(9, 48000));
        let mut osc10 = Box::new(AnalogOscillator::new(10, 48000));
        let mut osc11 = Box::new(AnalogOscillator::new(11, 48000));

        osc2.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(12.0/127.0)}));
        osc3.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(19.0/127.0)}));
        osc4.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(24.0/127.0)}));
        osc5.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(28.0/127.0)}));
        osc6.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(31.0/127.0)}));
        osc7.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(43.0/127.0)}));
        osc8.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(36.0/127.0)}));
        osc9.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(48.0/127.0)}));
        osc10.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(60.0/127.0)}));
        osc11.update(ModuleMessage::ComponentChange(ModuleMessageUnion{analog: super::module::analog::AnalogOscillatorUpdate::Frequency(72.0/127.0)}));
        
        Self {
            modules: vec![
                Box::new(Midi::new(0)),
                osc1,
                osc2,
                osc3,
                osc4,
                osc5,
                osc6,
                osc7,
                osc8,
                osc9,
                osc10,
                osc11,
                Box::new(Envelope::new(12)),
                Box::new(Butterworth::new(13, 48000)),
                Box::new(Delay::new(14, 48000)),
            ],
            cables: vec![
                Cable::new(0, 0, 12, 0),
                Cable::new(0, 2, 12, 1),
                Cable::new(12, 0, 1, 0),
                Cable::new(12, 0, 13, 1),
                Cable::new(13, 0, 14, 0),
                //Cable::new(12, 0, 2, 0),
                //Cable::new(12, 0, 3, 0),
                //Cable::new(12, 0, 4, 0),
                //Cable::new(12, 0, 5, 0),
                //Cable::new(12, 0, 6, 0),
                //Cable::new(12, 0, 7, 0),
                //Cable::new(12, 0, 8, 0),
                //Cable::new(12, 0, 9, 0),
                //Cable::new(12, 0, 10, 0),
                //Cable::new(12, 0, 11, 0),
                Cable::new(0, 1, 1, 1),
                Cable::new(0, 1, 2, 1),
                Cable::new(0, 1, 3, 1),
                Cable::new(0, 1, 4, 1),
                Cable::new(0, 1, 5, 1),
                Cable::new(0, 1, 6, 1),
                Cable::new(0, 1, 7, 1),
                Cable::new(0, 1, 8, 1),
                Cable::new(0, 1, 9, 1),
                Cable::new(0, 1, 10, 1),
                Cable::new(0, 1, 11, 1),
                Cable::new(1, 0, 13, 0),
                Cable::new(2, 0, 13, 0),
                Cable::new(3, 0, 13, 0),
                Cable::new(4, 0, 13, 0),
                Cable::new(5, 0, 13, 0),
                Cable::new(6, 0, 13, 0),
                Cable::new(7, 0, 13, 0),
                Cable::new(8, 0, 13, 0),
                Cable::new(9, 0, 13, 0),
                Cable::new(10, 0, 13, 0),
                Cable::new(11, 0, 13, 0),
                Cable::new(0, 1, 13, 1),
            ],
        }
    }

    pub fn process(&mut self) -> f32 {
        self.modules.iter_mut().for_each(|module| module.process());

        self.cables.iter().for_each(|cable| {
            let output_module_index = cable.source_module;
            let output = self.modules[output_module_index].get_output(cable.source_output);

            let input_module_index = cable.target_module;
            self.modules[input_module_index].modulate(cable.target_input, output);
        });

        self.modules[14].get_output(0)
    }

    pub fn update(&mut self, id: usize, msg: ModuleMessage) {
        self.modules[id].update(msg);
    }
}