use crate::block_data::BlockData;
use chrono::Local;
use core::time::Duration;
use env_logger::Builder;
use log::{info, warn, LevelFilter};
use std::io::Write;
use std::net::UdpSocket;

use super::{format_samples_json, Logger, PictorusLogger};

/// The UdpLogger is used to transmit data over the UDP protocol to the device manager.
pub struct UdpLogger<const N: usize> {
    pub file: Option<std::fs::File>,
    socket: Option<UdpSocket>,
    udp_publish_period: Duration,
    labels: [&'static str; N],
    publish_socket: String,
    last_udp_publish_time: Option<Duration>,
    _has_udp_connection: bool,
}

// Wait this long to re-establish connection to telemetry manager before giving up
const UDP_TIMEOUT: Duration = Duration::from_secs(10);

impl<const N: usize> UdpLogger<N> {
    pub fn new(labels: [&'static str; N], publish_period: Duration, publish_socket: &str) -> Self {
        let socket = if publish_socket.is_empty() || publish_period.is_zero() {
            None
        } else {
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            socket.set_nonblocking(true).unwrap();
            Some(socket)
        };

        UdpLogger {
            file: None,
            labels,
            socket,
            udp_publish_period: publish_period,
            publish_socket: publish_socket.to_string(),
            last_udp_publish_time: None,
            _has_udp_connection: true,
        }
    }
}

impl<const N: usize> PictorusLogger for UdpLogger<N> {
    fn add_samples(&mut self, app_time: Duration, current_state: &str, block_data: &[BlockData]) {
        let sample = format_samples_json(app_time, block_data, current_state, &self.labels);
        self.log(app_time, &sample, None);
    }
}

impl<const N: usize> Logger for UdpLogger<N> {
    fn should_log(&mut self, app_time: Duration) -> bool {
        self.udp_publish_period > Duration::ZERO
            && match self.last_udp_publish_time {
                None => true, // Broadcast if there's no previous broadcast time
                Some(last_broadcast) => (app_time - last_broadcast) >= self.udp_publish_period,
            }
    }

    fn log(&mut self, app_time: Duration, data: &str, _header: Option<String>) {
        if self.should_log(app_time) {
            if let Some(socket) = &mut self.socket {
                let time_since_last_udp_publish = match self.last_udp_publish_time {
                    Some(last_publish_time) => app_time - last_publish_time,
                    None => app_time,
                };
                match socket.send_to(data.as_bytes(), &self.publish_socket) {
                    Ok(_) => {
                        self.last_udp_publish_time = Some(app_time);
                        if !self._has_udp_connection {
                            info!("Regained UDP connection.");
                            self._has_udp_connection = true;
                        }
                    }
                    Err(_) => {
                        if self._has_udp_connection {
                            warn!("Lost UDP connection! Skipping telemetry transmit...");
                            self._has_udp_connection = false;
                        } else if time_since_last_udp_publish > UDP_TIMEOUT {
                            panic!(
                                "Unable to connect to telemetry manager after {:?}, aborting.",
                                UDP_TIMEOUT
                            );
                        }
                    }
                }
            }
        }
    }
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

    #[test]
    fn test_udp_data_logger_constructor() {
        let labels = ["label1", "label2"];
        let logging_rate_hz: u64 = 10;
        let log_period = Duration::from_micros(1_000_000 / logging_rate_hz);
        let publish_socket = ""; // Dont publish for this test

        // Verify we can construct a DataLogger
        let mut dl = UdpLogger::new(labels, log_period, publish_socket);

        // Verify we can pass it samples to log without errors
        let app_time = Duration::from_micros(1_234_000);
        let samples = Vec::from([BlockData::from_scalar(0.0), BlockData::from_scalar(1.0)]);
        let current_state = "main_state";
        dl.add_samples(app_time, current_state, &samples);
    }
}
