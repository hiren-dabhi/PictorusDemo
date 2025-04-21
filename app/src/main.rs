extern crate alloc;

use alloc::vec;
use core::time::Duration;
use corelib_traits::{Context as CorelibContext, GeneratorBlock, HasIc, ProcessBlock};
use linux_protocols::{
    create_clock_protocol, create_delay_protocol, StandardClock, StdDelayProtocol,
};
use pictorus_core_blocks::{ConstantBlock, DataWriteBlock, DelayBlock, SumBlock};
use rust_code_gen::block_data::{BlockData, ToPass};
use rust_code_gen::loggers::linux_logger::LinuxLogger;
use rust_code_gen::loggers::udp_logger::initialize_logging;
use rust_code_gen::loggers::PictorusLogger;
use rust_code_gen::utils::timing::{RunTime, Timing};
use rust_code_gen::utils::{
    custom_panic_handler, get_diagram_params, get_pictorus_vars, load_ic, load_param,
    DiagramParams, PictorusError, PictorusVars,
};

pub fn compile_info() -> &'static str {
    return "counter_68059cc7b7d81834df67e279 version : compiled 04/21/2025 - 05:49:08";
}

#[derive(Debug, Clone)]
pub enum State {
    Main7e27aState,
}

pub struct Main7e27aState {
    last_time_s: f64,
    constant1_0e831_param: <ConstantBlock<f64> as GeneratorBlock>::Parameters,
    constant1_0e831: ConstantBlock<f64>,
    delay1_0e834_param: <DelayBlock<f64, 1> as ProcessBlock>::Parameters,
    delay1_0e834: DelayBlock<f64, 1>,
    sum1_0e832_param: <SumBlock<(f64, f64)> as ProcessBlock>::Parameters,
    sum1_0e832: SumBlock<(f64, f64)>,
    data_write1_0e837_param: <DataWriteBlock<f64> as ProcessBlock>::Parameters,
    data_write1_0e837: DataWriteBlock<f64>,
}

impl Main7e27aState {
    pub fn new(_context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        let constant1_0e831_value =
            load_param::<f64>(&"constant1_0e831", &"value", 1.000000, &diagram_params);

        let constant1_0e831_ic = BlockData::from_element(1, 1, constant1_0e831_value);

        // Constant1
        let constant1_0e831_param =
            <ConstantBlock<f64> as GeneratorBlock>::Parameters::new(constant1_0e831_ic.to_pass());
        let constant1_0e831 = ConstantBlock::default();

        let delay1_0e834_ic = load_ic(
            &String::from("delay1_0e834"),
            &String::from("initial_condition"),
            BlockData::new(1, 1, &[0.0]),
            &diagram_params,
        );

        // Delay1
        let delay1_0e834_param =
            <DelayBlock<f64, 1> as ProcessBlock>::Parameters::new(delay1_0e834_ic.to_pass());
        let delay1_0e834 = DelayBlock::new(&delay1_0e834_param);

        let sum1_0e832_gains = load_param::<BlockData>(
            &"sum1_0e832",
            &"gains",
            BlockData::new(1, 2, &[1.0, 1.0]),
            &diagram_params,
        );

        // Sum1
        let sum1_0e832_param =
            <SumBlock<(f64, f64)> as ProcessBlock>::Parameters::new(sum1_0e832_gains.to_pass());
        let sum1_0e832 = SumBlock::default();

        // DataWrite1
        let data_write1_0e837_param = <DataWriteBlock<f64> as ProcessBlock>::Parameters::new();
        let data_write1_0e837 = DataWriteBlock::default();

        Main7e27aState {
            last_time_s: -1.0,
            constant1_0e831_param,
            constant1_0e831,
            delay1_0e834_param,
            delay1_0e834,
            sum1_0e832_param,
            sum1_0e832,
            data_write1_0e837_param,
            data_write1_0e837,
        }
    }

    pub fn run(&mut self, context: &mut Context) {
        let app_time_s = context.app_time_s();
        let runtime_ctx = context.get_runtime_context();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Main7e27aState iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // Constant1
        self.constant1_0e831
            .generate(&self.constant1_0e831_param, &runtime_ctx);
        // Delay1
        self.delay1_0e834.process(
            &self.delay1_0e834_param,
            &runtime_ctx,
            self.sum1_0e832.data.to_pass(),
        );
        // Sum1
        self.sum1_0e832.process(
            &self.sum1_0e832_param,
            &runtime_ctx,
            (
                self.constant1_0e831.data.to_pass(),
                self.delay1_0e834.data.to_pass(),
            ),
        );
        // Update DataStore with value from data_write1_0e837
        // DataWrite1
        self.data_write1_0e837.process(
            &self.data_write1_0e837_param,
            &runtime_ctx,
            self.sum1_0e832.data.to_pass(),
        );
        context.gds.counter_7e27b_512b0 = self.data_write1_0e837.data.scalar();

        self.last_time_s = app_time_s;
    }

    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        let output = vec![];
        output
    }

    pub fn post_run(&mut self) {}
}

pub struct StateManager {
    pub current_state: State,
    pub main7e27a_state: Main7e27aState,
}

impl StateManager {
    pub fn run(&mut self, context: &mut Context) {
        match self.current_state {
            State::Main7e27aState => self.main7e27a_state.run(context),
        };
    }
    pub fn get_output(&mut self) -> vec::Vec<BlockData> {
        [self.main7e27a_state.get_output()].concat()
    }
}

pub struct GlobalDataStore {
    pub counter_7e27b_512b0: f64,
}

impl GlobalDataStore {
    // Constructor
    pub fn new() -> GlobalDataStore {
        GlobalDataStore {
            counter_7e27b_512b0: 0.0,
        }
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
    data_logger: LinuxLogger<0>,
    context: Context,
}

impl AppInterface {
    pub fn new(context: Context, pictorus_vars: &PictorusVars) -> Self {
        let data_logger_path =
            std::path::PathBuf::from(&pictorus_vars.run_path).join("diagram_output.csv");
        let data_log_period = if pictorus_vars.data_log_rate_hz > 0.0 {
            Duration::from_secs_f64(1.0 / pictorus_vars.data_log_rate_hz)
        } else {
            Duration::ZERO
        };
        let labels: [&'static str; 0] = [];
        let data_logger = LinuxLogger::<0>::new(
            labels,
            Duration::from_micros(10000),
            &pictorus_vars.publish_socket,
            data_log_period,
            data_logger_path,
        );

        let state_manager = StateManager {
            current_state: State::Main7e27aState,
            main7e27a_state: Main7e27aState::new(&context),
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
            State::Main7e27aState => "main7e27a_state",
        };

        self.data_logger.add_samples(
            self.context.time(),
            logged_state_id,
            &self.state_manager.get_output(),
        );

        self.context.io_manager.flush_inputs();
    }
}

pub struct Context {
    gds: GlobalDataStore,
    io_manager: IoManager,
    runtime_context: rust_code_gen::utils::RuntimeContext,
}

impl Context {
    pub fn app_time_s(&self) -> f64 {
        self.runtime_context.app_time_s()
    }

    pub fn app_time_us(&self) -> u64 {
        self.runtime_context.app_time_us()
    }

    pub fn time(&self) -> Duration {
        self.runtime_context.time()
    }

    pub fn get_runtime_context(&self) -> rust_code_gen::utils::RuntimeContext {
        self.runtime_context
    }

    pub fn update_app_time(&mut self, app_time_us: u64) {
        self.runtime_context.update_app_time(app_time_us);
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

    let gds = GlobalDataStore::new();
    let (io_manager, mut timing) =
        IoManager::new(&diagram_params).expect("Unable to initialize IoManager!");
    let context = Context {
        gds,
        io_manager,
        runtime_context: rust_code_gen::utils::RuntimeContext::new(100000),
    };

    let mut app_interface = AppInterface::new(context, &pictorus_vars);

    let interrupt = Arc::new(std::sync::atomic::AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&interrupt)).unwrap();
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&interrupt)).unwrap();

    while timing.should_run(app_interface.context.app_time_us())
        && !interrupt.load(std::sync::atomic::Ordering::Relaxed)
    {
        app_interface.update();
        app_interface
            .context
            .update_app_time(timing.update(app_interface.context.app_time_us()));
    }

    log::info!("Exiting counter_68059cc7b7d81834df67e279.");
    std::process::ExitCode::SUCCESS
}
