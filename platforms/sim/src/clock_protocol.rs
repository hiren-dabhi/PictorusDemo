use embedded_time::{clock::Error, fraction::Fraction, Clock, Instant};

pub struct SimClockProtocol {}

impl Clock for SimClockProtocol {
    type T = u64;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000_000);

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        Ok(Instant::new(0))
    }
}

pub fn create_clock_protocol() -> SimClockProtocol {
    SimClockProtocol {}
}
