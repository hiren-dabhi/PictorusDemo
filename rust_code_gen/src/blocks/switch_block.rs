use log::debug;

use crate::block_data::BlockData;

pub struct SwitchBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub case_id: usize,
    pub cases: BlockData,
}

impl SwitchBlock {
    pub fn new(name: &'static str, ic: &BlockData, cases: &BlockData) -> SwitchBlock {
        SwitchBlock {
            name,
            case_id: cases.len(), // Set to the "default" case
            cases: cases.clone(),
            data: ic.clone(),
        }
    }
    pub fn run(&mut self, input: &BlockData, output_block_vals: &[&BlockData]) {
        // TODO: Right now, all the input blocks are run first, and this simply chooses
        // Which output to forward. Eventually codegenerator should be cleverer so
        // we only call the run function of the block that's been selected by this block
        let mut case_matched: bool = false;
        for (idx, case) in self.cases.iter().enumerate() {
            self.case_id = idx;
            // The last case is the default output if nothing else has matched
            if input.scalar() == *case {
                self.data = output_block_vals[idx].clone();
                case_matched = true;
                break;
            }
        }
        // If no cases matched, use the default (last option)
        if !case_matched {
            self.data = output_block_vals[output_block_vals.len() - 1].clone();
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_switch_block() {
        let cases = BlockData::from_vector(&[0.0, 1.0, -1.0]);
        let ic = BlockData::from_scalar(0.0);
        let mut block = SwitchBlock::new("Switch1", &ic, &cases);

        let signal1 = BlockData::from_scalar(1.0);
        let signal2 = BlockData::from_scalar(2.0);
        let signal3 = BlockData::from_scalar(-3.0);
        let output_block_vals = vec![&signal1, &signal2, &signal3];

        block.run(&BlockData::from_scalar(1.0), &output_block_vals);
        assert_eq!(block.data.scalar(), 2.0);

        block.run(&BlockData::from_scalar(-1.0), &output_block_vals);
        assert_eq!(block.data.scalar(), -3.0);

        block.run(&BlockData::from_scalar(0.0), &output_block_vals);
        assert_eq!(block.data.scalar(), 1.0);

        // Last option used if input doesn't match one of the cases
        block.run(&BlockData::from_scalar(1.2345), &output_block_vals);
        assert_eq!(block.data.scalar(), -3.0);
    }

    #[test]
    fn test_vectorized_switch_block() {
        let cases = BlockData::from_vector(&[0.0, 1.0]);
        let ic = BlockData::from_vector(&[0.0, 0.0]);
        let mut block = SwitchBlock::new("Switch1", &ic, &cases);

        let signal1 = BlockData::from_vector(&[1.0, 2.0]);
        let signal2 = BlockData::from_vector(&[3.0, 4.0]);
        let output_block_vals = vec![&signal1, &signal2];

        block.run(&BlockData::from_scalar(1.0), &output_block_vals);
        assert_eq!(block.data, signal2);

        block.run(&BlockData::from_scalar(0.0), &output_block_vals);
        assert_eq!(block.data, signal1);
    }
}
