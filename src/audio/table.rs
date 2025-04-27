use crate::audio::module::Module;
use crate::audio::module::midi::Midi;

use super::module::analog::AnalogOscillator;
use super::module::butterworth::Butterworth;
use super::module::chorus::Chorus;
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
        Self {
            modules: vec![
                Box::new(Midi::new(0)),
                Box::new(AnalogOscillator::new(1, 48000)),
                Box::new(AnalogOscillator::new(2, 48000)),
                Box::new(AnalogOscillator::new(3, 48000)),
                Box::new(AnalogOscillator::new(4, 48000)),
                Box::new(AnalogOscillator::new(5, 48000)),
                Box::new(AnalogOscillator::new(6, 48000)),
                Box::new(AnalogOscillator::new(7, 48000)),
                Box::new(AnalogOscillator::new(8, 48000)),
                Box::new(AnalogOscillator::new(9, 48000)),
                Box::new(AnalogOscillator::new(10, 48000)),
                Box::new(AnalogOscillator::new(11, 48000)),
                Box::new(AnalogOscillator::new(12, 48000)),
                Box::new(AnalogOscillator::new(13, 48000)),
                Box::new(AnalogOscillator::new(14, 48000)),
                Box::new(AnalogOscillator::new(15, 48000)),
                Box::new(AnalogOscillator::new(16, 48000)),
                Box::new(Envelope::new(17)),
                Box::new(Envelope::new(18)),
                Box::new(Envelope::new(19)),
                Box::new(Envelope::new(20)),
                Box::new(Envelope::new(21)),
                Box::new(Envelope::new(22)),
                Box::new(Envelope::new(23)),
                Box::new(Envelope::new(24)),
                Box::new(Envelope::new(25)),
                Box::new(Envelope::new(26)),
                Box::new(Envelope::new(27)),
                Box::new(Envelope::new(28)),
                Box::new(Envelope::new(29)),
                Box::new(Envelope::new(30)),
                Box::new(Envelope::new(31)),
                Box::new(Envelope::new(32)),
                Box::new(Butterworth::new(33, 48000)),
                Box::new(Butterworth::new(34, 48000)),
                Box::new(Butterworth::new(35, 48000)),
                Box::new(Butterworth::new(36, 48000)),
                Box::new(Butterworth::new(37, 48000)),
                Box::new(Butterworth::new(38, 48000)),
                Box::new(Butterworth::new(39, 48000)),
                Box::new(Butterworth::new(40, 48000)),
                Box::new(Butterworth::new(41, 48000)),
                Box::new(Butterworth::new(42, 48000)),
                Box::new(Butterworth::new(43, 48000)),
                Box::new(Butterworth::new(44, 48000)),
                Box::new(Butterworth::new(45, 48000)),
                Box::new(Butterworth::new(46, 48000)),
                Box::new(Butterworth::new(47, 48000)),
                Box::new(Butterworth::new(48, 48000)),
                Box::new(Chorus::new(49, 48000)),
                Box::new(Delay::new(50, 48000)),
            ],
            cables: vec![
                // Osc1
                Cable::new(0, 3, 17, 0),
                Cable::new(0, 5, 17, 1),
                Cable::new(17, 0, 1, 0),
                Cable::new(0, 4, 1, 1),
                Cable::new(1, 0, 33, 0),
                Cable::new(17, 0, 33, 1),
                Cable::new(33, 0, 49, 0),
                // Osc2
                Cable::new(0, 6, 18, 0),
                Cable::new(0, 8, 18, 1),
                Cable::new(18, 0, 2, 0),
                Cable::new(0, 7, 2, 1),
                Cable::new(2, 0, 34, 0),
                Cable::new(18, 0, 34, 1),
                Cable::new(34, 0, 49, 0),
                // Osc3
                Cable::new(0, 9, 19, 0),
                Cable::new(0, 11, 19, 1),
                Cable::new(19, 0, 3, 0),
                Cable::new(0, 10, 3, 1),
                Cable::new(3, 0, 35, 0),
                Cable::new(19, 0, 35, 1),
                Cable::new(35, 0, 49, 0),
                // Osc4
                Cable::new(0, 12, 20, 0),
                Cable::new(0, 14, 20, 1),
                Cable::new(20, 0, 4, 0),
                Cable::new(0, 13, 4, 1),
                Cable::new(4, 0, 36, 0),
                Cable::new(20, 0, 36, 1),
                Cable::new(36, 0, 49, 0),
                // Osc5
                Cable::new(0, 15, 21, 0),
                Cable::new(0, 17, 21, 1),
                Cable::new(21, 0, 5, 0),
                Cable::new(0, 16, 5, 1),
                Cable::new(5, 0, 37, 0),
                Cable::new(21, 0, 37, 1),
                Cable::new(37, 0, 49, 0),
                // Osc6
                Cable::new(0, 18, 22, 0),
                Cable::new(0, 20, 22, 1),
                Cable::new(22, 0, 6, 0),
                Cable::new(0, 19, 6, 1),
                Cable::new(6, 0, 38, 0),
                Cable::new(22, 0, 38, 1),
                Cable::new(38, 0, 49, 0),
                // Osc7
                Cable::new(0, 21, 23, 0),
                Cable::new(0, 23, 23, 1),
                Cable::new(23, 0, 7, 0),
                Cable::new(0, 22, 7, 1),
                Cable::new(7, 0, 39, 0),
                Cable::new(23, 0, 39, 1),
                Cable::new(39, 0, 49, 0),
                // Osc8
                Cable::new(0, 24, 24, 0),
                Cable::new(0, 26, 24, 1),
                Cable::new(24, 0, 8, 0),
                Cable::new(0, 25, 8, 1),
                Cable::new(8, 0, 40, 0),
                Cable::new(24, 0, 40, 1),
                Cable::new(40, 0, 49, 0),
                // Osc9
                Cable::new(0, 27, 25, 0),
                Cable::new(0, 29, 25, 1),
                Cable::new(25, 0, 9, 0),
                Cable::new(0, 28, 9, 1),
                Cable::new(9, 0, 41, 0),
                Cable::new(25, 0, 41, 1),
                Cable::new(41, 0, 49, 0),
                // Osc10
                Cable::new(0, 30, 26, 0),
                Cable::new(0, 32, 26, 1),
                Cable::new(26, 0, 10, 0),
                Cable::new(0, 31, 10, 1),
                Cable::new(10, 0, 42, 0),
                Cable::new(26, 0, 42, 1),
                Cable::new(42, 0, 49, 0),
                // Osc11
                Cable::new(0, 33, 27, 0),
                Cable::new(0, 35, 27, 1),
                Cable::new(27, 0, 11, 0),
                Cable::new(0, 34, 11, 1),
                Cable::new(11, 0, 43, 0),
                Cable::new(27, 0, 43, 1),
                Cable::new(43, 0, 49, 0),
                // Osc12
                Cable::new(0, 36, 28, 0),
                Cable::new(0, 38, 28, 1),
                Cable::new(28, 0, 12, 0),
                Cable::new(0, 37, 12, 1),
                Cable::new(12, 0, 44, 0),
                Cable::new(28, 0, 44, 1),
                Cable::new(44, 0, 49, 0),
                // Osc13
                Cable::new(0, 39, 29, 0),
                Cable::new(0, 41, 29, 1),
                Cable::new(29, 0, 13, 0),
                Cable::new(0, 40, 13, 1),
                Cable::new(13, 0, 45, 0),
                Cable::new(29, 0, 45, 1),
                Cable::new(45, 0, 49, 0),
                // Osc14
                Cable::new(0, 42, 30, 0),
                Cable::new(0, 44, 30, 1),
                Cable::new(30, 0, 14, 0),
                Cable::new(0, 43, 14, 1),
                Cable::new(14, 0, 46, 0),
                Cable::new(30, 0, 46, 1),
                Cable::new(46, 0, 49, 0),
                // Osc15
                Cable::new(0, 45, 31, 0),
                Cable::new(0, 47, 31, 1),
                Cable::new(31, 0, 15, 0),
                Cable::new(0, 46, 15, 1),
                Cable::new(15, 0, 47, 0),
                Cable::new(31, 0, 47, 1),
                Cable::new(47, 0, 49, 0),
                // Osc16
                Cable::new(0, 48, 32, 0),
                Cable::new(0, 50, 32, 1),
                Cable::new(32, 0, 16, 0),
                Cable::new(0, 49, 16, 1),
                Cable::new(16, 0, 48, 0),
                Cable::new(32, 0, 48, 1),
                Cable::new(48, 0, 49, 0),

                // Chorus to delay
                Cable::new(49, 0, 50, 0),
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

        self.modules[50].get_output(0)
    }

    pub fn update(&mut self, id: usize, msg: ModuleMessage) {
        self.modules[id].update(msg);
    }
}