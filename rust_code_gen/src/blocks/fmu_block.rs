use std::collections::HashMap;

use fmu_runner::{fmi2Type, Fmu, FmuInstance, FmuLibrary};
use utils::BlockData;

pub struct FmuBlock {
    name: &'static str,
    fmu_cs: FmuInstance<FmuLibrary>,
    pub data: Vec<BlockData>,
    input_signals: Vec<String>,
    output_signals: Vec<String>,
}

impl FmuBlock {
    pub fn new(
        name: &'static str,
        fmu_path: &str,
        fmu_params: &HashMap<&'static str, f64>,
        input_signals: Vec<String>,
        output_signals: Vec<String>,
    ) -> Self {
        let fmu = Fmu::unpack(fmu_path)
            .expect("Failed to unpack FMU")
            .load(fmi2Type::fmi2CoSimulation)
            .expect("Failed to load FMU for co-simulation");
        let fmu_cs = FmuInstance::instantiate(fmu, false).expect("Failed to instantiate FMU");
        let signals = fmu_cs.lib.variables();
        fmu_cs
            .setup_experiment(0.0, None, None)
            .expect("Failed to setup FMU experiment");
        fmu_cs
            .enter_initialization_mode()
            .expect("Failed to enter FMU initialization mode");
        let param_values = fmu_params
            .iter()
            .map(|(k, v)| (&signals[*k], *v))
            .collect::<HashMap<_, _>>();
        fmu_cs
            .set_reals(&param_values)
            .expect("Failed to set FMU parameters");
        fmu_cs
            .exit_initialization_mode()
            .expect("Failed to exit FMU initialization mode");

        FmuBlock {
            name,
            fmu_cs,
            data: vec![BlockData::from_scalar(0.0); output_signals.len()],
            input_signals,
            output_signals,
        }
    }

    pub fn run(&mut self, app_time_s: f64, time_step_s: f64, inputs: &[&BlockData]) {
        log::debug!("{}: Running", self.name);
        let signals = self.fmu_cs.lib.variables();
        if !self.input_signals.is_empty() {
            let input_signals = self
                .input_signals
                .iter()
                .enumerate()
                .map(|(i, signal)| (&signals[signal], inputs[i].scalar()))
                .collect::<HashMap<_, _>>();
            self.fmu_cs
                .set_reals(&input_signals)
                .expect("Failed to set FMU inputs");
        }

        // Not sure if this is right... We may just want to pass in the desired time step here instead
        // of the "measured" time step, otherwise it is 0 on the first tick which is invalid. Doing it this
        // way causes the first 2 ticks to emit the initial values which doesn't sem right
        if time_step_s > 0.0 {
            let step_start_time = app_time_s - time_step_s;
            self.fmu_cs
                .do_step(step_start_time, time_step_s, false)
                .expect("Failed to perform FMU step");
        }

        if self.output_signals.is_empty() {
            return;
        }

        let output_signals = self
            .output_signals
            .iter()
            .map(|signal| &signals[signal])
            .collect::<Vec<_>>();
        let outputs = self
            .fmu_cs
            .get_reals(&output_signals)
            .expect("Failed to get FMU outputs");
        for (i, output) in output_signals.iter().enumerate() {
            let output_val = outputs.get(output).expect("Failed to read FMU output");
            self.data[i] = BlockData::from_scalar(*output_val);
        }
        log::debug!("{}: Outputs: {:?}", self.name, outputs);
    }
}
