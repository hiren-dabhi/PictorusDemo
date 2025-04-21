use super::{format_samples_json, Logger, PictorusLogger};
use alloc::string::String;
use core::time::Duration;
use rtt_target::rprintln;
use utils::BlockData;

const LOG_HEAP_MIN_PERIOD: Duration = Duration::from_secs(1);

/// RttLogger transmits data over the RTT protocol. Has an additional
/// method to log heap changes.
pub struct RttLogger<const N: usize> {
    labels: [&'static str; N],
    publish_period: Duration,
    last_broadcast_time: Option<Duration>,
    previous_heap_used: usize,
    last_heap_log_time: Duration,
}

impl<const N: usize> RttLogger<N> {
    pub fn new(labels: [&'static str; N], publish_period: Duration) -> RttLogger<N> {
        RttLogger {
            labels,
            publish_period,
            last_broadcast_time: None,
            previous_heap_used: 0,
            last_heap_log_time: Duration::ZERO,
        }
    }

    /// Logs heap information if the heap size has changed since the last measurement.
    /// The heap doesn't live in the time series database, so it is logged separately
    /// as an [INFO] message.
    ///
    /// Currently logs only when a change in heap usage is detected.
    pub fn log_heap(&mut self, app_time: Duration, free: usize, used: usize) {
        // Only log heap usage if the heap usage has changed and at most once per second
        if self.previous_heap_used != used
            && app_time - self.last_heap_log_time >= LOG_HEAP_MIN_PERIOD
        {
            let free_f32 = free as f32 / 1000.0;
            let used_f32 = used as f32 / 1000.0;
            let percent_used = (used_f32 / (used_f32 + free_f32)) * 100.0;
            log::info!(
                "Heap Used: {:.3}kB, Heap Free: {:.3}kB, Heap Usage: {:.3}%",
                used_f32,
                free_f32,
                percent_used
            );
            self.previous_heap_used = used;
            self.last_heap_log_time = app_time;
        }
    }
}

impl<const N: usize> PictorusLogger for RttLogger<N> {
    fn add_samples(&mut self, app_time: Duration, current_state: &str, block_data: &[BlockData]) {
        let sample = format_samples_json(app_time, block_data, current_state, &self.labels);
        self.log(app_time, &sample, None);
    }
}

impl<const N: usize> Logger for RttLogger<N> {
    fn should_log(&mut self, app_time: Duration) -> bool {
        self.publish_period > Duration::ZERO
            && match self.last_broadcast_time {
                None => true, // Broadcast if there's no previous broadcast time
                Some(last_broadcast) => (app_time - last_broadcast) >= self.publish_period,
            }
    }

    fn log(&mut self, app_time: Duration, data: &str, _header: Option<String>) {
        if self.should_log(app_time) {
            rprintln!("{}", data);
            self.last_broadcast_time = Some(app_time);
        }
    }
}
