use crate::audio::module::Module;
use crate::audio::module::midi::Midi;

use super::module::analog::AnalogOscillator;
use super::module::ModuleMessage;

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
                Box::new(AnalogOscillator::new(1, 48000))
            ],
            cables: vec![
                Cable::new(0, 0, 1, 0),
                Cable::new(0, 1, 1, 1),
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

        self.modules[1].get_output(0)
    }

    pub fn update(&mut self, id: usize, msg: ModuleMessage) {
        self.modules[id].update(msg);
    }
}