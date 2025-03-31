use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum ChangeDetectionEnum {
    Any,
    Rising,
    Falling,
}

pub struct ChangeDetectionBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub method: ChangeDetectionEnum,
    pub last_input: BlockData,
}

impl ChangeDetectionBlock {
    pub fn new(name: &'static str, ic: &BlockData, method: &str) -> ChangeDetectionBlock {
        ChangeDetectionBlock {
            method: method.parse().unwrap(),
            name,
            data: ic.clone(),
            last_input: ic.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        match self.method {
            ChangeDetectionEnum::Any => {
                self.data = input.neq(&self.last_input);
            }
            ChangeDetectionEnum::Rising => {
                self.data = input.gt(&self.last_input);
            }
            ChangeDetectionEnum::Falling => {
                self.data = input.lt(&self.last_input);
            }
        }
        self.last_input = input.clone();
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_detection_any() {
        let ic = BlockData::from_vector(&[0.0, 0.0, 0.0]);
        let mut block = ChangeDetectionBlock::new("ChangeDetection1", &ic, "Any");

        block.run(&BlockData::from_vector(&[0.0, 0.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.0, 0.0]));

        block.run(&BlockData::from_vector(&[1.0, -1.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[1.0, 1.0, 0.0]));

        block.run(&BlockData::from_vector(&[1.0, -1.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.0, 0.0]));
    }
    #[test]
    fn test_change_detection_rising() {
        let ic = BlockData::from_vector(&[0.0, 0.0, 0.0]);
        let mut block = ChangeDetectionBlock::new("ChangeDetection1", &ic, "Rising");

        block.run(&BlockData::from_vector(&[0.0, 0.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.0, 0.0]));

        block.run(&BlockData::from_vector(&[1.0, -1.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[1.0, 0.0, 0.0]));

        block.run(&BlockData::from_vector(&[1.0, -1.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.0, 0.0]));
    }
    #[test]
    fn test_change_detection_falling() {
        let ic = BlockData::from_vector(&[0.0, 0.0, 0.0]);
        let mut block = ChangeDetectionBlock::new("ChangeDetection1", &ic, "Falling");

        block.run(&BlockData::from_vector(&[0.0, 0.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.0, 0.0]));

        block.run(&BlockData::from_vector(&[1.0, -1.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 1.0, 0.0]));

        block.run(&BlockData::from_vector(&[1.0, -1.0, 0.0]));
        assert_eq!(block.data, BlockData::from_vector(&[0.0, 0.0, 0.0]));
    }
}
