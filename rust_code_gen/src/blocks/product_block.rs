use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum ProductEnum {
    ComponentWise,
    MatrixMultiply,
}

pub struct ProductBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub signs: BlockData,
    pub method: ProductEnum,
}

impl ProductBlock {
    pub fn new(
        name: &'static str,
        initial_data: &BlockData,
        signs: &BlockData,
        method: &str,
    ) -> ProductBlock {
        ProductBlock {
            name,
            data: initial_data.clone(),
            signs: signs.clone(),
            method: method.parse().unwrap(),
        }
    }

    pub fn run(&mut self, inputs: &[&BlockData]) {
        match self.method {
            ProductEnum::ComponentWise => self._run_component_wise(inputs),
            ProductEnum::MatrixMultiply => self._run_matrix_multiply(inputs),
        }
        debug!("{} data: {:?}", self.name, self.data);
    }

    fn _run_component_wise(&mut self, inputs: &[&BlockData]) {
        let mut val = BlockData::ones_sizeof(&self.data);
        let eps = BlockData::scalar_sizeof(1e-50, &val); // Hack: Prevent div/0 by inserting eps
        for (idx, i) in inputs.iter().enumerate() {
            if self.signs[idx] < 0.0 {
                val /= &(*i + &eps); // Division is component-wise by default
            } else {
                val = val.component_mul(i); // Multiplication needs special function
            }
        }
        self.data = val;
    }

    fn _run_matrix_multiply(&mut self, inputs: &[&BlockData]) {
        let mut data = inputs[0].clone();
        for input in inputs.iter().skip(1) {
            data *= *input;
        }
        self.data = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use approx::assert_relative_eq;

    #[test]
    fn test_product_block_multiply() {
        let initial_data = BlockData::from_vector(&[0.0]);
        let signs = BlockData::from_vector(&[1.0, 1.0, 1.0]);
        let mut block = ProductBlock::new("ProductBlock1", &initial_data, &signs, "ComponentWise");

        let signal1 = BlockData::from_scalar(2.0);
        let signal2 = BlockData::from_scalar(3.0);
        let signal3 = BlockData::from_scalar(4.0);
        let inputs = vec![&signal1, &signal2, &signal3];

        block.run(&inputs);
        assert_eq!(block.data.scalar(), 24.0);

        let signal4 = BlockData::from_scalar(-3.0);
        let inputs_neg = vec![&signal1, &signal4, &signal3];
        block.run(&inputs_neg);
        assert_eq!(block.data.scalar(), -24.0);
    }

    #[test]
    fn test_product_block_divide() {
        let initial_data = BlockData::from_vector(&[0.0]);
        let signs = BlockData::from_vector(&[1.0, -1.0]);
        let mut block = ProductBlock::new("ProductBlock1", &initial_data, &signs, "ComponentWise");

        let signal1 = BlockData::from_scalar(2.0);
        let signal2 = BlockData::from_scalar(3.0);
        let inputs = vec![&signal1, &signal2];
        block.run(&inputs);

        assert_relative_eq!(block.data.scalar(), 0.667, max_relative = 0.01);
    }

    #[test]
    fn test_product_block_divide_by_zero() {
        let initial_data = BlockData::from_vector(&[0.0]);
        let signs = BlockData::from_vector(&[1.0, -1.0]);
        let mut block = ProductBlock::new("ProductBlock1", &initial_data, &signs, "ComponentWise");

        let signal1 = BlockData::from_scalar(2.0);
        let signal2 = BlockData::from_scalar(0.0);
        let inputs = vec![&signal1, &signal2];
        block.run(&inputs);

        assert_relative_eq!(block.data.scalar(), 2e50, max_relative = 0.01);
    }
}
