use core::time::Duration;

use crate::alloc::string::ToString;
use alloc::string::String;
use miniserde::json::{self, Value};
use utils::{BlockData, BlockDataType};

#[cfg(feature = "std")]
pub mod csv_logger;

#[cfg(feature = "std")]
pub mod linux_logger;

#[cfg(feature = "std")]
pub mod udp_logger;

#[cfg(feature = "rtt")]
pub mod rtt_logger;

/// The PictorusLogger trait is used to interface Pictorus block data and labels to various logging backends.
pub trait PictorusLogger {
    /// Trait method to add samples to the logger. Data is typically logged or broadcast here.
    fn add_samples(&mut self, app_time: Duration, current_state: &str, block_data: &[BlockData]);
}

/// The Logger trait is used to log data to a file or transmit via telemetry.
///
/// Current implementations:
///
/// CsvLogger can be used to format and log CSV data to a file.
/// UdpLogger can be used to format and transmit telemetry data over UDP.
/// RttLogger can be used to transmit telemetry data over RTT.
pub trait Logger {
    /// Trait method to determine if the logger should log data based on the app's current elapsed
    /// time.
    fn should_log(&mut self, app_time: Duration) -> bool;

    /// Trait method to log data, with an option header parameter, for example, when first
    /// logging to a CSV file, a packet header, or comments
    fn log(&mut self, app_time: Duration, data: &str, header: Option<String>);
}

pub fn format_samples_json(
    app_time: Duration,
    block_data: &[BlockData],
    current_state: &str,
    labels: &[&'static str],
) -> String {
    let mut m = json::Object::new();
    for (idx, entry) in block_data.iter().enumerate() {
        let json_data = match entry.get_type() {
            BlockDataType::Scalar => entry.to_json(),
            _ => Value::String(entry.stringify()),
        };
        m.insert(labels[idx].to_string(), json_data);
    }
    m.insert(
        "state_id".to_string(),
        Value::String(current_state.to_string()),
    );
    m.insert(
        "app_time_us".to_string(),
        Value::Number(miniserde::json::Number::U64(app_time.as_micros() as u64)),
    );
    json::to_string(&m)
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use crate::alloc::string::ToString;
    use crate::loggers::format_samples_json;
    use alloc::vec::Vec;
    use utils::BlockData;

    #[test]
    fn test_format_telemetry() {
        // Verify we can format samples of different array types to transmit over udp without errors
        let labels = ["vector", "scalar", "matrix", "bytesarray"];
        let samples = Vec::from([
            BlockData::from_vector(&[0.0, 2.0, 4.0]),
            BlockData::from_scalar(1.0),
            BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]),
            BlockData::from_bytes(&[1, 2, 3]),
        ]);
        let current_state = "main_state";

        let udp_telem = format_samples_json(Duration::ZERO, &samples, current_state, &labels);
        assert_eq!(udp_telem, "{\"app_time_us\":0,\"bytesarray\":\"[1,2,3]\",\"matrix\":\"[[5.0,6.0],[7.0,8.0]]\",\"scalar\":1.0,\"state_id\":\"main_state\",\"vector\":\"[[0.0,2.0,4.0]]\"}".to_string());
    }
}
