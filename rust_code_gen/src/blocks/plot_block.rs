pub struct PlotBlock {
    pub name: &'static str,
    pub value: f64, // Hack for data logger for now...
}

impl PlotBlock {
    pub fn new(name: &'static str) -> PlotBlock {
        PlotBlock { name, value: 0.0 }
    }
}
