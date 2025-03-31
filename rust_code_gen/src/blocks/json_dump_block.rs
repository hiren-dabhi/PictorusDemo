use alloc::str::FromStr;
use alloc::string::String;
use alloc::vec::Vec;

use crate::block_data::{BlockData, BlockDataType};
use miniserde::json::{self, Array, Value};
use utils::ParseEnumError;

#[derive(Debug)]
enum OutputType {
    Default,
    Utf8,
}

impl FromStr for OutputType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(Self::Default),
            "Utf8" => Ok(Self::Utf8),
            _ => Err(ParseEnumError),
        }
    }
}

fn parse_output_spec(data: &[String]) -> Vec<(OutputType, String)> {
    data.iter()
        .map(|d| d.split_once(':').expect("Invalid output data format"))
        .map(|(dt, field)| (dt.parse::<OutputType>().unwrap(), field.into()))
        .collect()
}

fn to_json_utf8(data: &BlockData) -> Value {
    // Attempt to convert bytes arrays to a json string. Other types will get quoted as a string
    match data.get_type() {
        BlockDataType::BytesArray => Value::String(data.raw_string()),
        _ => Value::String(data.stringify()),
    }
}

pub struct JsonDumpBlock {
    pub name: &'static str,
    pub data: BlockData,
    output_data: Vec<(OutputType, String)>,
}

impl JsonDumpBlock {
    pub fn new(name: &'static str, _: &BlockData, output_data: &[String]) -> Self {
        let output_data = parse_output_spec(output_data);
        Self {
            name,
            data: BlockData::from_bytes(b""),
            output_data,
        }
    }

    pub fn run(&mut self, inputs: &Vec<&BlockData>) {
        let json_data = if self.output_data.is_empty() {
            if inputs.len() == 1 {
                // If single data, dump straight to JSON
                inputs[0].to_json()
            } else {
                // Otherwise dump to an array
                let mut data: Array = json::Array::new();
                for &inpt in inputs {
                    data.push(inpt.to_json());
                }
                Value::Array(data)
            }
        } else {
            let mut data = json::Object::new();
            for (i, &inpt) in inputs.iter().enumerate() {
                if self.output_data.len() <= i {
                    break;
                }

                let (ot, key) = &self.output_data[i];
                let inpt_json = match ot {
                    OutputType::Default => inpt.to_json(),
                    OutputType::Utf8 => to_json_utf8(inpt),
                };
                data.insert(key.clone(), inpt_json);
            }
            Value::Object(data)
        };
        self.data.set_bytes(json::to_string(&json_data).as_bytes());
    }
}

#[cfg(test)]
#[allow(clippy::approx_constant)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_writes_array_data_if_no_labels() {
        let mut block = JsonDumpBlock::new("foo", &BlockData::from_bytes(b""), &[]);

        // First input to the block is a scalar
        let signal1 = BlockData::from_scalar(1.0);

        // Second input is a vector
        let signal2 = BlockData::from_vector(&[2.5, 3.14159]);

        // Third input is some bytes data
        let signal3 = BlockData::from_bytes(&[1, 2, 3]);

        // Fourth signal is a matrix
        let signal4 = BlockData::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);

        block.run(&vec![&signal1, &signal2, &signal3, &signal4]);

        let expected_string = r#"[1.0,[[2.5,3.14159]],[1,2,3],[[1.0,0.0],[0.0,1.0]]]"#.to_string();
        assert_eq!(block.data.raw_string(), expected_string)
    }

    #[test]
    fn test_writes_object_data_if_has_labels() {
        let mut block = JsonDumpBlock::new(
            "foo",
            &BlockData::from_bytes(b""),
            &[
                "Default:foo".into(),
                "Default:bar".into(),
                "Default:baz".into(),
                "Utf8:buzz".into(),
                "Default:matrix".into(),
            ],
        );

        // First input to the block is a scalar
        let signal1 = BlockData::from_scalar(1.0);

        // Second input is a vector
        let signal2 = BlockData::from_vector(&[2.5, 3.14159]);

        // Third input is some bytes data
        let signal3 = BlockData::from_bytes(&[1, 2, 3]);

        // Fourth input is a string
        let signal4 = BlockData::from_bytes(b"hello there");

        // Fifth signal is a matrix
        let signal5 = BlockData::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);

        block.run(&vec![&signal1, &signal2, &signal3, &signal4, &signal5]);

        let expected_string =
            r#"{"bar":[[2.5,3.14159]],"baz":[1,2,3],"buzz":"hello there","foo":1.0,"matrix":[[1.0,0.0],[0.0,1.0]]}"#.to_string();
        assert_eq!(block.data.raw_string(), expected_string)
    }

    #[test]
    fn test_writes_empty_input() {
        let mut block = JsonDumpBlock::new("foo", &BlockData::from_bytes(b""), &[]);
        block.run(&vec![]);
        assert_eq!(block.data.raw_string(), "[]");
    }

    #[test]
    fn test_writes_single_input_with_label() {
        let mut block = JsonDumpBlock::new(
            "foo",
            &BlockData::from_bytes(b""),
            &["Default:label".into()],
        );
        let signal1 = BlockData::from_scalar(1.0);
        block.run(&vec![&signal1]);
        assert_eq!(block.data.raw_string(), r#"{"label":1.0}"#);
    }

    #[test]
    #[should_panic]
    fn test_writes_invalid_output_data_spec() {
        let mut block =
            JsonDumpBlock::new("foo", &BlockData::from_bytes(b""), &["InvalidSpec".into()]);
        let signal1 = BlockData::from_scalar(1.0);
        block.run(&vec![&signal1]);
    }
}
