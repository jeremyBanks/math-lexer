use std::fmt::Debug;
use std::marker::PhantomData;
use crate::tokens::TokenType;

#[derive(PartialEq, Copy, Clone)]
enum NumberStates {
    Initial,
    Integer,
    BeginFractionalPart,
    NumberWithFractionalPart,
    BeginExponent,
    BeginSignedExponent,
    NumberWithExponent
}

pub trait StateRules: Sized {
    type ReturnType;
    type States: PartialEq + Copy + Clone;

    fn next_state(current_state: Self::States, input: char) -> Self::States;
}

struct NumberFSMStateRule;
impl StateRules for NumberFSMStateRule {
    type ReturnType = u32;
    type States = NumberStates;

    // accepting states are 2, 4, and 7
    fn next_state(current_state: Self::States, input: char) -> Self::States {
        match current_state {
            NumberStates::Initial => NumberStates::Integer,
            NumberStates::Integer => {
                match input {
                    '.' => NumberStates::BeginFractionalPart,
                    'e' | 'E' => NumberStates::BeginExponent,
                    _ => panic!("Unexpected token {}", input)
                }
            },
            NumberStates::BeginFractionalPart => {
                if input.is_numeric() {
                    NumberStates::NumberWithFractionalPart
                } else {
                    panic!("Unexpected token {}", input)
                }
            },
            NumberStates::NumberWithFractionalPart => {
                if input.is_numeric() {
                    NumberStates::NumberWithFractionalPart
                } else if input == 'e' || input == 'E' {
                    NumberStates::BeginExponent
                } else {
                    panic!("Unexpected token {}", input)
                }
            },
            NumberStates::BeginExponent => {
                if input.is_numeric() {
                    NumberStates::NumberWithExponent
                } else if input == '+' || input == '-' {
                    NumberStates::BeginSignedExponent
                } else {
                    panic!("Unexpected token {}", input)
                }
            },
            NumberStates::BeginSignedExponent => {
                if input.is_numeric() {
                    NumberStates::NumberWithExponent
                } else {
                    panic!("Unexpected token {}", input)
                }
            },
            NumberStates::NumberWithExponent => {
                if input.is_numeric() {
                    NumberStates::NumberWithExponent
                } else {
                    panic!("Unexpected token {}", input)
                }
            }
            _ => panic!("An error has occurred.")
        }
    }
}

// TODO: possibly remove is_valid, as result is wrapped in an Option
struct FSMResult {
    is_valid: bool,
    result: Option<String>,
}

// TODO: attempt to remove PhantomData and state_rules_type
pub struct FSM<T: StateRules> {
    initial_state: T::States,
    accepting_states: Vec<T::States>,
    state_rules_type: PhantomData<T>,
    cur_state: T::States,
}

impl<T: StateRules> FSM<T> {
    fn new(initial_state: T::States, accepting_states: Vec<T::States>) -> Self {
        Self { 
            initial_state: initial_state,
            accepting_states: accepting_states,
            state_rules_type: PhantomData,
            cur_state: initial_state,
        }
    }

    // We'll need to figure out a way to return the actual state output
    // so that the caller can push it to the Vec<Token>
    fn run(&mut self, input: String) -> FSMResult {
        for c in input.chars() {
            self.cur_state = T::next_state(self.cur_state, c);
        }

        let is_valid = self.accepting_states.contains(&self.cur_state);

        FSMResult {
            is_valid: is_valid,
            result: if is_valid { Some(input) } else { None }
        }
    }
}
