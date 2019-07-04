use std::fmt::Debug;
use crate::fsm::StateRules;

#[derive(PartialEq, Copy, Clone)]
pub enum NumberStates {
    Initial,
    Integer,
    BeginFractionalPart,
    NumberWithFractionalPart,
    BeginExponent,
    BeginSignedExponent,
    NumberWithExponent
}

pub struct NumberStateRules;
impl StateRules for NumberStateRules {
    type ReturnType = u32;
    type States = NumberStates;

    // accepting states are 2, 4, and 7
    fn next_state(current_state: Self::States, input: char) -> Self::States {
        match current_state {
            NumberStates::Initial => NumberStates::Integer,
            NumberStates::Integer => {
                if input.is_numeric() {
                    NumberStates::Integer
                } else {
                    match input {
                        '.' => NumberStates::BeginFractionalPart,
                        'e' | 'E' => NumberStates::BeginExponent,
                        _ => panic!("Unexpected token {}", input)
                    }
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