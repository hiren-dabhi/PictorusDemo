use log::debug;

use crate::block_data::BlockData;

pub struct ScatterPlotBlock {
    pub value: BlockData,
}

impl ScatterPlotBlock {
    pub fn new(name: &str) -> ScatterPlotBlock {
        debug!("Creating new ScatterPlotBlock '{}'", name);
        ScatterPlotBlock {
            value: BlockData::from_scalar(0.0),
        }
    }

    pub fn add_sample(&mut self) {}
}
