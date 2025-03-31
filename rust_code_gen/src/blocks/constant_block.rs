use log::debug;

use crate::block_data::BlockData;

pub struct ConstantBlock {
    pub data: BlockData,
}

impl ConstantBlock {
    pub fn new(name: &str, initial_data: &BlockData) -> ConstantBlock {
        debug!("{} data: {:?}", name, initial_data);
        ConstantBlock {
            data: initial_data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_block() {
        let _block = ConstantBlock::new("Constant1", &BlockData::from_scalar(1.2345));
        assert_eq!(_block.data.scalar(), 1.2345);
    }
}
