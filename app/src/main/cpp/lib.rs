extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use core::time::Duration;
use corelib_traits::{GeneratorBlock, Matrix, ProcessBlock};
use pictorus_core_blocks::{
    AggregateBlock, ArgMinMaxBlock, CompareToValueBlock, ComparisonBlock, ConstantBlock,
    CounterBlock, GainBlock, SumBlock,
};
use rust_code_gen::block_data::{BlockData, ToPass};
use rust_code_gen::blocks::{
    ComponentInputBlock, ComponentOutputBlock, DataReadBlock, DataWriteBlock, DelayBlock,
    EquationBlock, LogicalBlock, ProductBlock, VectorMergeBlock, VectorSliceBlock,
};
use rust_code_gen::data_logger::DataLogger;
use rust_code_gen::utils::{
    get_diagram_params, get_pictorus_vars, load_ic, load_param, s_to_us, us_to_s, PictorusError,
    PictorusVars,
};

pub fn compile_info() -> &'static str {
    return "cras_h_67e3ea9a6a4093c50166013a version : compiled 04/03/2025 - 05:15:56";
}

#[derive(Debug, Clone)]
pub enum State {
    Main6013bState,
}

pub struct Component2c4cdfComponent {
    last_time_s: f64,
    component_input1_c4ce0: ComponentInputBlock,
    constant22_c4d61_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant22_c4d61: ConstantBlock<f64>,
    constant7_c4cf7_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant7_c4cf7: ConstantBlock<f64>,
    overall_lower_bucket_c4ce3: ComponentInputBlock,
    vector_slice11_c4cf4: VectorSliceBlock,
    aggregate14_c4cf5_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate14_c4cf5: AggregateBlock<Matrix<1, 5, f64>>,
    product5_c4cf6: ProductBlock,
    equation5_c4d3b: EquationBlock,
    sum18_c4d3a_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum18_c4d3a: SumBlock<(f64, f64)>,
    overall_mid_bucket_c4ce4: ComponentInputBlock,
    vector_slice17_c4d0c: VectorSliceBlock,
    aggregate20_c4d0d_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate20_c4d0d: AggregateBlock<Matrix<1, 5, f64>>,
    constant13_c4d0f_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant13_c4d0f: ConstantBlock<f64>,
    product11_c4d0e: ProductBlock,
    equation11_c4d47: EquationBlock,
    sum24_c4d46_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum24_c4d46: SumBlock<(f64, f64)>,
    constant19_c4d27_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant19_c4d27: ConstantBlock<f64>,
    overall_upper_bucket_c4ce5: ComponentInputBlock,
    vector_slice23_c4d24: VectorSliceBlock,
    aggregate26_c4d25_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate26_c4d25: AggregateBlock<Matrix<1, 5, f64>>,
    product17_c4d26: ProductBlock,
    equation17_c4d53: EquationBlock,
    sum30_c4d52_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum30_c4d52: SumBlock<(f64, f64)>,
    sum36_c4d5a_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum36_c4d5a: SumBlock<(f64, f64, f64)>,
    vector_slice10_c4cf0: VectorSliceBlock,
    aggregate13_c4cf1_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate13_c4cf1: AggregateBlock<Matrix<1, 5, f64>>,
    constant6_c4cf3_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant6_c4cf3: ConstantBlock<f64>,
    product4_c4cf2: ProductBlock,
    equation4_c4d39: EquationBlock,
    sum17_c4d38_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum17_c4d38: SumBlock<(f64, f64)>,
    vector_slice16_c4d08: VectorSliceBlock,
    aggregate19_c4d09_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate19_c4d09: AggregateBlock<Matrix<1, 5, f64>>,
    constant12_c4d0b_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant12_c4d0b: ConstantBlock<f64>,
    product10_c4d0a: ProductBlock,
    equation10_c4d45: EquationBlock,
    sum23_c4d44_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum23_c4d44: SumBlock<(f64, f64)>,
    vector_slice22_c4d20: VectorSliceBlock,
    aggregate25_c4d21_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate25_c4d21: AggregateBlock<Matrix<1, 5, f64>>,
    constant18_c4d23_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant18_c4d23: ConstantBlock<f64>,
    product16_c4d22: ProductBlock,
    equation16_c4d51: EquationBlock,
    sum29_c4d50_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum29_c4d50: SumBlock<(f64, f64)>,
    sum35_c4d59_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum35_c4d59: SumBlock<(f64, f64, f64)>,
    constant14_c4d13_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant14_c4d13: ConstantBlock<f64>,
    vector_slice18_c4d10: VectorSliceBlock,
    aggregate21_c4d11_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate21_c4d11: AggregateBlock<Matrix<1, 5, f64>>,
    product12_c4d12: ProductBlock,
    equation12_c4d49: EquationBlock,
    sum25_c4d48_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum25_c4d48: SumBlock<(f64, f64)>,
    constant8_c4cfb_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant8_c4cfb: ConstantBlock<f64>,
    vector_slice12_c4cf8: VectorSliceBlock,
    aggregate15_c4cf9_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate15_c4cf9: AggregateBlock<Matrix<1, 5, f64>>,
    product6_c4cfa: ProductBlock,
    equation6_c4d3d: EquationBlock,
    sum19_c4d3c_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum19_c4d3c: SumBlock<(f64, f64)>,
    constant20_c4d2b_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant20_c4d2b: ConstantBlock<f64>,
    vector_slice24_c4d28: VectorSliceBlock,
    aggregate27_c4d29_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate27_c4d29: AggregateBlock<Matrix<1, 5, f64>>,
    product18_c4d2a: ProductBlock,
    equation18_c4d55: EquationBlock,
    sum31_c4d54_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum31_c4d54: SumBlock<(f64, f64)>,
    sum37_c4d5b_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum37_c4d5b: SumBlock<(f64, f64, f64)>,
    sum39_c4d5d_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum39_c4d5d: SumBlock<(f64, f64, f64)>,
    product20_c4d5f: ProductBlock,
    constant11_c4d07_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant11_c4d07: ConstantBlock<f64>,
    vector_slice15_c4d04: VectorSliceBlock,
    aggregate18_c4d05_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate18_c4d05: AggregateBlock<Matrix<1, 5, f64>>,
    product9_c4d06: ProductBlock,
    equation9_c4d43: EquationBlock,
    sum22_c4d42_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum22_c4d42: SumBlock<(f64, f64)>,
    constant17_c4d1f_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant17_c4d1f: ConstantBlock<f64>,
    vector_slice21_c4d1c: VectorSliceBlock,
    aggregate24_c4d1d_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate24_c4d1d: AggregateBlock<Matrix<1, 5, f64>>,
    product15_c4d1e: ProductBlock,
    equation15_c4d4f: EquationBlock,
    sum28_c4d4e_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum28_c4d4e: SumBlock<(f64, f64)>,
    vector_slice9_c4cec: VectorSliceBlock,
    aggregate12_c4ced_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate12_c4ced: AggregateBlock<Matrix<1, 5, f64>>,
    constant5_c4cef_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant5_c4cef: ConstantBlock<f64>,
    product3_c4cee: ProductBlock,
    equation3_c4d37: EquationBlock,
    sum16_c4d36_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum16_c4d36: SumBlock<(f64, f64)>,
    sum34_c4d58_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum34_c4d58: SumBlock<(f64, f64, f64)>,
    vector_slice7_c4ce1: VectorSliceBlock,
    aggregate10_c4ce2_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate10_c4ce2: AggregateBlock<Matrix<1, 5, f64>>,
    constant3_c4ce7_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant3_c4ce7: ConstantBlock<f64>,
    product1_c4ce6: ProductBlock,
    equation1_c4d2d: EquationBlock,
    sum14_c4d2c_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum14_c4d2c: SumBlock<(f64, f64)>,
    vector_slice13_c4cfc: VectorSliceBlock,
    aggregate16_c4cfd_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate16_c4cfd: AggregateBlock<Matrix<1, 5, f64>>,
    constant9_c4cff_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant9_c4cff: ConstantBlock<f64>,
    product7_c4cfe: ProductBlock,
    equation7_c4d3f: EquationBlock,
    sum20_c4d3e_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum20_c4d3e: SumBlock<(f64, f64)>,
    vector_slice19_c4d14: VectorSliceBlock,
    aggregate22_c4d15_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate22_c4d15: AggregateBlock<Matrix<1, 5, f64>>,
    constant15_c4d17_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant15_c4d17: ConstantBlock<f64>,
    product13_c4d16: ProductBlock,
    equation13_c4d4b: EquationBlock,
    sum26_c4d4a_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum26_c4d4a: SumBlock<(f64, f64)>,
    sum32_c4d56_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum32_c4d56: SumBlock<(f64, f64, f64)>,
    vector_slice8_c4ce8: VectorSliceBlock,
    aggregate11_c4ce9_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate11_c4ce9: AggregateBlock<Matrix<1, 5, f64>>,
    constant4_c4ceb_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant4_c4ceb: ConstantBlock<f64>,
    product2_c4cea: ProductBlock,
    equation2_c4d35: EquationBlock,
    sum15_c4d34_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum15_c4d34: SumBlock<(f64, f64)>,
    vector_slice14_c4d00: VectorSliceBlock,
    aggregate17_c4d01_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate17_c4d01: AggregateBlock<Matrix<1, 5, f64>>,
    constant10_c4d03_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant10_c4d03: ConstantBlock<f64>,
    product8_c4d02: ProductBlock,
    equation8_c4d41: EquationBlock,
    sum21_c4d40_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum21_c4d40: SumBlock<(f64, f64)>,
    constant16_c4d1b_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant16_c4d1b: ConstantBlock<f64>,
    vector_slice20_c4d18: VectorSliceBlock,
    aggregate23_c4d19_param: <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters,
    aggregate23_c4d19: AggregateBlock<Matrix<1, 5, f64>>,
    product14_c4d1a: ProductBlock,
    equation14_c4d4d: EquationBlock,
    sum27_c4d4c_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum27_c4d4c: SumBlock<(f64, f64)>,
    sum33_c4d57_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum33_c4d57: SumBlock<(f64, f64, f64)>,
    sum38_c4d5c_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum38_c4d5c: SumBlock<(f64, f64, f64)>,
    constant21_c4d60_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant21_c4d60: ConstantBlock<f64>,
    product19_c4d5e: ProductBlock,
    sum40_c4d62_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum40_c4d62: SumBlock<(f64, f64)>,
    entropdiff_5b137: ComponentInputBlock,
    comparison7_c4d63_param: <ComparisonBlock<f64> as ProcessBlock>::Parameters,
    comparison7_c4d63: ComparisonBlock<f64>,
    gain4_c4d67_param: <GainBlock<f64, f64> as ProcessBlock>::Parameters,
    gain4_c4d67: GainBlock<f64, f64>,
    comparison8_c4d64_param: <ComparisonBlock<f64> as ProcessBlock>::Parameters,
    comparison8_c4d64: ComparisonBlock<f64>,
    logical1_c4d66: LogicalBlock,
    component_output1_c4d68: ComponentOutputBlock,
}

impl Component2c4cdfComponent {
    pub fn new(context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        let component_input1_c4ce0_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput1
        let component_input1_c4ce0 =
            ComponentInputBlock::new("ComponentInput1", &component_input1_c4ce0_ic);

        let constant22_c4d61_value =
            load_param::<f64>(&"constant22_c4d61", &"value", 3.000000, &diagram_params);

        let constant22_c4d61_ic = BlockData::from_element(1, 1, constant22_c4d61_value);

        // Constant22
        let constant22_c4d61_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant22_c4d61_ic.to_pass());
        let constant22_c4d61 = ConstantBlock::default();

        let constant7_c4cf7_value =
            load_param::<f64>(&"constant7_c4cf7", &"value", 150.000000, &diagram_params);

        let constant7_c4cf7_ic = BlockData::from_element(1, 1, constant7_c4cf7_value);

        // Constant7
        let constant7_c4cf7_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant7_c4cf7_ic.to_pass());
        let constant7_c4cf7 = ConstantBlock::default();

        let overall_lower_bucket_c4ce3_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Overall_Lower_bucket
        let overall_lower_bucket_c4ce3 =
            ComponentInputBlock::new("Overall_Lower_bucket", &overall_lower_bucket_c4ce3_ic);

        let vector_slice11_c4cf4_row0 =
            load_param::<f64>(&"vector_slice11_c4cf4", &"row0", 0.000000, &diagram_params);
        let vector_slice11_c4cf4_col0 =
            load_param::<f64>(&"vector_slice11_c4cf4", &"col0", 4.000000, &diagram_params);
        let vector_slice11_c4cf4_shape = load_param::<BlockData>(
            &"vector_slice11_c4cf4",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice11_c4cf4_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice11
        let vector_slice11_c4cf4 = VectorSliceBlock::new(
            "VectorSlice11",
            &vector_slice11_c4cf4_ic,
            vector_slice11_c4cf4_row0,
            vector_slice11_c4cf4_col0,
            &vector_slice11_c4cf4_shape,
        );

        let aggregate14_c4cf5_method = load_param::<String>(
            &"aggregate14_c4cf5",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate14
        let aggregate14_c4cf5_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate14_c4cf5_method,
            );
        let aggregate14_c4cf5 = AggregateBlock::default();

        let product5_c4cf6_gains = load_param::<BlockData>(
            &"product5_c4cf6",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product5_c4cf6_method = load_param::<String>(
            &"product5_c4cf6",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product5_c4cf6_ic = BlockData::new(1, 1, &[0.0]);

        // Product5
        let product5_c4cf6 = ProductBlock::new(
            "Product5",
            &product5_c4cf6_ic,
            &product5_c4cf6_gains,
            &product5_c4cf6_method,
        );

        let equation5_c4d3b_ic = BlockData::new(1, 1, &[0.0]);

        // Equation5
        let equation5_c4d3b = EquationBlock::new("Equation5", &equation5_c4d3b_ic);

        let sum18_c4d3a_gains = load_param::<BlockData>(
            &"sum18_c4d3a",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum18
        let sum18_c4d3a_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum18_c4d3a_gains.to_pass());
        let sum18_c4d3a = SumBlock::default();

        let overall_mid_bucket_c4ce4_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Overall_Mid_bucket
        let overall_mid_bucket_c4ce4 =
            ComponentInputBlock::new("Overall_Mid_bucket", &overall_mid_bucket_c4ce4_ic);

        let vector_slice17_c4d0c_row0 =
            load_param::<f64>(&"vector_slice17_c4d0c", &"row0", 0.000000, &diagram_params);
        let vector_slice17_c4d0c_col0 =
            load_param::<f64>(&"vector_slice17_c4d0c", &"col0", 4.000000, &diagram_params);
        let vector_slice17_c4d0c_shape = load_param::<BlockData>(
            &"vector_slice17_c4d0c",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice17_c4d0c_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice17
        let vector_slice17_c4d0c = VectorSliceBlock::new(
            "VectorSlice17",
            &vector_slice17_c4d0c_ic,
            vector_slice17_c4d0c_row0,
            vector_slice17_c4d0c_col0,
            &vector_slice17_c4d0c_shape,
        );

        let aggregate20_c4d0d_method = load_param::<String>(
            &"aggregate20_c4d0d",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate20
        let aggregate20_c4d0d_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate20_c4d0d_method,
            );
        let aggregate20_c4d0d = AggregateBlock::default();

        let constant13_c4d0f_value =
            load_param::<f64>(&"constant13_c4d0f", &"value", 150.000000, &diagram_params);

        let constant13_c4d0f_ic = BlockData::from_element(1, 1, constant13_c4d0f_value);

        // Constant13
        let constant13_c4d0f_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant13_c4d0f_ic.to_pass());
        let constant13_c4d0f = ConstantBlock::default();

        let product11_c4d0e_gains = load_param::<BlockData>(
            &"product11_c4d0e",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product11_c4d0e_method = load_param::<String>(
            &"product11_c4d0e",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product11_c4d0e_ic = BlockData::new(1, 1, &[0.0]);

        // Product11
        let product11_c4d0e = ProductBlock::new(
            "Product11",
            &product11_c4d0e_ic,
            &product11_c4d0e_gains,
            &product11_c4d0e_method,
        );

        let equation11_c4d47_ic = BlockData::new(1, 1, &[0.0]);

        // Equation11
        let equation11_c4d47 = EquationBlock::new("Equation11", &equation11_c4d47_ic);

        let sum24_c4d46_gains = load_param::<BlockData>(
            &"sum24_c4d46",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum24
        let sum24_c4d46_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum24_c4d46_gains.to_pass());
        let sum24_c4d46 = SumBlock::default();

        let constant19_c4d27_value =
            load_param::<f64>(&"constant19_c4d27", &"value", 150.000000, &diagram_params);

        let constant19_c4d27_ic = BlockData::from_element(1, 1, constant19_c4d27_value);

        // Constant19
        let constant19_c4d27_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant19_c4d27_ic.to_pass());
        let constant19_c4d27 = ConstantBlock::default();

        let overall_upper_bucket_c4ce5_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Overall_Upper_bucket
        let overall_upper_bucket_c4ce5 =
            ComponentInputBlock::new("Overall_Upper_bucket", &overall_upper_bucket_c4ce5_ic);

        let vector_slice23_c4d24_row0 =
            load_param::<f64>(&"vector_slice23_c4d24", &"row0", 0.000000, &diagram_params);
        let vector_slice23_c4d24_col0 =
            load_param::<f64>(&"vector_slice23_c4d24", &"col0", 4.000000, &diagram_params);
        let vector_slice23_c4d24_shape = load_param::<BlockData>(
            &"vector_slice23_c4d24",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice23_c4d24_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice23
        let vector_slice23_c4d24 = VectorSliceBlock::new(
            "VectorSlice23",
            &vector_slice23_c4d24_ic,
            vector_slice23_c4d24_row0,
            vector_slice23_c4d24_col0,
            &vector_slice23_c4d24_shape,
        );

        let aggregate26_c4d25_method = load_param::<String>(
            &"aggregate26_c4d25",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate26
        let aggregate26_c4d25_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate26_c4d25_method,
            );
        let aggregate26_c4d25 = AggregateBlock::default();

        let product17_c4d26_gains = load_param::<BlockData>(
            &"product17_c4d26",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product17_c4d26_method = load_param::<String>(
            &"product17_c4d26",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product17_c4d26_ic = BlockData::new(1, 1, &[0.0]);

        // Product17
        let product17_c4d26 = ProductBlock::new(
            "Product17",
            &product17_c4d26_ic,
            &product17_c4d26_gains,
            &product17_c4d26_method,
        );

        let equation17_c4d53_ic = BlockData::new(1, 1, &[0.0]);

        // Equation17
        let equation17_c4d53 = EquationBlock::new("Equation17", &equation17_c4d53_ic);

        let sum30_c4d52_gains = load_param::<BlockData>(
            &"sum30_c4d52",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum30
        let sum30_c4d52_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum30_c4d52_gains.to_pass());
        let sum30_c4d52 = SumBlock::default();

        let sum36_c4d5a_gains = load_param::<BlockData>(
            &"sum36_c4d5a",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum36
        let sum36_c4d5a_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum36_c4d5a_gains.to_pass(),
        );
        let sum36_c4d5a = SumBlock::default();

        let vector_slice10_c4cf0_row0 =
            load_param::<f64>(&"vector_slice10_c4cf0", &"row0", 0.000000, &diagram_params);
        let vector_slice10_c4cf0_col0 =
            load_param::<f64>(&"vector_slice10_c4cf0", &"col0", 3.000000, &diagram_params);
        let vector_slice10_c4cf0_shape = load_param::<BlockData>(
            &"vector_slice10_c4cf0",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice10_c4cf0_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice10
        let vector_slice10_c4cf0 = VectorSliceBlock::new(
            "VectorSlice10",
            &vector_slice10_c4cf0_ic,
            vector_slice10_c4cf0_row0,
            vector_slice10_c4cf0_col0,
            &vector_slice10_c4cf0_shape,
        );

        let aggregate13_c4cf1_method = load_param::<String>(
            &"aggregate13_c4cf1",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate13
        let aggregate13_c4cf1_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate13_c4cf1_method,
            );
        let aggregate13_c4cf1 = AggregateBlock::default();

        let constant6_c4cf3_value =
            load_param::<f64>(&"constant6_c4cf3", &"value", 150.000000, &diagram_params);

        let constant6_c4cf3_ic = BlockData::from_element(1, 1, constant6_c4cf3_value);

        // Constant6
        let constant6_c4cf3_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant6_c4cf3_ic.to_pass());
        let constant6_c4cf3 = ConstantBlock::default();

        let product4_c4cf2_gains = load_param::<BlockData>(
            &"product4_c4cf2",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product4_c4cf2_method = load_param::<String>(
            &"product4_c4cf2",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product4_c4cf2_ic = BlockData::new(1, 1, &[0.0]);

        // Product4
        let product4_c4cf2 = ProductBlock::new(
            "Product4",
            &product4_c4cf2_ic,
            &product4_c4cf2_gains,
            &product4_c4cf2_method,
        );

        let equation4_c4d39_ic = BlockData::new(1, 1, &[0.0]);

        // Equation4
        let equation4_c4d39 = EquationBlock::new("Equation4", &equation4_c4d39_ic);

        let sum17_c4d38_gains = load_param::<BlockData>(
            &"sum17_c4d38",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum17
        let sum17_c4d38_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum17_c4d38_gains.to_pass());
        let sum17_c4d38 = SumBlock::default();

        let vector_slice16_c4d08_row0 =
            load_param::<f64>(&"vector_slice16_c4d08", &"row0", 0.000000, &diagram_params);
        let vector_slice16_c4d08_col0 =
            load_param::<f64>(&"vector_slice16_c4d08", &"col0", 3.000000, &diagram_params);
        let vector_slice16_c4d08_shape = load_param::<BlockData>(
            &"vector_slice16_c4d08",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice16_c4d08_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice16
        let vector_slice16_c4d08 = VectorSliceBlock::new(
            "VectorSlice16",
            &vector_slice16_c4d08_ic,
            vector_slice16_c4d08_row0,
            vector_slice16_c4d08_col0,
            &vector_slice16_c4d08_shape,
        );

        let aggregate19_c4d09_method = load_param::<String>(
            &"aggregate19_c4d09",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate19
        let aggregate19_c4d09_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate19_c4d09_method,
            );
        let aggregate19_c4d09 = AggregateBlock::default();

        let constant12_c4d0b_value =
            load_param::<f64>(&"constant12_c4d0b", &"value", 150.000000, &diagram_params);

        let constant12_c4d0b_ic = BlockData::from_element(1, 1, constant12_c4d0b_value);

        // Constant12
        let constant12_c4d0b_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant12_c4d0b_ic.to_pass());
        let constant12_c4d0b = ConstantBlock::default();

        let product10_c4d0a_gains = load_param::<BlockData>(
            &"product10_c4d0a",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product10_c4d0a_method = load_param::<String>(
            &"product10_c4d0a",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product10_c4d0a_ic = BlockData::new(1, 1, &[0.0]);

        // Product10
        let product10_c4d0a = ProductBlock::new(
            "Product10",
            &product10_c4d0a_ic,
            &product10_c4d0a_gains,
            &product10_c4d0a_method,
        );

        let equation10_c4d45_ic = BlockData::new(1, 1, &[0.0]);

        // Equation10
        let equation10_c4d45 = EquationBlock::new("Equation10", &equation10_c4d45_ic);

        let sum23_c4d44_gains = load_param::<BlockData>(
            &"sum23_c4d44",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum23
        let sum23_c4d44_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum23_c4d44_gains.to_pass());
        let sum23_c4d44 = SumBlock::default();

        let vector_slice22_c4d20_row0 =
            load_param::<f64>(&"vector_slice22_c4d20", &"row0", 0.000000, &diagram_params);
        let vector_slice22_c4d20_col0 =
            load_param::<f64>(&"vector_slice22_c4d20", &"col0", 3.000000, &diagram_params);
        let vector_slice22_c4d20_shape = load_param::<BlockData>(
            &"vector_slice22_c4d20",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice22_c4d20_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice22
        let vector_slice22_c4d20 = VectorSliceBlock::new(
            "VectorSlice22",
            &vector_slice22_c4d20_ic,
            vector_slice22_c4d20_row0,
            vector_slice22_c4d20_col0,
            &vector_slice22_c4d20_shape,
        );

        let aggregate25_c4d21_method = load_param::<String>(
            &"aggregate25_c4d21",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate25
        let aggregate25_c4d21_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate25_c4d21_method,
            );
        let aggregate25_c4d21 = AggregateBlock::default();

        let constant18_c4d23_value =
            load_param::<f64>(&"constant18_c4d23", &"value", 150.000000, &diagram_params);

        let constant18_c4d23_ic = BlockData::from_element(1, 1, constant18_c4d23_value);

        // Constant18
        let constant18_c4d23_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant18_c4d23_ic.to_pass());
        let constant18_c4d23 = ConstantBlock::default();

        let product16_c4d22_gains = load_param::<BlockData>(
            &"product16_c4d22",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product16_c4d22_method = load_param::<String>(
            &"product16_c4d22",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product16_c4d22_ic = BlockData::new(1, 1, &[0.0]);

        // Product16
        let product16_c4d22 = ProductBlock::new(
            "Product16",
            &product16_c4d22_ic,
            &product16_c4d22_gains,
            &product16_c4d22_method,
        );

        let equation16_c4d51_ic = BlockData::new(1, 1, &[0.0]);

        // Equation16
        let equation16_c4d51 = EquationBlock::new("Equation16", &equation16_c4d51_ic);

        let sum29_c4d50_gains = load_param::<BlockData>(
            &"sum29_c4d50",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum29
        let sum29_c4d50_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum29_c4d50_gains.to_pass());
        let sum29_c4d50 = SumBlock::default();

        let sum35_c4d59_gains = load_param::<BlockData>(
            &"sum35_c4d59",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum35
        let sum35_c4d59_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum35_c4d59_gains.to_pass(),
        );
        let sum35_c4d59 = SumBlock::default();

        let constant14_c4d13_value =
            load_param::<f64>(&"constant14_c4d13", &"value", 150.000000, &diagram_params);

        let constant14_c4d13_ic = BlockData::from_element(1, 1, constant14_c4d13_value);

        // Constant14
        let constant14_c4d13_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant14_c4d13_ic.to_pass());
        let constant14_c4d13 = ConstantBlock::default();

        let vector_slice18_c4d10_row0 =
            load_param::<f64>(&"vector_slice18_c4d10", &"row0", 0.000000, &diagram_params);
        let vector_slice18_c4d10_col0 =
            load_param::<f64>(&"vector_slice18_c4d10", &"col0", 5.000000, &diagram_params);
        let vector_slice18_c4d10_shape = load_param::<BlockData>(
            &"vector_slice18_c4d10",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice18_c4d10_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice18
        let vector_slice18_c4d10 = VectorSliceBlock::new(
            "VectorSlice18",
            &vector_slice18_c4d10_ic,
            vector_slice18_c4d10_row0,
            vector_slice18_c4d10_col0,
            &vector_slice18_c4d10_shape,
        );

        let aggregate21_c4d11_method = load_param::<String>(
            &"aggregate21_c4d11",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate21
        let aggregate21_c4d11_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate21_c4d11_method,
            );
        let aggregate21_c4d11 = AggregateBlock::default();

        let product12_c4d12_gains = load_param::<BlockData>(
            &"product12_c4d12",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product12_c4d12_method = load_param::<String>(
            &"product12_c4d12",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product12_c4d12_ic = BlockData::new(1, 1, &[0.0]);

        // Product12
        let product12_c4d12 = ProductBlock::new(
            "Product12",
            &product12_c4d12_ic,
            &product12_c4d12_gains,
            &product12_c4d12_method,
        );

        let equation12_c4d49_ic = BlockData::new(1, 1, &[0.0]);

        // Equation12
        let equation12_c4d49 = EquationBlock::new("Equation12", &equation12_c4d49_ic);

        let sum25_c4d48_gains = load_param::<BlockData>(
            &"sum25_c4d48",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum25
        let sum25_c4d48_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum25_c4d48_gains.to_pass());
        let sum25_c4d48 = SumBlock::default();

        let constant8_c4cfb_value =
            load_param::<f64>(&"constant8_c4cfb", &"value", 150.000000, &diagram_params);

        let constant8_c4cfb_ic = BlockData::from_element(1, 1, constant8_c4cfb_value);

        // Constant8
        let constant8_c4cfb_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant8_c4cfb_ic.to_pass());
        let constant8_c4cfb = ConstantBlock::default();

        let vector_slice12_c4cf8_row0 =
            load_param::<f64>(&"vector_slice12_c4cf8", &"row0", 0.000000, &diagram_params);
        let vector_slice12_c4cf8_col0 =
            load_param::<f64>(&"vector_slice12_c4cf8", &"col0", 5.000000, &diagram_params);
        let vector_slice12_c4cf8_shape = load_param::<BlockData>(
            &"vector_slice12_c4cf8",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice12_c4cf8_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice12
        let vector_slice12_c4cf8 = VectorSliceBlock::new(
            "VectorSlice12",
            &vector_slice12_c4cf8_ic,
            vector_slice12_c4cf8_row0,
            vector_slice12_c4cf8_col0,
            &vector_slice12_c4cf8_shape,
        );

        let aggregate15_c4cf9_method = load_param::<String>(
            &"aggregate15_c4cf9",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate15
        let aggregate15_c4cf9_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate15_c4cf9_method,
            );
        let aggregate15_c4cf9 = AggregateBlock::default();

        let product6_c4cfa_gains = load_param::<BlockData>(
            &"product6_c4cfa",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product6_c4cfa_method = load_param::<String>(
            &"product6_c4cfa",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product6_c4cfa_ic = BlockData::new(1, 1, &[0.0]);

        // Product6
        let product6_c4cfa = ProductBlock::new(
            "Product6",
            &product6_c4cfa_ic,
            &product6_c4cfa_gains,
            &product6_c4cfa_method,
        );

        let equation6_c4d3d_ic = BlockData::new(1, 1, &[0.0]);

        // Equation6
        let equation6_c4d3d = EquationBlock::new("Equation6", &equation6_c4d3d_ic);

        let sum19_c4d3c_gains = load_param::<BlockData>(
            &"sum19_c4d3c",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum19
        let sum19_c4d3c_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum19_c4d3c_gains.to_pass());
        let sum19_c4d3c = SumBlock::default();

        let constant20_c4d2b_value =
            load_param::<f64>(&"constant20_c4d2b", &"value", 150.000000, &diagram_params);

        let constant20_c4d2b_ic = BlockData::from_element(1, 1, constant20_c4d2b_value);

        // Constant20
        let constant20_c4d2b_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant20_c4d2b_ic.to_pass());
        let constant20_c4d2b = ConstantBlock::default();

        let vector_slice24_c4d28_row0 =
            load_param::<f64>(&"vector_slice24_c4d28", &"row0", 0.000000, &diagram_params);
        let vector_slice24_c4d28_col0 =
            load_param::<f64>(&"vector_slice24_c4d28", &"col0", 5.000000, &diagram_params);
        let vector_slice24_c4d28_shape = load_param::<BlockData>(
            &"vector_slice24_c4d28",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice24_c4d28_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice24
        let vector_slice24_c4d28 = VectorSliceBlock::new(
            "VectorSlice24",
            &vector_slice24_c4d28_ic,
            vector_slice24_c4d28_row0,
            vector_slice24_c4d28_col0,
            &vector_slice24_c4d28_shape,
        );

        let aggregate27_c4d29_method = load_param::<String>(
            &"aggregate27_c4d29",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate27
        let aggregate27_c4d29_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate27_c4d29_method,
            );
        let aggregate27_c4d29 = AggregateBlock::default();

        let product18_c4d2a_gains = load_param::<BlockData>(
            &"product18_c4d2a",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product18_c4d2a_method = load_param::<String>(
            &"product18_c4d2a",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product18_c4d2a_ic = BlockData::new(1, 1, &[0.0]);

        // Product18
        let product18_c4d2a = ProductBlock::new(
            "Product18",
            &product18_c4d2a_ic,
            &product18_c4d2a_gains,
            &product18_c4d2a_method,
        );

        let equation18_c4d55_ic = BlockData::new(1, 1, &[0.0]);

        // Equation18
        let equation18_c4d55 = EquationBlock::new("Equation18", &equation18_c4d55_ic);

        let sum31_c4d54_gains = load_param::<BlockData>(
            &"sum31_c4d54",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum31
        let sum31_c4d54_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum31_c4d54_gains.to_pass());
        let sum31_c4d54 = SumBlock::default();

        let sum37_c4d5b_gains = load_param::<BlockData>(
            &"sum37_c4d5b",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum37
        let sum37_c4d5b_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum37_c4d5b_gains.to_pass(),
        );
        let sum37_c4d5b = SumBlock::default();

        let sum39_c4d5d_gains = load_param::<BlockData>(
            &"sum39_c4d5d",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum39
        let sum39_c4d5d_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum39_c4d5d_gains.to_pass(),
        );
        let sum39_c4d5d = SumBlock::default();

        let product20_c4d5f_gains = load_param::<BlockData>(
            &"product20_c4d5f",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product20_c4d5f_method = load_param::<String>(
            &"product20_c4d5f",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product20_c4d5f_ic = BlockData::new(1, 1, &[0.0]);

        // Product20
        let product20_c4d5f = ProductBlock::new(
            "Product20",
            &product20_c4d5f_ic,
            &product20_c4d5f_gains,
            &product20_c4d5f_method,
        );

        let constant11_c4d07_value =
            load_param::<f64>(&"constant11_c4d07", &"value", 150.000000, &diagram_params);

        let constant11_c4d07_ic = BlockData::from_element(1, 1, constant11_c4d07_value);

        // Constant11
        let constant11_c4d07_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant11_c4d07_ic.to_pass());
        let constant11_c4d07 = ConstantBlock::default();

        let vector_slice15_c4d04_row0 =
            load_param::<f64>(&"vector_slice15_c4d04", &"row0", 0.000000, &diagram_params);
        let vector_slice15_c4d04_col0 =
            load_param::<f64>(&"vector_slice15_c4d04", &"col0", 2.000000, &diagram_params);
        let vector_slice15_c4d04_shape = load_param::<BlockData>(
            &"vector_slice15_c4d04",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice15_c4d04_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice15
        let vector_slice15_c4d04 = VectorSliceBlock::new(
            "VectorSlice15",
            &vector_slice15_c4d04_ic,
            vector_slice15_c4d04_row0,
            vector_slice15_c4d04_col0,
            &vector_slice15_c4d04_shape,
        );

        let aggregate18_c4d05_method = load_param::<String>(
            &"aggregate18_c4d05",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate18
        let aggregate18_c4d05_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate18_c4d05_method,
            );
        let aggregate18_c4d05 = AggregateBlock::default();

        let product9_c4d06_gains = load_param::<BlockData>(
            &"product9_c4d06",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product9_c4d06_method = load_param::<String>(
            &"product9_c4d06",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product9_c4d06_ic = BlockData::new(1, 1, &[0.0]);

        // Product9
        let product9_c4d06 = ProductBlock::new(
            "Product9",
            &product9_c4d06_ic,
            &product9_c4d06_gains,
            &product9_c4d06_method,
        );

        let equation9_c4d43_ic = BlockData::new(1, 1, &[0.0]);

        // Equation9
        let equation9_c4d43 = EquationBlock::new("Equation9", &equation9_c4d43_ic);

        let sum22_c4d42_gains = load_param::<BlockData>(
            &"sum22_c4d42",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum22
        let sum22_c4d42_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum22_c4d42_gains.to_pass());
        let sum22_c4d42 = SumBlock::default();

        let constant17_c4d1f_value =
            load_param::<f64>(&"constant17_c4d1f", &"value", 150.000000, &diagram_params);

        let constant17_c4d1f_ic = BlockData::from_element(1, 1, constant17_c4d1f_value);

        // Constant17
        let constant17_c4d1f_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant17_c4d1f_ic.to_pass());
        let constant17_c4d1f = ConstantBlock::default();

        let vector_slice21_c4d1c_row0 =
            load_param::<f64>(&"vector_slice21_c4d1c", &"row0", 0.000000, &diagram_params);
        let vector_slice21_c4d1c_col0 =
            load_param::<f64>(&"vector_slice21_c4d1c", &"col0", 2.000000, &diagram_params);
        let vector_slice21_c4d1c_shape = load_param::<BlockData>(
            &"vector_slice21_c4d1c",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice21_c4d1c_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice21
        let vector_slice21_c4d1c = VectorSliceBlock::new(
            "VectorSlice21",
            &vector_slice21_c4d1c_ic,
            vector_slice21_c4d1c_row0,
            vector_slice21_c4d1c_col0,
            &vector_slice21_c4d1c_shape,
        );

        let aggregate24_c4d1d_method = load_param::<String>(
            &"aggregate24_c4d1d",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate24
        let aggregate24_c4d1d_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate24_c4d1d_method,
            );
        let aggregate24_c4d1d = AggregateBlock::default();

        let product15_c4d1e_gains = load_param::<BlockData>(
            &"product15_c4d1e",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product15_c4d1e_method = load_param::<String>(
            &"product15_c4d1e",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product15_c4d1e_ic = BlockData::new(1, 1, &[0.0]);

        // Product15
        let product15_c4d1e = ProductBlock::new(
            "Product15",
            &product15_c4d1e_ic,
            &product15_c4d1e_gains,
            &product15_c4d1e_method,
        );

        let equation15_c4d4f_ic = BlockData::new(1, 1, &[0.0]);

        // Equation15
        let equation15_c4d4f = EquationBlock::new("Equation15", &equation15_c4d4f_ic);

        let sum28_c4d4e_gains = load_param::<BlockData>(
            &"sum28_c4d4e",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum28
        let sum28_c4d4e_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum28_c4d4e_gains.to_pass());
        let sum28_c4d4e = SumBlock::default();

        let vector_slice9_c4cec_row0 =
            load_param::<f64>(&"vector_slice9_c4cec", &"row0", 0.000000, &diagram_params);
        let vector_slice9_c4cec_col0 =
            load_param::<f64>(&"vector_slice9_c4cec", &"col0", 2.000000, &diagram_params);
        let vector_slice9_c4cec_shape = load_param::<BlockData>(
            &"vector_slice9_c4cec",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice9_c4cec_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice9
        let vector_slice9_c4cec = VectorSliceBlock::new(
            "VectorSlice9",
            &vector_slice9_c4cec_ic,
            vector_slice9_c4cec_row0,
            vector_slice9_c4cec_col0,
            &vector_slice9_c4cec_shape,
        );

        let aggregate12_c4ced_method = load_param::<String>(
            &"aggregate12_c4ced",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate12
        let aggregate12_c4ced_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate12_c4ced_method,
            );
        let aggregate12_c4ced = AggregateBlock::default();

        let constant5_c4cef_value =
            load_param::<f64>(&"constant5_c4cef", &"value", 150.000000, &diagram_params);

        let constant5_c4cef_ic = BlockData::from_element(1, 1, constant5_c4cef_value);

        // Constant5
        let constant5_c4cef_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant5_c4cef_ic.to_pass());
        let constant5_c4cef = ConstantBlock::default();

        let product3_c4cee_gains = load_param::<BlockData>(
            &"product3_c4cee",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product3_c4cee_method = load_param::<String>(
            &"product3_c4cee",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product3_c4cee_ic = BlockData::new(1, 1, &[0.0]);

        // Product3
        let product3_c4cee = ProductBlock::new(
            "Product3",
            &product3_c4cee_ic,
            &product3_c4cee_gains,
            &product3_c4cee_method,
        );

        let equation3_c4d37_ic = BlockData::new(1, 1, &[0.0]);

        // Equation3
        let equation3_c4d37 = EquationBlock::new("Equation3", &equation3_c4d37_ic);

        let sum16_c4d36_gains = load_param::<BlockData>(
            &"sum16_c4d36",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum16
        let sum16_c4d36_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum16_c4d36_gains.to_pass());
        let sum16_c4d36 = SumBlock::default();

        let sum34_c4d58_gains = load_param::<BlockData>(
            &"sum34_c4d58",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum34
        let sum34_c4d58_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum34_c4d58_gains.to_pass(),
        );
        let sum34_c4d58 = SumBlock::default();

        let vector_slice7_c4ce1_row0 =
            load_param::<f64>(&"vector_slice7_c4ce1", &"row0", 0.000000, &diagram_params);
        let vector_slice7_c4ce1_col0 = context.gds.slicestart_c4cdf_82841.clone();
        let vector_slice7_c4ce1_shape = load_param::<BlockData>(
            &"vector_slice7_c4ce1",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice7_c4ce1_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice7
        let vector_slice7_c4ce1 = VectorSliceBlock::new(
            "VectorSlice7",
            &vector_slice7_c4ce1_ic,
            vector_slice7_c4ce1_row0,
            vector_slice7_c4ce1_col0,
            &vector_slice7_c4ce1_shape,
        );

        let aggregate10_c4ce2_method = load_param::<String>(
            &"aggregate10_c4ce2",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate10
        let aggregate10_c4ce2_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate10_c4ce2_method,
            );
        let aggregate10_c4ce2 = AggregateBlock::default();

        let constant3_c4ce7_value =
            load_param::<f64>(&"constant3_c4ce7", &"value", 150.000000, &diagram_params);

        let constant3_c4ce7_ic = BlockData::from_element(1, 1, constant3_c4ce7_value);

        // Constant3
        let constant3_c4ce7_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant3_c4ce7_ic.to_pass());
        let constant3_c4ce7 = ConstantBlock::default();

        let product1_c4ce6_gains = load_param::<BlockData>(
            &"product1_c4ce6",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product1_c4ce6_method = load_param::<String>(
            &"product1_c4ce6",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product1_c4ce6_ic = BlockData::new(1, 1, &[0.0]);

        // Product1
        let product1_c4ce6 = ProductBlock::new(
            "Product1",
            &product1_c4ce6_ic,
            &product1_c4ce6_gains,
            &product1_c4ce6_method,
        );

        let equation1_c4d2d_ic = BlockData::new(1, 1, &[0.0]);

        // Equation1
        let equation1_c4d2d = EquationBlock::new("Equation1", &equation1_c4d2d_ic);

        let sum14_c4d2c_gains = load_param::<BlockData>(
            &"sum14_c4d2c",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum14
        let sum14_c4d2c_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum14_c4d2c_gains.to_pass());
        let sum14_c4d2c = SumBlock::default();

        let vector_slice13_c4cfc_row0 =
            load_param::<f64>(&"vector_slice13_c4cfc", &"row0", 0.000000, &diagram_params);
        let vector_slice13_c4cfc_col0 =
            load_param::<f64>(&"vector_slice13_c4cfc", &"col0", 0.000000, &diagram_params);
        let vector_slice13_c4cfc_shape = load_param::<BlockData>(
            &"vector_slice13_c4cfc",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice13_c4cfc_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice13
        let vector_slice13_c4cfc = VectorSliceBlock::new(
            "VectorSlice13",
            &vector_slice13_c4cfc_ic,
            vector_slice13_c4cfc_row0,
            vector_slice13_c4cfc_col0,
            &vector_slice13_c4cfc_shape,
        );

        let aggregate16_c4cfd_method = load_param::<String>(
            &"aggregate16_c4cfd",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate16
        let aggregate16_c4cfd_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate16_c4cfd_method,
            );
        let aggregate16_c4cfd = AggregateBlock::default();

        let constant9_c4cff_value =
            load_param::<f64>(&"constant9_c4cff", &"value", 150.000000, &diagram_params);

        let constant9_c4cff_ic = BlockData::from_element(1, 1, constant9_c4cff_value);

        // Constant9
        let constant9_c4cff_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant9_c4cff_ic.to_pass());
        let constant9_c4cff = ConstantBlock::default();

        let product7_c4cfe_gains = load_param::<BlockData>(
            &"product7_c4cfe",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product7_c4cfe_method = load_param::<String>(
            &"product7_c4cfe",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product7_c4cfe_ic = BlockData::new(1, 1, &[0.0]);

        // Product7
        let product7_c4cfe = ProductBlock::new(
            "Product7",
            &product7_c4cfe_ic,
            &product7_c4cfe_gains,
            &product7_c4cfe_method,
        );

        let equation7_c4d3f_ic = BlockData::new(1, 1, &[0.0]);

        // Equation7
        let equation7_c4d3f = EquationBlock::new("Equation7", &equation7_c4d3f_ic);

        let sum20_c4d3e_gains = load_param::<BlockData>(
            &"sum20_c4d3e",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum20
        let sum20_c4d3e_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum20_c4d3e_gains.to_pass());
        let sum20_c4d3e = SumBlock::default();

        let vector_slice19_c4d14_row0 =
            load_param::<f64>(&"vector_slice19_c4d14", &"row0", 0.000000, &diagram_params);
        let vector_slice19_c4d14_col0 =
            load_param::<f64>(&"vector_slice19_c4d14", &"col0", 0.000000, &diagram_params);
        let vector_slice19_c4d14_shape = load_param::<BlockData>(
            &"vector_slice19_c4d14",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice19_c4d14_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice19
        let vector_slice19_c4d14 = VectorSliceBlock::new(
            "VectorSlice19",
            &vector_slice19_c4d14_ic,
            vector_slice19_c4d14_row0,
            vector_slice19_c4d14_col0,
            &vector_slice19_c4d14_shape,
        );

        let aggregate22_c4d15_method = load_param::<String>(
            &"aggregate22_c4d15",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate22
        let aggregate22_c4d15_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate22_c4d15_method,
            );
        let aggregate22_c4d15 = AggregateBlock::default();

        let constant15_c4d17_value =
            load_param::<f64>(&"constant15_c4d17", &"value", 150.000000, &diagram_params);

        let constant15_c4d17_ic = BlockData::from_element(1, 1, constant15_c4d17_value);

        // Constant15
        let constant15_c4d17_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant15_c4d17_ic.to_pass());
        let constant15_c4d17 = ConstantBlock::default();

        let product13_c4d16_gains = load_param::<BlockData>(
            &"product13_c4d16",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product13_c4d16_method = load_param::<String>(
            &"product13_c4d16",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product13_c4d16_ic = BlockData::new(1, 1, &[0.0]);

        // Product13
        let product13_c4d16 = ProductBlock::new(
            "Product13",
            &product13_c4d16_ic,
            &product13_c4d16_gains,
            &product13_c4d16_method,
        );

        let equation13_c4d4b_ic = BlockData::new(1, 1, &[0.0]);

        // Equation13
        let equation13_c4d4b = EquationBlock::new("Equation13", &equation13_c4d4b_ic);

        let sum26_c4d4a_gains = load_param::<BlockData>(
            &"sum26_c4d4a",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum26
        let sum26_c4d4a_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum26_c4d4a_gains.to_pass());
        let sum26_c4d4a = SumBlock::default();

        let sum32_c4d56_gains = load_param::<BlockData>(
            &"sum32_c4d56",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum32
        let sum32_c4d56_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum32_c4d56_gains.to_pass(),
        );
        let sum32_c4d56 = SumBlock::default();

        let vector_slice8_c4ce8_row0 =
            load_param::<f64>(&"vector_slice8_c4ce8", &"row0", 0.000000, &diagram_params);
        let vector_slice8_c4ce8_col0 =
            load_param::<f64>(&"vector_slice8_c4ce8", &"col0", 1.000000, &diagram_params);
        let vector_slice8_c4ce8_shape = load_param::<BlockData>(
            &"vector_slice8_c4ce8",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice8_c4ce8_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice8
        let vector_slice8_c4ce8 = VectorSliceBlock::new(
            "VectorSlice8",
            &vector_slice8_c4ce8_ic,
            vector_slice8_c4ce8_row0,
            vector_slice8_c4ce8_col0,
            &vector_slice8_c4ce8_shape,
        );

        let aggregate11_c4ce9_method = load_param::<String>(
            &"aggregate11_c4ce9",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate11
        let aggregate11_c4ce9_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate11_c4ce9_method,
            );
        let aggregate11_c4ce9 = AggregateBlock::default();

        let constant4_c4ceb_value =
            load_param::<f64>(&"constant4_c4ceb", &"value", 150.000000, &diagram_params);

        let constant4_c4ceb_ic = BlockData::from_element(1, 1, constant4_c4ceb_value);

        // Constant4
        let constant4_c4ceb_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant4_c4ceb_ic.to_pass());
        let constant4_c4ceb = ConstantBlock::default();

        let product2_c4cea_gains = load_param::<BlockData>(
            &"product2_c4cea",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product2_c4cea_method = load_param::<String>(
            &"product2_c4cea",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product2_c4cea_ic = BlockData::new(1, 1, &[0.0]);

        // Product2
        let product2_c4cea = ProductBlock::new(
            "Product2",
            &product2_c4cea_ic,
            &product2_c4cea_gains,
            &product2_c4cea_method,
        );

        let equation2_c4d35_ic = BlockData::new(1, 1, &[0.0]);

        // Equation2
        let equation2_c4d35 = EquationBlock::new("Equation2", &equation2_c4d35_ic);

        let sum15_c4d34_gains = load_param::<BlockData>(
            &"sum15_c4d34",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum15
        let sum15_c4d34_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum15_c4d34_gains.to_pass());
        let sum15_c4d34 = SumBlock::default();

        let vector_slice14_c4d00_row0 =
            load_param::<f64>(&"vector_slice14_c4d00", &"row0", 0.000000, &diagram_params);
        let vector_slice14_c4d00_col0 =
            load_param::<f64>(&"vector_slice14_c4d00", &"col0", 1.000000, &diagram_params);
        let vector_slice14_c4d00_shape = load_param::<BlockData>(
            &"vector_slice14_c4d00",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice14_c4d00_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice14
        let vector_slice14_c4d00 = VectorSliceBlock::new(
            "VectorSlice14",
            &vector_slice14_c4d00_ic,
            vector_slice14_c4d00_row0,
            vector_slice14_c4d00_col0,
            &vector_slice14_c4d00_shape,
        );

        let aggregate17_c4d01_method = load_param::<String>(
            &"aggregate17_c4d01",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate17
        let aggregate17_c4d01_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate17_c4d01_method,
            );
        let aggregate17_c4d01 = AggregateBlock::default();

        let constant10_c4d03_value =
            load_param::<f64>(&"constant10_c4d03", &"value", 150.000000, &diagram_params);

        let constant10_c4d03_ic = BlockData::from_element(1, 1, constant10_c4d03_value);

        // Constant10
        let constant10_c4d03_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant10_c4d03_ic.to_pass());
        let constant10_c4d03 = ConstantBlock::default();

        let product8_c4d02_gains = load_param::<BlockData>(
            &"product8_c4d02",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product8_c4d02_method = load_param::<String>(
            &"product8_c4d02",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product8_c4d02_ic = BlockData::new(1, 1, &[0.0]);

        // Product8
        let product8_c4d02 = ProductBlock::new(
            "Product8",
            &product8_c4d02_ic,
            &product8_c4d02_gains,
            &product8_c4d02_method,
        );

        let equation8_c4d41_ic = BlockData::new(1, 1, &[0.0]);

        // Equation8
        let equation8_c4d41 = EquationBlock::new("Equation8", &equation8_c4d41_ic);

        let sum21_c4d40_gains = load_param::<BlockData>(
            &"sum21_c4d40",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum21
        let sum21_c4d40_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum21_c4d40_gains.to_pass());
        let sum21_c4d40 = SumBlock::default();

        let constant16_c4d1b_value =
            load_param::<f64>(&"constant16_c4d1b", &"value", 150.000000, &diagram_params);

        let constant16_c4d1b_ic = BlockData::from_element(1, 1, constant16_c4d1b_value);

        // Constant16
        let constant16_c4d1b_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant16_c4d1b_ic.to_pass());
        let constant16_c4d1b = ConstantBlock::default();

        let vector_slice20_c4d18_row0 =
            load_param::<f64>(&"vector_slice20_c4d18", &"row0", 0.000000, &diagram_params);
        let vector_slice20_c4d18_col0 =
            load_param::<f64>(&"vector_slice20_c4d18", &"col0", 1.000000, &diagram_params);
        let vector_slice20_c4d18_shape = load_param::<BlockData>(
            &"vector_slice20_c4d18",
            &"shape",
            BlockData::new(1, 2, &[1.0, 5.0]),
            &diagram_params,
        );

        let vector_slice20_c4d18_ic = BlockData::from_element(1, 5, 0.0);

        // VectorSlice20
        let vector_slice20_c4d18 = VectorSliceBlock::new(
            "VectorSlice20",
            &vector_slice20_c4d18_ic,
            vector_slice20_c4d18_row0,
            vector_slice20_c4d18_col0,
            &vector_slice20_c4d18_shape,
        );

        let aggregate23_c4d19_method = load_param::<String>(
            &"aggregate23_c4d19",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate23
        let aggregate23_c4d19_param =
            <AggregateBlock<Matrix<1, 5, f64>> as ProcessBlock>::Parameters::new(
                &aggregate23_c4d19_method,
            );
        let aggregate23_c4d19 = AggregateBlock::default();

        let product14_c4d1a_gains = load_param::<BlockData>(
            &"product14_c4d1a",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product14_c4d1a_method = load_param::<String>(
            &"product14_c4d1a",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product14_c4d1a_ic = BlockData::new(1, 1, &[0.0]);

        // Product14
        let product14_c4d1a = ProductBlock::new(
            "Product14",
            &product14_c4d1a_ic,
            &product14_c4d1a_gains,
            &product14_c4d1a_method,
        );

        let equation14_c4d4d_ic = BlockData::new(1, 1, &[0.0]);

        // Equation14
        let equation14_c4d4d = EquationBlock::new("Equation14", &equation14_c4d4d_ic);

        let sum27_c4d4c_gains = load_param::<BlockData>(
            &"sum27_c4d4c",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum27
        let sum27_c4d4c_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum27_c4d4c_gains.to_pass());
        let sum27_c4d4c = SumBlock::default();

        let sum33_c4d57_gains = load_param::<BlockData>(
            &"sum33_c4d57",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum33
        let sum33_c4d57_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum33_c4d57_gains.to_pass(),
        );
        let sum33_c4d57 = SumBlock::default();

        let sum38_c4d5c_gains = load_param::<BlockData>(
            &"sum38_c4d5c",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum38
        let sum38_c4d5c_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum38_c4d5c_gains.to_pass(),
        );
        let sum38_c4d5c = SumBlock::default();

        let constant21_c4d60_value =
            load_param::<f64>(&"constant21_c4d60", &"value", 3.000000, &diagram_params);

        let constant21_c4d60_ic = BlockData::from_element(1, 1, constant21_c4d60_value);

        // Constant21
        let constant21_c4d60_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant21_c4d60_ic.to_pass());
        let constant21_c4d60 = ConstantBlock::default();

        let product19_c4d5e_gains = load_param::<BlockData>(
            &"product19_c4d5e",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );
        let product19_c4d5e_method = load_param::<String>(
            &"product19_c4d5e",
            &"method",
            String::from("ComponentWise"),
            &diagram_params,
        );

        let product19_c4d5e_ic = BlockData::new(1, 1, &[0.0]);

        // Product19
        let product19_c4d5e = ProductBlock::new(
            "Product19",
            &product19_c4d5e_ic,
            &product19_c4d5e_gains,
            &product19_c4d5e_method,
        );

        let sum40_c4d62_gains = load_param::<BlockData>(
            &"sum40_c4d62",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum40
        let sum40_c4d62_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum40_c4d62_gains.to_pass());
        let sum40_c4d62 = SumBlock::default();

        let entropdiff_5b137_ic = BlockData::new(1, 1, &[0.0]);

        // Entropdiff
        let entropdiff_5b137 = ComponentInputBlock::new("Entropdiff", &entropdiff_5b137_ic);

        let comparison7_c4d63_method = load_param::<String>(
            &"comparison7_c4d63",
            &"method",
            String::from("GreaterThan"),
            &diagram_params,
        );

        // Comparison7
        let comparison7_c4d63_param =
            <ComparisonBlock<f64> as ProcessBlock>::Parameters::new(&comparison7_c4d63_method);
        let comparison7_c4d63 = ComparisonBlock::default();

        let gain4_c4d67_gain =
            load_param::<f64>(&"gain4_c4d67", &"gain", -1.000000, &diagram_params);

        // Gain4
        let gain4_c4d67_param =
            <GainBlock<f64, f64> as ProcessBlock>::Parameters::new(gain4_c4d67_gain);
        let gain4_c4d67 = GainBlock::default();

        let comparison8_c4d64_method = load_param::<String>(
            &"comparison8_c4d64",
            &"method",
            String::from("LessThan"),
            &diagram_params,
        );

        // Comparison8
        let comparison8_c4d64_param =
            <ComparisonBlock<f64> as ProcessBlock>::Parameters::new(&comparison8_c4d64_method);
        let comparison8_c4d64 = ComparisonBlock::default();

        let logical1_c4d66_method = load_param::<String>(
            &"logical1_c4d66",
            &"method",
            String::from("Or"),
            &diagram_params,
        );

        let logical1_c4d66_ic = BlockData::new(1, 1, &[0.0]);

        // Logical1
        let logical1_c4d66 =
            LogicalBlock::new("Logical1", &logical1_c4d66_ic, &logical1_c4d66_method);

        let component_output1_c4d68_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentOutput1
        let component_output1_c4d68 =
            ComponentOutputBlock::new("ComponentOutput1", &component_output1_c4d68_ic);

        Component2c4cdfComponent {
            last_time_s: -1.0,
            component_input1_c4ce0,
            constant22_c4d61_param,
            constant22_c4d61,
            constant7_c4cf7_param,
            constant7_c4cf7,
            overall_lower_bucket_c4ce3,
            vector_slice11_c4cf4,
            aggregate14_c4cf5_param,
            aggregate14_c4cf5,
            product5_c4cf6,
            equation5_c4d3b,
            sum18_c4d3a_param,
            sum18_c4d3a,
            overall_mid_bucket_c4ce4,
            vector_slice17_c4d0c,
            aggregate20_c4d0d_param,
            aggregate20_c4d0d,
            constant13_c4d0f_param,
            constant13_c4d0f,
            product11_c4d0e,
            equation11_c4d47,
            sum24_c4d46_param,
            sum24_c4d46,
            constant19_c4d27_param,
            constant19_c4d27,
            overall_upper_bucket_c4ce5,
            vector_slice23_c4d24,
            aggregate26_c4d25_param,
            aggregate26_c4d25,
            product17_c4d26,
            equation17_c4d53,
            sum30_c4d52_param,
            sum30_c4d52,
            sum36_c4d5a_param,
            sum36_c4d5a,
            vector_slice10_c4cf0,
            aggregate13_c4cf1_param,
            aggregate13_c4cf1,
            constant6_c4cf3_param,
            constant6_c4cf3,
            product4_c4cf2,
            equation4_c4d39,
            sum17_c4d38_param,
            sum17_c4d38,
            vector_slice16_c4d08,
            aggregate19_c4d09_param,
            aggregate19_c4d09,
            constant12_c4d0b_param,
            constant12_c4d0b,
            product10_c4d0a,
            equation10_c4d45,
            sum23_c4d44_param,
            sum23_c4d44,
            vector_slice22_c4d20,
            aggregate25_c4d21_param,
            aggregate25_c4d21,
            constant18_c4d23_param,
            constant18_c4d23,
            product16_c4d22,
            equation16_c4d51,
            sum29_c4d50_param,
            sum29_c4d50,
            sum35_c4d59_param,
            sum35_c4d59,
            constant14_c4d13_param,
            constant14_c4d13,
            vector_slice18_c4d10,
            aggregate21_c4d11_param,
            aggregate21_c4d11,
            product12_c4d12,
            equation12_c4d49,
            sum25_c4d48_param,
            sum25_c4d48,
            constant8_c4cfb_param,
            constant8_c4cfb,
            vector_slice12_c4cf8,
            aggregate15_c4cf9_param,
            aggregate15_c4cf9,
            product6_c4cfa,
            equation6_c4d3d,
            sum19_c4d3c_param,
            sum19_c4d3c,
            constant20_c4d2b_param,
            constant20_c4d2b,
            vector_slice24_c4d28,
            aggregate27_c4d29_param,
            aggregate27_c4d29,
            product18_c4d2a,
            equation18_c4d55,
            sum31_c4d54_param,
            sum31_c4d54,
            sum37_c4d5b_param,
            sum37_c4d5b,
            sum39_c4d5d_param,
            sum39_c4d5d,
            product20_c4d5f,
            constant11_c4d07_param,
            constant11_c4d07,
            vector_slice15_c4d04,
            aggregate18_c4d05_param,
            aggregate18_c4d05,
            product9_c4d06,
            equation9_c4d43,
            sum22_c4d42_param,
            sum22_c4d42,
            constant17_c4d1f_param,
            constant17_c4d1f,
            vector_slice21_c4d1c,
            aggregate24_c4d1d_param,
            aggregate24_c4d1d,
            product15_c4d1e,
            equation15_c4d4f,
            sum28_c4d4e_param,
            sum28_c4d4e,
            vector_slice9_c4cec,
            aggregate12_c4ced_param,
            aggregate12_c4ced,
            constant5_c4cef_param,
            constant5_c4cef,
            product3_c4cee,
            equation3_c4d37,
            sum16_c4d36_param,
            sum16_c4d36,
            sum34_c4d58_param,
            sum34_c4d58,
            vector_slice7_c4ce1,
            aggregate10_c4ce2_param,
            aggregate10_c4ce2,
            constant3_c4ce7_param,
            constant3_c4ce7,
            product1_c4ce6,
            equation1_c4d2d,
            sum14_c4d2c_param,
            sum14_c4d2c,
            vector_slice13_c4cfc,
            aggregate16_c4cfd_param,
            aggregate16_c4cfd,
            constant9_c4cff_param,
            constant9_c4cff,
            product7_c4cfe,
            equation7_c4d3f,
            sum20_c4d3e_param,
            sum20_c4d3e,
            vector_slice19_c4d14,
            aggregate22_c4d15_param,
            aggregate22_c4d15,
            constant15_c4d17_param,
            constant15_c4d17,
            product13_c4d16,
            equation13_c4d4b,
            sum26_c4d4a_param,
            sum26_c4d4a,
            sum32_c4d56_param,
            sum32_c4d56,
            vector_slice8_c4ce8,
            aggregate11_c4ce9_param,
            aggregate11_c4ce9,
            constant4_c4ceb_param,
            constant4_c4ceb,
            product2_c4cea,
            equation2_c4d35,
            sum15_c4d34_param,
            sum15_c4d34,
            vector_slice14_c4d00,
            aggregate17_c4d01_param,
            aggregate17_c4d01,
            constant10_c4d03_param,
            constant10_c4d03,
            product8_c4d02,
            equation8_c4d41,
            sum21_c4d40_param,
            sum21_c4d40,
            constant16_c4d1b_param,
            constant16_c4d1b,
            vector_slice20_c4d18,
            aggregate23_c4d19_param,
            aggregate23_c4d19,
            product14_c4d1a,
            equation14_c4d4d,
            sum27_c4d4c_param,
            sum27_c4d4c,
            sum33_c4d57_param,
            sum33_c4d57,
            sum38_c4d5c_param,
            sum38_c4d5c,
            constant21_c4d60_param,
            constant21_c4d60,
            product19_c4d5e,
            sum40_c4d62_param,
            sum40_c4d62,
            entropdiff_5b137,
            comparison7_c4d63_param,
            comparison7_c4d63,
            gain4_c4d67_param,
            gain4_c4d67,
            comparison8_c4d64_param,
            comparison8_c4d64,
            logical1_c4d66,
            component_output1_c4d68,
        }
    }

    pub fn run(
        &mut self,
        context: &mut Context,
        compare_to_value1_fa02a: &CompareToValueBlock<f64>,
        overall_lower_bucket_c4cd9: &ComponentOutputBlock,
        overall_mid_bucket_c4cd5: &ComponentOutputBlock,
        overall_upper_bucket_c4cd1: &ComponentOutputBlock,
        entropy_diff_5b135: &ComponentInputBlock,
    ) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component2c4cdfComponent iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput1
        self.component_input1_c4ce0
            .run(&compare_to_value1_fa02a.data);
        // Constant22
        self.constant22_c4d61
            .generate(&self.constant22_c4d61_param, context);
        // Constant7
        self.constant7_c4cf7
            .generate(&self.constant7_c4cf7_param, context);
        // Overall_Lower_bucket
        self.overall_lower_bucket_c4ce3
            .run(&overall_lower_bucket_c4cd9.data);
        // VectorSlice11
        self.vector_slice11_c4cf4
            .run(&self.overall_lower_bucket_c4ce3.data);
        // Aggregate14
        self.aggregate14_c4cf5.process(
            &self.aggregate14_c4cf5_param,
            context,
            &self.vector_slice11_c4cf4.data.to_pass(),
        );
        // Product5
        self.product5_c4cf6.run(&vec![
            &self.aggregate14_c4cf5.data,
            &self.constant7_c4cf7.data,
        ]);
        let equation5_c4d3b_expression = 0.434294481903252 * self.product5_c4cf6.data.scalar().ln();

        self.equation5_c4d3b
            .run(&BlockData::from_scalar(equation5_c4d3b_expression));
        // Sum18
        self.sum18_c4d3a.process(
            &self.sum18_c4d3a_param,
            context,
            (
                self.product5_c4cf6.data.to_pass(),
                self.equation5_c4d3b.data.to_pass(),
            ),
        );
        // Overall_Mid_bucket
        self.overall_mid_bucket_c4ce4
            .run(&overall_mid_bucket_c4cd5.data);
        // VectorSlice17
        self.vector_slice17_c4d0c
            .run(&self.overall_mid_bucket_c4ce4.data);
        // Aggregate20
        self.aggregate20_c4d0d.process(
            &self.aggregate20_c4d0d_param,
            context,
            &self.vector_slice17_c4d0c.data.to_pass(),
        );
        // Constant13
        self.constant13_c4d0f
            .generate(&self.constant13_c4d0f_param, context);
        // Product11
        self.product11_c4d0e.run(&vec![
            &self.aggregate20_c4d0d.data,
            &self.constant13_c4d0f.data,
        ]);
        let equation11_c4d47_expression =
            0.434294481903252 * self.product11_c4d0e.data.scalar().ln();

        self.equation11_c4d47
            .run(&BlockData::from_scalar(equation11_c4d47_expression));
        // Sum24
        self.sum24_c4d46.process(
            &self.sum24_c4d46_param,
            context,
            (
                self.product11_c4d0e.data.to_pass(),
                self.equation11_c4d47.data.to_pass(),
            ),
        );
        // Constant19
        self.constant19_c4d27
            .generate(&self.constant19_c4d27_param, context);
        // Overall_Upper_bucket
        self.overall_upper_bucket_c4ce5
            .run(&overall_upper_bucket_c4cd1.data);
        // VectorSlice23
        self.vector_slice23_c4d24
            .run(&self.overall_upper_bucket_c4ce5.data);
        // Aggregate26
        self.aggregate26_c4d25.process(
            &self.aggregate26_c4d25_param,
            context,
            &self.vector_slice23_c4d24.data.to_pass(),
        );
        // Product17
        self.product17_c4d26.run(&vec![
            &self.aggregate26_c4d25.data,
            &self.constant19_c4d27.data,
        ]);
        let equation17_c4d53_expression =
            0.434294481903252 * self.product17_c4d26.data.scalar().ln();

        self.equation17_c4d53
            .run(&BlockData::from_scalar(equation17_c4d53_expression));
        // Sum30
        self.sum30_c4d52.process(
            &self.sum30_c4d52_param,
            context,
            (
                self.product17_c4d26.data.to_pass(),
                self.equation17_c4d53.data.to_pass(),
            ),
        );
        // Sum36
        self.sum36_c4d5a.process(
            &self.sum36_c4d5a_param,
            context,
            (
                self.sum18_c4d3a.data.to_pass(),
                self.sum24_c4d46.data.to_pass(),
                self.sum30_c4d52.data.to_pass(),
            ),
        );
        // VectorSlice10
        self.vector_slice10_c4cf0
            .run(&self.overall_lower_bucket_c4ce3.data);
        // Aggregate13
        self.aggregate13_c4cf1.process(
            &self.aggregate13_c4cf1_param,
            context,
            &self.vector_slice10_c4cf0.data.to_pass(),
        );
        // Constant6
        self.constant6_c4cf3
            .generate(&self.constant6_c4cf3_param, context);
        // Product4
        self.product4_c4cf2.run(&vec![
            &self.aggregate13_c4cf1.data,
            &self.constant6_c4cf3.data,
        ]);
        let equation4_c4d39_expression = 0.434294481903252 * self.product4_c4cf2.data.scalar().ln();

        self.equation4_c4d39
            .run(&BlockData::from_scalar(equation4_c4d39_expression));
        // Sum17
        self.sum17_c4d38.process(
            &self.sum17_c4d38_param,
            context,
            (
                self.product4_c4cf2.data.to_pass(),
                self.equation4_c4d39.data.to_pass(),
            ),
        );
        // VectorSlice16
        self.vector_slice16_c4d08
            .run(&self.overall_mid_bucket_c4ce4.data);
        // Aggregate19
        self.aggregate19_c4d09.process(
            &self.aggregate19_c4d09_param,
            context,
            &self.vector_slice16_c4d08.data.to_pass(),
        );
        // Constant12
        self.constant12_c4d0b
            .generate(&self.constant12_c4d0b_param, context);
        // Product10
        self.product10_c4d0a.run(&vec![
            &self.aggregate19_c4d09.data,
            &self.constant12_c4d0b.data,
        ]);
        let equation10_c4d45_expression =
            0.434294481903252 * self.product10_c4d0a.data.scalar().ln();

        self.equation10_c4d45
            .run(&BlockData::from_scalar(equation10_c4d45_expression));
        // Sum23
        self.sum23_c4d44.process(
            &self.sum23_c4d44_param,
            context,
            (
                self.product10_c4d0a.data.to_pass(),
                self.equation10_c4d45.data.to_pass(),
            ),
        );
        // VectorSlice22
        self.vector_slice22_c4d20
            .run(&self.overall_upper_bucket_c4ce5.data);
        // Aggregate25
        self.aggregate25_c4d21.process(
            &self.aggregate25_c4d21_param,
            context,
            &self.vector_slice22_c4d20.data.to_pass(),
        );
        // Constant18
        self.constant18_c4d23
            .generate(&self.constant18_c4d23_param, context);
        // Product16
        self.product16_c4d22.run(&vec![
            &self.aggregate25_c4d21.data,
            &self.constant18_c4d23.data,
        ]);
        let equation16_c4d51_expression =
            0.434294481903252 * self.product16_c4d22.data.scalar().ln();

        self.equation16_c4d51
            .run(&BlockData::from_scalar(equation16_c4d51_expression));
        // Sum29
        self.sum29_c4d50.process(
            &self.sum29_c4d50_param,
            context,
            (
                self.product16_c4d22.data.to_pass(),
                self.equation16_c4d51.data.to_pass(),
            ),
        );
        // Sum35
        self.sum35_c4d59.process(
            &self.sum35_c4d59_param,
            context,
            (
                self.sum17_c4d38.data.to_pass(),
                self.sum23_c4d44.data.to_pass(),
                self.sum29_c4d50.data.to_pass(),
            ),
        );
        // Constant14
        self.constant14_c4d13
            .generate(&self.constant14_c4d13_param, context);
        // VectorSlice18
        self.vector_slice18_c4d10
            .run(&self.overall_mid_bucket_c4ce4.data);
        // Aggregate21
        self.aggregate21_c4d11.process(
            &self.aggregate21_c4d11_param,
            context,
            &self.vector_slice18_c4d10.data.to_pass(),
        );
        // Product12
        self.product12_c4d12.run(&vec![
            &self.aggregate21_c4d11.data,
            &self.constant14_c4d13.data,
        ]);
        let equation12_c4d49_expression =
            0.434294481903252 * self.product12_c4d12.data.scalar().ln();

        self.equation12_c4d49
            .run(&BlockData::from_scalar(equation12_c4d49_expression));
        // Sum25
        self.sum25_c4d48.process(
            &self.sum25_c4d48_param,
            context,
            (
                self.product12_c4d12.data.to_pass(),
                self.equation12_c4d49.data.to_pass(),
            ),
        );
        // Constant8
        self.constant8_c4cfb
            .generate(&self.constant8_c4cfb_param, context);
        // VectorSlice12
        self.vector_slice12_c4cf8
            .run(&self.overall_lower_bucket_c4ce3.data);
        // Aggregate15
        self.aggregate15_c4cf9.process(
            &self.aggregate15_c4cf9_param,
            context,
            &self.vector_slice12_c4cf8.data.to_pass(),
        );
        // Product6
        self.product6_c4cfa.run(&vec![
            &self.aggregate15_c4cf9.data,
            &self.constant8_c4cfb.data,
        ]);
        let equation6_c4d3d_expression = 0.434294481903252 * self.product6_c4cfa.data.scalar().ln();

        self.equation6_c4d3d
            .run(&BlockData::from_scalar(equation6_c4d3d_expression));
        // Sum19
        self.sum19_c4d3c.process(
            &self.sum19_c4d3c_param,
            context,
            (
                self.product6_c4cfa.data.to_pass(),
                self.equation6_c4d3d.data.to_pass(),
            ),
        );
        // Constant20
        self.constant20_c4d2b
            .generate(&self.constant20_c4d2b_param, context);
        // VectorSlice24
        self.vector_slice24_c4d28
            .run(&self.overall_upper_bucket_c4ce5.data);
        // Aggregate27
        self.aggregate27_c4d29.process(
            &self.aggregate27_c4d29_param,
            context,
            &self.vector_slice24_c4d28.data.to_pass(),
        );
        // Product18
        self.product18_c4d2a.run(&vec![
            &self.aggregate27_c4d29.data,
            &self.constant20_c4d2b.data,
        ]);
        let equation18_c4d55_expression =
            0.434294481903252 * self.product18_c4d2a.data.scalar().ln();

        self.equation18_c4d55
            .run(&BlockData::from_scalar(equation18_c4d55_expression));
        // Sum31
        self.sum31_c4d54.process(
            &self.sum31_c4d54_param,
            context,
            (
                self.product18_c4d2a.data.to_pass(),
                self.equation18_c4d55.data.to_pass(),
            ),
        );
        // Sum37
        self.sum37_c4d5b.process(
            &self.sum37_c4d5b_param,
            context,
            (
                self.sum19_c4d3c.data.to_pass(),
                self.sum25_c4d48.data.to_pass(),
                self.sum31_c4d54.data.to_pass(),
            ),
        );
        // Sum39
        self.sum39_c4d5d.process(
            &self.sum39_c4d5d_param,
            context,
            (
                self.sum35_c4d59.data.to_pass(),
                self.sum36_c4d5a.data.to_pass(),
                self.sum37_c4d5b.data.to_pass(),
            ),
        );
        // Product20
        self.product20_c4d5f
            .run(&vec![&self.sum39_c4d5d.data, &self.constant22_c4d61.data]);
        // Constant11
        self.constant11_c4d07
            .generate(&self.constant11_c4d07_param, context);
        // VectorSlice15
        self.vector_slice15_c4d04
            .run(&self.overall_mid_bucket_c4ce4.data);
        // Aggregate18
        self.aggregate18_c4d05.process(
            &self.aggregate18_c4d05_param,
            context,
            &self.vector_slice15_c4d04.data.to_pass(),
        );
        // Product9
        self.product9_c4d06.run(&vec![
            &self.aggregate18_c4d05.data,
            &self.constant11_c4d07.data,
        ]);
        let equation9_c4d43_expression = 0.434294481903252 * self.product9_c4d06.data.scalar().ln();

        self.equation9_c4d43
            .run(&BlockData::from_scalar(equation9_c4d43_expression));
        // Sum22
        self.sum22_c4d42.process(
            &self.sum22_c4d42_param,
            context,
            (
                self.product9_c4d06.data.to_pass(),
                self.equation9_c4d43.data.to_pass(),
            ),
        );
        // Constant17
        self.constant17_c4d1f
            .generate(&self.constant17_c4d1f_param, context);
        // VectorSlice21
        self.vector_slice21_c4d1c
            .run(&self.overall_upper_bucket_c4ce5.data);
        // Aggregate24
        self.aggregate24_c4d1d.process(
            &self.aggregate24_c4d1d_param,
            context,
            &self.vector_slice21_c4d1c.data.to_pass(),
        );
        // Product15
        self.product15_c4d1e.run(&vec![
            &self.aggregate24_c4d1d.data,
            &self.constant17_c4d1f.data,
        ]);
        let equation15_c4d4f_expression =
            0.434294481903252 * self.product15_c4d1e.data.scalar().ln();

        self.equation15_c4d4f
            .run(&BlockData::from_scalar(equation15_c4d4f_expression));
        // Sum28
        self.sum28_c4d4e.process(
            &self.sum28_c4d4e_param,
            context,
            (
                self.product15_c4d1e.data.to_pass(),
                self.equation15_c4d4f.data.to_pass(),
            ),
        );
        // VectorSlice9
        self.vector_slice9_c4cec
            .run(&self.overall_lower_bucket_c4ce3.data);
        // Aggregate12
        self.aggregate12_c4ced.process(
            &self.aggregate12_c4ced_param,
            context,
            &self.vector_slice9_c4cec.data.to_pass(),
        );
        // Constant5
        self.constant5_c4cef
            .generate(&self.constant5_c4cef_param, context);
        // Product3
        self.product3_c4cee.run(&vec![
            &self.aggregate12_c4ced.data,
            &self.constant5_c4cef.data,
        ]);
        let equation3_c4d37_expression = 0.434294481903252 * self.product3_c4cee.data.scalar().ln();

        self.equation3_c4d37
            .run(&BlockData::from_scalar(equation3_c4d37_expression));
        // Sum16
        self.sum16_c4d36.process(
            &self.sum16_c4d36_param,
            context,
            (
                self.product3_c4cee.data.to_pass(),
                self.equation3_c4d37.data.to_pass(),
            ),
        );
        // Sum34
        self.sum34_c4d58.process(
            &self.sum34_c4d58_param,
            context,
            (
                self.sum16_c4d36.data.to_pass(),
                self.sum22_c4d42.data.to_pass(),
                self.sum28_c4d4e.data.to_pass(),
            ),
        );
        // VectorSlice7
        self.vector_slice7_c4ce1
            .run(&self.overall_lower_bucket_c4ce3.data);
        // Aggregate10
        self.aggregate10_c4ce2.process(
            &self.aggregate10_c4ce2_param,
            context,
            &self.vector_slice7_c4ce1.data.to_pass(),
        );
        // Constant3
        self.constant3_c4ce7
            .generate(&self.constant3_c4ce7_param, context);
        // Product1
        self.product1_c4ce6.run(&vec![
            &self.aggregate10_c4ce2.data,
            &self.constant3_c4ce7.data,
        ]);
        let equation1_c4d2d_expression = 0.434294481903252 * self.product1_c4ce6.data.scalar().ln();

        self.equation1_c4d2d
            .run(&BlockData::from_scalar(equation1_c4d2d_expression));
        // Sum14
        self.sum14_c4d2c.process(
            &self.sum14_c4d2c_param,
            context,
            (
                self.product1_c4ce6.data.to_pass(),
                self.equation1_c4d2d.data.to_pass(),
            ),
        );
        // VectorSlice13
        self.vector_slice13_c4cfc
            .run(&self.overall_mid_bucket_c4ce4.data);
        // Aggregate16
        self.aggregate16_c4cfd.process(
            &self.aggregate16_c4cfd_param,
            context,
            &self.vector_slice13_c4cfc.data.to_pass(),
        );
        // Constant9
        self.constant9_c4cff
            .generate(&self.constant9_c4cff_param, context);
        // Product7
        self.product7_c4cfe.run(&vec![
            &self.aggregate16_c4cfd.data,
            &self.constant9_c4cff.data,
        ]);
        let equation7_c4d3f_expression = 0.434294481903252 * self.product7_c4cfe.data.scalar().ln();

        self.equation7_c4d3f
            .run(&BlockData::from_scalar(equation7_c4d3f_expression));
        // Sum20
        self.sum20_c4d3e.process(
            &self.sum20_c4d3e_param,
            context,
            (
                self.product7_c4cfe.data.to_pass(),
                self.equation7_c4d3f.data.to_pass(),
            ),
        );
        // VectorSlice19
        self.vector_slice19_c4d14
            .run(&self.overall_upper_bucket_c4ce5.data);
        // Aggregate22
        self.aggregate22_c4d15.process(
            &self.aggregate22_c4d15_param,
            context,
            &self.vector_slice19_c4d14.data.to_pass(),
        );
        // Constant15
        self.constant15_c4d17
            .generate(&self.constant15_c4d17_param, context);
        // Product13
        self.product13_c4d16.run(&vec![
            &self.aggregate22_c4d15.data,
            &self.constant15_c4d17.data,
        ]);
        let equation13_c4d4b_expression =
            0.434294481903252 * self.product13_c4d16.data.scalar().ln();

        self.equation13_c4d4b
            .run(&BlockData::from_scalar(equation13_c4d4b_expression));
        // Sum26
        self.sum26_c4d4a.process(
            &self.sum26_c4d4a_param,
            context,
            (
                self.product13_c4d16.data.to_pass(),
                self.equation13_c4d4b.data.to_pass(),
            ),
        );
        // Sum32
        self.sum32_c4d56.process(
            &self.sum32_c4d56_param,
            context,
            (
                self.sum14_c4d2c.data.to_pass(),
                self.sum20_c4d3e.data.to_pass(),
                self.sum26_c4d4a.data.to_pass(),
            ),
        );
        // VectorSlice8
        self.vector_slice8_c4ce8
            .run(&self.overall_lower_bucket_c4ce3.data);
        // Aggregate11
        self.aggregate11_c4ce9.process(
            &self.aggregate11_c4ce9_param,
            context,
            &self.vector_slice8_c4ce8.data.to_pass(),
        );
        // Constant4
        self.constant4_c4ceb
            .generate(&self.constant4_c4ceb_param, context);
        // Product2
        self.product2_c4cea.run(&vec![
            &self.aggregate11_c4ce9.data,
            &self.constant4_c4ceb.data,
        ]);
        let equation2_c4d35_expression = 0.434294481903252 * self.product2_c4cea.data.scalar().ln();

        self.equation2_c4d35
            .run(&BlockData::from_scalar(equation2_c4d35_expression));
        // Sum15
        self.sum15_c4d34.process(
            &self.sum15_c4d34_param,
            context,
            (
                self.product2_c4cea.data.to_pass(),
                self.equation2_c4d35.data.to_pass(),
            ),
        );
        // VectorSlice14
        self.vector_slice14_c4d00
            .run(&self.overall_mid_bucket_c4ce4.data);
        // Aggregate17
        self.aggregate17_c4d01.process(
            &self.aggregate17_c4d01_param,
            context,
            &self.vector_slice14_c4d00.data.to_pass(),
        );
        // Constant10
        self.constant10_c4d03
            .generate(&self.constant10_c4d03_param, context);
        // Product8
        self.product8_c4d02.run(&vec![
            &self.aggregate17_c4d01.data,
            &self.constant10_c4d03.data,
        ]);
        let equation8_c4d41_expression = 0.434294481903252 * self.product8_c4d02.data.scalar().ln();

        self.equation8_c4d41
            .run(&BlockData::from_scalar(equation8_c4d41_expression));
        // Sum21
        self.sum21_c4d40.process(
            &self.sum21_c4d40_param,
            context,
            (
                self.product8_c4d02.data.to_pass(),
                self.equation8_c4d41.data.to_pass(),
            ),
        );
        // Constant16
        self.constant16_c4d1b
            .generate(&self.constant16_c4d1b_param, context);
        // VectorSlice20
        self.vector_slice20_c4d18
            .run(&self.overall_upper_bucket_c4ce5.data);
        // Aggregate23
        self.aggregate23_c4d19.process(
            &self.aggregate23_c4d19_param,
            context,
            &self.vector_slice20_c4d18.data.to_pass(),
        );
        // Product14
        self.product14_c4d1a.run(&vec![
            &self.aggregate23_c4d19.data,
            &self.constant16_c4d1b.data,
        ]);
        let equation14_c4d4d_expression =
            0.434294481903252 * self.product14_c4d1a.data.scalar().ln();

        self.equation14_c4d4d
            .run(&BlockData::from_scalar(equation14_c4d4d_expression));
        // Sum27
        self.sum27_c4d4c.process(
            &self.sum27_c4d4c_param,
            context,
            (
                self.product14_c4d1a.data.to_pass(),
                self.equation14_c4d4d.data.to_pass(),
            ),
        );
        // Sum33
        self.sum33_c4d57.process(
            &self.sum33_c4d57_param,
            context,
            (
                self.sum15_c4d34.data.to_pass(),
                self.sum21_c4d40.data.to_pass(),
                self.sum27_c4d4c.data.to_pass(),
            ),
        );
        // Sum38
        self.sum38_c4d5c.process(
            &self.sum38_c4d5c_param,
            context,
            (
                self.sum32_c4d56.data.to_pass(),
                self.sum33_c4d57.data.to_pass(),
                self.sum34_c4d58.data.to_pass(),
            ),
        );
        // Constant21
        self.constant21_c4d60
            .generate(&self.constant21_c4d60_param, context);
        // Product19
        self.product19_c4d5e
            .run(&vec![&self.sum38_c4d5c.data, &self.constant21_c4d60.data]);
        // Sum40
        self.sum40_c4d62.process(
            &self.sum40_c4d62_param,
            context,
            (
                self.product19_c4d5e.data.to_pass(),
                self.product20_c4d5f.data.to_pass(),
            ),
        );
        // Entropdiff
        self.entropdiff_5b137.run(&entropy_diff_5b135.data);
        // Comparison7
        self.comparison7_c4d63.process(
            &self.comparison7_c4d63_param,
            context,
            (
                self.sum40_c4d62.data.to_pass(),
                self.entropdiff_5b137.data.to_pass(),
            ),
        );
        // Gain4
        self.gain4_c4d67.process(
            &self.gain4_c4d67_param,
            context,
            self.entropdiff_5b137.data.to_pass(),
        );
        // Comparison8
        self.comparison8_c4d64.process(
            &self.comparison8_c4d64_param,
            context,
            (
                self.sum40_c4d62.data.to_pass(),
                self.gain4_c4d67.data.to_pass(),
            ),
        );
        // Logical1
        self.logical1_c4d66.run(&vec![
            &self.comparison7_c4d63.data,
            &self.comparison8_c4d64.data,
        ]);
        // ComponentOutput1
        self.component_output1_c4d68.run(&self.logical1_c4d66.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Component1f9fa4Component {
    last_time_s: f64,
    component_input3_f9fa5: ComponentInputBlock,
    vector_merge1_f9fa6: VectorMergeBlock,
    vector_merge2_f9faf: VectorMergeBlock,
    vector_merge3_f9fb5: VectorMergeBlock,
    component_output2_f9fb8: ComponentOutputBlock,
}

impl Component1f9fa4Component {
    pub fn new(_context: &Context) -> Self {
        let component_input3_f9fa5_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput3
        let component_input3_f9fa5 =
            ComponentInputBlock::new("ComponentInput3", &component_input3_f9fa5_ic);

        let vector_merge1_f9fa6_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge1
        let vector_merge1_f9fa6 = VectorMergeBlock::new("VectorMerge1", &vector_merge1_f9fa6_ic);

        let vector_merge2_f9faf_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge2
        let vector_merge2_f9faf = VectorMergeBlock::new("VectorMerge2", &vector_merge2_f9faf_ic);

        let vector_merge3_f9fb5_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge3
        let vector_merge3_f9fb5 = VectorMergeBlock::new("VectorMerge3", &vector_merge3_f9fb5_ic);

        let component_output2_f9fb8_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // ComponentOutput2
        let component_output2_f9fb8 =
            ComponentOutputBlock::new("ComponentOutput2", &component_output2_f9fb8_ic);

        Component1f9fa4Component {
            last_time_s: -1.0,
            component_input3_f9fa5,
            vector_merge1_f9fa6,
            vector_merge2_f9faf,
            vector_merge3_f9fb5,
            component_output2_f9fb8,
        }
    }

    pub fn run(&mut self, context: &mut Context, sum2_c4ca8: &SumBlock<(f64, f64)>) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component1f9fa4Component iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput3
        self.component_input3_f9fa5.run(&sum2_c4ca8.data);
        // VectorMerge1
        self.vector_merge1_f9fa6.run(&vec![
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
        ]);
        // VectorMerge2
        self.vector_merge2_f9faf.run(&vec![
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
            &self.component_input3_f9fa5.data,
        ]);
        // VectorMerge3
        self.vector_merge3_f9fb5.run(&vec![
            &self.vector_merge1_f9fa6.data,
            &self.vector_merge2_f9faf.data,
        ]);
        // ComponentOutput2
        self.component_output2_f9fb8
            .run(&self.vector_merge3_f9fb5.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Component3f9fbcComponent {
    last_time_s: f64,
    component_input3_f9fbd: ComponentInputBlock,
    vector_merge1_f9fbe: VectorMergeBlock,
    vector_merge2_f9fbf: VectorMergeBlock,
    vector_merge3_f9fc0: VectorMergeBlock,
    component_output2_f9fc1: ComponentOutputBlock,
}

impl Component3f9fbcComponent {
    pub fn new(_context: &Context) -> Self {
        let component_input3_f9fbd_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput3
        let component_input3_f9fbd =
            ComponentInputBlock::new("ComponentInput3", &component_input3_f9fbd_ic);

        let vector_merge1_f9fbe_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge1
        let vector_merge1_f9fbe = VectorMergeBlock::new("VectorMerge1", &vector_merge1_f9fbe_ic);

        let vector_merge2_f9fbf_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge2
        let vector_merge2_f9fbf = VectorMergeBlock::new("VectorMerge2", &vector_merge2_f9fbf_ic);

        let vector_merge3_f9fc0_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge3
        let vector_merge3_f9fc0 = VectorMergeBlock::new("VectorMerge3", &vector_merge3_f9fc0_ic);

        let component_output2_f9fc1_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // ComponentOutput2
        let component_output2_f9fc1 =
            ComponentOutputBlock::new("ComponentOutput2", &component_output2_f9fc1_ic);

        Component3f9fbcComponent {
            last_time_s: -1.0,
            component_input3_f9fbd,
            vector_merge1_f9fbe,
            vector_merge2_f9fbf,
            vector_merge3_f9fc0,
            component_output2_f9fc1,
        }
    }

    pub fn run(&mut self, context: &mut Context, sum5_c4cab: &SumBlock<(f64, f64)>) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component3f9fbcComponent iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput3
        self.component_input3_f9fbd.run(&sum5_c4cab.data);
        // VectorMerge1
        self.vector_merge1_f9fbe.run(&vec![
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
        ]);
        // VectorMerge2
        self.vector_merge2_f9fbf.run(&vec![
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
            &self.component_input3_f9fbd.data,
        ]);
        // VectorMerge3
        self.vector_merge3_f9fc0.run(&vec![
            &self.vector_merge1_f9fbe.data,
            &self.vector_merge2_f9fbf.data,
        ]);
        // ComponentOutput2
        self.component_output2_f9fc1
            .run(&self.vector_merge3_f9fc0.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Component4f9fd1Component {
    last_time_s: f64,
    component_input3_f9fd2: ComponentInputBlock,
    vector_merge1_f9fd3: VectorMergeBlock,
    vector_merge2_f9fd4: VectorMergeBlock,
    vector_merge3_f9fd5: VectorMergeBlock,
    component_output2_f9fd6: ComponentOutputBlock,
}

impl Component4f9fd1Component {
    pub fn new(_context: &Context) -> Self {
        let component_input3_f9fd2_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput3
        let component_input3_f9fd2 =
            ComponentInputBlock::new("ComponentInput3", &component_input3_f9fd2_ic);

        let vector_merge1_f9fd3_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge1
        let vector_merge1_f9fd3 = VectorMergeBlock::new("VectorMerge1", &vector_merge1_f9fd3_ic);

        let vector_merge2_f9fd4_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge2
        let vector_merge2_f9fd4 = VectorMergeBlock::new("VectorMerge2", &vector_merge2_f9fd4_ic);

        let vector_merge3_f9fd5_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge3
        let vector_merge3_f9fd5 = VectorMergeBlock::new("VectorMerge3", &vector_merge3_f9fd5_ic);

        let component_output2_f9fd6_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // ComponentOutput2
        let component_output2_f9fd6 =
            ComponentOutputBlock::new("ComponentOutput2", &component_output2_f9fd6_ic);

        Component4f9fd1Component {
            last_time_s: -1.0,
            component_input3_f9fd2,
            vector_merge1_f9fd3,
            vector_merge2_f9fd4,
            vector_merge3_f9fd5,
            component_output2_f9fd6,
        }
    }

    pub fn run(&mut self, context: &mut Context, sum8_c4cac: &SumBlock<(f64, f64)>) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component4f9fd1Component iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput3
        self.component_input3_f9fd2.run(&sum8_c4cac.data);
        // VectorMerge1
        self.vector_merge1_f9fd3.run(&vec![
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
        ]);
        // VectorMerge2
        self.vector_merge2_f9fd4.run(&vec![
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
            &self.component_input3_f9fd2.data,
        ]);
        // VectorMerge3
        self.vector_merge3_f9fd5.run(&vec![
            &self.vector_merge1_f9fd3.data,
            &self.vector_merge2_f9fd4.data,
        ]);
        // ComponentOutput2
        self.component_output2_f9fd6
            .run(&self.vector_merge3_f9fd5.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Component5f9fe6Component {
    last_time_s: f64,
    component_input3_f9fe7: ComponentInputBlock,
    vector_merge1_f9fe8: VectorMergeBlock,
    vector_merge2_f9fe9: VectorMergeBlock,
    vector_merge3_f9fea: VectorMergeBlock,
    component_output2_f9feb: ComponentOutputBlock,
}

impl Component5f9fe6Component {
    pub fn new(_context: &Context) -> Self {
        let component_input3_f9fe7_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput3
        let component_input3_f9fe7 =
            ComponentInputBlock::new("ComponentInput3", &component_input3_f9fe7_ic);

        let vector_merge1_f9fe8_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge1
        let vector_merge1_f9fe8 = VectorMergeBlock::new("VectorMerge1", &vector_merge1_f9fe8_ic);

        let vector_merge2_f9fe9_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge2
        let vector_merge2_f9fe9 = VectorMergeBlock::new("VectorMerge2", &vector_merge2_f9fe9_ic);

        let vector_merge3_f9fea_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge3
        let vector_merge3_f9fea = VectorMergeBlock::new("VectorMerge3", &vector_merge3_f9fea_ic);

        let component_output2_f9feb_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // ComponentOutput2
        let component_output2_f9feb =
            ComponentOutputBlock::new("ComponentOutput2", &component_output2_f9feb_ic);

        Component5f9fe6Component {
            last_time_s: -1.0,
            component_input3_f9fe7,
            vector_merge1_f9fe8,
            vector_merge2_f9fe9,
            vector_merge3_f9fea,
            component_output2_f9feb,
        }
    }

    pub fn run(&mut self, context: &mut Context, sum3_c4cbc: &SumBlock<(f64, f64)>) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component5f9fe6Component iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput3
        self.component_input3_f9fe7.run(&sum3_c4cbc.data);
        // VectorMerge1
        self.vector_merge1_f9fe8.run(&vec![
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
        ]);
        // VectorMerge2
        self.vector_merge2_f9fe9.run(&vec![
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
            &self.component_input3_f9fe7.data,
        ]);
        // VectorMerge3
        self.vector_merge3_f9fea.run(&vec![
            &self.vector_merge1_f9fe8.data,
            &self.vector_merge2_f9fe9.data,
        ]);
        // ComponentOutput2
        self.component_output2_f9feb
            .run(&self.vector_merge3_f9fea.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Component6f9ffbComponent {
    last_time_s: f64,
    component_input3_f9ffc: ComponentInputBlock,
    vector_merge1_f9ffd: VectorMergeBlock,
    vector_merge2_f9ffe: VectorMergeBlock,
    vector_merge3_f9fff: VectorMergeBlock,
    component_output2_fa000: ComponentOutputBlock,
}

impl Component6f9ffbComponent {
    pub fn new(_context: &Context) -> Self {
        let component_input3_f9ffc_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput3
        let component_input3_f9ffc =
            ComponentInputBlock::new("ComponentInput3", &component_input3_f9ffc_ic);

        let vector_merge1_f9ffd_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge1
        let vector_merge1_f9ffd = VectorMergeBlock::new("VectorMerge1", &vector_merge1_f9ffd_ic);

        let vector_merge2_f9ffe_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge2
        let vector_merge2_f9ffe = VectorMergeBlock::new("VectorMerge2", &vector_merge2_f9ffe_ic);

        let vector_merge3_f9fff_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge3
        let vector_merge3_f9fff = VectorMergeBlock::new("VectorMerge3", &vector_merge3_f9fff_ic);

        let component_output2_fa000_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // ComponentOutput2
        let component_output2_fa000 =
            ComponentOutputBlock::new("ComponentOutput2", &component_output2_fa000_ic);

        Component6f9ffbComponent {
            last_time_s: -1.0,
            component_input3_f9ffc,
            vector_merge1_f9ffd,
            vector_merge2_f9ffe,
            vector_merge3_f9fff,
            component_output2_fa000,
        }
    }

    pub fn run(&mut self, context: &mut Context, sum6_c4cbe: &SumBlock<(f64, f64)>) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component6f9ffbComponent iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput3
        self.component_input3_f9ffc.run(&sum6_c4cbe.data);
        // VectorMerge1
        self.vector_merge1_f9ffd.run(&vec![
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
        ]);
        // VectorMerge2
        self.vector_merge2_f9ffe.run(&vec![
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
            &self.component_input3_f9ffc.data,
        ]);
        // VectorMerge3
        self.vector_merge3_f9fff.run(&vec![
            &self.vector_merge1_f9ffd.data,
            &self.vector_merge2_f9ffe.data,
        ]);
        // ComponentOutput2
        self.component_output2_fa000
            .run(&self.vector_merge3_f9fff.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Component7fa010Component {
    last_time_s: f64,
    component_input3_fa011: ComponentInputBlock,
    vector_merge1_fa012: VectorMergeBlock,
    vector_merge2_fa013: VectorMergeBlock,
    vector_merge3_fa014: VectorMergeBlock,
    component_output2_fa015: ComponentOutputBlock,
}

impl Component7fa010Component {
    pub fn new(_context: &Context) -> Self {
        let component_input3_fa011_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput3
        let component_input3_fa011 =
            ComponentInputBlock::new("ComponentInput3", &component_input3_fa011_ic);

        let vector_merge1_fa012_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge1
        let vector_merge1_fa012 = VectorMergeBlock::new("VectorMerge1", &vector_merge1_fa012_ic);

        let vector_merge2_fa013_ic = BlockData::new(1, 5, &[0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge2
        let vector_merge2_fa013 = VectorMergeBlock::new("VectorMerge2", &vector_merge2_fa013_ic);

        let vector_merge3_fa014_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge3
        let vector_merge3_fa014 = VectorMergeBlock::new("VectorMerge3", &vector_merge3_fa014_ic);

        let component_output2_fa015_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // ComponentOutput2
        let component_output2_fa015 =
            ComponentOutputBlock::new("ComponentOutput2", &component_output2_fa015_ic);

        Component7fa010Component {
            last_time_s: -1.0,
            component_input3_fa011,
            vector_merge1_fa012,
            vector_merge2_fa013,
            vector_merge3_fa014,
            component_output2_fa015,
        }
    }

    pub fn run(&mut self, context: &mut Context, sum9_c4cc3: &SumBlock<(f64, f64)>) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Component7fa010Component iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput3
        self.component_input3_fa011.run(&sum9_c4cc3.data);
        // VectorMerge1
        self.vector_merge1_fa012.run(&vec![
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
        ]);
        // VectorMerge2
        self.vector_merge2_fa013.run(&vec![
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
            &self.component_input3_fa011.data,
        ]);
        // VectorMerge3
        self.vector_merge3_fa014.run(&vec![
            &self.vector_merge1_fa012.data,
            &self.vector_merge2_fa013.data,
        ]);
        // ComponentOutput2
        self.component_output2_fa015
            .run(&self.vector_merge3_fa014.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Crashdetection1c4ca4Component {
    last_time_s: f64,
    component_input2_c4ca5: ComponentInputBlock,
    speed_c4cc0: ComponentInputBlock,
    arg_min_max3_f9f90_param: <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    arg_min_max3_f9f90: ArgMinMaxBlock<Matrix<1, 10, f64>>,
    arg_min_max4_f9f91_param: <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    arg_min_max4_f9f91: ArgMinMaxBlock<Matrix<1, 10, f64>>,
    sum4_c4cbd_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum4_c4cbd: SumBlock<(f64, f64)>,
    gain2_c4cb7_param: <GainBlock<f64, f64> as ProcessBlock>::Parameters,
    gain2_c4cb7: GainBlock<f64, f64>,
    sum5_c4cab_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum5_c4cab: SumBlock<(f64, f64)>,
    component3f9fbc_component: Component3f9fbcComponent,
    comparison3_c4cb8_param: <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    comparison3_c4cb8: ComparisonBlock<Matrix<1, 10, f64>>,
    aggregate3_c4cb1_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate3_c4cb1: AggregateBlock<Matrix<1, 10, f64>>,
    current_c4ca6: ComponentInputBlock,
    arg_min_max1_f9f89_param: <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    arg_min_max1_f9f89: ArgMinMaxBlock<Matrix<1, 10, f64>>,
    arg_min_max2_f9f8a_param: <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    arg_min_max2_f9f8a: ArgMinMaxBlock<Matrix<1, 10, f64>>,
    sum1_c4ca7_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum1_c4ca7: SumBlock<(f64, f64)>,
    gain1_c4cad_param: <GainBlock<f64, f64> as ProcessBlock>::Parameters,
    gain1_c4cad: GainBlock<f64, f64>,
    sum2_c4ca8_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum2_c4ca8: SumBlock<(f64, f64)>,
    component1f9fa4_component: Component1f9fa4Component,
    comparison1_c4ca9_param: <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    comparison1_c4ca9: ComparisonBlock<Matrix<1, 10, f64>>,
    aggregate1_c4cae_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate1_c4cae: AggregateBlock<Matrix<1, 10, f64>>,
    ay_c4cc6: ComponentInputBlock,
    arg_min_max5_f9f92_param: <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    arg_min_max5_f9f92: ArgMinMaxBlock<Matrix<1, 10, f64>>,
    arg_min_max6_f9f93_param: <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    arg_min_max6_f9f93: ArgMinMaxBlock<Matrix<1, 10, f64>>,
    sum7_c4cc1_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum7_c4cc1: SumBlock<(f64, f64)>,
    gain3_c4cc2_param: <GainBlock<f64, f64> as ProcessBlock>::Parameters,
    gain3_c4cc2: GainBlock<f64, f64>,
    sum8_c4cac_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum8_c4cac: SumBlock<(f64, f64)>,
    component4f9fd1_component: Component4f9fd1Component,
    comparison5_c4cb9_param: <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    comparison5_c4cb9: ComparisonBlock<Matrix<1, 10, f64>>,
    aggregate5_c4cba_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate5_c4cba: AggregateBlock<Matrix<1, 10, f64>>,
    sum10_c4cc7_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum10_c4cc7: SumBlock<(f64, f64, f64)>,
    sum3_c4cbc_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum3_c4cbc: SumBlock<(f64, f64)>,
    component5f9fe6_component: Component5f9fe6Component,
    comparison2_c4cb6_param: <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    comparison2_c4cb6: ComparisonBlock<Matrix<1, 10, f64>>,
    aggregate2_c4caa_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate2_c4caa: AggregateBlock<Matrix<1, 10, f64>>,
    sum9_c4cc3_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum9_c4cc3: SumBlock<(f64, f64)>,
    component7fa010_component: Component7fa010Component,
    comparison6_c4cc4_param: <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    comparison6_c4cc4: ComparisonBlock<Matrix<1, 10, f64>>,
    aggregate6_c4cc5_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate6_c4cc5: AggregateBlock<Matrix<1, 10, f64>>,
    sum6_c4cbe_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum6_c4cbe: SumBlock<(f64, f64)>,
    component6f9ffb_component: Component6f9ffbComponent,
    comparison4_c4cbf_param: <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    comparison4_c4cbf: ComparisonBlock<Matrix<1, 10, f64>>,
    aggregate4_c4cb2_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate4_c4cb2: AggregateBlock<Matrix<1, 10, f64>>,
    sum11_c4cc8_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum11_c4cc8: SumBlock<(f64, f64, f64)>,
    constant1_c4cca_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant1_c4cca: ConstantBlock<f64>,
    sum12_c4cc9_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum12_c4cc9: SumBlock<(f64, f64, f64)>,
    delay5_c4cd2: DelayBlock,
    vector_slice5_c4cd4: VectorSliceBlock,
    vector_merge5_c4cd3: VectorMergeBlock,
    overall_mid_bucket_c4cd5: ComponentOutputBlock,
    delay6_c4cd6: DelayBlock,
    vector_slice6_c4cd8: VectorSliceBlock,
    vector_merge6_c4cd7: VectorMergeBlock,
    overall_lower_bucket_c4cd9: ComponentOutputBlock,
    delay4_c4cce: DelayBlock,
    vector_slice4_c4cd0: VectorSliceBlock,
    vector_merge4_c4ccf: VectorMergeBlock,
    overall_upper_bucket_c4cd1: ComponentOutputBlock,
    aggregate9_c4cdc_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate9_c4cdc: AggregateBlock<Matrix<1, 10, f64>>,
    aggregate7_c4cda_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate7_c4cda: AggregateBlock<Matrix<1, 10, f64>>,
    aggregate8_c4cdb_param: <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters,
    aggregate8_c4cdb: AggregateBlock<Matrix<1, 10, f64>>,
    sum13_c4cdd_param: <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters,
    sum13_c4cdd: SumBlock<(f64, f64, f64)>,
    total_samples_c4cde: ComponentOutputBlock,
}

impl Crashdetection1c4ca4Component {
    pub fn new(context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        let component_input2_c4ca5_ic = BlockData::new(1, 1, &[0.0]);

        // ComponentInput2
        let component_input2_c4ca5 =
            ComponentInputBlock::new("ComponentInput2", &component_input2_c4ca5_ic);

        let speed_c4cc0_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Speed
        let speed_c4cc0 = ComponentInputBlock::new("Speed", &speed_c4cc0_ic);

        let arg_min_max3_f9f90_method = load_param::<String>(
            &"arg_min_max3_f9f90",
            &"method",
            String::from("Max"),
            &diagram_params,
        );

        // ArgMinMax3
        let arg_min_max3_f9f90_param =
            <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &arg_min_max3_f9f90_method,
            );
        let arg_min_max3_f9f90 = ArgMinMaxBlock::default();

        let arg_min_max4_f9f91_method = load_param::<String>(
            &"arg_min_max4_f9f91",
            &"method",
            String::from("Min"),
            &diagram_params,
        );

        // ArgMinMax4
        let arg_min_max4_f9f91_param =
            <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &arg_min_max4_f9f91_method,
            );
        let arg_min_max4_f9f91 = ArgMinMaxBlock::default();

        let sum4_c4cbd_gains = load_param::<BlockData>(
            &"sum4_c4cbd",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum4
        let sum4_c4cbd_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum4_c4cbd_gains.to_pass());
        let sum4_c4cbd = SumBlock::default();

        let gain2_c4cb7_gain =
            load_param::<f64>(&"gain2_c4cb7", &"gain", 0.330000, &diagram_params);

        // Gain2
        let gain2_c4cb7_param =
            <GainBlock<f64, f64> as ProcessBlock>::Parameters::new(gain2_c4cb7_gain);
        let gain2_c4cb7 = GainBlock::default();

        let sum5_c4cab_gains = load_param::<BlockData>(
            &"sum5_c4cab",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum5
        let sum5_c4cab_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum5_c4cab_gains.to_pass());
        let sum5_c4cab = SumBlock::default();

        let component3f9fbc_component = Component3f9fbcComponent::new(context);

        let comparison3_c4cb8_method = load_param::<String>(
            &"comparison3_c4cb8",
            &"method",
            String::from("GreaterOrEqual"),
            &diagram_params,
        );

        // Comparison3
        let comparison3_c4cb8_param =
            <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &comparison3_c4cb8_method,
            );
        let comparison3_c4cb8 = ComparisonBlock::default();

        let aggregate3_c4cb1_method = load_param::<String>(
            &"aggregate3_c4cb1",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate3
        let aggregate3_c4cb1_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate3_c4cb1_method,
            );
        let aggregate3_c4cb1 = AggregateBlock::default();

        let current_c4ca6_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Current
        let current_c4ca6 = ComponentInputBlock::new("Current", &current_c4ca6_ic);

        let arg_min_max1_f9f89_method = load_param::<String>(
            &"arg_min_max1_f9f89",
            &"method",
            String::from("Max"),
            &diagram_params,
        );

        // ArgMinMax1
        let arg_min_max1_f9f89_param =
            <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &arg_min_max1_f9f89_method,
            );
        let arg_min_max1_f9f89 = ArgMinMaxBlock::default();

        let arg_min_max2_f9f8a_method = load_param::<String>(
            &"arg_min_max2_f9f8a",
            &"method",
            String::from("Min"),
            &diagram_params,
        );

        // ArgMinMax2
        let arg_min_max2_f9f8a_param =
            <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &arg_min_max2_f9f8a_method,
            );
        let arg_min_max2_f9f8a = ArgMinMaxBlock::default();

        let sum1_c4ca7_gains = load_param::<BlockData>(
            &"sum1_c4ca7",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum1
        let sum1_c4ca7_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum1_c4ca7_gains.to_pass());
        let sum1_c4ca7 = SumBlock::default();

        let gain1_c4cad_gain =
            load_param::<f64>(&"gain1_c4cad", &"gain", 0.330000, &diagram_params);

        // Gain1
        let gain1_c4cad_param =
            <GainBlock<f64, f64> as ProcessBlock>::Parameters::new(gain1_c4cad_gain);
        let gain1_c4cad = GainBlock::default();

        let sum2_c4ca8_gains = load_param::<BlockData>(
            &"sum2_c4ca8",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum2
        let sum2_c4ca8_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum2_c4ca8_gains.to_pass());
        let sum2_c4ca8 = SumBlock::default();

        let component1f9fa4_component = Component1f9fa4Component::new(context);

        let comparison1_c4ca9_method = load_param::<String>(
            &"comparison1_c4ca9",
            &"method",
            String::from("GreaterOrEqual"),
            &diagram_params,
        );

        // Comparison1
        let comparison1_c4ca9_param =
            <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &comparison1_c4ca9_method,
            );
        let comparison1_c4ca9 = ComparisonBlock::default();

        let aggregate1_c4cae_method = load_param::<String>(
            &"aggregate1_c4cae",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate1
        let aggregate1_c4cae_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate1_c4cae_method,
            );
        let aggregate1_c4cae = AggregateBlock::default();

        let ay_c4cc6_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Ay
        let ay_c4cc6 = ComponentInputBlock::new("Ay", &ay_c4cc6_ic);

        let arg_min_max5_f9f92_method = load_param::<String>(
            &"arg_min_max5_f9f92",
            &"method",
            String::from("Min"),
            &diagram_params,
        );

        // ArgMinMax5
        let arg_min_max5_f9f92_param =
            <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &arg_min_max5_f9f92_method,
            );
        let arg_min_max5_f9f92 = ArgMinMaxBlock::default();

        let arg_min_max6_f9f93_method = load_param::<String>(
            &"arg_min_max6_f9f93",
            &"method",
            String::from("Max"),
            &diagram_params,
        );

        // ArgMinMax6
        let arg_min_max6_f9f93_param =
            <ArgMinMaxBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &arg_min_max6_f9f93_method,
            );
        let arg_min_max6_f9f93 = ArgMinMaxBlock::default();

        let sum7_c4cc1_gains = load_param::<BlockData>(
            &"sum7_c4cc1",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum7
        let sum7_c4cc1_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum7_c4cc1_gains.to_pass());
        let sum7_c4cc1 = SumBlock::default();

        let gain3_c4cc2_gain =
            load_param::<f64>(&"gain3_c4cc2", &"gain", 0.330000, &diagram_params);

        // Gain3
        let gain3_c4cc2_param =
            <GainBlock<f64, f64> as ProcessBlock>::Parameters::new(gain3_c4cc2_gain);
        let gain3_c4cc2 = GainBlock::default();

        let sum8_c4cac_gains = load_param::<BlockData>(
            &"sum8_c4cac",
            &"gains",
            BlockData::new(1, 2, &[1.0, -1.0]),
            &diagram_params,
        );

        // Sum8
        let sum8_c4cac_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum8_c4cac_gains.to_pass());
        let sum8_c4cac = SumBlock::default();

        let component4f9fd1_component = Component4f9fd1Component::new(context);

        let comparison5_c4cb9_method = load_param::<String>(
            &"comparison5_c4cb9",
            &"method",
            String::from("GreaterOrEqual"),
            &diagram_params,
        );

        // Comparison5
        let comparison5_c4cb9_param =
            <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &comparison5_c4cb9_method,
            );
        let comparison5_c4cb9 = ComparisonBlock::default();

        let aggregate5_c4cba_method = load_param::<String>(
            &"aggregate5_c4cba",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate5
        let aggregate5_c4cba_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate5_c4cba_method,
            );
        let aggregate5_c4cba = AggregateBlock::default();

        let sum10_c4cc7_gains = load_param::<BlockData>(
            &"sum10_c4cc7",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum10
        let sum10_c4cc7_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum10_c4cc7_gains.to_pass(),
        );
        let sum10_c4cc7 = SumBlock::default();

        let sum3_c4cbc_gains = load_param::<BlockData>(
            &"sum3_c4cbc",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum3
        let sum3_c4cbc_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum3_c4cbc_gains.to_pass());
        let sum3_c4cbc = SumBlock::default();

        let component5f9fe6_component = Component5f9fe6Component::new(context);

        let comparison2_c4cb6_method = load_param::<String>(
            &"comparison2_c4cb6",
            &"method",
            String::from("LessThan"),
            &diagram_params,
        );

        // Comparison2
        let comparison2_c4cb6_param =
            <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &comparison2_c4cb6_method,
            );
        let comparison2_c4cb6 = ComparisonBlock::default();

        let aggregate2_c4caa_method = load_param::<String>(
            &"aggregate2_c4caa",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate2
        let aggregate2_c4caa_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate2_c4caa_method,
            );
        let aggregate2_c4caa = AggregateBlock::default();

        let sum9_c4cc3_gains = load_param::<BlockData>(
            &"sum9_c4cc3",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum9
        let sum9_c4cc3_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum9_c4cc3_gains.to_pass());
        let sum9_c4cc3 = SumBlock::default();

        let component7fa010_component = Component7fa010Component::new(context);

        let comparison6_c4cc4_method = load_param::<String>(
            &"comparison6_c4cc4",
            &"method",
            String::from("LessThan"),
            &diagram_params,
        );

        // Comparison6
        let comparison6_c4cc4_param =
            <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &comparison6_c4cc4_method,
            );
        let comparison6_c4cc4 = ComparisonBlock::default();

        let aggregate6_c4cc5_method = load_param::<String>(
            &"aggregate6_c4cc5",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate6
        let aggregate6_c4cc5_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate6_c4cc5_method,
            );
        let aggregate6_c4cc5 = AggregateBlock::default();

        let sum6_c4cbe_gains = load_param::<BlockData>(
            &"sum6_c4cbe",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum6
        let sum6_c4cbe_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum6_c4cbe_gains.to_pass());
        let sum6_c4cbe = SumBlock::default();

        let component6f9ffb_component = Component6f9ffbComponent::new(context);

        let comparison4_c4cbf_method = load_param::<String>(
            &"comparison4_c4cbf",
            &"method",
            String::from("LessThan"),
            &diagram_params,
        );

        // Comparison4
        let comparison4_c4cbf_param =
            <ComparisonBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &comparison4_c4cbf_method,
            );
        let comparison4_c4cbf = ComparisonBlock::default();

        let aggregate4_c4cb2_method = load_param::<String>(
            &"aggregate4_c4cb2",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate4
        let aggregate4_c4cb2_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate4_c4cb2_method,
            );
        let aggregate4_c4cb2 = AggregateBlock::default();

        let sum11_c4cc8_gains = load_param::<BlockData>(
            &"sum11_c4cc8",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum11
        let sum11_c4cc8_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum11_c4cc8_gains.to_pass(),
        );
        let sum11_c4cc8 = SumBlock::default();

        let constant1_c4cca_value =
            load_param::<f64>(&"constant1_c4cca", &"value", 30.000000, &diagram_params);

        let constant1_c4cca_ic = BlockData::from_element(1, 1, constant1_c4cca_value);

        // Constant1
        let constant1_c4cca_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant1_c4cca_ic.to_pass());
        let constant1_c4cca = ConstantBlock::default();

        let sum12_c4cc9_gains = load_param::<BlockData>(
            &"sum12_c4cc9",
            &"gains",
            BlockData::new(1, 3, &[1.0, -1.0, -1.0]),
            &diagram_params,
        );

        // Sum12
        let sum12_c4cc9_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum12_c4cc9_gains.to_pass(),
        );
        let sum12_c4cc9 = SumBlock::default();

        let delay5_c4cd2_value =
            load_param::<f64>(&"delay5_c4cd2", &"value", 1.000000, &diagram_params);
        let delay5_c4cd2_method = load_param::<String>(
            &"delay5_c4cd2",
            &"method",
            String::from("Iterations"),
            &diagram_params,
        );

        let delay5_c4cd2_ic = load_ic(
            &String::from("delay5_c4cd2"),
            &String::from("initial_condition"),
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            &diagram_params,
        );

        // Delay5
        let delay5_c4cd2 = DelayBlock::new(
            "Delay5",
            &delay5_c4cd2_ic,
            delay5_c4cd2_value,
            &delay5_c4cd2_method,
        );

        let vector_slice5_c4cd4_row0 =
            load_param::<f64>(&"vector_slice5_c4cd4", &"row0", 0.000000, &diagram_params);
        let vector_slice5_c4cd4_col0 =
            load_param::<f64>(&"vector_slice5_c4cd4", &"col0", 0.000000, &diagram_params);
        let vector_slice5_c4cd4_shape = load_param::<BlockData>(
            &"vector_slice5_c4cd4",
            &"shape",
            BlockData::new(1, 2, &[1.0, 9.0]),
            &diagram_params,
        );

        let vector_slice5_c4cd4_ic = BlockData::from_element(1, 9, 0.0);

        // VectorSlice5
        let vector_slice5_c4cd4 = VectorSliceBlock::new(
            "VectorSlice5",
            &vector_slice5_c4cd4_ic,
            vector_slice5_c4cd4_row0,
            vector_slice5_c4cd4_col0,
            &vector_slice5_c4cd4_shape,
        );

        let vector_merge5_c4cd3_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge5
        let vector_merge5_c4cd3 = VectorMergeBlock::new("VectorMerge5", &vector_merge5_c4cd3_ic);

        let overall_mid_bucket_c4cd5_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Overall_Mid_bucket
        let overall_mid_bucket_c4cd5 =
            ComponentOutputBlock::new("Overall_Mid_bucket", &overall_mid_bucket_c4cd5_ic);

        let delay6_c4cd6_value =
            load_param::<f64>(&"delay6_c4cd6", &"value", 1.000000, &diagram_params);
        let delay6_c4cd6_method = load_param::<String>(
            &"delay6_c4cd6",
            &"method",
            String::from("Iterations"),
            &diagram_params,
        );

        let delay6_c4cd6_ic = load_ic(
            &String::from("delay6_c4cd6"),
            &String::from("initial_condition"),
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            &diagram_params,
        );

        // Delay6
        let delay6_c4cd6 = DelayBlock::new(
            "Delay6",
            &delay6_c4cd6_ic,
            delay6_c4cd6_value,
            &delay6_c4cd6_method,
        );

        let vector_slice6_c4cd8_row0 =
            load_param::<f64>(&"vector_slice6_c4cd8", &"row0", 0.000000, &diagram_params);
        let vector_slice6_c4cd8_col0 =
            load_param::<f64>(&"vector_slice6_c4cd8", &"col0", 0.000000, &diagram_params);
        let vector_slice6_c4cd8_shape = load_param::<BlockData>(
            &"vector_slice6_c4cd8",
            &"shape",
            BlockData::new(1, 2, &[1.0, 9.0]),
            &diagram_params,
        );

        let vector_slice6_c4cd8_ic = BlockData::from_element(1, 9, 0.0);

        // VectorSlice6
        let vector_slice6_c4cd8 = VectorSliceBlock::new(
            "VectorSlice6",
            &vector_slice6_c4cd8_ic,
            vector_slice6_c4cd8_row0,
            vector_slice6_c4cd8_col0,
            &vector_slice6_c4cd8_shape,
        );

        let vector_merge6_c4cd7_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge6
        let vector_merge6_c4cd7 = VectorMergeBlock::new("VectorMerge6", &vector_merge6_c4cd7_ic);

        let overall_lower_bucket_c4cd9_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Overall_Lower_bucket
        let overall_lower_bucket_c4cd9 =
            ComponentOutputBlock::new("Overall_Lower_bucket", &overall_lower_bucket_c4cd9_ic);

        let delay4_c4cce_value =
            load_param::<f64>(&"delay4_c4cce", &"value", 1.000000, &diagram_params);
        let delay4_c4cce_method = load_param::<String>(
            &"delay4_c4cce",
            &"method",
            String::from("Iterations"),
            &diagram_params,
        );

        let delay4_c4cce_ic = load_ic(
            &String::from("delay4_c4cce"),
            &String::from("initial_condition"),
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            &diagram_params,
        );

        // Delay4
        let delay4_c4cce = DelayBlock::new(
            "Delay4",
            &delay4_c4cce_ic,
            delay4_c4cce_value,
            &delay4_c4cce_method,
        );

        let vector_slice4_c4cd0_row0 =
            load_param::<f64>(&"vector_slice4_c4cd0", &"row0", 0.000000, &diagram_params);
        let vector_slice4_c4cd0_col0 =
            load_param::<f64>(&"vector_slice4_c4cd0", &"col0", 0.000000, &diagram_params);
        let vector_slice4_c4cd0_shape = load_param::<BlockData>(
            &"vector_slice4_c4cd0",
            &"shape",
            BlockData::new(1, 2, &[1.0, 9.0]),
            &diagram_params,
        );

        let vector_slice4_c4cd0_ic = BlockData::from_element(1, 9, 0.0);

        // VectorSlice4
        let vector_slice4_c4cd0 = VectorSliceBlock::new(
            "VectorSlice4",
            &vector_slice4_c4cd0_ic,
            vector_slice4_c4cd0_row0,
            vector_slice4_c4cd0_col0,
            &vector_slice4_c4cd0_shape,
        );

        let vector_merge4_c4ccf_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge4
        let vector_merge4_c4ccf = VectorMergeBlock::new("VectorMerge4", &vector_merge4_c4ccf_ic);

        let overall_upper_bucket_c4cd1_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // Overall_Upper_bucket
        let overall_upper_bucket_c4cd1 =
            ComponentOutputBlock::new("Overall_Upper_bucket", &overall_upper_bucket_c4cd1_ic);

        let aggregate9_c4cdc_method = load_param::<String>(
            &"aggregate9_c4cdc",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate9
        let aggregate9_c4cdc_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate9_c4cdc_method,
            );
        let aggregate9_c4cdc = AggregateBlock::default();

        let aggregate7_c4cda_method = load_param::<String>(
            &"aggregate7_c4cda",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate7
        let aggregate7_c4cda_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate7_c4cda_method,
            );
        let aggregate7_c4cda = AggregateBlock::default();

        let aggregate8_c4cdb_method = load_param::<String>(
            &"aggregate8_c4cdb",
            &"method",
            String::from("Sum"),
            &diagram_params,
        );

        // Aggregate8
        let aggregate8_c4cdb_param =
            <AggregateBlock<Matrix<1, 10, f64>> as ProcessBlock>::Parameters::new(
                &aggregate8_c4cdb_method,
            );
        let aggregate8_c4cdb = AggregateBlock::default();

        let sum13_c4cdd_gains = load_param::<BlockData>(
            &"sum13_c4cdd",
            &"gains",
            BlockData::new(1, 3, &[1.0, 1.0, 1.0]),
            &diagram_params,
        );

        // Sum13
        let sum13_c4cdd_param = <SumBlock<(f64, f64, f64)> as ProcessBlock>::Parameters::new(
            sum13_c4cdd_gains.to_pass(),
        );
        let sum13_c4cdd = SumBlock::default();

        let total_samples_c4cde_ic = BlockData::new(1, 1, &[0.0]);

        // Total_samples
        let total_samples_c4cde =
            ComponentOutputBlock::new("Total_samples", &total_samples_c4cde_ic);

        Crashdetection1c4ca4Component {
            last_time_s: -1.0,
            component_input2_c4ca5,
            speed_c4cc0,
            arg_min_max3_f9f90_param,
            arg_min_max3_f9f90,
            arg_min_max4_f9f91_param,
            arg_min_max4_f9f91,
            sum4_c4cbd_param,
            sum4_c4cbd,
            gain2_c4cb7_param,
            gain2_c4cb7,
            sum5_c4cab_param,
            sum5_c4cab,
            component3f9fbc_component,
            comparison3_c4cb8_param,
            comparison3_c4cb8,
            aggregate3_c4cb1_param,
            aggregate3_c4cb1,
            current_c4ca6,
            arg_min_max1_f9f89_param,
            arg_min_max1_f9f89,
            arg_min_max2_f9f8a_param,
            arg_min_max2_f9f8a,
            sum1_c4ca7_param,
            sum1_c4ca7,
            gain1_c4cad_param,
            gain1_c4cad,
            sum2_c4ca8_param,
            sum2_c4ca8,
            component1f9fa4_component,
            comparison1_c4ca9_param,
            comparison1_c4ca9,
            aggregate1_c4cae_param,
            aggregate1_c4cae,
            ay_c4cc6,
            arg_min_max5_f9f92_param,
            arg_min_max5_f9f92,
            arg_min_max6_f9f93_param,
            arg_min_max6_f9f93,
            sum7_c4cc1_param,
            sum7_c4cc1,
            gain3_c4cc2_param,
            gain3_c4cc2,
            sum8_c4cac_param,
            sum8_c4cac,
            component4f9fd1_component,
            comparison5_c4cb9_param,
            comparison5_c4cb9,
            aggregate5_c4cba_param,
            aggregate5_c4cba,
            sum10_c4cc7_param,
            sum10_c4cc7,
            sum3_c4cbc_param,
            sum3_c4cbc,
            component5f9fe6_component,
            comparison2_c4cb6_param,
            comparison2_c4cb6,
            aggregate2_c4caa_param,
            aggregate2_c4caa,
            sum9_c4cc3_param,
            sum9_c4cc3,
            component7fa010_component,
            comparison6_c4cc4_param,
            comparison6_c4cc4,
            aggregate6_c4cc5_param,
            aggregate6_c4cc5,
            sum6_c4cbe_param,
            sum6_c4cbe,
            component6f9ffb_component,
            comparison4_c4cbf_param,
            comparison4_c4cbf,
            aggregate4_c4cb2_param,
            aggregate4_c4cb2,
            sum11_c4cc8_param,
            sum11_c4cc8,
            constant1_c4cca_param,
            constant1_c4cca,
            sum12_c4cc9_param,
            sum12_c4cc9,
            delay5_c4cd2,
            vector_slice5_c4cd4,
            vector_merge5_c4cd3,
            overall_mid_bucket_c4cd5,
            delay6_c4cd6,
            vector_slice6_c4cd8,
            vector_merge6_c4cd7,
            overall_lower_bucket_c4cd9,
            delay4_c4cce,
            vector_slice4_c4cd0,
            vector_merge4_c4ccf,
            overall_upper_bucket_c4cd1,
            aggregate9_c4cdc_param,
            aggregate9_c4cdc,
            aggregate7_c4cda_param,
            aggregate7_c4cda,
            aggregate8_c4cdb_param,
            aggregate8_c4cdb,
            sum13_c4cdd_param,
            sum13_c4cdd,
            total_samples_c4cde,
        }
    }

    pub fn run(
        &mut self,
        context: &mut Context,
        compare_to_value3_c4c92: &CompareToValueBlock<f64>,
        vector_merge7_c4c96: &VectorMergeBlock,
        vector_merge8_c4c99: &VectorMergeBlock,
        vector_merge9_c4c9c: &VectorMergeBlock,
    ) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Crashdetection1c4ca4Component iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // ComponentInput2
        self.component_input2_c4ca5
            .run(&compare_to_value3_c4c92.data);
        // Speed
        self.speed_c4cc0.run(&vector_merge8_c4c99.data);
        // ArgMinMax3
        self.arg_min_max3_f9f90.process(
            &self.arg_min_max3_f9f90_param,
            context,
            &self.speed_c4cc0.data.to_pass(),
        );
        // ArgMinMax4
        self.arg_min_max4_f9f91.process(
            &self.arg_min_max4_f9f91_param,
            context,
            &self.speed_c4cc0.data.to_pass(),
        );
        // Sum4
        self.sum4_c4cbd.process(
            &self.sum4_c4cbd_param,
            context,
            (
                self.arg_min_max3_f9f90.data.to_pass(),
                self.arg_min_max4_f9f91.data.to_pass(),
            ),
        );
        // Gain2
        self.gain2_c4cb7.process(
            &self.gain2_c4cb7_param,
            context,
            self.sum4_c4cbd.data.to_pass(),
        );
        // Sum5
        self.sum5_c4cab.process(
            &self.sum5_c4cab_param,
            context,
            (
                self.arg_min_max3_f9f90.data.to_pass(),
                self.gain2_c4cb7.data.to_pass(),
            ),
        );
        // Component: Component3
        self.component3f9fbc_component
            .run(context, &self.sum5_c4cab);
        // Comparison3
        self.comparison3_c4cb8.process(
            &self.comparison3_c4cb8_param,
            context,
            (
                &self.speed_c4cc0.data.to_pass(),
                &self
                    .component3f9fbc_component
                    .component_output2_f9fc1
                    .data
                    .to_pass(),
            ),
        );
        // Aggregate3
        self.aggregate3_c4cb1.process(
            &self.aggregate3_c4cb1_param,
            context,
            &self.comparison3_c4cb8.data.to_pass(),
        );
        // Current
        self.current_c4ca6.run(&vector_merge7_c4c96.data);
        // ArgMinMax1
        self.arg_min_max1_f9f89.process(
            &self.arg_min_max1_f9f89_param,
            context,
            &self.current_c4ca6.data.to_pass(),
        );
        // ArgMinMax2
        self.arg_min_max2_f9f8a.process(
            &self.arg_min_max2_f9f8a_param,
            context,
            &self.current_c4ca6.data.to_pass(),
        );
        // Sum1
        self.sum1_c4ca7.process(
            &self.sum1_c4ca7_param,
            context,
            (
                self.arg_min_max1_f9f89.data.to_pass(),
                self.arg_min_max2_f9f8a.data.to_pass(),
            ),
        );
        // Gain1
        self.gain1_c4cad.process(
            &self.gain1_c4cad_param,
            context,
            self.sum1_c4ca7.data.to_pass(),
        );
        // Sum2
        self.sum2_c4ca8.process(
            &self.sum2_c4ca8_param,
            context,
            (
                self.arg_min_max1_f9f89.data.to_pass(),
                self.gain1_c4cad.data.to_pass(),
            ),
        );
        // Component: Component1
        self.component1f9fa4_component
            .run(context, &self.sum2_c4ca8);
        // Comparison1
        self.comparison1_c4ca9.process(
            &self.comparison1_c4ca9_param,
            context,
            (
                &self.current_c4ca6.data.to_pass(),
                &self
                    .component1f9fa4_component
                    .component_output2_f9fb8
                    .data
                    .to_pass(),
            ),
        );
        // Aggregate1
        self.aggregate1_c4cae.process(
            &self.aggregate1_c4cae_param,
            context,
            &self.comparison1_c4ca9.data.to_pass(),
        );
        // Ay
        self.ay_c4cc6.run(&vector_merge9_c4c9c.data);
        // ArgMinMax5
        self.arg_min_max5_f9f92.process(
            &self.arg_min_max5_f9f92_param,
            context,
            &self.ay_c4cc6.data.to_pass(),
        );
        // ArgMinMax6
        self.arg_min_max6_f9f93.process(
            &self.arg_min_max6_f9f93_param,
            context,
            &self.ay_c4cc6.data.to_pass(),
        );
        // Sum7
        self.sum7_c4cc1.process(
            &self.sum7_c4cc1_param,
            context,
            (
                self.arg_min_max6_f9f93.data.to_pass(),
                self.arg_min_max5_f9f92.data.to_pass(),
            ),
        );
        // Gain3
        self.gain3_c4cc2.process(
            &self.gain3_c4cc2_param,
            context,
            self.sum7_c4cc1.data.to_pass(),
        );
        // Sum8
        self.sum8_c4cac.process(
            &self.sum8_c4cac_param,
            context,
            (
                self.arg_min_max6_f9f93.data.to_pass(),
                self.gain3_c4cc2.data.to_pass(),
            ),
        );
        // Component: Component4
        self.component4f9fd1_component
            .run(context, &self.sum8_c4cac);
        // Comparison5
        self.comparison5_c4cb9.process(
            &self.comparison5_c4cb9_param,
            context,
            (
                &self.ay_c4cc6.data.to_pass(),
                &self
                    .component4f9fd1_component
                    .component_output2_f9fd6
                    .data
                    .to_pass(),
            ),
        );
        // Aggregate5
        self.aggregate5_c4cba.process(
            &self.aggregate5_c4cba_param,
            context,
            &self.comparison5_c4cb9.data.to_pass(),
        );
        // Sum10
        self.sum10_c4cc7.process(
            &self.sum10_c4cc7_param,
            context,
            (
                self.aggregate1_c4cae.data.to_pass(),
                self.aggregate3_c4cb1.data.to_pass(),
                self.aggregate5_c4cba.data.to_pass(),
            ),
        );
        // Sum3
        self.sum3_c4cbc.process(
            &self.sum3_c4cbc_param,
            context,
            (
                self.gain1_c4cad.data.to_pass(),
                self.arg_min_max2_f9f8a.data.to_pass(),
            ),
        );
        // Component: Component5
        self.component5f9fe6_component
            .run(context, &self.sum3_c4cbc);
        // Comparison2
        self.comparison2_c4cb6.process(
            &self.comparison2_c4cb6_param,
            context,
            (
                &self.current_c4ca6.data.to_pass(),
                &self
                    .component5f9fe6_component
                    .component_output2_f9feb
                    .data
                    .to_pass(),
            ),
        );
        // Aggregate2
        self.aggregate2_c4caa.process(
            &self.aggregate2_c4caa_param,
            context,
            &self.comparison2_c4cb6.data.to_pass(),
        );
        // Sum9
        self.sum9_c4cc3.process(
            &self.sum9_c4cc3_param,
            context,
            (
                self.gain3_c4cc2.data.to_pass(),
                self.arg_min_max5_f9f92.data.to_pass(),
            ),
        );
        // Component: Component7
        self.component7fa010_component
            .run(context, &self.sum9_c4cc3);
        // Comparison6
        self.comparison6_c4cc4.process(
            &self.comparison6_c4cc4_param,
            context,
            (
                &self.ay_c4cc6.data.to_pass(),
                &self
                    .component7fa010_component
                    .component_output2_fa015
                    .data
                    .to_pass(),
            ),
        );
        // Aggregate6
        self.aggregate6_c4cc5.process(
            &self.aggregate6_c4cc5_param,
            context,
            &self.comparison6_c4cc4.data.to_pass(),
        );
        // Sum6
        self.sum6_c4cbe.process(
            &self.sum6_c4cbe_param,
            context,
            (
                self.gain2_c4cb7.data.to_pass(),
                self.arg_min_max4_f9f91.data.to_pass(),
            ),
        );
        // Component: Component6
        self.component6f9ffb_component
            .run(context, &self.sum6_c4cbe);
        // Comparison4
        self.comparison4_c4cbf.process(
            &self.comparison4_c4cbf_param,
            context,
            (
                &self.speed_c4cc0.data.to_pass(),
                &self
                    .component6f9ffb_component
                    .component_output2_fa000
                    .data
                    .to_pass(),
            ),
        );
        // Aggregate4
        self.aggregate4_c4cb2.process(
            &self.aggregate4_c4cb2_param,
            context,
            &self.comparison4_c4cbf.data.to_pass(),
        );
        // Sum11
        self.sum11_c4cc8.process(
            &self.sum11_c4cc8_param,
            context,
            (
                self.aggregate2_c4caa.data.to_pass(),
                self.aggregate4_c4cb2.data.to_pass(),
                self.aggregate6_c4cc5.data.to_pass(),
            ),
        );
        // Constant1
        self.constant1_c4cca
            .generate(&self.constant1_c4cca_param, context);
        // Sum12
        self.sum12_c4cc9.process(
            &self.sum12_c4cc9_param,
            context,
            (
                self.constant1_c4cca.data.to_pass(),
                self.sum10_c4cc7.data.to_pass(),
                self.sum11_c4cc8.data.to_pass(),
            ),
        );
        // Delay5
        self.delay5_c4cd2
            .run(&self.vector_merge5_c4cd3.data, app_time_s);
        // VectorSlice5
        self.vector_slice5_c4cd4.run(&self.delay5_c4cd2.data);
        // VectorMerge5
        self.vector_merge5_c4cd3.run(&vec![
            &self.sum12_c4cc9.data,
            &self.vector_slice5_c4cd4.data,
        ]);
        // Overall_Mid_bucket
        self.overall_mid_bucket_c4cd5
            .run(&self.vector_merge5_c4cd3.data);
        // Delay6
        self.delay6_c4cd6
            .run(&self.vector_merge6_c4cd7.data, app_time_s);
        // VectorSlice6
        self.vector_slice6_c4cd8.run(&self.delay6_c4cd6.data);
        // VectorMerge6
        self.vector_merge6_c4cd7.run(&vec![
            &self.sum11_c4cc8.data,
            &self.vector_slice6_c4cd8.data,
        ]);
        // Overall_Lower_bucket
        self.overall_lower_bucket_c4cd9
            .run(&self.vector_merge6_c4cd7.data);
        // Delay4
        self.delay4_c4cce
            .run(&self.vector_merge4_c4ccf.data, app_time_s);
        // VectorSlice4
        self.vector_slice4_c4cd0.run(&self.delay4_c4cce.data);
        // VectorMerge4
        self.vector_merge4_c4ccf.run(&vec![
            &self.sum10_c4cc7.data,
            &self.vector_slice4_c4cd0.data,
        ]);
        // Overall_Upper_bucket
        self.overall_upper_bucket_c4cd1
            .run(&self.vector_merge4_c4ccf.data);
        // Aggregate9
        self.aggregate9_c4cdc.process(
            &self.aggregate9_c4cdc_param,
            context,
            &self.vector_merge6_c4cd7.data.to_pass(),
        );
        // Aggregate7
        self.aggregate7_c4cda.process(
            &self.aggregate7_c4cda_param,
            context,
            &self.vector_merge4_c4ccf.data.to_pass(),
        );
        // Aggregate8
        self.aggregate8_c4cdb.process(
            &self.aggregate8_c4cdb_param,
            context,
            &self.vector_merge5_c4cd3.data.to_pass(),
        );
        // Sum13
        self.sum13_c4cdd.process(
            &self.sum13_c4cdd_param,
            context,
            (
                self.aggregate7_c4cda.data.to_pass(),
                self.aggregate8_c4cdb.data.to_pass(),
                self.aggregate9_c4cdc.data.to_pass(),
            ),
        );
        // Total_samples
        self.total_samples_c4cde.run(&self.sum13_c4cdd.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let mut output = vec![];
        output.extend(self.component1f9fa4_component.get_output());
        output.extend(self.component3f9fbc_component.get_output());
        output.extend(self.component4f9fd1_component.get_output());
        output.extend(self.component5f9fe6_component.get_output());
        output.extend(self.component6f9ffb_component.get_output());
        output.extend(self.component7fa010_component.get_output());
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Crashnewc4c8fComponent {
    last_time_s: f64,
    entropy_diff_5b135: ComponentInputBlock,
    delay8_c4c98: DelayBlock,
    vector_slice26_c4c9a: VectorSliceBlock,
    curr_5b133: ComponentInputBlock,
    vector_merge8_c4c99: VectorMergeBlock,
    delay7_c4c95: DelayBlock,
    vector_slice25_c4c97: VectorSliceBlock,
    speed_5b132: ComponentInputBlock,
    vector_merge7_c4c96: VectorMergeBlock,
    delay9_c4c9b: DelayBlock,
    vector_slice27_c4c9d: VectorSliceBlock,
    ay_5b134: ComponentInputBlock,
    vector_merge9_c4c9c: VectorMergeBlock,
    cone_5b146: ComponentInputBlock,
    counter3_c4c91_param: <CounterBlock<f64, bool> as ProcessBlock>::Parameters,
    counter3_c4c91: CounterBlock<f64, bool>,
    compare_to_value3_c4c92_param: <CompareToValueBlock<f64> as ProcessBlock>::Parameters,
    compare_to_value3_c4c92: CompareToValueBlock<f64>,
    crashdetection1c4ca4_component: Crashdetection1c4ca4Component,
    equation19_fa028: EquationBlock,
    compare_to_value1_fa02a_param: <CompareToValueBlock<f64> as ProcessBlock>::Parameters,
    compare_to_value1_fa02a: CompareToValueBlock<f64>,
    component2c4cdf_component: Component2c4cdfComponent,
    crash_flag_5b13b: ComponentOutputBlock,
}

impl Crashnewc4c8fComponent {
    pub fn new(context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        let entropy_diff_5b135_ic = BlockData::new(1, 1, &[0.0]);

        // EntropyDiff
        let entropy_diff_5b135 = ComponentInputBlock::new("EntropyDiff", &entropy_diff_5b135_ic);

        let delay8_c4c98_value =
            load_param::<f64>(&"delay8_c4c98", &"value", 1.000000, &diagram_params);
        let delay8_c4c98_method = load_param::<String>(
            &"delay8_c4c98",
            &"method",
            String::from("Iterations"),
            &diagram_params,
        );

        let delay8_c4c98_ic = load_ic(
            &String::from("delay8_c4c98"),
            &String::from("initial_condition"),
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            &diagram_params,
        );

        // Delay8
        let delay8_c4c98 = DelayBlock::new(
            "Delay8",
            &delay8_c4c98_ic,
            delay8_c4c98_value,
            &delay8_c4c98_method,
        );

        let vector_slice26_c4c9a_row0 =
            load_param::<f64>(&"vector_slice26_c4c9a", &"row0", 0.000000, &diagram_params);
        let vector_slice26_c4c9a_col0 =
            load_param::<f64>(&"vector_slice26_c4c9a", &"col0", 0.000000, &diagram_params);
        let vector_slice26_c4c9a_shape = load_param::<BlockData>(
            &"vector_slice26_c4c9a",
            &"shape",
            BlockData::new(1, 2, &[1.0, 9.0]),
            &diagram_params,
        );

        let vector_slice26_c4c9a_ic = BlockData::from_element(1, 9, 0.0);

        // VectorSlice26
        let vector_slice26_c4c9a = VectorSliceBlock::new(
            "VectorSlice26",
            &vector_slice26_c4c9a_ic,
            vector_slice26_c4c9a_row0,
            vector_slice26_c4c9a_col0,
            &vector_slice26_c4c9a_shape,
        );

        let curr_5b133_ic = BlockData::new(1, 1, &[0.0]);

        // Curr
        let curr_5b133 = ComponentInputBlock::new("Curr", &curr_5b133_ic);

        let vector_merge8_c4c99_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge8
        let vector_merge8_c4c99 = VectorMergeBlock::new("VectorMerge8", &vector_merge8_c4c99_ic);

        let delay7_c4c95_value =
            load_param::<f64>(&"delay7_c4c95", &"value", 1.000000, &diagram_params);
        let delay7_c4c95_method = load_param::<String>(
            &"delay7_c4c95",
            &"method",
            String::from("Iterations"),
            &diagram_params,
        );

        let delay7_c4c95_ic = load_ic(
            &String::from("delay7_c4c95"),
            &String::from("initial_condition"),
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            &diagram_params,
        );

        // Delay7
        let delay7_c4c95 = DelayBlock::new(
            "Delay7",
            &delay7_c4c95_ic,
            delay7_c4c95_value,
            &delay7_c4c95_method,
        );

        let vector_slice25_c4c97_row0 =
            load_param::<f64>(&"vector_slice25_c4c97", &"row0", 0.000000, &diagram_params);
        let vector_slice25_c4c97_col0 =
            load_param::<f64>(&"vector_slice25_c4c97", &"col0", 0.000000, &diagram_params);
        let vector_slice25_c4c97_shape = load_param::<BlockData>(
            &"vector_slice25_c4c97",
            &"shape",
            BlockData::new(1, 2, &[1.0, 9.0]),
            &diagram_params,
        );

        let vector_slice25_c4c97_ic = BlockData::from_element(1, 9, 0.0);

        // VectorSlice25
        let vector_slice25_c4c97 = VectorSliceBlock::new(
            "VectorSlice25",
            &vector_slice25_c4c97_ic,
            vector_slice25_c4c97_row0,
            vector_slice25_c4c97_col0,
            &vector_slice25_c4c97_shape,
        );

        let speed_5b132_ic = BlockData::new(1, 1, &[0.0]);

        // Speed
        let speed_5b132 = ComponentInputBlock::new("Speed", &speed_5b132_ic);

        let vector_merge7_c4c96_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge7
        let vector_merge7_c4c96 = VectorMergeBlock::new("VectorMerge7", &vector_merge7_c4c96_ic);

        let delay9_c4c9b_value =
            load_param::<f64>(&"delay9_c4c9b", &"value", 1.000000, &diagram_params);
        let delay9_c4c9b_method = load_param::<String>(
            &"delay9_c4c9b",
            &"method",
            String::from("Iterations"),
            &diagram_params,
        );

        let delay9_c4c9b_ic = load_ic(
            &String::from("delay9_c4c9b"),
            &String::from("initial_condition"),
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            &diagram_params,
        );

        // Delay9
        let delay9_c4c9b = DelayBlock::new(
            "Delay9",
            &delay9_c4c9b_ic,
            delay9_c4c9b_value,
            &delay9_c4c9b_method,
        );

        let vector_slice27_c4c9d_row0 =
            load_param::<f64>(&"vector_slice27_c4c9d", &"row0", 0.000000, &diagram_params);
        let vector_slice27_c4c9d_col0 =
            load_param::<f64>(&"vector_slice27_c4c9d", &"col0", 0.000000, &diagram_params);
        let vector_slice27_c4c9d_shape = load_param::<BlockData>(
            &"vector_slice27_c4c9d",
            &"shape",
            BlockData::new(1, 2, &[1.0, 9.0]),
            &diagram_params,
        );

        let vector_slice27_c4c9d_ic = BlockData::from_element(1, 9, 0.0);

        // VectorSlice27
        let vector_slice27_c4c9d = VectorSliceBlock::new(
            "VectorSlice27",
            &vector_slice27_c4c9d_ic,
            vector_slice27_c4c9d_row0,
            vector_slice27_c4c9d_col0,
            &vector_slice27_c4c9d_shape,
        );

        let ay_5b134_ic = BlockData::new(1, 1, &[0.0]);

        // Ay
        let ay_5b134 = ComponentInputBlock::new("Ay", &ay_5b134_ic);

        let vector_merge9_c4c9c_ic =
            BlockData::new(1, 10, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        // VectorMerge9
        let vector_merge9_c4c9c = VectorMergeBlock::new("VectorMerge9", &vector_merge9_c4c9c_ic);

        let cone_5b146_ic = BlockData::new(1, 1, &[0.0]);

        // Cone
        let cone_5b146 = ComponentInputBlock::new("Cone", &cone_5b146_ic);

        // Counter3
        let counter3_c4c91_param = <CounterBlock<f64, bool> as ProcessBlock>::Parameters::new();
        let counter3_c4c91 = CounterBlock::default();

        let compare_to_value3_c4c92_method = load_param::<String>(
            &"compare_to_value3_c4c92",
            &"method",
            String::from("GreaterOrEqual"),
            &diagram_params,
        );
        let compare_to_value3_c4c92_value = load_param::<f64>(
            &"compare_to_value3_c4c92",
            &"value",
            10.000000,
            &diagram_params,
        );

        // CompareToValue3
        let compare_to_value3_c4c92_param =
            <CompareToValueBlock<f64> as ProcessBlock>::Parameters::new(
                &compare_to_value3_c4c92_method,
                compare_to_value3_c4c92_value,
            );
        let compare_to_value3_c4c92 = CompareToValueBlock::default();

        let crashdetection1c4ca4_component = Crashdetection1c4ca4Component::new(context);

        let equation19_fa028_ic = BlockData::new(1, 1, &[0.0]);

        // Equation19
        let equation19_fa028 = EquationBlock::new("Equation19", &equation19_fa028_ic);

        let compare_to_value1_fa02a_method = load_param::<String>(
            &"compare_to_value1_fa02a",
            &"method",
            String::from("Equal"),
            &diagram_params,
        );
        let compare_to_value1_fa02a_value = load_param::<f64>(
            &"compare_to_value1_fa02a",
            &"value",
            0.000000,
            &diagram_params,
        );

        // CompareToValue1
        let compare_to_value1_fa02a_param =
            <CompareToValueBlock<f64> as ProcessBlock>::Parameters::new(
                &compare_to_value1_fa02a_method,
                compare_to_value1_fa02a_value,
            );
        let compare_to_value1_fa02a = CompareToValueBlock::default();

        let component2c4cdf_component = Component2c4cdfComponent::new(context);

        let crash_flag_5b13b_ic = BlockData::new(1, 1, &[0.0]);

        // CrashFlag
        let crash_flag_5b13b = ComponentOutputBlock::new("CrashFlag", &crash_flag_5b13b_ic);

        Crashnewc4c8fComponent {
            last_time_s: -1.0,
            entropy_diff_5b135,
            delay8_c4c98,
            vector_slice26_c4c9a,
            curr_5b133,
            vector_merge8_c4c99,
            delay7_c4c95,
            vector_slice25_c4c97,
            speed_5b132,
            vector_merge7_c4c96,
            delay9_c4c9b,
            vector_slice27_c4c9d,
            ay_5b134,
            vector_merge9_c4c9c,
            cone_5b146,
            counter3_c4c91_param,
            counter3_c4c91,
            compare_to_value3_c4c92_param,
            compare_to_value3_c4c92,
            crashdetection1c4ca4_component,
            equation19_fa028,
            compare_to_value1_fa02a_param,
            compare_to_value1_fa02a,
            component2c4cdf_component,
            crash_flag_5b13b,
        }
    }

    pub fn run(
        &mut self,
        context: &mut Context,
        constant2_5b148: &ConstantBlock<f64>,
        data_read1_5b12e: &DataReadBlock,
        data_read2_5b12f: &DataReadBlock,
        data_read3_5b130: &DataReadBlock,
        data_read4_5b131: &DataReadBlock,
    ) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Crashnewc4c8fComponent iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // EntropyDiff
        self.entropy_diff_5b135.run(&data_read4_5b131.data);
        // Delay8
        self.delay8_c4c98
            .run(&self.vector_merge8_c4c99.data, app_time_s);
        // VectorSlice26
        self.vector_slice26_c4c9a.run(&self.delay8_c4c98.data);
        // Curr
        self.curr_5b133.run(&data_read2_5b12f.data);
        // VectorMerge8
        self.vector_merge8_c4c99.run(&vec![
            &self.curr_5b133.data,
            &self.vector_slice26_c4c9a.data,
        ]);
        // Delay7
        self.delay7_c4c95
            .run(&self.vector_merge7_c4c96.data, app_time_s);
        // VectorSlice25
        self.vector_slice25_c4c97.run(&self.delay7_c4c95.data);
        // Speed
        self.speed_5b132.run(&data_read1_5b12e.data);
        // VectorMerge7
        self.vector_merge7_c4c96.run(&vec![
            &self.speed_5b132.data,
            &self.vector_slice25_c4c97.data,
        ]);
        // Delay9
        self.delay9_c4c9b
            .run(&self.vector_merge9_c4c9c.data, app_time_s);
        // VectorSlice27
        self.vector_slice27_c4c9d.run(&self.delay9_c4c9b.data);
        // Ay
        self.ay_5b134.run(&data_read3_5b130.data);
        // VectorMerge9
        self.vector_merge9_c4c9c
            .run(&vec![&self.ay_5b134.data, &self.vector_slice27_c4c9d.data]);
        // Cone
        self.cone_5b146.run(&constant2_5b148.data);
        // Counter3
        self.counter3_c4c91.process(
            &self.counter3_c4c91_param,
            context,
            (
                self.cone_5b146.data.to_pass(),
                self.compare_to_value3_c4c92.data.to_pass(),
            ),
        );
        // CompareToValue3
        self.compare_to_value3_c4c92.process(
            &self.compare_to_value3_c4c92_param,
            context,
            self.counter3_c4c91.data.to_pass(),
        );

        if self.compare_to_value3_c4c92.data.any() {
            // Component: Crash_detection1
            self.crashdetection1c4ca4_component.run(
                context,
                &self.compare_to_value3_c4c92,
                &self.vector_merge7_c4c96,
                &self.vector_merge8_c4c99,
                &self.vector_merge9_c4c9c,
            );
        }

        let equation19_fa028_expression = 1.0
            * self
                .crashdetection1c4ca4_component
                .total_samples_c4cde
                .data
                .scalar()
            - 300.0
                * (0.00333333333333333
                    * self
                        .crashdetection1c4ca4_component
                        .total_samples_c4cde
                        .data
                        .scalar())
                .floor();

        self.equation19_fa028
            .run(&BlockData::from_scalar(equation19_fa028_expression));
        // CompareToValue1
        self.compare_to_value1_fa02a.process(
            &self.compare_to_value1_fa02a_param,
            context,
            self.equation19_fa028.data.to_pass(),
        );

        if self.compare_to_value1_fa02a.data.any() {
            // Component: Component2
            self.component2c4cdf_component.run(
                context,
                &self.compare_to_value1_fa02a,
                &self
                    .crashdetection1c4ca4_component
                    .overall_lower_bucket_c4cd9,
                &self.crashdetection1c4ca4_component.overall_mid_bucket_c4cd5,
                &self
                    .crashdetection1c4ca4_component
                    .overall_upper_bucket_c4cd1,
                &self.entropy_diff_5b135,
            );
        }

        // CrashFlag
        self.crash_flag_5b13b
            .run(&self.component2c4cdf_component.component_output1_c4d68.data);

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let mut output = vec![];
        output.extend(self.component2c4cdf_component.get_output());
        output.extend(self.crashdetection1c4ca4_component.get_output());
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct Main6013bState {
    last_time_s: f64,
    data_read1_5b12e: DataReadBlock,
    data_read2_5b12f: DataReadBlock,
    data_read3_5b130: DataReadBlock,
    data_read4_5b131: DataReadBlock,
    constant2_5b148_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant2_5b148: ConstantBlock<f64>,
    crashnewc4c8f_component: Crashnewc4c8fComponent,
    data_write2_5b13d: DataWriteBlock,
}

impl Main6013bState {
    pub fn new(context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        // DataRead1
        let data_read1_5b12e = DataReadBlock::new("DataRead1");

        // DataRead2
        let data_read2_5b12f = DataReadBlock::new("DataRead2");

        // DataRead3
        let data_read3_5b130 = DataReadBlock::new("DataRead3");

        // DataRead4
        let data_read4_5b131 = DataReadBlock::new("DataRead4");

        let constant2_5b148_value =
            load_param::<f64>(&"constant2_5b148", &"value", 1.000000, &diagram_params);

        let constant2_5b148_ic = BlockData::from_element(1, 1, constant2_5b148_value);

        // Constant2
        let constant2_5b148_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant2_5b148_ic.to_pass());
        let constant2_5b148 = ConstantBlock::default();

        let crashnewc4c8f_component = Crashnewc4c8fComponent::new(context);

        // DataWrite2
        let data_write2_5b13d = DataWriteBlock::new("DataWrite2");

        Main6013bState {
            last_time_s: -1.0,
            data_read1_5b12e,
            data_read2_5b12f,
            data_read3_5b130,
            data_read4_5b131,
            constant2_5b148_param,
            constant2_5b148,
            crashnewc4c8f_component,
            data_write2_5b13d,
        }
    }

    pub fn run(&mut self, context: &mut Context) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Main6013bState iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        self.data_read1_5b12e
            .run(&BlockData::from_scalar(context.gds.speed_6013c_d41d0));
        self.data_read2_5b12f
            .run(&BlockData::from_scalar(context.gds.curr_6013c_d0257));
        self.data_read3_5b130
            .run(&BlockData::from_scalar(context.gds.ay_6013c_2880e));
        self.data_read4_5b131
            .run(&BlockData::from_scalar(context.gds.entropydiff_6013c_a7bdb));
        // Constant2
        self.constant2_5b148
            .generate(&self.constant2_5b148_param, context);
        // Component: Crash_New
        self.crashnewc4c8f_component.run(
            context,
            &self.constant2_5b148,
            &self.data_read1_5b12e,
            &self.data_read2_5b12f,
            &self.data_read3_5b130,
            &self.data_read4_5b131,
        );

        // Update DataStore with value from data_write2_5b13d
        // DataWrite2
        self.data_write2_5b13d
            .run(&self.crashnewc4c8f_component.crash_flag_5b13b.data);
        context.gds.crashflag_6013c_525d0 = self.data_write2_5b13d.data.scalar();

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let mut output = vec![];
        output.extend(self.crashnewc4c8f_component.get_output());
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct StateManager {
    pub current_state: State,
    pub main6013b_state: Main6013bState,
}

impl StateManager {
    pub fn run(&mut self, context: &mut Context) {
        match self.current_state {
            State::Main6013bState => self.main6013b_state.run(context),
        };
    }
    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        [self.main6013b_state.get_output()].concat()
    }
}

pub struct GlobalDataStore {
    pub ay_6013c_2880e: f64,
    pub crashflag_6013c_525d0: f64,
    pub curr_6013c_d0257: f64,
    pub entropydiff_6013c_a7bdb: f64,
    pub slicestart_c4cdf_82841: f64,
    pub speed_6013c_d41d0: f64,
}

impl GlobalDataStore {
    // Constructor
    pub fn new() -> GlobalDataStore {
        GlobalDataStore {
            ay_6013c_2880e: 0.0,
            crashflag_6013c_525d0: 0.0,
            curr_6013c_d0257: 0.0,
            entropydiff_6013c_a7bdb: 0.0,
            slicestart_c4cdf_82841: 0.0,
            speed_6013c_d41d0: 0.0,
        }
    }
}

//  ----- C interface methods ----- //
#[repr(C)]
pub struct AppDataInput {
    pub Speed: f64,
    pub Curr: f64,
    pub Ay: f64,
    pub EntropyDiff: f64,
}

#[repr(C)]
pub struct AppDataOutput {
    pub CrashFlag: f64,
}

#[no_mangle]
pub extern "C" fn app_interface_new() -> *mut AppInterface {
    /*
    Allows users to create an AppInterface object, to control
    app execution from other languages.
    */
    let pictorus_vars = get_pictorus_vars();

    let gds = GlobalDataStore::new();
    let io_manager = IoManager::new().expect("Unable to initialize IoManager!");
    let context = Context {
        gds,
        io_manager,
        app_time_us: 0,
        app_timestep_us: 100000,
    };

    let app_interface = AppInterface::new(context, &pictorus_vars);

    Box::into_raw(Box::new(app_interface))
}

#[no_mangle]
pub extern "C" fn app_interface_free(app: *mut AppInterface) {
    /*
    Allows users to free an AppInterface object from memory from other languages.
    */

    if app.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(app);
    }
}

#[no_mangle]
pub extern "C" fn app_interface_update(
    app: *mut AppInterface,
    app_time_s: f64,
    input_data: *mut AppDataInput,
) -> AppDataOutput {
    /*
    Allows users to iterate one execution step for a given AppInterface
    */

    let app_interface = unsafe {
        assert!(!app.is_null());
        &mut *app
    };
    let input_data = unsafe {
        assert!(!input_data.is_null());
        &*input_data
    };
    app_interface.context.gds.speed_6013c_d41d0 = input_data.Speed;
    app_interface.context.gds.curr_6013c_d0257 = input_data.Curr;
    app_interface.context.gds.ay_6013c_2880e = input_data.Ay;
    app_interface.context.gds.entropydiff_6013c_a7bdb = input_data.EntropyDiff;

    app_interface.context.app_time_us = s_to_us(app_time_s);
    app_interface.update();

    AppDataOutput {
        CrashFlag: app_interface.context.gds.crashflag_6013c_525d0,
    }
}
//  ------------------------------ //

pub struct IoManager {}

impl IoManager {
    pub fn new() -> Result<Self, PictorusError> {
        let io_manager = IoManager {};
        Ok(io_manager)
    }

    pub fn flush_inputs(&mut self) {}
}

pub struct AppInterface {
    state_manager: StateManager,
    data_logger: DataLogger,
    context: Context,
}

impl AppInterface {
    pub fn new(context: Context, pictorus_vars: &PictorusVars) -> Self {
        let data_logger_path =
            std::path::PathBuf::from(&pictorus_vars.run_path).join("diagram_output.csv");
        let labels: Vec<String> = vec![];
        let data_logger = DataLogger::new(
            labels,
            pictorus_vars.data_log_rate_hz,
            data_logger_path,
            &pictorus_vars.publish_socket,
            100,
        );

        let state_manager = StateManager {
            current_state: State::Main6013bState,
            main6013b_state: Main6013bState::new(&context),
        };

        Self {
            state_manager,
            data_logger,
            context,
        }
    }

    pub fn update(&mut self) {
        self.state_manager.run(&mut self.context);

        let logged_state_id = match self.state_manager.current_state {
            State::Main6013bState => "main6013b_state",
        };

        // TODO: Can simplify all this to data_logger.maybe_update(&context, &state_manager);
        if self.data_logger.should_log(self.context.app_time_us)
            || self.data_logger.should_broadcast(self.context.app_time_us)
        {
            self.data_logger.add_samples(
                self.context.app_time_us,
                logged_state_id,
                &self.state_manager.get_output(),
            );
        }

        self.context.io_manager.flush_inputs();
    }
}

pub struct Context {
    gds: GlobalDataStore,
    io_manager: IoManager,
    app_time_us: u64,
    app_timestep_us: u64,
}

impl Context {
    pub fn app_time_s(&mut self) -> f64 {
        us_to_s(self.app_time_us)
    }
}

impl corelib_traits::Context for Context {
    fn timestep(&self) -> Duration {
        Duration::from_micros(self.app_timestep_us)
    }

    fn time(&self) -> Duration {
        Duration::from_micros(self.app_time_us)
    }
}
