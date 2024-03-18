use crate::types::{InputParam, FlagParam};

#[derive(Debug)]
pub struct CatOptions {
    inputs: Vec<InputParam>,
    flags: Vec<FlagParam>
}

impl CatOptions {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            flags: Vec::new()
        }
    }

    pub fn has_flag(&self, f: FlagParam) -> bool {
        self.flags.contains(&f)
    }

    pub fn add_flag(&mut self, f: FlagParam) {
        self.flags.push(f)
    }

    pub fn add_input(&mut self, i: InputParam) {
        self.inputs.push(i)
    }

    pub fn inputs(&self) -> &Vec<InputParam> {
        &self.inputs
    }

    pub fn flags(&self) -> &Vec<FlagParam> {
        &self.flags
    }
}
