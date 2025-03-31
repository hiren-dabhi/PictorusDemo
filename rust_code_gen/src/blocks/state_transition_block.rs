// StateTransitionBlock are non-op blocks
pub struct StateTransitionBlock {
    pub name: &'static str,
}

impl StateTransitionBlock {
    pub fn new(name: &'static str) -> StateTransitionBlock {
        StateTransitionBlock { name }
    }
    pub fn run(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transition_block() {
        let mut _block = StateTransitionBlock::new("StateTransition");
    }
}
