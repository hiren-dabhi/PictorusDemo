use log::debug;

use crate::block_data::BlockData;

pub struct DeadbandBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub lower_limit: BlockData,
    pub upper_limit: BlockData,
}

impl DeadbandBlock {
    pub fn new(
        name: &'static str,
        ic: &BlockData,
        lower_limit: f64,
        upper_limit: f64,
    ) -> DeadbandBlock {
        DeadbandBlock {
            name,
            data: ic.clone(),
            lower_limit: BlockData::scalar_sizeof(lower_limit, ic),
            upper_limit: BlockData::scalar_sizeof(upper_limit, ic),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = input.clone();
        let in_deadband = self
            .data
            .lt(&self.upper_limit)
            .and(&self.data.gt(&self.lower_limit));
        self.data
            .component_set(&in_deadband, &BlockData::zeros_sizeof(&self.data));
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadband_block() {
        let ic = BlockData::from_scalar(0.0);
        let lower_limit = -1.0;
        let upper_limit = 1.0;
        let mut block = DeadbandBlock::new("Deadband1", &ic, lower_limit, upper_limit);

        // Anything exactly at the deadband limits maintains data
        block.run(&BlockData::from_scalar(-1.0));
        assert_eq!(block.data.scalar(), -1.0);
        block.run(&BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 1.0);

        // Anything between the deadband is set to zero.
        block.run(&BlockData::from_scalar(-0.99999999));
        assert_eq!(block.data.scalar(), 0.0);
        block.run(&BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 0.0);
        block.run(&BlockData::from_scalar(0.99999));
        assert_eq!(block.data.scalar(), 0.0);

        // Anything way outside the deadband maintains data
        block.run(&BlockData::from_scalar(-1000.0));
        assert_eq!(block.data.scalar(), -1000.0);
        block.run(&BlockData::from_scalar(1000.0));
        assert_eq!(block.data.scalar(), 1000.0);
    }
}
