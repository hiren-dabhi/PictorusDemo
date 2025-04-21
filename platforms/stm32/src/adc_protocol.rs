use corelib_traits::{Context, InputBlock, PassBy};
use embassy_stm32::adc::{Adc, AnyAdcChannel};
use pictorus_core_blocks::AdcBlockParams;
use protocols::AdcProtocol;

pub struct AdcWrapper<'a, T: embassy_stm32::adc::Instance> {
    adc: Adc<'a, T>,
    channel: AnyAdcChannel<T>,
    buffer: Option<u16>,
    output: u16,
}

impl<T> InputBlock for AdcWrapper<'_, T>
where
    T: embassy_stm32::adc::Instance,
{
    type Output = u16;
    type Parameters = AdcBlockParams;

    fn input(
        &mut self,
        _parameters: &Self::Parameters,
        _context: &dyn Context,
    ) -> PassBy<'_, Self::Output> {
        self.read();
        self.output = self.buffer.expect("ADC read failed");
        self.output
    }
}

impl<'a, T> AdcWrapper<'a, T>
where
    T: embassy_stm32::adc::Instance,
{
    pub fn new(adc: Adc<'a, T>, channel: AnyAdcChannel<T>) -> Self {
        Self {
            adc,
            channel,
            buffer: None,
            output: 0,
        }
    }
}

impl<T> AdcProtocol for AdcWrapper<'_, T>
where
    T: embassy_stm32::adc::Instance,
{
    fn read(&mut self) -> u16 {
        if self.buffer.is_none() {
            self.buffer = Some(self.adc.read(&mut self.channel));
        }

        self.buffer.expect("ADC read failed")
    }

    fn flush(&mut self) {
        self.buffer = None;
    }
}
