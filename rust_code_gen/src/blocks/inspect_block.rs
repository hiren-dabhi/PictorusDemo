use log::debug;

use crate::block_data::BlockData;

pub struct InspectBlock {
    pub value: BlockData,
}

impl InspectBlock {
    pub fn new(name: &str) -> InspectBlock {
        debug!("Creating new InspectBlock '{}'", name);
        InspectBlock {
            value: BlockData::from_scalar(0.0),
        }
    }

    pub fn add_sample(&mut self) {}
}
