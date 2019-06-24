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
    fn next_state(fsm: &mut FSM<Self>, input: char) -> u8 {
        unimplemented!()    
    }
}

pub struct FSM<T: StateRules> {
    states: Vec<u8>,
    initial_state: u8,
    accepting_states: Vec<u8>,
    state_rules_type: PhantomData<T>
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
            state_rules_type: PhantomData
        }
    }

    fn run(&mut self, input: String) -> Option<T> {
        let mut state = self.initial_state;
        let mut chs = input.chars();


        T::next_state(self, chs.next().unwrap());

        unimplemented!()
    }
}
