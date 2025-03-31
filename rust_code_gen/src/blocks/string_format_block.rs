use log::debug;

use crate::block_data::BlockData;

pub struct StringFormatBlock {
    pub name: &'static str,
    pub data: BlockData,
}

impl StringFormatBlock {
    pub fn new(name: &'static str, _: &BlockData) -> StringFormatBlock {
        StringFormatBlock {
            name,
            data: BlockData::from_bytes(b""),
        }
    }
    pub fn run(&mut self, input: &str) {
        // StringFormatBlock passes through the input data. We need to generate the format!
        // macro call because rust does not natively support dynamic format strings.
        // There are a few crates that do this, but they have limited functionality.
        // See: https://stackoverflow.com/a/32580595
        self.data.set_bytes(input.as_bytes());
        debug!("{} data: {:?}", self.name, self.data);
    }
}
