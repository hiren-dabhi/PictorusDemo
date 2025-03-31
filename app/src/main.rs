extern crate alloc;

use alloc::vec;
use core::time::Duration;
use corelib_traits::{GeneratorBlock, ProcessBlock};
use linux_protocols::{
    create_clock_protocol, create_delay_protocol, StandardClock, StdDelayProtocol,
};
use pictorus_core_blocks::{ConstantBlock, GainBlock};
use rust_code_gen::block_data::{BlockData, ToPass};
use rust_code_gen::data_logger::{initialize_logging, DataLogger};
use rust_code_gen::timing::{RunTime, Timing};
use rust_code_gen::utils::{
    custom_panic_handler, get_diagram_params, get_pictorus_vars, load_param, us_to_s,
    DiagramParams, PictorusError, PictorusVars,
};

pub fn compile_info() -> &'static str {
    return "demoapp_67ea8ab66a4093c50166016a version : compiled 03/31/2025 - 12:37:05";
}

#[derive(Debug, Clone)]
pub enum State {
    Maina146eState,
}

pub struct Maina146eState {
    last_time_s: f64,
    constant1_a146a_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant1_a146a: ConstantBlock<f64>,
    gain1_a146b_param: <GainBlock<f64, f64> as ProcessBlock>::Parameters,
    gain1_a146b: GainBlock<f64, f64>,
}

impl Maina146eState {
    pub fn new(_context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        let constant1_a146a_value =
            load_param::<f64>(&"constant1_a146a", &"value", 1.000000, &diagram_params);

        let constant1_a146a_ic = BlockData::from_element(1, 1, constant1_a146a_value);

        // Constant1
        let constant1_a146a_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant1_a146a_ic.to_pass());
        let constant1_a146a = ConstantBlock::default();

        let gain1_a146b_gain =
            load_param::<f64>(&"gain1_a146b", &"gain", 2.000000, &diagram_params);

        // Gain1
        let gain1_a146b_param =
            <GainBlock<f64, f64> as ProcessBlock>::Parameters::new(gain1_a146b_gain);
        let gain1_a146b = GainBlock::default();

        Maina146eState {
            last_time_s: -1.0,
            constant1_a146a_param,
            constant1_a146a,
            gain1_a146b_param,
            gain1_a146b,
        }
    }

    pub fn run(&mut self, context: &mut Context) {
        let app_time_s = context.app_time_s();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Maina146eState iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // Constant1
        self.constant1_a146a
            .generate(&self.constant1_a146a_param, context);
        // Gain1
        self.gain1_a146b.process(
            &self.gain1_a146b_param,
            context,
            self.constant1_a146a.data.to_pass(),
        );

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![self.gain1_a146b.data.clone()];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct StateManager {
    pub current_state: State,
    pub maina146e_state: Maina146eState,
}

impl StateManager {
    pub fn run(&mut self, context: &mut Context) {
        match self.current_state {
            State::Maina146eState => self.maina146e_state.run(context),
        };
    }
    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        [self.maina146e_state.get_output()].concat()
    }
}

pub struct IoManager {}

impl IoManager {
    pub fn new(
        diagram_params: &DiagramParams,
    ) -> Result<(Self, Timing<StandardClock, StdDelayProtocol>), PictorusError> {
        let app_run_time_s = load_param::<f64>(
            &String::from("app"),
            &String::from("run_time_s"),
            10.0,
            &diagram_params,
        );
        let app_hertz = load_param::<f64>(
            &String::from("app"),
            &String::from("hertz"),
            10.0,
            &diagram_params,
        );
        let use_realtime = true;
        let app_clock = create_clock_protocol();
        let app_delay = create_delay_protocol();
        let timing = Timing::new(
            RunTime::from_f64_seconds(app_run_time_s),
            app_hertz,
            use_realtime,
            app_clock,
            app_delay,
        );

        let io_manager = IoManager {};
        Ok((io_manager, timing))
    }

    pub fn flush_inputs(&mut self) {}
}

pub struct AppInterface {
    state_manager: StateManager,
    data_logger: DataLogger,
    context: Context,
}

impl AppInterface {
    pub fn new(context: Context, pictorus_vars: &PictorusVars) -> Self {
        let data_logger_path =
            std::path::PathBuf::from(&pictorus_vars.run_path).join("diagram_output.csv");
        let labels: Vec<String> = vec![String::from("gain1_a146b.0")];
        let data_logger = DataLogger::new(
            labels,
            pictorus_vars.data_log_rate_hz,
            data_logger_path,
            &pictorus_vars.publish_socket,
            100,
        );

        let state_manager = StateManager {
            current_state: State::Maina146eState,
            maina146e_state: Maina146eState::new(&context),
        };

        Self {
            state_manager,
            data_logger,
            context,
        }
    }

    pub fn update(&mut self) {
        self.state_manager.run(&mut self.context);

        let logged_state_id = match self.state_manager.current_state {
            State::Maina146eState => "maina146e_state",
        };

        // TODO: Can simplify all this to data_logger.maybe_update(&context, &state_manager);
        if self.data_logger.should_log(self.context.app_time_us)
            || self.data_logger.should_broadcast(self.context.app_time_us)
        {
            self.data_logger.add_samples(
                self.context.app_time_us,
                logged_state_id,
                &self.state_manager.get_output(),
            );
        }

        self.context.io_manager.flush_inputs();
    }
}

pub struct Context {
    io_manager: IoManager,
    app_time_us: u64,
    app_timestep_us: u64,
}

impl Context {
    pub fn app_time_s(&mut self) -> f64 {
        us_to_s(self.app_time_us)
    }
}

impl corelib_traits::Context for Context {
    fn timestep(&self) -> Duration {
        Duration::from_micros(self.app_timestep_us)
    }

    fn time(&self) -> Duration {
        Duration::from_micros(self.app_time_us)
    }
}

fn main() -> std::process::ExitCode {
    use std::sync::Arc;

    let pictorus_vars = get_pictorus_vars();
    let diagram_params = get_diagram_params(&pictorus_vars);

    let og_panic = std::panic::take_hook();
    let run_path_clone = pictorus_vars.run_path.clone();
    std::panic::set_hook(Box::new(move |panic_info| {
        custom_panic_handler(panic_info, &run_path_clone);
        og_panic(panic_info)
    }));

    initialize_logging();
    log::info!("{}", compile_info());

    let (io_manager, mut timing) =
        IoManager::new(&diagram_params).expect("Unable to initialize IoManager!");
    let context = Context {
        io_manager,
        app_time_us: 0,
        app_timestep_us: 100000,
    };

    let mut app_interface = AppInterface::new(context, &pictorus_vars);

    let interrupt = Arc::new(std::sync::atomic::AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&interrupt)).unwrap();
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&interrupt)).unwrap();

    while timing.should_run(app_interface.context.app_time_us)
        && !interrupt.load(std::sync::atomic::Ordering::Relaxed)
    {
        app_interface.update();
        app_interface.context.app_time_us = timing.update(app_interface.context.app_time_us);
    }

    log::info!("Exiting demoapp_67ea8ab66a4093c50166016a.");
    std::process::ExitCode::SUCCESS
}
