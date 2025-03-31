use crate::block_data::{BlockData, BlockDataType};
use crate::utils::us_to_s;
use chrono::{Local, Utc};
use env_logger::Builder;
use log::{info, warn, LevelFilter};
use miniserde::json;
use miniserde::json::Value;
use std::fs::File;
use std::io::Write;
use std::net::UdpSocket;

pub struct DataLogger {
    pub csv_log_rate_hz: u64,
    pub output_path: std::path::PathBuf,
    pub app_start_epoch: u64,
    pub file: std::fs::File,
    socket: Option<UdpSocket>,
    udp_publish_rate_hz: u64,
    labels: Vec<String>,
    publish_socket: String,
    last_csv_log_time: Option<u64>,
    last_udp_publish_time: Option<u64>,
    _has_udp_connection: bool,
}

// Wait this long to re-establish connection to telemetry manager before giving up
const UDP_TIMEOUT_S: u64 = 10;

impl DataLogger {
    pub fn new(
        labels: Vec<String>,
        csv_log_rate_hz: f64,
        output_path: std::path::PathBuf,
        publish_socket: &str,
        udp_publish_rate_hz: u64,
    ) -> DataLogger {
        let mut file_obj = File::create("/dev/null").unwrap();
        if csv_log_rate_hz > 0.0 {
            info!("DataLogger CSV output rate: {} hz", csv_log_rate_hz);
            info!("Streaming data output to file: {}", output_path.display());
            file_obj = File::create(std::path::PathBuf::from(&output_path)).unwrap();
        } else {
            info!("Not streaming output to file, logging rate set to zero.");
        }

        let socket = if publish_socket.is_empty() || udp_publish_rate_hz == 0 {
            None
        } else {
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            socket.set_nonblocking(true).unwrap();
            Some(socket)
        };

        DataLogger {
            csv_log_rate_hz: csv_log_rate_hz as u64,
            file: file_obj,
            output_path,
            app_start_epoch: Utc::now()
                .timestamp_micros()
                .try_into()
                .expect("Could not cast app start epoch as u64"),
            labels,
            socket,
            udp_publish_rate_hz,
            publish_socket: publish_socket.to_string(),
            last_csv_log_time: None,
            last_udp_publish_time: None,
            _has_udp_connection: true,
        }
    }

    pub fn should_broadcast(&mut self, app_time_us: u64) -> bool {
        !self.publish_socket.is_empty()
            && match self.last_udp_publish_time {
                None => true, // Broadcast if there's no previous broadcast time
                Some(last_broadcast) => {
                    (app_time_us - last_broadcast) >= 1_000_000 / self.udp_publish_rate_hz
                }
            }
    }

    pub fn should_log(&mut self, app_time_us: u64) -> bool {
        self.csv_log_rate_hz > 0
            && match self.last_csv_log_time {
                None => true, // Log if there's no previous log time
                Some(last_log) => (app_time_us - last_log) >= 1_000_000 / self.csv_log_rate_hz,
            }
    }

    pub fn add_samples(&mut self, app_time_us: u64, current_state: &str, block_data: &[BlockData]) {
        if self.should_broadcast(app_time_us) {
            let socket_data = format_udp_telemetry(block_data, current_state, &self.labels);
            self.transmit_telem(socket_data, app_time_us);
        }

        if self.should_log(app_time_us) {
            if self.last_csv_log_time.is_none() {
                let csv_header = format_csv_header(&self.labels);
                writeln!(self.file, "{}", csv_header).ok();
            }

            let utc_time = self.app_start_epoch + app_time_us;
            self.last_csv_log_time = Some(app_time_us);

            let csv_data = format_csv_data(block_data, current_state, app_time_us, utc_time);
            writeln!(self.file, "{}", csv_data).ok();
        }
    }
    pub fn transmit_telem(&mut self, socket_data: String, app_time_us: u64) {
        if let Some(socket) = &mut self.socket {
            let time_since_last_udp_publish = match self.last_udp_publish_time {
                Some(last_publish_time) => app_time_us - last_publish_time,
                None => app_time_us,
            };
            match socket.send_to(socket_data.as_bytes(), &self.publish_socket) {
                Ok(_) => {
                    self.last_udp_publish_time = Some(app_time_us);
                    if !self._has_udp_connection {
                        info!("Regained UDP connection.");
                        self._has_udp_connection = true;
                    }
                }
                Err(_) => {
                    if self._has_udp_connection {
                        warn!("Lost UDP connection! Skipping telemetry transmit...");
                        self._has_udp_connection = false;
                    } else if time_since_last_udp_publish > 1_000_000 * UDP_TIMEOUT_S {
                        panic!(
                            "Unable to connect to telemetry manager after {}s, aborting.",
                            UDP_TIMEOUT_S
                        );
                    }
                }
            }
        }
    }
}

pub fn format_udp_telemetry(
    block_data: &[BlockData],
    current_state: &str,
    labels: &[String],
) -> String {
    let mut m = json::Object::new();
    for (idx, entry) in block_data.iter().enumerate() {
        let json_data = match entry.get_type() {
            BlockDataType::Scalar => entry.to_json(),
            _ => Value::String(entry.stringify()),
        };
        m.insert(labels[idx].clone(), json_data);
    }
    m.insert(
        "state_id".to_string(),
        Value::String(current_state.to_string()),
    );
    json::to_string(&m)
}

pub fn format_csv_header(labels: &Vec<String>) -> String {
    let default_labels = ["state_id".into(), "timestamp".into(), "utctime".into()];
    [&default_labels, labels.as_slice()]
        .concat()
        .join(",")
        .to_string()
}

pub fn format_csv_data(
    block_data: &[BlockData],
    current_state: &str,
    app_time_us: u64,
    utc_time: u64,
) -> String {
    let default_values = [
        current_state.to_string(),
        us_to_s::<u64, f64>(app_time_us).to_string(),
        us_to_s::<u64, f64>(utc_time).to_string(),
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

pub fn initialize_logging() {
    let mut log_level: LevelFilter = LevelFilter::Info;
    if std::env::var("LOG_LEVEL").is_ok() {
        log_level = std::env::var("LOG_LEVEL")
            .unwrap()
            .parse()
            .unwrap_or(LevelFilter::Info);
    }
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%+"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log_level)
        .init();
    log::info!("Log level: {}", log_level);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::s_to_us;

    #[test]
    fn test_data_logger_constructor() {
        let labels = Vec::from(["label1".to_string(), "label2".to_string()]);
        let logging_rate_hz: f64 = 10.0;
        let output_path = std::path::PathBuf::from("/dev/null");
        let publish_socket = ""; // Dont publish for this test

        // Verify we can construct a DataLogger
        let mut dl = DataLogger::new(labels, logging_rate_hz, output_path, publish_socket, 100);

        // Verify we can pass it samples to log without errors
        let app_time_us = 1_234_000;
        let samples = Vec::from([BlockData::from_scalar(0.0), BlockData::from_scalar(1.0)]);
        let current_state = "main_state";
        dl.add_samples(app_time_us, current_state, &samples);
    }

    #[test]
    fn test_data_logger_csv_update() {
        let labels = Vec::from(["label1".to_string(), "label2".to_string()]);
        let logging_rate_hz: f64 = 10.0;
        let output_path = std::path::PathBuf::from("/dev/null");
        let publish_socket = ""; // Dont publish for this test

        let mut dl = DataLogger::new(labels, logging_rate_hz, output_path, publish_socket, 100);
        let fake_samples = Vec::from([BlockData::from_scalar(0.0), BlockData::from_scalar(1.0)]);
        let current_state = "main_state";

        // last CSV write initialized to u64::MAX
        assert_eq!(dl.last_csv_log_time, None);

        let first_sample_time_us = s_to_us(0.0); // Should always log on first iteration
        dl.add_samples(first_sample_time_us, current_state, &fake_samples);
        assert_eq!(dl.last_csv_log_time, Some(0));

        // Won't log again for 0.10s (10 hz)
        dl.add_samples(s_to_us(0.001), current_state, &fake_samples);
        assert_eq!(dl.last_csv_log_time, Some(0));

        // This should update
        dl.add_samples(s_to_us(0.123), current_state, &fake_samples);
        assert_eq!(dl.last_csv_log_time, Some(s_to_us(0.123)));
    }

    #[test]
    fn test_format_udp_telemetry() {
        // Verify we can format samples of different array types to transmit over udp without errors
        let labels = Vec::from([
            "vector".to_string(),
            "scalar".to_string(),
            "matrix".to_string(),
            "bytesarray".to_string(),
        ]);
        let samples = Vec::from([
            BlockData::from_vector(&[0.0, 2.0, 4.0]),
            BlockData::from_scalar(1.0),
            BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]),
            BlockData::from_bytes(&[1, 2, 3]),
        ]);
        let current_state = "main_state";

        let udp_telem = format_udp_telemetry(&samples, current_state, &labels);
        assert_eq!(udp_telem, "{\"bytesarray\":\"[1,2,3]\",\"matrix\":\"[[5.0,6.0],[7.0,8.0]]\",\"scalar\":1.0,\"state_id\":\"main_state\",\"vector\":\"[[0.0,2.0,4.0]]\"}".to_string());
    }

    #[test]
    fn test_csv_formatting() {
        // Verify we can format samples of different array types for CSV logging without errors
        let labels = Vec::from([
            "vector".to_string(),
            "scalar".to_string(),
            "matrix".to_string(),
            "bytesarray".to_string(),
        ]);

        let app_time_us = 1_234_000;
        let utc_time_us = 2_234_000;
        let samples = Vec::from([
            BlockData::from_vector(&[0.0, 2.0, 4.0]),
            BlockData::from_scalar(1.0),
            BlockData::new(2, 2, &[5.0, 6.0, 7.0, 8.0]),
            BlockData::from_bytes(&[1, 2, 3]),
        ]);
        let current_state = "main_state";

        let csv_header = format_csv_header(&labels);
        assert_eq!(
            csv_header,
            "state_id,timestamp,utctime,vector,scalar,matrix,bytesarray".to_string()
        );

        let csv_data = format_csv_data(&samples, current_state, app_time_us, utc_time_us);
        assert_eq!(
            csv_data,
            ("main_state,1.234,2.234,\"[[0.0,2.0,4.0]]\",1.0,\"[[5.0,6.0],[7.0,8.0]]\",\"[1,2,3]\"")
        );
    }

    #[test]
    fn test_csv_formatting_empty() {
        // Verify we can format samples of different array types for CSV logging without errors
        let labels = Vec::new();

        let app_time_us = 1_234_000;
        let utc_time_us = 2_234_000;
        let current_state = "main_state";
        let samples = Vec::new();

        let csv_header = format_csv_header(&labels);
        assert_eq!(csv_header, "state_id,timestamp,utctime".to_string());

        let csv_data = format_csv_data(&samples, current_state, app_time_us, utc_time_us);
        assert_eq!(csv_data, ("main_state,1.234,2.234"));
    }
}
