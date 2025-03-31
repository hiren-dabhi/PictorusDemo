use embedded_hal_02::Pwm;
use log::debug;

use crate::block_data::{BlockData, BlockDataType};

// Control a PWM pin
pub struct PwmBlock {
    name: &'static str,
}

impl PwmBlock {
    pub fn new(name: &'static str) -> PwmBlock {
        PwmBlock { name }
    }
    pub fn run(
        &mut self,
        frequency: &BlockData,
        duty_cycle: &BlockData,
        proto: &mut impl Pwm<Time = f64, Duty = f64, Channel = ()>,
    ) {
        let frequency = match frequency.get_type() {
            BlockDataType::Scalar => frequency.scalar(),
            _ => panic!("Not Implemented"),
        };

        let duty_cycle = match duty_cycle.get_type() {
            BlockDataType::Scalar => duty_cycle.scalar(),
            _ => panic!("Not Implemented"),
        };

        debug!(
            "{} Controlling pwm to freq: {}, dc: {}",
            self.name, frequency, duty_cycle
        );

        let period = 1.0 / frequency;
        if proto.get_period() != period {
            proto.set_period(period);
        }

        if proto.get_duty(()) != duty_cycle {
            proto.set_duty((), duty_cycle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mockall has problems with the Pwm trait, so rolling something custom
    struct MockPwmProtocol {
        pub duty_cycle: f64,
        pub period: f64,
    }

    impl MockPwmProtocol {
        fn new() -> MockPwmProtocol {
            MockPwmProtocol {
                duty_cycle: 0.0,
                period: 0.0,
            }
        }
    }

    impl Pwm for MockPwmProtocol {
        type Channel = ();
        type Duty = f64;
        type Time = f64;

        fn disable(&mut self, _: ()) {}
        fn enable(&mut self, _: ()) {}
        fn get_duty(&self, _: ()) -> f64 {
            0.0
        }
        fn get_max_duty(&self) -> f64 {
            0.0
        }
        fn set_duty(&mut self, _: (), duty: f64) {
            self.duty_cycle = duty;
        }
        fn get_period(&self) -> f64 {
            0.0
        }
        fn set_period<P>(&mut self, period: P)
        where
            P: Into<f64>,
        {
            self.period = period.into();
        }
    }

    #[test]
    fn test_pwm_block_sets_frequency_and_duty_cycle() {
        let mut proto = MockPwmProtocol::new();
        let mut block = PwmBlock::new("Pwm1");
        block.run(
            &BlockData::from_scalar(50.0),
            &BlockData::from_scalar(25.0),
            &mut proto,
        );

        assert_eq!(proto.period, 0.02);
        assert_eq!(proto.duty_cycle, 25.0);
    }
}
