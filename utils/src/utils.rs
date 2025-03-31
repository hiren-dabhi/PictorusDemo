extern crate alloc;
use alloc::str;
use alloc::string::String;
use alloc::vec::Vec;
use core::{convert::Infallible, time::Duration};
use num_traits::{AsPrimitive, Float};

use crate::block_data::{BlockData, BlockDataType};
use log::debug;
use miniserde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct ParseEnumError;

pub struct PictorusVars {
    pub run_path: String,
    pub data_log_rate_hz: f64,
    pub transmit_enabled: bool,
    pub publish_socket: String,
}

// TODO Can we create an error type for these functions? Could we use Option<> instead?
#[allow(clippy::result_unit_err)]
pub fn buffer_to_scalar(buf: &[u8]) -> Result<f64, ()> {
    debug!("Converting buffer to scalar: {:?}", buf);
    string_to_scalar(str::from_utf8(buf).or(Err(()))?)
}

#[allow(clippy::result_unit_err)]
pub fn string_to_scalar(val: &str) -> Result<f64, ()> {
    debug!("Converting string to scalar: {:?}", val);
    val.trim().parse().or(Err(()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PictorusError {
    pub err_type: String,
    pub message: String,
}

impl PictorusError {
    pub fn new(err_type: String, message: String) -> Self {
        PictorusError { err_type, message }
    }
}

impl From<Infallible> for PictorusError {
    fn from(_: Infallible) -> Self {
        unreachable!();
    }
}

pub fn parse_select_spec(data: &[String]) -> Vec<(BlockDataType, usize)> {
    data.iter()
        .map(|d| d.split_once(':').expect("Invalid select data format"))
        .map(|(dt, index)| {
            (
                dt.parse::<BlockDataType>().unwrap(),
                string_to_scalar(index).unwrap() as usize,
            )
        })
        .collect()
}

pub fn update_state_output(outputs: &mut [BlockData], block: &BlockData, index: usize) {
    // Simple helper to only clone into the output of a State if the Block data has
    // actually updated.
    if *block == outputs[index] {
        return;
    }
    outputs[index] = block.clone();
}

pub fn positive_duration(f: f64) -> Duration {
    Duration::from_secs_f64(f64::max(0.0, f))
}

pub fn us_to_s<T, U>(time_us: T) -> U
where
    T: AsPrimitive<U> + Copy,
    U: Float + 'static,
{
    let million: U = U::from(1_000_000).unwrap();
    let seconds_part = (time_us.as_() / million).floor();
    let microseconds_remainder = time_us.as_() % million;
    seconds_part + microseconds_remainder / million
}

pub fn s_to_us<T, U>(time_s: T) -> U
where
    T: Float + AsPrimitive<U> + Copy,
    U: AsPrimitive<T> + Copy,
{
    let million: T = T::from(1_000_000).unwrap();
    (time_s * million).as_()
}

// All remaining std-only methods here
cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use super::*;
        use log::{info, warn};
        use miniserde::json;
        use std::prelude::rust_2021::*;

        use std::panic::PanicHookInfo;
        use std::collections::HashMap;
        use std::fs;
        use std::format;

        pub type DiagramParams = HashMap<String, HashMap<String, String>>;

        // Trait definition for parameters loadable from DiagramParams or env vars
        pub trait LoadableParams: Sized {
            fn parse(source: &str, default: Option<&Self>) -> Option<Self>;
        }

        impl LoadableParams for String {
            fn parse(source: &str, _default: Option<&Self>) -> Option<Self> {
                Some(source.to_string())
            }
        }

        impl LoadableParams for f64 {
            fn parse(source: &str, _default: Option<&Self>) -> Option<Self> {
                source.parse().ok()
            }
        }

        impl LoadableParams for Vec<String> {
            fn parse(source: &str, _default: Option<&Self>) -> Option<Self> {
                Some(string_to_vec(source))
            }
        }

        impl LoadableParams for BlockData {
            fn parse(source: &str, default: Option<&Self>) -> Option<Self> {
                let parsed_val: Vec<f64> = string_to_vec::<f64>(source);

                match (parsed_val.len(), default) {
                    (1, Some(d)) => Some(BlockData::scalar_sizeof(parsed_val[0], d)),
                    (len, Some(d)) if len == d.n_elements() => Some(BlockData::from_row_slice(d.nrows(), d.ncols(), &parsed_val)),
                    _ => Some(BlockData::from_vector(&parsed_val)),
                }
            }
        }

        // Generic function to load a parameter from env or params
        pub fn load_param<T: LoadableParams + std::fmt::Debug>(
            block_name: &str,
            var_name: &str,
            default: T,
            blocks_map: &DiagramParams,
        ) -> T {
            let env_var_name = format!("{}_{}", block_name.to_uppercase(), var_name.to_uppercase());

            // Try loading from ENV
            if let Ok(env_var) = std::env::var(&env_var_name) {
                if let Some(parsed) = T::parse(&env_var, Some(&default)) {
                    info!("Found env variable {} with value '{:?}'", env_var_name, parsed);
                    return parsed;
                }
            }

            // Try loading from DiagramParams
            if let Some(params_map) = blocks_map.get(block_name).and_then(|map| map.get(var_name)) {
                if let Some(parsed) = T::parse(params_map, Some(&default)) {
                    info!(
                        "Parsing and loading {}_{} from params file with value {:?}",
                        block_name, var_name, parsed
                    );
                    return parsed;
                }
            }

            // Otherwise return the default
            default
        }

        pub fn load_ic(block_name: &str, var_name: &str, default: BlockData, blocks_map: &DiagramParams) -> BlockData {
            let ic = load_param::<BlockData>(block_name, var_name, default.clone(), blocks_map);

            if !ic.same_size(&default) {
                panic!(
                    "Cannot load parameter {}:{} with size {:?}, required size is {:?}",
                    block_name, var_name, ic.size(), default.size());
            }
            ic
        }

        fn string_to_vec<T: str::FromStr>(vec_str: &str) -> Vec<T> {
            let cleaned_str = vec_str.replace(['[', ']', '\"'], "").replace(", ", ",");
            if cleaned_str.is_empty() {
                return Vec::new();
            }

            cleaned_str
                .split(',')
                .filter_map(|s| s.parse::<T>().ok())
                .collect::<Vec<_>>()
        }

        pub fn get_diagram_params(vars: &PictorusVars) -> DiagramParams {
            // Load diagram variables from diagram_params.json, if present.
            let diagram_params_path =
                std::path::PathBuf::from(&vars.run_path).join("diagram_params.json");
            info!(
                "Looking for diagram params file: {}",
                diagram_params_path.display()
            );
            let params_file = std::fs::read_to_string(diagram_params_path);
            let input_params_json = match params_file {
                Ok(val) => {
                    info!("Found params file!");
                    val
                }
                Err(_err) => {
                    info!("No params file found.");
                    String::from("{}")
                }
            };
            let diagram_params = json::from_str(input_params_json.as_str()).unwrap_or_else(|_| {
                warn!("Error parsing params file, using empty params map.");
                HashMap::<String, HashMap<String, String>>::new()
            });
            diagram_params
        }

        pub fn get_pictorus_vars() -> PictorusVars {
            // Load special environment variables that control app execution, or use safe defaults if not present.
            PictorusVars {
                data_log_rate_hz: std::env::var("APP_DATA_LOG_RATE_HZ")
                    .unwrap_or("0".to_string())
                    .trim()
                    .parse()
                    .unwrap(),
                run_path: std::env::var("APP_RUN_PATH").unwrap_or("".to_string()),
                transmit_enabled: std::env::var("APP_TRANSMIT_ENABLED")
                    .unwrap_or("true".to_string())
                    .parse()
                    .unwrap(),
                publish_socket: std::env::var("APP_PUBLISH_SOCKET").unwrap_or("".to_string()),
            }
        }

        pub fn dump_error(err: &PictorusError, run_path: &str) {
            let path = std::path::PathBuf::from(run_path).join("pictorus_errors.json");
            info!("Error log path: {:?}", path);
            fs::write(path, json::to_string(err)).ok();
        }

        pub fn custom_panic_handler(panic_info: &PanicHookInfo, run_path: &str) {
            warn!("Unhandled panic, dumping stack trace to error log...");
            let err = PictorusError {
                err_type: "unhandled".to_string(),
                message: panic_info.to_string(),
            };
            dump_error(&err, run_path);
        }

        pub fn get_block_type<T>(_: &T) -> String {
            // Pass in a block, get back it's name!
            let name_str = std::any::type_name::<T>().to_string();
            let name_vec: Vec<&str> = name_str.split("::").collect();
            String::from(name_vec[name_vec.len() - 1])
        }
    }
}

#[cfg(all(test, feature = "std"))]
#[allow(clippy::approx_constant)]
mod tests {
    use super::*;
    use alloc::vec;
    use pretty_assertions::assert_eq;
    use temp_env::with_vars;

    #[test]
    fn test_load_param_f64() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert("test_var".to_string(), "3.14159".to_string());
            params
        });

        let default = 5.0;

        // Test env_var path
        with_vars(vec![("TEST_BLOCK_TEST_VAR", Some("42.0"))], || {
            let result_env = load_param::<f64>("test_block", "test_var", default, &diagram_params);
            assert_eq!(result_env, 42.0);
        });

        // Test diagram_param_path
        let result_param = load_param::<f64>("test_block", "test_var", default, &diagram_params);
        assert_eq!(result_param, 3.14159);

        // Test default path
        let result_default = load_param::<f64>("test_block", "foo", default, &diagram_params);
        assert_eq!(result_default, default);
    }

    #[test]
    fn test_load_param_string() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert("test_var".to_string(), "hello".to_string());
            params
        });

        with_vars(vec![("TEST_BLOCK_TEST_VAR", Some("bar"))], || {
            let result_env = load_param::<String>(
                "test_block",
                "test_var",
                "default_string".to_string(),
                &diagram_params,
            );
            assert_eq!(result_env, "bar".to_string());
        });

        let result_param = load_param::<String>(
            "test_block",
            "test_var",
            "default_string".to_string(),
            &diagram_params,
        );

        assert_eq!(result_param, "hello".to_string());

        let result_default = load_param::<String>(
            "test_block",
            "foo",
            "default_string".to_string(),
            &diagram_params,
        );
        assert_eq!(result_default, "default_string".to_string());
    }

    #[test]
    fn test_load_param_vec_string() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert(
                "select_indices".to_string(),
                "[Scalar:2, Scalar:3]".to_string(),
            );
            params
        });

        let default = Vec::from([String::from("Scalar:1"), String::from("BytesArray:3")]);

        with_vars(
            vec![("TEST_BLOCK_TEST_VAR", Some("[Foo, Bar]".to_string()))],
            || {
                let result_env = load_param::<Vec<String>>(
                    "test_block",
                    "test_var",
                    vec!["default_string".to_string()],
                    &diagram_params,
                );
                assert_eq!(result_env, vec!["Foo".to_string(), "Bar".to_string()]);
            },
        );

        let result_param = load_param::<Vec<String>>(
            "test_block",
            "select_indices",
            default.clone(),
            &diagram_params,
        );

        assert_eq!(
            result_param,
            Vec::from([String::from("Scalar:2"), String::from("Scalar:3")])
        );

        let result_default =
            load_param::<Vec<String>>("test_block", "foo", default.clone(), &diagram_params);
        assert_eq!(result_default, default.clone());
    }

    #[test]
    fn test_load_param_vec_f64() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert(
                "test_var".to_string(),
                "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]".to_string(),
            );
            params
        });

        let default = BlockData::new(2, 3, &[7., 8., 9., 10., 11., 12.]);

        with_vars(
            vec![(
                "TEST_BLOCK_TEST_VAR",
                Some("[-1.0, -2.0, -3.0, -4.0, -5.0, -6.0]"),
            )],
            || {
                let result_env = load_param::<BlockData>(
                    "test_block",
                    "test_var",
                    default.clone(),
                    &diagram_params,
                );
                assert_eq!(
                    result_env,
                    BlockData::new(2, 3, &[-1., -2., -3., -4., -5., -6.])
                );
            },
        );

        let result_param =
            load_param::<BlockData>("test_block", "test_var", default.clone(), &diagram_params);

        assert_eq!(
            result_param,
            BlockData::new(2, 3, &[1., 2., 3., 4., 5., 6.])
        );

        let result_default =
            load_param::<BlockData>("test_block", "foo", default.clone(), &diagram_params);
        assert_eq!(result_default, default.clone());
    }

    #[test]
    fn test_load_param_vec_f64_scalar_override_vector_default() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert("test_var".to_string(), "42.0".to_string());
            params
        });

        let default = BlockData::from_vector(&[1.0, 1.0, 1.0]);

        with_vars(vec![("TEST_BLOCK_TEST_VAR", Some("[-13.0]"))], || {
            let result_env =
                load_param::<BlockData>("test_block", "test_var", default.clone(), &diagram_params);
            assert_eq!(result_env, BlockData::from_vector(&[-13., -13., -13.]));
        });

        let result_param =
            load_param::<BlockData>("test_block", "test_var", default.clone(), &diagram_params);

        // If override is a scalar of different dimensions from default, use default dimensions
        assert_eq!(result_param, BlockData::from_vector(&[42.0, 42.0, 42.0]));

        let result_default =
            load_param::<BlockData>("test_block", "foo", default.clone(), &diagram_params);
        assert_eq!(result_default, default.clone());
    }

    #[test]
    #[should_panic(
        expected = "Cannot load parameter test_block:test_var with size (1, 2), required size is (1, 3)"
    )]
    fn test_load_ic_vec_f64_vector_override_vector_default() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert("test_var".to_string(), "[42.0, 43.0]".to_string());
            params
        });

        let default = BlockData::from_vector(&[1.0, 1.0, 1.0]);
        load_ic("test_block", "test_var", default.clone(), &diagram_params);
    }

    #[test]
    fn test_load_param_matrix_f64() {
        let mut diagram_params = DiagramParams::new();
        diagram_params.insert("test_block".to_string(), {
            let mut params = HashMap::new();
            params.insert(
                "test_var".to_string(),
                "[[1.0, 2.0], [3.0, 4.0]]".to_string(),
            );
            params
        });

        let default = BlockData::new(2, 2, &[7., 8., 9., 10.]);

        with_vars(
            vec![("TEST_BLOCK_TEST_VAR", Some("[[3.0, 4.0],[5.0, 6.0]]"))],
            || {
                let result_env = load_param::<BlockData>(
                    "test_block",
                    "test_var",
                    default.clone(),
                    &diagram_params,
                );
                assert_eq!(result_env, BlockData::new(2, 2, &[3., 4., 5., 6.]));
            },
        );

        let result_param =
            load_param::<BlockData>("test_block", "test_var", default.clone(), &diagram_params);

        assert_eq!(result_param, BlockData::new(2, 2, &[1., 2., 3., 4.]));

        let result_default =
            load_param::<BlockData>("test_block", "foo", default.clone(), &diagram_params);

        assert_eq!(result_default, default.clone());
    }

    #[test]
    fn test_string_to_vec_f64() {
        assert_eq!(
            string_to_vec::<f64>(&String::from("[1, 2, 3]")),
            vec![1., 2., 3.]
        );
        // If just a single value without brackets passed in, should still get a vector back
        assert_eq!(
            string_to_vec::<f64>(&String::from("3.14159")),
            vec![3.14159]
        );
    }

    #[test]
    fn test_positive_duration() {
        assert_eq!(positive_duration(-2.5), Duration::from_secs_f64(0.0));
        assert_eq!(positive_duration(2.5), Duration::from_secs_f64(2.5));
    }

    #[test]
    fn test_us_to_s() {
        assert_eq!(us_to_s::<u64, f64>(1_234_567), 1.234567f64);
    }

    #[test]
    fn test_s_to_us() {
        assert_eq!(s_to_us::<f64, u64>(1.234567f64), 1_234_567);
    }
}
