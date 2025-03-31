use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
enum NotMethod {
    Logical,
    Bitwise,
}

pub struct NotBlock {
    pub name: &'static str,
    pub data: BlockData,
    method: NotMethod,
}

impl NotBlock {
    pub fn new(name: &'static str, ic: &BlockData, method: &str) -> NotBlock {
        NotBlock {
            name,
            data: ic.clone(),
            method: method.parse().unwrap(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        self.data = match self.method {
            NotMethod::Bitwise => input.component_bitnot(),
            NotMethod::Logical => input.logical_not(),
        };
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_block_logical() {
        let ic = BlockData::from_scalar(0.0);
        let mut block = NotBlock::new("Not1", &ic, "Logical");

        block.run(&BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 0.0);
        block.run(&BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), 1.0);

        block.run(&BlockData::from_scalar(-1.2));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(&BlockData::from_scalar(1.2));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(&BlockData::from_row_slice(1, 4, &[1.0, 0.0, -1.2, 1.2]));
        let expected = [0.0, 1.0, 0.0, 0.0];
        assert!(block.data.vector().iter().eq(expected.iter()));
    }

    #[test]
    fn test_not_block_bitwise() {
        let ic = BlockData::from_scalar(0.0);
        let mut block = NotBlock::new("Not1", &ic, "Bitwise");

        block.run(&BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), -2.0);
        block.run(&BlockData::from_scalar(42.0));
        assert_eq!(block.data.scalar(), -43.0);

        block.run(&BlockData::from_scalar(-1.2));
        assert_eq!(block.data.scalar(), 0.0);

        block.run(&BlockData::from_scalar(1.2));
        assert_eq!(block.data.scalar(), -2.0);

        block.run(&BlockData::from_row_slice(1, 4, &[1.0, 42.0, -1.2, 1.2]));
        assert_eq!(
            block.data,
            BlockData::from_vector(&[-2.0, -43.0, 0.0, -2.0])
        )
    }
}
