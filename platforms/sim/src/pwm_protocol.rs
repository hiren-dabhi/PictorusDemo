use core::convert::Infallible;
use embedded_hal_02::Pwm;

pub struct SimPwmProtocol {}

impl Pwm for SimPwmProtocol {
    type Channel = ();
    type Time = f64;
    type Duty = f64;

    fn disable(&mut self, _channel: Self::Channel) {}
    fn enable(&mut self, _channel: Self::Channel) {}
    fn get_period(&self) -> Self::Time {
        0.0
    }
    fn get_duty(&self, _channel: Self::Channel) -> Self::Duty {
        0.0
    }
    fn get_max_duty(&self) -> Self::Duty {
        0.0
    }
    fn set_duty(&mut self, _channel: Self::Channel, _duty: Self::Duty) {}
    fn set_period<P: Into<Self::Time>>(&mut self, _period: P) {}
}

pub fn create_pwm_protocol(_pin: f64) -> Result<SimPwmProtocol, Infallible> {
    Ok(SimPwmProtocol {})
}
