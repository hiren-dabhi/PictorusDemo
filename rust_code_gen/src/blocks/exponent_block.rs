use log::debug;

use crate::block_data::BlockData;

pub struct ExponentBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub coefficient: BlockData,
    pub preserve_sign: bool,
}

impl ExponentBlock {
    pub fn new(
        name: &'static str,
        ic: &BlockData,
        coefficient: f64,
        preserve_sign: f64,
    ) -> ExponentBlock {
        ExponentBlock {
            name,
            data: ic.clone(),
            coefficient: BlockData::scalar_sizeof(coefficient, ic),
            preserve_sign: preserve_sign != 0.0,
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        let mut val = input.clone();

        let is_negative_root = input
            .ltz()
            .and(&self.coefficient.lt(&BlockData::ones_sizeof(input)));

        if is_negative_root.sum() > 0.0 && !self.preserve_sign {
            panic!("Negative input to Exponent with coefficient < 1.0!");
        }

        // Make sure we avoid root errors by making negative datas positive first
        // Need to do this for cases where "preserve_sign" is true. Otherwise panic.
        val.component_set(&is_negative_root, &val.abs());

        val = val.powf(&self.coefficient);

        if self.preserve_sign {
            let should_flip_sign = val.sign().neq(&input.sign());
            val.component_set(&should_flip_sign, &(val.clone() * -1.0));
        }

        self.data = val;
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponent_block() {
        let preserve_sign: f64 = 0.0; // False
        let coefficient: f64 = 2.0;
        let ic = BlockData::from_vector(&[0.0, 0.0]);
        let mut block_pow2 = ExponentBlock::new("Exponent1", &ic, coefficient, preserve_sign);

        block_pow2.run(&BlockData::from_vector(&[3.0, -4.0]));
        assert_eq!(block_pow2.data, BlockData::from_vector(&[9.0, 16.0]));
    }

    #[test]
    fn test_preserve_sign() {
        let mut block_pow2_preserve =
            ExponentBlock::new("Exponent1", &BlockData::from_scalar(0.0), 2.0, 1.0);
        block_pow2_preserve.run(&BlockData::from_scalar(3.0));
        assert_eq!(block_pow2_preserve.data.scalar(), 9.0);
        block_pow2_preserve.run(&BlockData::from_scalar(-3.0));
        assert_eq!(block_pow2_preserve.data.scalar(), -9.0);
    }

    #[test]
    fn test_exponent_block_roots() {
        let mut block_sqrt2 =
            ExponentBlock::new("Exponent1", &BlockData::from_scalar(0.0), 0.5, 0.0);
        block_sqrt2.run(&BlockData::from_scalar(9.0));
        assert_eq!(block_sqrt2.data.scalar(), 3.0);
    }

    #[test]
    #[should_panic]
    fn test_root_panics_with_negative_input_and_no_sign_preservation() {
        let coefficient = 0.5;
        let preserve_sign: f64 = 0.0; // False

        let mut block_sqrt2 = ExponentBlock::new(
            "Exponent1",
            &BlockData::from_scalar(0.0),
            coefficient,
            preserve_sign,
        );
        block_sqrt2.run(&BlockData::from_scalar(-9.0));
    }
}
