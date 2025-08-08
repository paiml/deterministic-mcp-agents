use std::marker::PhantomData;
use std::collections::HashMap;

pub struct FsmBuilder<S, E> {
    initial_state: Option<S>,
    transitions: Vec<Transition<S, E>>,
    _phantom: PhantomData<(S, E)>,
}

pub struct Transition<S, E> {
    from: S,
    to: S,
    event: E,
    guard: Option<Box<dyn Fn(&S, &E) -> bool>>,
}

impl<S: Clone, E> FsmBuilder<S, E> {
    pub fn new() -> Self {
        Self {
            initial_state: None,
            transitions: Vec::new(),
            _phantom: PhantomData,
        }
    }
    
    pub fn initial_state(mut self, state: S) -> Self {
        self.initial_state = Some(state);
        self
    }
    
    pub fn transition(mut self, from: S, to: S, event: E) -> Self {
        self.transitions.push(Transition {
            from,
            to,
            event,
            guard: None,
        });
        self
    }
}