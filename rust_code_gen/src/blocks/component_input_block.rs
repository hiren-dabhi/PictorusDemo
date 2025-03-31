use log::debug;

use crate::block_data::BlockData;

// Component Input/Output are just passthrough for now
pub struct ComponentInputBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl ComponentInputBlock {
    pub fn new(name: &'static str, ic: &BlockData) -> ComponentInputBlock {
        ComponentInputBlock {
            name,
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = input.clone();
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_input_block() {
        let ic = BlockData::from_scalar(0.0);
        let mut block = ComponentInputBlock::new("ComponentInput1", &ic);

        // Simplest of blocks - just passes along the input as output
        assert_eq!(block.data.scalar(), 0.0);
        block.run(&BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 1.0);
        block.run(&BlockData::from_scalar(-2.0));
        assert_eq!(block.data.scalar(), -2.0);
    }
}
