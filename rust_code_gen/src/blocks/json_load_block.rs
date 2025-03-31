use crate::{
    block_data::{BlockData, BlockDataType},
    stale_tracker::StaleTracker,
    traits::IsValid,
};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use log::debug;
use miniserde::json::{self, Number, Object, Value};

fn parse_array(val: &Value) -> Result<Vec<f64>, ()> {
    match val {
        Value::Array(v) => v
            .iter()
            .map(|n| match n {
                Value::Number(v) => Ok(parse_number(v)),
                _ => Err(()),
            })
            .collect(),
        _ => Err(()),
    }
}
fn parse_numeric_value(val: &Value) -> Result<BlockData, ()> {
    match val {
        Value::Number(v) => Ok(BlockData::from_scalar(parse_number(v))),
        Value::Array(v) => {
            if v.len() == 0 {
                // Assume this is an empty number array
                return Ok(BlockData::from_vector(&[]));
            }

            match &v[0] {
                Value::Number(_) => {
                    let data: Vec<f64> = v
                        .iter()
                        .map(|n| match n {
                            Value::Number(v) => Ok(parse_number(v)),
                            _ => Err(()),
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(BlockData::from_vector(&data))
                }
                Value::Array(v_inner) => {
                    let rows = v.len();
                    let cols = v_inner.len();
                    let data: Vec<f64> = v
                        .iter()
                        .map(parse_array)
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .flatten()
                        .collect();
                    Ok(BlockData::from_row_slice(rows, cols, &data))
                }
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

fn parse_number(num_val: &Number) -> f64 {
    match num_val {
        Number::F64(v) => *v,
        Number::I64(v) => *v as f64,
        Number::U64(v) => *v as f64,
    }
}

fn parse_string_value(val: &Value) -> Result<BlockData, ()> {
    match val {
        Value::String(v) => Ok(BlockData::from_bytes(v.as_bytes())),
        _ => Err(()),
    }
}

fn parse_value(value: &Value, data_type: &BlockDataType) -> Result<BlockData, ()> {
    match data_type {
        BlockDataType::Scalar => parse_numeric_value(value),
        BlockDataType::BytesArray => parse_string_value(value),
        _ => {
            debug!("Unhandled data type: {:?}", data_type);
            Err(())
        }
    }
}

fn parse_select_spec(data: &[String]) -> Vec<(BlockDataType, String)> {
    data.iter()
        .map(|d| d.split_once(':').expect("Invalid select data format"))
        .map(|(dt, field)| (dt.parse::<BlockDataType>().unwrap(), field.into()))
        .collect()
}

pub struct JsonLoadBlock {
    pub name: &'static str,
    pub select_data: Vec<(BlockDataType, String)>,
    pub data: Vec<BlockData>,
    pub stale_check: StaleTracker,
}

impl JsonLoadBlock {
    pub fn new(name: &'static str, select_data: &[String], stale_age_ms: f64) -> Self {
        let select_data = parse_select_spec(select_data);
        let data = if !select_data.is_empty() {
            select_data
                .iter()
                .map(|(dt, _)| match dt {
                    BlockDataType::BytesArray => BlockData::from_bytes(b""),
                    _ => BlockData::from_scalar(0.0),
                })
                .collect()
        } else {
            vec![BlockData::from_scalar(0.0)]
        };

        Self {
            name,
            select_data,
            data,
            stale_check: StaleTracker::from_ms(stale_age_ms),
        }
    }

    pub fn run(&mut self, input: &BlockData, app_time_s: f64) {
        let res = match input.get_type() {
            BlockDataType::BytesArray => self.parse_bytes(input),
            _ => panic!("JsonLoadBlock only supports byte data"),
        };

        if res.is_ok() {
            self.stale_check.mark_updated(app_time_s);
        }
    }

    fn parse_bytes(&mut self, input: &BlockData) -> Result<(), ()> {
        let str_data = input.raw_string();
        debug!("{}: Received JSON string: {}", self.name, &str_data);
        if self.select_data.is_empty() {
            // Parse as single item. Only support scalars for now
            let data: Value = json::from_str(&str_data).or(Err(()))?;
            self.data[0] = parse_value(&data, &BlockDataType::Scalar)?;
        } else {
            // Parse as object
            let data: Object = json::from_str(&str_data).or(Err(()))?;
            for (i, (dt, field)) in self.select_data.iter().enumerate() {
                let value = data.get(field).ok_or(())?;
                self.data[i] = parse_value(value, dt)?;
            }
        };

        Ok(())
    }
}

impl IsValid for JsonLoadBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reads_scalar_data_if_no_selectors() {
        let app_time_s = 0.1;
        let input_data = BlockData::from_bytes(br#"1.2"#);
        let mut block = JsonLoadBlock::new("foo", &[], 0.0);
        block.run(&input_data, app_time_s);
        assert_eq!(block.data, vec![BlockData::from_scalar(1.2)]);
        assert!(block.is_valid(app_time_s).all());
    }

    #[test]
    fn test_reads_object_data_if_has_selectors() {
        let input_data = BlockData::from_bytes(br#" {"foo": 99.0, "bar": "hello", "baz": [1.0, 2.0], "buzz": [[1.0, 0.0],[0.0, 1.0]]} "#);
        let mut block = JsonLoadBlock::new(
            "foo",
            &[
                "Scalar:foo".into(),
                "BytesArray:bar".into(),
                "Scalar:baz".into(),
                "Scalar:buzz".into(),
            ],
            1000.0,
        );
        block.run(&input_data, 0.1);
        assert_eq!(
            block.data,
            vec![
                BlockData::from_scalar(99.0),
                BlockData::from_bytes(b"hello"),
                BlockData::from_vector(&[1.0, 2.0]),
                BlockData::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0])
            ]
        );
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_reads_invalid_json_input_without_panicking() {
        let input_data = BlockData::from_bytes(b"invalid_json");
        let mut block = JsonLoadBlock::new("foo", &[], 1000.0);
        block.run(&input_data, 0.1);
        assert!(!block.is_valid(0.1).all());
    }

    #[test]
    fn test_reads_non_existing_key_in_selector() {
        let input_data = BlockData::from_bytes(br#"{"foo": 99.0, "bar": "hello"}"#);
        let mut block = JsonLoadBlock::new("foo", &["Scalar:non_existing_key".into()], 1000.0);
        block.run(&input_data, 0.1);
        assert_eq!(block.data, vec![BlockData::from_scalar(0.0)]);
        assert!(!block.is_valid(0.1).all());
    }

    #[test]
    fn test_reads_empty_input() {
        let input_data = BlockData::from_bytes(b"");
        let mut block = JsonLoadBlock::new("foo", &[], 1000.0);
        block.run(&input_data, 0.1);
        assert_eq!(block.data, vec![BlockData::from_scalar(0.0)]);
        assert!(!block.is_valid(0.1).all());
    }

    #[test]
    fn test_reads_numeric_array() {
        let input_data = BlockData::from_bytes(br#"[1.0, 2.0, 3.0]"#);
        let mut block = JsonLoadBlock::new("foo", &[], 1000.0);
        block.run(&input_data, 0.1);
        assert_eq!(block.data, vec![BlockData::from_vector(&[1.0, 2.0, 3.0])]);
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_reads_empty_numeric_array() {
        let input_data = BlockData::from_bytes(br#"[]"#);
        let mut block = JsonLoadBlock::new("foo", &[], 1000.0);
        block.run(&input_data, 0.1);
        assert_eq!(block.data, vec![BlockData::from_vector(&[])]);
        assert!(block.is_valid(0.1).all());
    }

    #[test]
    fn test_reads_numeric_matrix() {
        let input_data = BlockData::from_bytes(br#"[[1.0, 2.0], [3.0, 4.0]]"#);
        let mut block = JsonLoadBlock::new("foo", &[], 1000.0);
        block.run(&input_data, 0.1);
        assert_eq!(
            block.data,
            vec![BlockData::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0])]
        );
        assert!(block.is_valid(0.1).all());
    }
}
