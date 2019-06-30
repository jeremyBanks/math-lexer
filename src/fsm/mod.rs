use std::fmt::Debug;
use std::marker::PhantomData;
use crate::tokens::TokenType;

pub trait StateRules: Sized {
    type ReturnType;
    fn next_state(fsm: &mut FSM<Self>, input: char) -> u8;
}

struct NumberFSMStateRule;
impl StateRules for NumberFSMStateRule {
    type ReturnType = u32;
    // accepting states are 2, 4, and 7
    fn next_state(fsm: &mut FSM<Self>, input: char) -> u8 {

        unimplemented!()    
    }
}

// TODO: possibly remove is_valid, as result is wrapped in an Option
struct FSMResult {
    is_valid: bool,
    result: Option<String>,
}

// TODO: attempt to remove PhantomData and state_rules_type
pub struct FSM<T: StateRules> {
    states: Vec<u8>,
    initial_state: u8,
    accepting_states: Vec<u8>,
    state_rules_type: PhantomData<T>,
}

impl<T: StateRules> FSM<T> {
    fn new(states_count: u8, initial_state: u8, accepting_states: Vec<u8>) -> Self {
        let mut states = Vec::with_capacity(states_count as usize);
        for i in 1..=states_count {
            states.push(i);
        }

        Self { 
            states: states,
            initial_state: initial_state,
            accepting_states: accepting_states,
            state_rules_type: PhantomData,
        }
    }

    // We'll need to figure out a way to return the actual state output
    // so that the caller can push it to the Vec<Token>
    fn run(&mut self, input: String) -> FSMResult {
        let mut state = self.initial_state;

        for c in input.chars() {
            state = T::next_state(self, c);
        }

        let is_valid = self.accepting_states.contains(&state);

        FSMResult {
            is_valid: is_valid,
            result: if is_valid { Some(input) } else { None }
        }
    }
}
