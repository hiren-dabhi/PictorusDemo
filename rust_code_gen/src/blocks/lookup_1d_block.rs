use log::debug;

use crate::block_data::BlockData;

#[derive(strum::EnumString)]
pub enum Lookup1DBlockInterpEnum {
    Linear,
    Nearest,
}

pub struct Lookup1DBlock {
    pub name: &'static str,
    pub interp_method: Lookup1DBlockInterpEnum,
    pub data: BlockData,
    pub break_points: BlockData,
    pub data_points: BlockData,
    pub max_idx: usize,
}

impl Lookup1DBlock {
    pub fn new(
        name: &'static str,
        break_points: &BlockData,
        data_points: &BlockData,
        interp_method: &str,
    ) -> Lookup1DBlock {
        if !break_points.same_size(data_points) {
            panic!(
                "{}: Breakpoints and Data Points arrays must be same size!",
                name
            )
        }
        Lookup1DBlock {
            name,
            interp_method: interp_method.parse().unwrap(),
            data: BlockData::from_scalar(0.0),
            max_idx: break_points.len() - 1,
            break_points: break_points.clone(),
            data_points: data_points.clone(),
        }
    }

    pub fn run(&mut self, lookup_point: &BlockData) {
        let mut data = BlockData::zeros_sizeof(lookup_point);
        for (idx, lookup_val) in lookup_point.iter().enumerate() {
            data[idx] = self._do_lookup(*lookup_val);
        }
        self.data = data;
        debug!("{} data: {:?}", self.name, self.data);
    }
    fn _do_lookup(&self, lookup_point_val: f64) -> f64 {
        if lookup_point_val < self.break_points[0] {
            debug!(
                "{} lookup data {:?} below lower bounds of {}. Setting to lower bound.",
                self.name, lookup_point_val, self.break_points[0]
            );
            return self.data_points[0];
        } else if lookup_point_val >= self.break_points[self.max_idx] {
            debug!(
                "{} lookup data {:?} above upper bounds of {}. Setting to upper bound.",
                self.name, lookup_point_val, self.break_points[self.max_idx]
            );
            return self.data_points[self.max_idx];
        }
        match self.interp_method {
            Lookup1DBlockInterpEnum::Linear => self.linear_interpolation(lookup_point_val),
            Lookup1DBlockInterpEnum::Nearest => self.nearest_interpolation(lookup_point_val),
        }
    }

    fn linear_interpolation(&self, lookup_point_val: f64) -> f64 {
        let mut idx: usize = 0;
        for (i, break_point) in self.break_points.iter().enumerate() {
            if lookup_point_val < *break_point {
                idx = i;
                break;
            }
        }
        let k: f64 = (lookup_point_val - self.break_points[idx - 1])
            / (self.break_points[idx] - self.break_points[idx - 1]);
        self.data_points[idx - 1] + k * (self.data_points[idx] - self.data_points[idx - 1])
    }

    fn nearest_interpolation(&self, lookup_point_val: f64) -> f64 {
        let mut idx: usize = 0;
        for (i, break_point) in self.break_points.iter().enumerate() {
            if lookup_point_val < *break_point {
                idx = i;
                break;
            }
        }
        let delt_high: f64 = self.break_points[idx] - lookup_point_val;
        let delt_low: f64 = lookup_point_val - self.break_points[idx - 1];

        match delt_high > delt_low {
            true => self.data_points[idx - 1],
            false => self.data_points[idx],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup1d_block_linear() {
        let break_points = BlockData::from_vector(&[0.0, 1.0, 2.0]);
        let data_points = BlockData::from_vector(&[-1.0, 1.0, 10.0]);

        let mut block = Lookup1DBlock::new("Lookup1D", &break_points, &data_points, "Linear");

        block.run(&BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), -1.0);
        block.run(&BlockData::from_scalar(1.0));
        assert_eq!(block.data.scalar(), 1.0);
        block.run(&BlockData::from_scalar(0.5));
        assert_eq!(block.data.scalar(), 0.0);
        block.run(&BlockData::from_scalar(1.5));
        assert_eq!(block.data.scalar(), 11.0 / 2.0);

        // Verify clamps output
        block.run(&BlockData::from_scalar(3.0));
        assert_eq!(block.data.scalar(), 10.0);
        block.run(&BlockData::from_scalar(-100.0));
        assert_eq!(block.data.scalar(), -1.0);
    }

    #[test]
    fn test_lookup1d_block_nearest() {
        let break_points = BlockData::from_vector(&[0.0, 1.0, 2.0]);
        let data_points = BlockData::from_vector(&[-1.0, 1.0, 10.0]);

        let mut block = Lookup1DBlock::new("Lookup1D", &break_points, &data_points, "Nearest");

        block.run(&BlockData::from_scalar(0.0));
        assert_eq!(block.data.scalar(), -1.0);
        block.run(&BlockData::from_scalar(0.25));
        assert_eq!(block.data.scalar(), -1.0);
        block.run(&BlockData::from_scalar(0.5));
        assert_eq!(block.data.scalar(), 1.0);
        block.run(&BlockData::from_scalar(0.75));
        assert_eq!(block.data.scalar(), 1.0);
        block.run(&BlockData::from_scalar(1.75));
        assert_eq!(block.data.scalar(), 10.0);
    }

    #[test]
    fn test_lookup1d_block_vectorized() {
        let break_points = BlockData::from_vector(&[0.0, 1.0, 2.0]);
        let data_points = BlockData::from_vector(&[-1.0, 1.0, 10.0]);

        let mut block = Lookup1DBlock::new("Lookup1D", &break_points, &data_points, "Linear");

        let vector_signal = BlockData::from_vector(&[0.0, 1.0, 0.5, 1.5]);

        block.run(&vector_signal);
        assert_eq!(
            block.data,
            BlockData::from_vector(&[-1.0, 1.0, 0.0, 11.0 / 2.0])
        );

        // Verify clamps output, and matrix support
        let matrix_signal = BlockData::new(2, 2, &[3.0, 300.0, -100.0, -10000.0]);
        block.run(&matrix_signal);
        assert_eq!(block.data, BlockData::new(2, 2, &[10.0, 10.0, -1.0, -1.0]));
    }
    #[test]
    #[should_panic]
    fn test_lookup1d_block_panics_with_mismatched_array_sizes() {
        let break_points = BlockData::from_vector(&[0.0, 1.0, 2.0]);
        let data_points = BlockData::from_vector(&[-1.0, 1.0]);

        let _block = Lookup1DBlock::new("Lookup1D", &break_points, &data_points, "Linear");
    }
}
