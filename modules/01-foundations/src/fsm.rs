use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Init,
    Running,
    Paused,
    Complete,
    Error,
}

#[derive(Debug, Clone)]
pub enum Event {
    Start,
    Pause,
    Resume,
    Finish,
    Fail(String),
}

pub struct Transition<S, E> {
    from: S,
    to: S,
    event: E,
    guard: Option<Box<dyn Fn(&S, &E) -> bool>>,
}

pub struct FSM<S, E> {
    current_state: S,
    transitions: Vec<Transition<S, E>>,
    transition_count: usize,
    last_transition_time: Option<Instant>,
}

impl FSM<State, Event> {
    pub fn new(initial_state: State) -> Self {
        Self {
            current_state: initial_state,
            transitions: Vec::new(),
            transition_count: 0,
            last_transition_time: None,
        }
    }

    pub fn add_transition(mut self, from: State, to: State, event: Event) -> Self {
        self.transitions.push(Transition {
            from,
            to,
            event,
            guard: None,
        });
        self
    }

    pub fn process_event(&mut self, event: Event) -> Result<State, String> {
        let start = Instant::now();

        for transition in &self.transitions {
            if transition.from == self.current_state {
                if std::mem::discriminant(&transition.event) == std::mem::discriminant(&event) {
                    self.current_state = transition.to;
                    self.transition_count += 1;
                    self.last_transition_time = Some(start);
                    return Ok(self.current_state);
                }
            }
        }

        Err(format!(
            "No valid transition from {:?} with event {:?}",
            self.current_state, event
        ))
    }

    pub fn current_state(&self) -> State {
        self.current_state
    }

    pub fn transition_count(&self) -> usize {
        self.transition_count
    }

    pub fn last_transition_duration(&self) -> Option<std::time::Duration> {
        self.last_transition_time.map(|t| t.elapsed())
    }
}

pub fn create_basic_fsm() -> FSM<State, Event> {
    FSM::new(State::Init)
        .add_transition(State::Init, State::Running, Event::Start)
        .add_transition(State::Running, State::Paused, Event::Pause)
        .add_transition(State::Paused, State::Running, Event::Resume)
        .add_transition(State::Running, State::Complete, Event::Finish)
        .add_transition(
            State::Running,
            State::Error,
            Event::Fail("error".to_string()),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fsm_transitions() {
        let mut fsm = create_basic_fsm();
        assert_eq!(fsm.current_state(), State::Init);

        assert!(fsm.process_event(Event::Start).is_ok());
        assert_eq!(fsm.current_state(), State::Running);

        assert!(fsm.process_event(Event::Pause).is_ok());
        assert_eq!(fsm.current_state(), State::Paused);

        assert!(fsm.process_event(Event::Resume).is_ok());
        assert_eq!(fsm.current_state(), State::Running);

        assert!(fsm.process_event(Event::Finish).is_ok());
        assert_eq!(fsm.current_state(), State::Complete);

        assert_eq!(fsm.transition_count(), 4);
    }

    #[test]
    fn test_invalid_transition() {
        let mut fsm = create_basic_fsm();
        assert!(fsm.process_event(Event::Pause).is_err());
    }

    #[test]
    fn test_transition_performance() {
        let mut fsm = create_basic_fsm();
        let _ = fsm.process_event(Event::Start);

        if let Some(duration) = fsm.last_transition_duration() {
            assert!(duration.as_micros() < 100);
        }
    }
}
