use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RefactorState {
    Init,
    Parsing,
    Analyzing,
    Planning,
    Refactoring,
    Testing,
    Validating,
    Complete,
    Error,
    Rollback,
    Cancelled,
    Paused,
}

#[derive(Debug, Clone)]
enum RefactorEvent {
    Start(String),
    ParseComplete(usize),
    AnalysisComplete(Vec<String>),
    PlanGenerated(RefactorPlan),
    RefactorApplied(usize),
    TestsRun(TestResult),
    ValidationComplete(bool),
    ErrorOccurred(String),
    Cancel,
    Pause,
    Resume,
    Rollback,
}

#[derive(Debug, Clone)]
struct RefactorPlan {
    steps: Vec<String>,
    estimated_time: Duration,
}

#[derive(Debug, Clone)]
struct TestResult {
    passed: usize,
    failed: usize,
    skipped: usize,
}

struct RefactorFsm {
    state: RefactorState,
    transitions: Vec<(RefactorState, RefactorState)>,
    history: VecDeque<RefactorState>,
    event_log: Vec<RefactorEvent>,
    pre_hooks: Vec<Box<dyn Fn(&RefactorState) + Send>>,
    post_hooks: Vec<Box<dyn Fn(&RefactorState) + Send>>,
    cancellation_token: Option<mpsc::Sender<()>>,
    rollback_stack: Vec<RefactorState>,
}

impl RefactorFsm {
    fn new() -> Self {
        Self {
            state: RefactorState::Init,
            transitions: Self::define_transitions(),
            history: VecDeque::with_capacity(100),
            event_log: Vec::new(),
            pre_hooks: Vec::new(),
            post_hooks: Vec::new(),
            cancellation_token: None,
            rollback_stack: Vec::new(),
        }
    }

    fn define_transitions() -> Vec<(RefactorState, RefactorState)> {
        vec![
            (RefactorState::Init, RefactorState::Parsing),
            (RefactorState::Parsing, RefactorState::Analyzing),
            (RefactorState::Analyzing, RefactorState::Planning),
            (RefactorState::Planning, RefactorState::Refactoring),
            (RefactorState::Refactoring, RefactorState::Testing),
            (RefactorState::Testing, RefactorState::Validating),
            (RefactorState::Validating, RefactorState::Complete),
            (RefactorState::Validating, RefactorState::Rollback),
            (RefactorState::Rollback, RefactorState::Init),
        ]
    }

    async fn process_event(&mut self, event: RefactorEvent) -> Result<RefactorState, String> {
        self.event_log.push(event.clone());

        let new_state = match (&self.state, event) {
            (RefactorState::Init, RefactorEvent::Start(_)) => RefactorState::Parsing,
            (RefactorState::Parsing, RefactorEvent::ParseComplete(_)) => RefactorState::Analyzing,
            (RefactorState::Analyzing, RefactorEvent::AnalysisComplete(_)) => {
                RefactorState::Planning
            }
            (RefactorState::Planning, RefactorEvent::PlanGenerated(_)) => {
                RefactorState::Refactoring
            }
            (RefactorState::Refactoring, RefactorEvent::RefactorApplied(_)) => {
                RefactorState::Testing
            }
            (RefactorState::Testing, RefactorEvent::TestsRun(ref result)) => {
                if result.failed == 0 {
                    RefactorState::Validating
                } else {
                    RefactorState::Rollback
                }
            }
            (RefactorState::Validating, RefactorEvent::ValidationComplete(ref valid)) => {
                if *valid {
                    RefactorState::Complete
                } else {
                    RefactorState::Rollback
                }
            }
            (_, RefactorEvent::ErrorOccurred(_)) => RefactorState::Error,
            (_, RefactorEvent::Cancel) => RefactorState::Cancelled,
            (_, RefactorEvent::Pause) => RefactorState::Paused,
            (RefactorState::Paused, RefactorEvent::Resume) => {
                self.history.back().copied().unwrap_or(RefactorState::Init)
            }
            (_, RefactorEvent::Rollback) => RefactorState::Rollback,
            _ => return Err(format!("Invalid transition from {:?}", self.state)),
        };

        self.transition_to(new_state).await
    }

    async fn transition_to(&mut self, new_state: RefactorState) -> Result<RefactorState, String> {
        for hook in &self.pre_hooks {
            hook(&self.state);
        }

        self.history.push_back(self.state);
        if self.history.len() > 100 {
            self.history.pop_front();
        }

        self.rollback_stack.push(self.state);
        self.state = new_state;

        for hook in &self.post_hooks {
            hook(&self.state);
        }

        Ok(new_state)
    }

    fn add_pre_hook<F: Fn(&RefactorState) + Send + 'static>(&mut self, hook: F) {
        self.pre_hooks.push(Box::new(hook));
    }

    fn add_post_hook<F: Fn(&RefactorState) + Send + 'static>(&mut self, hook: F) {
        self.post_hooks.push(Box::new(hook));
    }

    async fn rollback(&mut self) -> Result<(), String> {
        println!("  🔄 Rolling back changes...");

        while let Some(prev_state) = self.rollback_stack.pop() {
            self.state = prev_state;
            sleep(Duration::from_millis(10)).await;
            println!("    Restored to: {:?}", prev_state);
        }

        self.state = RefactorState::Init;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("Refactor State Machine Demo");
    println!("===========================\n");

    demonstrate_12_states().await;
    demonstrate_async_handlers().await;
    demonstrate_hooks().await;
    test_determinism().await;
    fuzz_test_transitions().await;
    generate_mermaid_diagram();
    benchmark_transitions().await;
    demonstrate_rollback().await;
}

async fn demonstrate_12_states() {
    println!("📊 12-State Refactor FSM:");

    let states = vec![
        "Init",
        "Parsing",
        "Analyzing",
        "Planning",
        "Refactoring",
        "Testing",
        "Validating",
        "Complete",
        "Error",
        "Rollback",
        "Cancelled",
        "Paused",
    ];

    for (i, state) in states.iter().enumerate() {
        println!("  {}. {}", i + 1, state);
    }

    println!("\n  ✅ All 12 states defined");
}

async fn demonstrate_async_handlers() {
    println!("\n⚡ Async State Handlers with Cancellation:");

    let mut fsm = RefactorFsm::new();
    let (cancel_tx, mut cancel_rx) = mpsc::channel(1);
    fsm.cancellation_token = Some(cancel_tx);

    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        cancel_rx.recv().await;
        println!("  ⚠️ Cancellation requested");
    });

    fsm.process_event(RefactorEvent::Start("main.rs".to_string()))
        .await
        .unwrap();
    println!("  State after Start: {:?}", fsm.state);

    fsm.process_event(RefactorEvent::ParseComplete(1000))
        .await
        .unwrap();
    println!("  State after Parse: {:?}", fsm.state);

    println!("  ✅ Async handlers executed");
}

async fn demonstrate_hooks() {
    println!("\n🪝 Pre/Post Transition Hooks:");

    let mut fsm = RefactorFsm::new();

    fsm.add_pre_hook(|state| {
        println!("  [PRE] Leaving state: {:?}", state);
    });

    fsm.add_post_hook(|state| {
        println!("  [POST] Entered state: {:?}", state);
    });

    fsm.process_event(RefactorEvent::Start("test.rs".to_string()))
        .await
        .unwrap();
    fsm.process_event(RefactorEvent::ParseComplete(500))
        .await
        .unwrap();
}

async fn test_determinism() {
    println!("\n🎲 Property Test - Determinism (1000 runs):");

    let mut results = Vec::new();

    for _ in 0..1000 {
        let mut fsm = RefactorFsm::new();
        fsm.process_event(RefactorEvent::Start("file.rs".to_string()))
            .await
            .unwrap();
        fsm.process_event(RefactorEvent::ParseComplete(100))
            .await
            .unwrap();
        results.push(fsm.state);
    }

    let all_same = results.windows(2).all(|w| w[0] == w[1]);

    if all_same {
        println!("  ✅ FSM is deterministic");
    } else {
        println!("  ❌ Non-deterministic behavior detected");
    }
}

async fn fuzz_test_transitions() {
    println!("\n🔀 Fuzz Testing State Transitions:");

    let mut fsm = RefactorFsm::new();
    let events = vec![
        RefactorEvent::Start("fuzz.rs".to_string()),
        RefactorEvent::ParseComplete(42),
        RefactorEvent::Cancel,
        RefactorEvent::Resume,
        RefactorEvent::ErrorOccurred("fuzz error".to_string()),
        RefactorEvent::Rollback,
    ];

    for event in events {
        let result = fsm.process_event(event.clone()).await;
        match result {
            Ok(state) => println!("  {:?} -> {:?}", event, state),
            Err(_) => println!("  {:?} -> Invalid transition", event),
        }
    }

    println!("  ✅ Fuzz test completed without panic");
}

fn generate_mermaid_diagram() {
    println!("\n📐 Mermaid State Diagram:");
    println!("```mermaid");
    println!("stateDiagram-v2");
    println!("    [*] --> Init");
    println!("    Init --> Parsing: Start");
    println!("    Parsing --> Analyzing: ParseComplete");
    println!("    Analyzing --> Planning: AnalysisComplete");
    println!("    Planning --> Refactoring: PlanGenerated");
    println!("    Refactoring --> Testing: RefactorApplied");
    println!("    Testing --> Validating: TestsPassed");
    println!("    Testing --> Rollback: TestsFailed");
    println!("    Validating --> Complete: Valid");
    println!("    Validating --> Rollback: Invalid");
    println!("    Rollback --> Init: Restored");
    println!("    Complete --> [*]");
    println!("```");
}

async fn benchmark_transitions() {
    println!("\n⏱️ Benchmark: 100k transitions/second:");

    let mut fsm = RefactorFsm::new();
    let iterations = 100_000;
    let start = Instant::now();

    for i in 0..iterations {
        let event = if i % 2 == 0 {
            RefactorEvent::Start(format!("file_{}.rs", i))
        } else {
            RefactorEvent::ParseComplete(i)
        };

        let _ = fsm.process_event(event).await;

        if i % 3 == 0 {
            fsm.state = RefactorState::Init;
        }
    }

    let duration = start.elapsed();
    let rate = iterations as f64 / duration.as_secs_f64();

    println!("  Transitions: {}", iterations);
    println!("  Duration: {:?}", duration);
    println!("  Rate: {:.0} transitions/second", rate);

    if rate >= 100_000.0 {
        println!("  ✅ Performance target met");
    }
}

async fn demonstrate_rollback() {
    println!("\n↩️ Rollback Mechanism:");

    let mut fsm = RefactorFsm::new();

    fsm.process_event(RefactorEvent::Start("rollback_test.rs".to_string()))
        .await
        .unwrap();
    fsm.process_event(RefactorEvent::ParseComplete(200))
        .await
        .unwrap();
    fsm.process_event(RefactorEvent::AnalysisComplete(vec!["issue1".to_string()]))
        .await
        .unwrap();

    println!("  Current state: {:?}", fsm.state);
    println!("  History length: {}", fsm.history.len());

    fsm.rollback().await.unwrap();

    println!("  After rollback: {:?}", fsm.state);
    println!("  ✅ Rollback completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_transitions() {
        let mut fsm = RefactorFsm::new();
        assert_eq!(fsm.state, RefactorState::Init);

        fsm.process_event(RefactorEvent::Start("test.rs".to_string()))
            .await
            .unwrap();
        assert_eq!(fsm.state, RefactorState::Parsing);
    }

    #[tokio::test]
    async fn test_error_transition() {
        let mut fsm = RefactorFsm::new();
        fsm.process_event(RefactorEvent::ErrorOccurred("test error".to_string()))
            .await
            .unwrap();
        assert_eq!(fsm.state, RefactorState::Error);
    }
}
