use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Instant,
};
use utils::positive_duration;

pub struct ThreadManager {
    thread_handles: Vec<JoinHandle<()>>,
    run: Arc<AtomicBool>,
    timestep_s: f64,
}
impl ThreadManager {
    pub fn new(hertz: f64) -> Self {
        ThreadManager {
            run: Arc::new(AtomicBool::new(true)),
            thread_handles: Vec::new(),
            timestep_s: 1.0 / hertz,
        }
    }
    pub fn register<F>(&mut self, update: F)
    where
        F: Fn(),
        F: Send + 'static,
    {
        let run_thread = self.run.clone();
        let timestep_s = self.timestep_s;
        let thread_handle = thread::spawn(move || {
            while run_thread.load(Ordering::Relaxed) {
                let iter_start = Instant::now();
                update();
                let iter_dur = Instant::now() - iter_start;
                let remaining_time_s = timestep_s - iter_dur.as_secs_f64();
                std::thread::sleep(positive_duration(remaining_time_s));
            }
        });
        self.thread_handles.push(thread_handle);
    }

    pub fn stop(&mut self) {
        self.run.store(false, Ordering::Relaxed);
        // Wait for all threads to complete
        while let Some(cur_thread) = self.thread_handles.pop() {
            cur_thread.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadManager;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_thread_manager_basic_functionality() {
        let mut thread_manager = ThreadManager::new(10.0);

        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        // Test registering a thread and updating the counter.
        thread_manager.register(move || {
            let mut counter_lock = counter_clone.lock().unwrap();
            *counter_lock += 1;
        });

        // Allow the thread to run for a short period.
        std::thread::sleep(Duration::from_millis(100));

        // Test stopping the thread and ensuring it stops.
        thread_manager.stop();

        // Check if the counter was updated.
        let final_count = *counter.lock().unwrap();
        assert!(final_count > 0);
    }

    #[test]
    fn test_thread_manager_multiple_threads() {
        let mut thread_manager = ThreadManager::new(10.0);

        let counter1 = Arc::new(Mutex::new(0));
        let counter1_clone = counter1.clone();

        let counter2 = Arc::new(Mutex::new(0));
        let counter2_clone = counter2.clone();

        // Test registering multiple threads and updating the counters.
        thread_manager.register(move || {
            let mut counter_lock = counter1_clone.lock().unwrap();
            *counter_lock += 1;
        });

        thread_manager.register(move || {
            let mut counter_lock = counter2_clone.lock().unwrap();
            *counter_lock += 1;
        });

        // Allow the threads to run for a short period.
        std::thread::sleep(Duration::from_millis(100));

        // Test stopping the threads and ensuring they stop.
        thread_manager.stop();

        // Check if the counters were updated.
        let final_count1 = *counter1.lock().unwrap();
        let final_count2 = *counter2.lock().unwrap();
        assert!(final_count1 > 0);
        assert!(final_count2 > 0);
    }
}
