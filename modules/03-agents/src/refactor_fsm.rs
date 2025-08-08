#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefactorState {
    Init,
    Parsing,
    Analyzing,
    Planning,
    Refactoring,
    Testing,
    Validating,
    Complete,
    Error,
}

pub struct RefactorFsm {
    state: RefactorState,
}

impl RefactorFsm {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: RefactorState::Init,
        }
    }

    #[must_use]
    pub fn state(&self) -> RefactorState {
        self.state
    }
}

impl Default for RefactorFsm {
    fn default() -> Self {
        Self::new()
    }
}
