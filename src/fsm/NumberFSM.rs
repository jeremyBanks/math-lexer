use crate::fsm::FSM;

pub struct NumberFSM {
    states: Vec<u8>,
    initial_state: u8,
    accepting_states: Vec<u8>,
}

impl FSM for NumberFSM {
    fn new(states_count: u8, initial_state: u8, accepting_states: Vec<u8>) -> Self {
        let mut states = Vec::with_capacity(states_count as usize);
        for i in 1..=states_count {
            states.push(i);
        }

        Self { 
            states: states,
            initial_state: initial_state,
            accepting_states: accepting_states
        }
    }

    fn next_state(&mut self, input: char) {
        unimplemented!()
    }

    fn run(&self, input: String) {
        unimplemented!()
    }
}