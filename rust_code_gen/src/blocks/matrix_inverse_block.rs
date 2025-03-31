use crate::block_data::BlockData;
use crate::traits::IsValid;
use log::debug;
use utils::ParseEnumError;

use core::str::FromStr;

pub enum MatrixInverseEnum {
    Inverse,
    SVD,
}

pub struct MatrixInverseBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub method: MatrixInverseEnum,
    valid: BlockData,
}

impl FromStr for MatrixInverseEnum {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Inverse" => Ok(Self::Inverse),
            "SVD" => Ok(Self::SVD),
            _ => Err(ParseEnumError),
        }
    }
}

impl MatrixInverseBlock {
    pub fn new(name: &'static str, initial_data: &BlockData, method: &str) -> MatrixInverseBlock {
        MatrixInverseBlock {
            name,
            data: initial_data.transpose(),
            method: method.parse::<MatrixInverseEnum>().unwrap(),
            valid: BlockData::scalar_from_bool(true),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        let data = match self.method {
            MatrixInverseEnum::Inverse => input.inverse(),
            MatrixInverseEnum::SVD => input.pseudo_inverse(f64::EPSILON),
        };

        match data {
            Some(data) => {
                self.data = data;
                self.valid.set_scalar_bool(true);
            }
            None => {
                debug!("{}: Failed to perform inverse", self.name);
                self.valid.set_scalar_bool(false);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
}

impl IsValid for MatrixInverseBlock {
    fn is_valid(&self, _: f64) -> BlockData {
        self.valid.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_data::BlockDataType;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_matrix_inverse() {
        let initial_val = BlockData::new(2, 2, &[0., 0., 0., 0.]);
        let mut invert_block = MatrixInverseBlock::new("MatrixInverse", &initial_val, "Inverse");

        let input = BlockData::new(2, 2, &[1., 2., 3., 4.]);
        invert_block.run(&input);

        assert!(invert_block.data.get_type() == BlockDataType::Matrix);
        // From numpy
        assert!(invert_block.data == BlockData::new(2, 2, &[-2., 1., 1.5, -0.5]));
        assert!(invert_block.is_valid(0.1).all());
    }

    #[test]
    fn test_svd_robustness_compared_to_inverse() {
        let det_zero_input = BlockData::new(3, 3, &[1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 3.0, 6.0, 8.0]);
        let initial_val = BlockData::new(3, 1, &[0.0, 0.0, 0.0]);

        // Regular inverse method panics with an input determinant == 0.0
        let mut invert_block = MatrixInverseBlock::new("MatrixInverse", &initial_val, "Inverse");
        invert_block.run(&det_zero_input);
        assert!(!invert_block.is_valid(0.1).all());

        // SVD-based pseudo-inverse method should be fine
        let mut svd_block = MatrixInverseBlock::new("MatrixInverse", &initial_val, "SVD");
        svd_block.run(&det_zero_input);
        assert!(svd_block.is_valid(0.1).all());
    }

    #[test]
    fn test_pseudo_inverse_square_nonsingular() {
        let initial_val = BlockData::new(2, 2, &[0., 0., 0., 0.]);
        let mut invert_block = MatrixInverseBlock::new("MatrixInverse", &initial_val, "SVD");

        let matrix = BlockData::new(3, 3, &[4.0, 7.0, 2.0, 1.0, 6.0, 9.0, 5.0, 3.0, 8.0]);

        // From numpy
        let expected_inverse = BlockData::new(
            3,
            3,
            &[
                0.07266436,
                -0.17301038,
                0.17647059,
                0.12802768,
                0.07612457,
                -0.11764706,
                -0.09342561,
                0.07958478,
                0.05882353,
            ],
        );

        invert_block.run(&matrix);
        assert_abs_diff_eq!(invert_block.data, expected_inverse, epsilon = 1e-8);
        assert!(invert_block.is_valid(0.1).all());
    }

    #[test]
    fn test_pseudo_inverse_nonsquare() {
        let initial_val = BlockData::new(3, 2, &[0., 0., 0., 0., 0., 0.]);
        let mut invert_block = MatrixInverseBlock::new("MatrixInverse", &initial_val, "SVD");

        let matrix = BlockData::new(3, 2, &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);

        // From numpy
        let expected_inverse = BlockData::new(
            2,
            3,
            &[
                -0.94444444,
                -0.11111111,
                0.72222222,
                0.44444444,
                0.11111111,
                -0.22222222,
            ],
        );

        invert_block.run(&matrix);
        assert_abs_diff_eq!(invert_block.data, expected_inverse, epsilon = 1e-8);
        assert!(invert_block.is_valid(0.1).all());
    }
}
