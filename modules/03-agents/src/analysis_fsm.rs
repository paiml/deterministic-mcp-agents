pub struct AnalysisFsm {
    #[allow(dead_code)]
    state: String,
}

impl AnalysisFsm {
    #[must_use]
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
