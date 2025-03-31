use corelib_traits::Context;
use derive_new::new;
use std::time::Duration;

#[derive(Debug, Copy, Clone, new)]
pub struct StubContext {
    pub time: Duration,
    pub timestep: Duration,
}

impl Default for StubContext {
    fn default() -> Self {
        Self::new(Duration::from_secs(0), Duration::from_millis(100))
    }
}

impl Context for StubContext {
    fn time(&self) -> Duration {
        self.time
    }

    fn timestep(&self) -> Duration {
        self.timestep
    }
}

#[derive(Debug, new, Clone, Copy, Default)]
pub struct StubRuntime {
    pub context: StubContext,
}

impl StubRuntime {
    pub fn tick(&mut self) {
        self.context.time += self.context.timestep;
    }

    pub fn context(&self) -> StubContext {
        self.context
    }

    pub fn set_time(&mut self, time: Duration) {
        self.context.time = time;
    }
}
