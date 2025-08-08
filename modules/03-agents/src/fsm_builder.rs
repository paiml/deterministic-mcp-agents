use std::marker::PhantomData;

pub struct FsmBuilder<S, E> {
    initial_state: Option<S>,
    transitions: Vec<Transition<S, E>>,
    _phantom: PhantomData<(S, E)>,
}

type GuardFn<S, E> = Box<dyn Fn(&S, &E) -> bool>;

#[allow(dead_code)]
pub struct Transition<S, E> {
    from: S,
    to: S,
    event: E,
    guard: Option<GuardFn<S, E>>,
}

impl<S: Clone, E> FsmBuilder<S, E> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            initial_state: None,
            transitions: Vec::new(),
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn initial_state(mut self, state: S) -> Self {
        self.initial_state = Some(state);
        self
    }

    #[must_use]
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

impl<S: Clone, E> Default for FsmBuilder<S, E> {
    fn default() -> Self {
        Self::new()
    }
}
