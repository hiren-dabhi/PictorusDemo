use log::debug;

use crate::block_data::BlockData;

pub struct RustCodeBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl RustCodeBlock {
    pub fn new(name: &'static str, ic: &BlockData) -> RustCodeBlock {
        RustCodeBlock {
            name,
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, input: f64) {
        // TODO: RustCodeBlock currently just passes through solution computed
        // externally. Should eventually move that logic in here?
        self.data = BlockData::from_scalar(input);
        self.data.fix_non_finite();
        debug!("{} data: {:?}", self.name, self.data);
    }
}
