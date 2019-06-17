use std::fmt::Debug;

pub trait FSM {
    fn new(states_count: u8, initial_state: u8, accepting_states: Vec<i8>) -> Self;
    fn next_state(&mut self, input: char);
    fn run(&self, input: String);
}
