use crate::audio::module::{Module, ModuleId};
use crate::audio::module::midi::Midi;
use crate::audio::sample::Sample;

use super::module::ModuleMessage;

struct Cable {
    source_module: ModuleId,
    source_output: usize,
    target_module: ModuleId,
    target_input: usize,
}

pub struct ModTable {
    module_count: usize,
    modules: Vec<Box<dyn Module>>,
    outputs: Vec<Box<[Sample]>>,
    cables: Vec<Cable>,
}

impl ModTable {
    pub fn new() -> Self {
        Self {
            module_count: 1,
            modules: vec![Box::new(Midi::new(ModuleId::from(1)))],
            outputs: Vec::new(),
            cables: Vec::new(),
        }
    }

    pub fn process(&mut self) -> Sample {
        self.outputs = Vec::with_capacity(self.modules.len());

        for module in &mut self.modules {
            let output = module.process();
            self.outputs.push(output);
        }

        for cable in &self.cables {
            let output_module_index: usize = cable.source_module.into();
            let output = self.outputs[output_module_index][cable.source_output];

            let input_module_index: usize = cable.target_module.into();
            self.modules[input_module_index].modulate(cable.target_input, output);
        }

        self.outputs[self.outputs.len() - 1][0]
    }

    pub fn update(&mut self, id: ModuleId, msg: ModuleMessage) {
        let id: usize = id.into();
        self.modules[id].update(msg);
    }
}