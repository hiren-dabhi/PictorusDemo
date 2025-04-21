use core::time::Duration;
use std::path::PathBuf;

use super::{csv_logger::CsvLogger, udp_logger::UdpLogger, PictorusLogger};

/// LinuxLogger for Linux systems that logs data via UDP telemetry using
/// the device manager as well as a CSV file.
pub struct LinuxLogger<const N: usize> {
    udp_logger: UdpLogger<N>,
    csv_logger: CsvLogger<N>,
}

impl<const N: usize> LinuxLogger<N> {
    pub fn new(
        labels: [&'static str; N],
        udp_log_period: Duration,
        udp_socket: &str,
        csv_log_period: Duration,
        csv_output_path: PathBuf,
    ) -> Self {
        LinuxLogger {
            udp_logger: UdpLogger::new(labels, udp_log_period, udp_socket),
            csv_logger: CsvLogger::new(labels, csv_log_period, csv_output_path),
        }
    }
}

impl<const N: usize> PictorusLogger for LinuxLogger<N> {
    fn add_samples(
        &mut self,
        app_time: Duration,
        current_state: &str,
        block_data: &[utils::BlockData],
    ) {
        self.udp_logger
            .add_samples(app_time, current_state, block_data);
        self.csv_logger
            .add_samples(app_time, current_state, block_data);
    }
}
