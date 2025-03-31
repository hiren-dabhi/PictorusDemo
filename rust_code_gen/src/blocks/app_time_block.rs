use log::debug;

use crate::block_data::BlockData;

pub struct AppTimeBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl AppTimeBlock {
    pub fn new(name: &'static str) -> AppTimeBlock {
        AppTimeBlock {
            name,
            data: BlockData::from_scalar(0.0),
        }
    }
    pub fn run(&mut self, time: f64) {
        self.data.set_scalar(time);
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_time_block() {
        let mut block = AppTimeBlock::new("AppTime1");

        block.run(0.0);
        assert_eq!(block.data.scalar(), 0.0);

        block.run(1.0);
        assert_eq!(block.data.scalar(), 1.0);
    }
}
