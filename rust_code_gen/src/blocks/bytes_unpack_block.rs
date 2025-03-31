use core::cmp::max;

use crate::{
    block_data::BlockData,
    byte_data::{parse_byte_data_spec, try_unpack_data, ByteDataError, ByteOrderSpec, DataType},
    stale_tracker::StaleTracker,
    traits::IsValid,
};
use log::debug;

use alloc::vec;
use alloc::vec::Vec;

// TODO I think every String can actually be a &'static str
use alloc::string::String;

pub struct BytesUnpackBlock {
    pub name: &'static str,
    pub data: Vec<BlockData>,
    pub unpack_data: Vec<(DataType, ByteOrderSpec)>,
    pub stale_check: StaleTracker,
}
impl BytesUnpackBlock {
    pub fn new(name: &'static str, unpack_data: &[String], stale_age_ms: f64) -> Self {
        let unpack_data = parse_byte_data_spec(unpack_data);
        let data = vec![BlockData::from_scalar(0.0); max(unpack_data.len(), 1)];
        BytesUnpackBlock {
            name,
            data,
            unpack_data,
            stale_check: StaleTracker::from_ms(stale_age_ms),
        }
    }

    pub fn run(&mut self, input: &BlockData, app_time_s: f64) {
        let res = if self.unpack_data.is_empty() {
            self.unpack_default(input)
        } else {
            self.unpack_data(input)
        };

        if res.is_ok() {
            self.stale_check.mark_updated(app_time_s);
        }
        debug!("{}: {:?}", self.name, self.data);
    }

    // TODO this can be folded into "unpack data"
    fn unpack_default(&mut self, input: &BlockData) -> Result<(), ByteDataError> {
        let buf = input.to_bytes();
        let dt = DataType::F64;
        if buf.chunks_exact(dt.byte_size()).len() == 0 {
            return Err(ByteDataError::InsufficientData);
        }
        for (i, chunk) in buf.chunks_exact(dt.byte_size()).enumerate() {
            let val = try_unpack_data::<byteorder::BigEndian>(chunk, &dt)
                .or(Err(ByteDataError::UnpackError))?;
            self.data[i].set_scalar(val);
        }

        Ok(())
    }

    fn unpack_data(&mut self, input: &BlockData) -> Result<(), ByteDataError> {
        let buf = input.to_bytes();
        let mut num_read = 0;
        for (i, (dt, bo)) in self.unpack_data.iter().enumerate() {
            if self.data.len() <= i {
                return Err(ByteDataError::InsufficientData);
            }

            let val = match bo {
                ByteOrderSpec::BigEndian => {
                    try_unpack_data::<byteorder::BigEndian>(&buf[num_read..], dt)
                }
                ByteOrderSpec::LittleEndian => {
                    try_unpack_data::<byteorder::LittleEndian>(&buf[num_read..], dt)
                }
            }?;
            num_read += dt.byte_size();

            self.data[i].set_scalar(val);
        }
        Ok(())
    }
}

impl IsValid for BytesUnpackBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

    #[test]
    fn test_unpacks_default_data() {
        let expected = 100f64;
        let mut block = BytesUnpackBlock::new("foo", &[], 1000.0);
        block.run(&BlockData::from_bytes(&expected.to_be_bytes()), 1.0);

        assert_eq!(block.data, vec![BlockData::from_scalar(expected)]);
        assert!(block.is_valid(1.1).all());
    }

    #[test]
    fn test_fails_to_unpack_default_data() {
        let mut block = BytesUnpackBlock::new("foo", &[], 1000.0);
        block.run(&BlockData::from_bytes(b""), 1.0);

        assert_eq!(block.data, vec![BlockData::from_scalar(0.0)]);
        assert!(!block.is_valid(1.1).all());
    }

    #[test]
    fn test_unpacks_packed_binary_data() {
        let mut res_buf = Vec::new();
        let i16_val = 42i16;
        let f32_val = 1.234f32;
        res_buf.write_i16::<BigEndian>(i16_val).unwrap();
        res_buf.write_f32::<LittleEndian>(f32_val).unwrap();

        let mut block = BytesUnpackBlock::new(
            "foo",
            &["I16:BigEndian".into(), "F32:LittleEndian".into()],
            1000.0,
        );
        block.run(&BlockData::from_bytes(&res_buf), 1.0);

        assert_eq!(
            block.data,
            vec![
                BlockData::from_scalar(i16_val as f64),
                BlockData::from_scalar(f32_val as f64)
            ]
        );
        assert!(block.is_valid(1.1).all());
    }

    #[test]
    fn test_fails_to_unpack_packed_binary_data() {
        let mut block = BytesUnpackBlock::new(
            "foo",
            &["I16:BigEndian".into(), "F32:LittleEndian".into()],
            1000.0,
        );
        block.run(&BlockData::from_bytes(b""), 1.0);

        assert_eq!(
            block.data,
            vec![BlockData::from_scalar(0.0), BlockData::from_scalar(0.0)]
        );
        assert!(!block.is_valid(1.1).all());
    }
}
