use corelib_traits::GeneratorBlock;
use utils::BlockData;

pub struct Parameters<const CHARS: usize> {
    pub value: [u8; CHARS],
}

impl<const CHARS: usize> Parameters<CHARS> {
    pub fn new(input: [u8; CHARS]) -> Self {
        Self { value: input }
    }
}

pub struct BytesLiteralBlock<const CHARS: usize> {
    buffer: [u8; CHARS],
    pub data: BlockData,
}

impl<const CHARS: usize> Default for BytesLiteralBlock<CHARS> {
    fn default() -> Self {
        Self {
            data: BlockData::from_bytes(&[0; CHARS]),
            buffer: [0; CHARS],
        }
    }
}

impl<const CHARS: usize> GeneratorBlock for BytesLiteralBlock<CHARS> {
    type Output = [u8];
    type Parameters = Parameters<CHARS>;

    fn generate(
        &mut self,
        parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
    ) -> corelib_traits::PassBy<Self::Output> {
        self.data = BlockData::from_bytes(&parameters.value);
        self.buffer = parameters.value;
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::time::Duration;
    use corelib_traits_testing::StubContext;
    use std::string::String;
    use utils::ToPass;

    #[test]
    fn test_constant_block() {
        let mut block = BytesLiteralBlock::<11>::default();

        let bytes_literal_ic = BlockData::from_bytes(String::from("Hello World").as_bytes());

        let parameters = Parameters::new(bytes_literal_ic.to_pass());
        let context = StubContext::new(Duration::from_secs(0), Duration::from_millis(100));

        let output = block.generate(&parameters, &context);
        assert_eq!(output, "Hello World".as_bytes());
        assert_eq!(block.data, BlockData::from_bytes("Hello World".as_bytes()));
    }
}
