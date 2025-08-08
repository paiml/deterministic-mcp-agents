use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QualityError {
    #[error("Complexity threshold exceeded: {0} > {1}")]
    ComplexityExceeded(u32, u32),
    
    #[error("SATD violation found: {0}")]
    SatdViolation(String),
    
    #[error("Coverage below threshold: {0}% < {1}%")]
    CoverageBelowThreshold(f64, f64),
    
    #[error("Dead code percentage exceeded: {0}% > {1}%")]
    DeadCodeExceeded(f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGateConfig {
    pub max_complexity: u32,
    pub allow_satd: bool,
    pub min_coverage: f64,
    pub max_dead_code: f64,
}

impl Default for QualityGateConfig {
    fn default() -> Self {
        Self {
            max_complexity: 20,
            allow_satd: false,
            min_coverage: 95.0,
            max_dead_code: 5.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComplexityChecker {
    max_complexity: u32,
}

impl ComplexityChecker {
    pub fn new(max_complexity: u32) -> Self {
        Self { max_complexity }
    }
    
    pub fn check(&self, complexity: u32) -> Result<(), QualityError> {
        if complexity > self.max_complexity {
            Err(QualityError::ComplexityExceeded(complexity, self.max_complexity))
        } else {
            Ok(())
        }
    }
    
    pub fn calculate_cyclomatic(&self, code: &str) -> u32 {
        let mut complexity = 1;
        
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("if ") || trimmed.starts_with("else if ") {
                complexity += 1;
            }
            if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
                complexity += 1;
            }
            if trimmed.starts_with("match ") {
                complexity += 1;
            }
            if trimmed.contains(" && ") || trimmed.contains(" || ") {
                complexity += 1;
            }
        }
        
        complexity
    }
}

#[derive(Debug, Clone)]
pub struct SatdScanner {
    patterns: Vec<String>,
}

impl SatdScanner {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                "TODO".to_string(),
                "FIXME".to_string(),
                "HACK".to_string(),
                "XXX".to_string(),
                "REFACTOR".to_string(),
                "OPTIMIZE".to_string(),
            ],
        }
    }
    
    pub fn scan(&self, code: &str) -> Vec<String> {
        let mut violations = Vec::new();
        
        for (line_num, line) in code.lines().enumerate() {
            for pattern in &self.patterns {
                if line.contains(pattern) {
                    violations.push(format!("Line {}: {}", line_num + 1, line.trim()));
                }
            }
        }
        
        violations
    }
    
    pub fn check(&self, code: &str) -> Result<(), QualityError> {
        let violations = self.scan(code);
        if !violations.is_empty() {
            Err(QualityError::SatdViolation(violations.join(", ")))
        } else {
            Ok(())
        }
    }
}

impl Default for SatdScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CoverageValidator {
    min_coverage: f64,
}

impl CoverageValidator {
    pub fn new(min_coverage: f64) -> Self {
        Self { min_coverage }
    }
    
    pub fn validate(&self, coverage: f64) -> Result<(), QualityError> {
        if coverage < self.min_coverage {
            Err(QualityError::CoverageBelowThreshold(coverage, self.min_coverage))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleEngine {
    rules: HashMap<String, Box<dyn Fn(&str) -> bool>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
    
    pub fn evaluate(&self, code: &str) -> Vec<String> {
        let mut violations = Vec::new();
        
        for (name, rule) in &self.rules {
            if !rule(code) {
                violations.push(name.clone());
            }
        }
        
        violations
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReport {
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifResult {
    pub message: SarifMessage,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifMessage {
    pub text: String,
}

pub fn generate_sarif_report(violations: Vec<String>) -> SarifReport {
    SarifReport {
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "quality-gates".to_string(),
                    version: "0.1.0".to_string(),
                },
            },
            results: violations
                .into_iter()
                .map(|v| SarifResult {
                    message: SarifMessage { text: v },
                    level: "error".to_string(),
                })
                .collect(),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complexity_checker() {
        let checker = ComplexityChecker::new(20);
        assert!(checker.check(15).is_ok());
        assert!(checker.check(25).is_err());
    }
    
    #[test]
    fn test_calculate_cyclomatic() {
        let checker = ComplexityChecker::new(20);
        let code = r#"
            fn example() {
                if condition {
                    for item in items {
                        if item.is_valid() || item.is_special() {
                            process(item);
                        }
                    }
                }
            }
        "#;
        
        let complexity = checker.calculate_cyclomatic(code);
        assert!(complexity > 1);
    }
    
    #[test]
    fn test_satd_scanner() {
        let scanner = SatdScanner::new();
        
        let clean_code = "fn add(a: i32, b: i32) -> i32 { a + b }";
        assert!(scanner.check(clean_code).is_ok());
        
        let dirty_code = "fn add(a: i32, b: i32) -> i32 { a + b } // TODO: add overflow check";
        assert!(scanner.check(dirty_code).is_err());
    }
    
    #[test]
    fn test_coverage_validator() {
        let validator = CoverageValidator::new(95.0);
        assert!(validator.validate(96.0).is_ok());
        assert!(validator.validate(94.0).is_err());
    }
}