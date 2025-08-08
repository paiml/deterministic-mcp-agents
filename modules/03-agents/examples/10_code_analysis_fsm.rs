use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
enum AnalysisState {
    Init,
    Parsing,
    Analyzing,
    Verifying,
    Complete,
}

struct CodeAnalysisFsm {
    state: AnalysisState,
}

impl CodeAnalysisFsm {
    fn new() -> Self {
        Self {
            state: AnalysisState::Init,
        }
    }

    fn parse(&mut self) -> Result<(), String> {
        if self.state != AnalysisState::Init {
            return Err("Invalid state for parsing".to_string());
        }
        self.state = AnalysisState::Parsing;
        Ok(())
    }

    fn analyze(&mut self) -> Result<(), String> {
        if self.state != AnalysisState::Parsing {
            return Err("Must parse before analyzing".to_string());
        }
        self.state = AnalysisState::Analyzing;
        Ok(())
    }

    fn verify(&mut self) -> Result<(), String> {
        if self.state != AnalysisState::Analyzing {
            return Err("Must analyze before verifying".to_string());
        }
        self.state = AnalysisState::Verifying;
        Ok(())
    }
}

fn main() {
    println!("Code Analysis FSM Demo");
    println!("======================\n");

    let mut fsm = CodeAnalysisFsm::new();

    println!("Pipeline: parse → analyze → verify");

    fsm.parse().unwrap();
    println!("✅ Parsing complete");

    fsm.analyze().unwrap();
    println!("✅ Analysis complete");

    fsm.verify().unwrap();
    println!("✅ Verification complete");

    let start = Instant::now();
    for _ in 0..1000 {
        let mut fsm = CodeAnalysisFsm::new();
        fsm.parse().unwrap();
        fsm.analyze().unwrap();
        fsm.verify().unwrap();
    }
    println!("\n⚡ 1000 pipelines in {:?}", start.elapsed());
}
