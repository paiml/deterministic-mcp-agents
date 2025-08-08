pub struct AnalysisFsm {
    state: String,
}

impl AnalysisFsm {
    pub fn new() -> Self {
        Self {
            state: "init".to_string(),
        }
    }
}

impl Default for AnalysisFsm {
    fn default() -> Self {
        Self::new()
    }
}