use log::debug;
use num_traits::Float;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum TrigonometryEnum {
    Sine,
    Cosine,
    Tangent,
    ArcSine,
    ArcCosine,
    ArcTangent,
    SineHyperbolic,
    CosineHyperbolic,
    TangentHyperbolic,
    ArcSineHyperbolic,
    ArcCosineHyperbolic,
    ArcTangentHyperbolic,
}

pub struct TrigonometryBlock {
    pub name: &'static str,
    pub method: TrigonometryEnum,
    pub data: BlockData,
}

impl TrigonometryBlock {
    pub fn new(name: &'static str, ic: &BlockData, method: &str) -> TrigonometryBlock {
        TrigonometryBlock {
            name,
            method: method.parse().unwrap(),
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        match self.method {
            TrigonometryEnum::Sine => {
                self.data = input.map(Float::sin);
            }
            TrigonometryEnum::Cosine => {
                self.data = input.map(Float::cos);
            }
            TrigonometryEnum::Tangent => {
                self.data = input.map(Float::tan);
            }
            TrigonometryEnum::ArcSine => {
                self.data = input.map(Float::asin);
            }
            TrigonometryEnum::ArcCosine => {
                self.data = input.map(Float::acos);
            }
            TrigonometryEnum::ArcTangent => {
                self.data = input.map(Float::atan);
            }
            TrigonometryEnum::SineHyperbolic => {
                self.data = input.map(Float::sinh);
            }
            TrigonometryEnum::CosineHyperbolic => {
                self.data = input.map(Float::cosh);
            }
            TrigonometryEnum::TangentHyperbolic => {
                self.data = input.map(Float::tanh);
            }
            TrigonometryEnum::ArcSineHyperbolic => {
                self.data = input.map(Float::asinh);
            }
            TrigonometryEnum::ArcCosineHyperbolic => {
                self.data = input.map(Float::acosh);
            }
            TrigonometryEnum::ArcTangentHyperbolic => {
                self.data = input.map(Float::atanh);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;
    use rstest::rstest;

    #[rstest]
    #[case::sin_0("Sine", 0.0, 0.0)]
    #[case::sin_pi_2("Sine", PI / 2.0, 1.0)]
    #[case::cos_0("Cosine", 0.0, 1.0)]
    #[case::cos_pi_2("Cosine", PI / 2.0, 0.0)]
    #[case::tan_0("Tangent", 0.0, 0.0)]
    #[case::tan_pi_4("Tangent", PI / 4.0, 1.0)]
    #[case::asin_0("ArcSine", 0.0, 0.0)]
    #[case::asin_1("ArcSine", 1.0, PI / 2.0)]
    #[case::acos_1("ArcCosine", 1.0, 0.0)]
    #[case::acos_0("ArcCosine", 0.0, PI / 2.0)]
    #[case::atan_0("ArcTangent", 0.0, 0.0)]
    #[case::atan_1("ArcTangent", 1.0, PI / 4.0)]
    #[case::sinh_0("SineHyperbolic", 0.0, 0.0)]
    #[case::sinh_1("SineHyperbolic", 1.0, 1.17520)]
    #[case::cosh_0("CosineHyperbolic", 0.0, 1.0)]
    #[case::cosh_1("CosineHyperbolic", 1.0, 1.54308)]
    #[case::tanh_0("TangentHyperbolic", 0.0, 0.0)]
    #[case::tanh_1("TangentHyperbolic", 1.0, 0.76159)]
    #[case::asinh_0("ArcSineHyperbolic", 0.0, 0.0)]
    #[case::asinh_1_17520("ArcSineHyperbolic", 1.17520, 1.0)]
    #[case::acosh_1("ArcCosineHyperbolic", 1.0, 0.0)]
    #[case::acosh_1_54308("ArcCosineHyperbolic", 1.54308, 1.0)]
    #[case::atanh_0("ArcTangentHyperbolic", 0.0, 0.0)]
    #[case::atanh_0_76159("ArcTangentHyperbolic", 0.76159, 1.0)]
    fn test_trig_functions(#[case] name: &'static str, #[case] input: f64, #[case] expected: f64) {
        let ic = BlockData::from_scalar(0.0);
        let mut block = TrigonometryBlock::new(name, &ic, name);

        block.run(&BlockData::from_scalar(input));
        assert_relative_eq!(block.data.scalar(), expected, max_relative = 0.00001);
    }

    #[test]
    fn test_trigonometry_block_vectorized() {
        let ic = BlockData::from_vector(&[0.0, 0.0]);
        let mut sine_block = TrigonometryBlock::new("Trig1", &ic, "Sine");

        sine_block.run(&BlockData::from_vector(&[0.0, PI / 2.0]));
        assert_relative_eq!(
            sine_block.data,
            BlockData::from_vector(&[0.0, 1.0]),
            max_relative = 0.00001
        );
    }
}
