use chrono::Utc;
use core::time::Duration;
use log::info;
use miniserde::json::{self, Value};
use std::fs::File;
use std::io::Write;
use utils::{BlockData, BlockDataType};

use super::{Logger, PictorusLogger};

/// CsvLogger logs data to a file in CSV format.
///
/// Note, this uses a UTC time to be passed into the log. Other loggers
/// may use the app time in conjunction with the a device manager starting
/// timestamp to calculate the UTC time.
pub struct CsvLogger<const N: usize> {
    labels: [&'static str; N],
    last_csv_log_time: Option<Duration>,
    pub csv_log_period: Duration,
    pub file: std::fs::File,
    pub output_path: std::path::PathBuf,
    pub app_start_epoch: Duration,
}

impl<const N: usize> CsvLogger<N> {
    pub fn new(
        labels: [&'static str; N],
        csv_log_period: Duration,
        output_path: std::path::PathBuf,
    ) -> Self {
        let mut file_obj = File::create("/dev/null").unwrap();
        if !csv_log_period.is_zero() {
            info!("DataLogger CSV output period: {:?}", csv_log_period);
            info!("Streaming data output to file: {}", output_path.display());
            file_obj = File::create(std::path::PathBuf::from(&output_path)).unwrap();
        } else {
            info!("Not streaming output to file, logging rate set to zero.");
        }

        CsvLogger {
            labels,
            last_csv_log_time: None,
            csv_log_period,
            file: file_obj,
            output_path,
            app_start_epoch: Duration::from_micros(
                Utc::now()
                    .timestamp_micros()
                    .try_into()
                    .expect("Could not cast app start epoch as u64"),
            ),
        }
    }
}

impl<const N: usize> PictorusLogger for CsvLogger<N> {
    fn add_samples(&mut self, app_time: Duration, current_state: &str, block_data: &[BlockData]) {
        if self.should_log(app_time) {
            let utc_time = self.app_start_epoch + app_time;

            let sample = format_samples_csv(block_data, current_state, app_time, utc_time);

            let mut csv_header: Option<String> = None;

            if self.last_csv_log_time.is_none() {
                let header = format_header_csv(&self.labels);
                csv_header = Some(header);
            }

            self.log(app_time, &sample, csv_header);
        }
    }
}

impl<const N: usize> Logger for CsvLogger<N> {
    fn should_log(&mut self, app_time: Duration) -> bool {
        self.csv_log_period > Duration::ZERO
            && match self.last_csv_log_time {
                None => true, // Log if there's no previous log time
                Some(last_log) => (app_time - last_log) >= self.csv_log_period,
            }
    }

    fn log(&mut self, app_time: Duration, data: &str, header: Option<String>) {
        if header.is_some() {
            writeln!(self.file, "{}", header.expect("A CSV header")).ok();
        }
        writeln!(self.file, "{}", data).ok();
        self.last_csv_log_time = Some(app_time);
    }
}

pub fn format_header_csv<const N: usize>(labels: &[&'static str; N]) -> String {
    let default_labels = ["state_id", "timestamp", "utctime"];
    [&default_labels, labels.as_slice()]
        .concat()
        .join(",")
        .to_string()
}

pub fn format_samples_csv(
    block_data: &[BlockData],
    current_state: &str,
    app_time: Duration,
    utc_time: Duration,
) -> String {
    let default_values = [
        current_state.to_string(),
        app_time.as_secs_f64().to_string(),
        utc_time.as_secs_f64().to_string(),
    ];
    let block_values: Vec<String> = block_data
        .iter()
        .map(|bd| match bd.get_type() {
            BlockDataType::Scalar => bd.stringify(),
            // For non-scalar values we want to quote the entire entry as a string
            _ => json::to_string(&Value::String(bd.stringify())),
        })
        .collect();
    [&default_values, block_values.as_slice()]
        .concat()
        .join(",")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::BlockData;

    use crate::loggers::PictorusLogger;

    #[test]
    fn test_csv_formatting() {
        // Verify we can format samples of different array types for CSV logging without errors
        let labels = ["vector", "scalar", "matrix", "bytesarray"];

        let app_time = Duration::from_micros(1_234_000);
        let utc_time = Duration::from_micros(2_234_000);
        let samples = Vec::from([
            BlockData::from_vector(&[0.0, 2.0, 4.0]),
            BlockData::from_scalar(1.0),
            BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]),
            BlockData::from_bytes(&[1, 2, 3]),
        ]);
        let current_state = "main_state";

        let csv_header = format_header_csv(&labels);
        assert_eq!(
            csv_header,
            "state_id,timestamp,utctime,vector,scalar,matrix,bytesarray".to_string()
        );

        let csv_data = format_samples_csv(&samples, current_state, app_time, utc_time);
        assert_eq!(
            csv_data,
            ("main_state,1.234,2.234,\"[[0.0,2.0,4.0]]\",1.0,\"[[5.0,6.0],[7.0,8.0]]\",\"[1,2,3]\"")
        );
    }

    #[test]
    fn test_csv_formatting_empty() {
        // Verify we can format samples of different array types for CSV logging without errors
        let labels = [];

        let app_time = Duration::from_micros(1_234_000);
        let utc_time = Duration::from_micros(2_234_000);
        let current_state = "main_state";
        let samples = Vec::new();

        let csv_header = format_header_csv(&labels);
        assert_eq!(csv_header, "state_id,timestamp,utctime".to_string());

        let csv_data = format_samples_csv(&samples, current_state, app_time, utc_time);
        assert_eq!(csv_data, ("main_state,1.234,2.234"));
    }

    #[test]
    fn test_data_logger_csv_update() {
        let labels = ["label1", "label2"];
        let logging_rate_hz: u64 = 10; // 10 hz
        let log_period = Duration::from_micros(1_000_000 / logging_rate_hz);
        let output_path = std::path::PathBuf::from("/dev/null");

        let mut dl = CsvLogger::new(labels, log_period, output_path);
        let fake_samples = Vec::from([BlockData::from_scalar(0.0), BlockData::from_scalar(1.0)]);
        let current_state = "main_state";

        // last CSV write initialized to u64::MAX
        assert_eq!(dl.last_csv_log_time, None);

        dl.add_samples(Duration::ZERO, current_state, &fake_samples);
        assert_eq!(dl.last_csv_log_time, Some(Duration::ZERO));

        // Won't log again for 0.10s (10 hz)
        dl.add_samples(Duration::from_millis(1), current_state, &fake_samples);
        assert_eq!(dl.last_csv_log_time, Some(Duration::ZERO));

        // This should update
        dl.add_samples(Duration::from_millis(123), current_state, &fake_samples);
        assert_eq!(dl.last_csv_log_time, Some(Duration::from_millis(123)));
    }
}
