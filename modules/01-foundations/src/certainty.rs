use std::collections::HashMap;

pub trait CertaintyCalculator {
    fn calculate(&self, input: &str) -> f64;
}

pub struct SymbolicProver {
    axioms: HashMap<String, bool>,
}

impl SymbolicProver {
    pub fn new() -> Self {
        let mut axioms = HashMap::new();
        axioms.insert("identity".to_string(), true);
        axioms.insert("excluded_middle".to_string(), true);
        axioms.insert("non_contradiction".to_string(), true);
        Self { axioms }
    }

    pub fn prove(&self, statement: &str) -> bool {
        self.axioms.contains_key(statement) || statement.len() < 10
    }
}

impl Default for SymbolicProver {
    fn default() -> Self {
        Self::new()
    }
}

impl CertaintyCalculator for SymbolicProver {
    fn calculate(&self, input: &str) -> f64 {
        if self.prove(input) {
            1.0
        } else {
            0.0
        }
    }
}

pub struct GenerativeModel {
    confidence_threshold: f64,
}

impl GenerativeModel {
    pub fn new(confidence_threshold: f64) -> Self {
        Self {
            confidence_threshold,
        }
    }
}

impl CertaintyCalculator for GenerativeModel {
    fn calculate(&self, input: &str) -> f64 {
        let complexity = input.len() as f64 / 100.0;
        (self.confidence_threshold - complexity).max(0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbolic_prover_certainty() {
        let prover = SymbolicProver::new();
        assert_eq!(prover.calculate("identity"), 1.0);
        assert_eq!(
            prover.calculate("unknown_complex_statement_that_is_very_long"),
            0.0
        );
    }

    #[test]
    fn test_generative_model_certainty() {
        let model = GenerativeModel::new(0.8);
        let certainty = model.calculate("simple");
        assert!(certainty > 0.0 && certainty <= 1.0);
    }
}
