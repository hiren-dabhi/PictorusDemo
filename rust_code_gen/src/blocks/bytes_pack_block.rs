use crate::{
    block_data::BlockData,
    byte_data::{parse_byte_data_spec, try_pack_data, ByteOrderSpec, DataType},
};
use log::warn;

pub struct BytesPackBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub pack_data: Vec<(DataType, ByteOrderSpec)>,
    pack_data_len: usize,
}

use alloc::string::String;
use alloc::vec::Vec;

impl BytesPackBlock {
    pub fn new(name: &'static str, _: &BlockData, pack_data: &[String]) -> Self {
        let pack_data = parse_byte_data_spec(pack_data);
        let pack_data_len = pack_data.iter().map(|(dt, _)| dt.byte_size()).sum();
        Self {
            name,
            data: BlockData::from_bytes(b""),
            pack_data,
            pack_data_len,
        }
    }
    pub fn run(&mut self, inputs: &Vec<&BlockData>) {
        let mut byte_data: Vec<u8> = Vec::new();
        if self.pack_data.is_empty() {
            for inpt in inputs {
                byte_data.extend(inpt.scalar().to_be_bytes())
            }
        } else {
            byte_data.resize(self.pack_data_len, 0);
            let mut num_written = 0;
            for (i, (dt, bo)) in self.pack_data.iter().enumerate() {
                let res = match bo {
                    ByteOrderSpec::BigEndian => try_pack_data::<byteorder::BigEndian>(
                        &mut byte_data[num_written..],
                        inputs[i].scalar(),
                        dt,
                    ),
                    ByteOrderSpec::LittleEndian => try_pack_data::<byteorder::LittleEndian>(
                        &mut byte_data[num_written..],
                        inputs[i].scalar(),
                        dt,
                    ),
                };

                match res {
                    Ok(n) => num_written += n,
                    Err(e) => {
                        warn!("Failed to encode data at position {}: {:?}", i, e);
                        break;
                    }
                }
            }
            byte_data.resize(num_written, 0);
        }

        self.data.set_bytes(&byte_data);
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use alloc::vec;
    use byteorder::{BigEndian, WriteBytesExt};

    #[test]
    fn test_packs_f64_big_endian_if_no_format_specified() {
        let input = 123.45;
        let mut block = BytesPackBlock::new("foo", &BlockData::from_bytes(b""), &[]);
        block.run(&vec![&BlockData::from_scalar(input)]);
        assert_eq!(block.data, BlockData::from_bytes(&input.to_be_bytes()));
    }

    #[test]
    fn test_packs_data_based_on_spec() {
        let mut expected = Vec::new();
        let input1 = 255.0;
        let input2 = 123.0;
        expected.write_f32::<BigEndian>(input1 as f32).unwrap();
        expected.write_u24::<BigEndian>(input2 as u32).unwrap();

        let mut block = BytesPackBlock::new(
            "foo",
            &BlockData::from_bytes(b""),
            &[String::from("F32:BigEndian"), String::from("U24:BigEndian")],
        );
        block.run(&vec![
            &BlockData::from_scalar(input1),
            &BlockData::from_scalar(input2),
        ]);

        assert_eq!(block.data, BlockData::from_bytes(&expected));
    }

    #[test]
    fn test_packs_nothing_if_no_input() {
        let mut block = BytesPackBlock::new("foo", &BlockData::from_bytes(b""), &[]);
        block.run(&vec![]);

        assert_eq!(block.data, BlockData::from_bytes(&[]));
    }

    #[test]
    fn test_packs_data_with_various_data_types_and_byte_orders() {
        let mut expected = Vec::new();
        let input1 = 1000.0;
        let input2 = 12345.0;
        let input3 = -1234.0;
        expected
            .write_i16::<byteorder::LittleEndian>(input1 as i16)
            .unwrap();
        expected
            .write_u16::<byteorder::BigEndian>(input2 as u16)
            .unwrap();
        expected
            .write_i32::<byteorder::BigEndian>(input3 as i32)
            .unwrap();

        let mut block = BytesPackBlock::new(
            "foo",
            &BlockData::from_bytes(b""),
            &[
                String::from("I16:LittleEndian"),
                String::from("U16:BigEndian"),
                String::from("I32:BigEndian"),
            ],
        );
        block.run(&vec![
            &BlockData::from_scalar(input1),
            &BlockData::from_scalar(input2),
            &BlockData::from_scalar(input3),
        ]);

        assert_eq!(block.data, BlockData::from_bytes(&expected));
    }

    #[test]
    fn test_packs_data_little_endian() {
        let mut expected = Vec::new();
        let input1 = 0.0;
        let input2 = 25.0;
        let input3 = 0.0;
        expected.write_u8(input1 as u8).unwrap();
        expected
            .write_i16::<byteorder::LittleEndian>(input2 as i16)
            .unwrap();
        expected
            .write_i16::<byteorder::LittleEndian>(input3 as i16)
            .unwrap();

        let mut block = BytesPackBlock::new(
            "foo",
            &BlockData::from_bytes(b""),
            &[
                String::from("U8:LittleEndian"),
                String::from("I16:LittleEndian"),
                String::from("I16:LittleEndian"),
            ],
        );
        block.run(&vec![
            &BlockData::from_scalar(input1),
            &BlockData::from_scalar(input2),
            &BlockData::from_scalar(input3),
        ]);

        assert_eq!(block.data, BlockData::from_bytes(&expected));
    }

    #[test]
    fn test_packs_data_big_endian() {
        let mut expected = Vec::new();
        let input1 = 0.0;
        let input2 = 25.0;
        let input3 = 0.0;
        expected.write_u8(input1 as u8).unwrap();
        expected.write_i16::<BigEndian>(input2 as i16).unwrap();
        expected.write_i16::<BigEndian>(input3 as i16).unwrap();

        let mut block = BytesPackBlock::new(
            "foo",
            &BlockData::from_bytes(b""),
            &[
                String::from("U8:BigEndian"),
                String::from("I16:BigEndian"),
                String::from("I16:BigEndian"),
            ],
        );
        block.run(&vec![
            &BlockData::from_scalar(input1),
            &BlockData::from_scalar(input2),
            &BlockData::from_scalar(input3),
        ]);

        assert_eq!(block.data, BlockData::from_bytes(&expected));
    }
}
