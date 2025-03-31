use crate::block_data::{BlockData, BlockDataType};
use alloc::format;
use log::debug;
use protocols::DisplayProtocol;

pub struct CharacterDisplayBlock {
    name: &'static str,
    pub x_offset: f64,
}
impl CharacterDisplayBlock {
    pub fn new(name: &'static str, x_offset: f64) -> CharacterDisplayBlock {
        CharacterDisplayBlock {
            name,
            x_offset,
        }
    }
    pub fn run(&mut self, input: &BlockData, proto: &mut impl DisplayProtocol) {
        debug!("{}: {:?}", self.name, input);
        let display_value = match input.get_type() {
            BlockDataType::Scalar => CharacterDisplayBlock::format_scalar(input.scalar()),
            BlockDataType::BytesArray => input.raw_string(),
            _ => input.stringify(),
        };

        proto.render(&display_value, self.x_offset);
    }

    fn format_scalar(value: f64) -> String {
        format!("{:.2}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use protocols::MockDisplayProtocol;

    #[test]
    fn test_character_display_block_writes_scalar_data() {
        let mut proto = MockDisplayProtocol::new();
        let input = 255.;
        let offset = 1.0;
        proto
            .expect_render()
            .with(eq("255.00"), eq(offset))
            .return_const(());

        let mut block = CharacterDisplayBlock::new("Display1", offset);
        block.run(&BlockData::from_scalar(input), &mut proto);
    }

    #[test]
    fn test_character_display_block_writes_byte_data() {
        let mut proto = MockDisplayProtocol::new();
        let input = "foo bar";
        let offset = 0.0;
        proto
            .expect_render()
            .with(eq(input), eq(offset))
            .return_const(());

        let mut block = CharacterDisplayBlock::new("Display1", offset);
        block.run(&BlockData::from_bytes(input.as_bytes()), &mut proto);
    }
}
