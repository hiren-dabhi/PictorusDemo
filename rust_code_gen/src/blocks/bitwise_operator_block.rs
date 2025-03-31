use crate::block_data::BlockData;

#[derive(Debug, strum::EnumString)]
enum OperatorType {
    And,
    Or,
    Xor,
}

pub struct BitwiseOperatorBlock {
    pub name: &'static str,
    pub data: BlockData,
    method: OperatorType,
}

impl BitwiseOperatorBlock {
    pub fn new(name: &'static str, ic: &BlockData, method: &str) -> BitwiseOperatorBlock {
        BitwiseOperatorBlock {
            name,
            data: ic.clone(),
            method: method.parse().unwrap(),
        }
    }
    pub fn run(&mut self, inputs: &[&BlockData]) {
        self.data = inputs
            .iter()
            .skip(1)
            .fold(inputs[0].clone(), |acc, &e| match self.method {
                OperatorType::And => &acc & e,
                OperatorType::Or => &acc | e,
                OperatorType::Xor => &acc ^ e,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_and() {
        let mut block = BitwiseOperatorBlock::new("bitwise", &BlockData::from_scalar(0.), "And");
        block.run(&[
            &BlockData::from_scalar(1.),
            &BlockData::from_scalar(0.),
            &BlockData::from_scalar(1.),
            &BlockData::from_scalar(0.),
        ]);
        assert_eq!(block.data, BlockData::from_scalar(0.));
    }

    #[test]
    fn test_run_or() {
        let mut block = BitwiseOperatorBlock::new("bitwise", &BlockData::from_scalar(0.), "Or");
        block.run(&[
            &BlockData::from_scalar(1.),
            &BlockData::from_scalar(0.),
            &BlockData::from_scalar(1.),
            &BlockData::from_scalar(0.),
        ]);
        assert_eq!(block.data, BlockData::from_scalar(1.));
    }

    #[test]
    fn test_run_xor() {
        let mut block = BitwiseOperatorBlock::new("bitwise", &BlockData::from_scalar(0.), "Xor");
        block.run(&[
            &BlockData::from_scalar(1.),
            &BlockData::from_scalar(0.),
            &BlockData::from_scalar(1.),
            &BlockData::from_scalar(0.),
        ]);
        assert_eq!(block.data, BlockData::from_scalar(0.));
    }
}
