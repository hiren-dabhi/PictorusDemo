use log::debug;

use crate::block_data::BlockData;

pub struct EquationBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl EquationBlock {
    pub fn new(name: &'static str, ic: &BlockData) -> EquationBlock {
        EquationBlock {
            name,
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        // TODO: Equation block currently just passes through solution computed
        // externally. Should eventually move that logic in here?
        self.data = input.clone();
        self.data.fix_non_finite();
        debug!("{} data: {:?}", self.name, self.data);
    }
}
