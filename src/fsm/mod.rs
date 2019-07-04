use std::marker::PhantomData;

pub trait StateRules: Sized {
    type ReturnType;
    type States: PartialEq + Copy + Clone;

    fn next_state(current_state: Self::States, input: char) -> Self::States;
}

// TODO: possibly remove is_valid, as result is wrapped in an Option
pub struct FSMResult {
    pub result: Option<String>,
}

// TODO: attempt to remove PhantomData and state_rules_type
pub struct FSM<T: StateRules> {
    initial_state: T::States,
    accepting_states: Vec<T::States>,
    state_rules_type: PhantomData<T>
}

impl<T: StateRules> FSM<T> {
    pub fn new(initial_state: T::States, accepting_states: Vec<T::States>) -> Self {
        Self { 
            initial_state: initial_state,
            accepting_states: accepting_states,
            state_rules_type: PhantomData,
        }
    }

    // We'll need to figure out a way to return the actual state output
    // so that the caller can push it to the Vec<Token>
    pub fn run(self, input: String) -> Option<String> {
        let mut state = self.initial_state;
        for c in input.chars() {
            state = T::next_state(state, c);
        }

        let is_valid = self.accepting_states.contains(&state);

        if is_valid {
            Some(input)
        } else {
            None
        }
    }
}
