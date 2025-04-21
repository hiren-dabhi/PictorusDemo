use corelib_traits::{Matrix, OutputBlock};
use embassy_stm32::dac::Dac;
use pictorus_core_blocks::DacBlockParams;
use protocols::DacProtocol;

pub struct DacWrapper<
    'a,
    T: embassy_stm32::dac::Instance,
    const CHANNELS: usize,
    const SAMPLES: usize,
> {
    dac: Dac<'a, T>,
}

impl<'a, T, const CHANNELS: usize, const SAMPLES: usize> DacWrapper<'a, T, CHANNELS, SAMPLES>
where
    T: embassy_stm32::dac::Instance,
{
    pub fn new(dac: Dac<'a, T>) -> Self {
        Self { dac }
    }

    pub fn configure(&mut self) {
        // Note: A lot of the configuration options disable the DAC
        self.dac
            .ch1()
            .set_trigger(embassy_stm32::dac::TriggerSel::Software);
        self.dac
            .ch2()
            .set_trigger(embassy_stm32::dac::TriggerSel::Software);

        self.dac.ch1().set_triggering(true);
        self.dac.ch2().set_triggering(true);

        // Re-enable the DAC after making all the settings adjustments
        self.dac.ch1().enable();
        self.dac.ch2().enable();
    }
}

impl<const CHANNELS: usize, const SAMPLES: usize, T> DacProtocol<CHANNELS, SAMPLES>
    for DacWrapper<'_, T, CHANNELS, SAMPLES>
where
    T: embassy_stm32::dac::Instance,
{
    fn write(&mut self, value: &[[u16; SAMPLES]; CHANNELS]) {
        self.dac
            .ch1()
            .set(embassy_stm32::dac::Value::Bit12Right(value[0][0]));
        self.dac
            .ch2()
            .set(embassy_stm32::dac::Value::Bit12Right(value[1][0]));
        self.dac.ch1().trigger();
        self.dac.ch2().trigger();
    }
}

impl<const CHANNELS: usize, const SAMPLES: usize, T> OutputBlock
    for DacWrapper<'_, T, CHANNELS, SAMPLES>
where
    T: embassy_stm32::dac::Instance,
{
    type Inputs = Matrix<SAMPLES, CHANNELS, f64>;
    type Parameters = DacBlockParams;

    fn output(
        &mut self,
        _parameters: &Self::Parameters,
        _context: &dyn corelib_traits::Context,
        inputs: corelib_traits::PassBy<'_, Self::Inputs>,
    ) {
        let mut input = [[0; SAMPLES]; CHANNELS];
        for (channel_number, channel) in inputs.data.iter().enumerate().take(CHANNELS) {
            for (sample_number, sample) in channel.iter().enumerate().take(SAMPLES) {
                input[channel_number][sample_number] = *sample as u16;
            }
        }
        self.write(&input);
    }
}
