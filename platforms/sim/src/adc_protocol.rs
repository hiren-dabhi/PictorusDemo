use protocols::AdcProtocol;

pub struct SimAdc {}

impl SimAdc {
    pub fn new() -> Self {
        SimAdc {}
    }
}

impl Default for SimAdc {
    fn default() -> Self {
        SimAdc::new()
    }
}

impl AdcProtocol for SimAdc {
    fn read(&mut self) -> u16 {
        0
    }

    fn flush(&mut self) {
        // Do nothing
    }
}
