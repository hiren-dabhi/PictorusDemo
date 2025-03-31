use crate::block_data::BlockData;

pub struct HistogramPlotBlock {
    pub name: &'static str,
    pub value: BlockData,
}

impl HistogramPlotBlock {
    pub fn new(name: &'static str) -> HistogramPlotBlock {
        HistogramPlotBlock {
            name,
            value: BlockData::from_scalar(0.0),
        }
    }

    pub fn add_sample(&mut self) {
        // TODO: Do we want to switch to storing values here for telemetry?
    }
}
