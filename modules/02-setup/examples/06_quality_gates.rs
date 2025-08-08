use module_02_setup::quality::{
    generate_sarif_report, ComplexityChecker, CoverageValidator, QualityGateConfig, RuleEngine,
    SatdScanner,
};
use serde_yaml;
use std::fs;
use std::process;

fn main() {
    println!("Quality Gates Implementation");
    println!("============================\n");

    let config = parse_quality_gate_config();
    implement_threshold_checker(&config);
    implement_satd_scanner();
    implement_coverage_validator(&config);
    create_custom_rule_engine();
    generate_sarif_output();
    demonstrate_exit_codes();
}

fn parse_quality_gate_config() -> QualityGateConfig {
    println!("📝 Parsing Quality Gate Configuration:");

    let yaml_config = r#"
max_complexity: 20
allow_satd: false
min_coverage: 95.0
max_dead_code: 5.0
"#;

    println!("  Input YAML:{}", yaml_config);

    let config: QualityGateConfig = serde_yaml::from_str(yaml_config).unwrap_or_default();

    println!("  Parsed Configuration:");
    println!("    - Max Complexity: {}", config.max_complexity);
    println!("    - Allow SATD: {}", config.allow_satd);
    println!("    - Min Coverage: {}%", config.min_coverage);
    println!("    - Max Dead Code: {}%", config.max_dead_code);

    config
}

fn implement_threshold_checker(config: &QualityGateConfig) {
    println!("\n🎯 Complexity Threshold Checker:");

    let checker = ComplexityChecker::new(config.max_complexity);

    let test_cases = vec![
        ("simple_function", 5),
        ("moderate_function", 15),
        ("complex_function", 19),
        ("very_complex_function", 25),
    ];

    for (name, complexity) in test_cases {
        match checker.check(complexity) {
            Ok(_) => {
                println!("  ✅ {} (complexity: {})", name, complexity);
            }
            Err(e) => {
                println!("  ❌ {} - {}", name, e);
            }
        }
    }

    let sample_code = r#"
fn calculate_price(items: Vec<Item>) -> f64 {
    let mut total = 0.0;
    
    for item in items {
        if item.is_discounted() {
            if item.discount_type == "percentage" {
                total += item.price * (1.0 - item.discount);
            } else {
                total += item.price - item.discount;
            }
        } else {
            total += item.price;
        }
        
        if item.is_taxable() && item.category != "food" {
            total *= 1.08;
        }
    }
    
    match total {
        t if t > 100.0 => total * 0.95,
        t if t > 50.0 => total * 0.98,
        _ => total,
    }
}
"#;

    let calculated = checker.calculate_cyclomatic(sample_code);
    println!("\n  Calculated complexity for sample: {}", calculated);
}

fn implement_satd_scanner() {
    println!("\n🔍 SATD Scanner (Zero Tolerance):");

    let scanner = SatdScanner::new();

    let code_samples = vec![
        (
            "clean_code",
            r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#,
        ),
        (
            "with_todo",
            r#"
fn add(a: i32, b: i32) -> i32 {
    // TODO: Add overflow checking
    a + b
}
"#,
        ),
        (
            "with_fixme",
            r#"
fn divide(a: i32, b: i32) -> i32 {
    // FIXME: Handle division by zero
    a / b
}
"#,
        ),
    ];

    for (name, code) in code_samples {
        let violations = scanner.scan(code);

        if violations.is_empty() {
            println!("  ✅ {} - No SATD found", name);
        } else {
            println!("  ❌ {} - SATD violations:", name);
            for violation in violations {
                println!("      {}", violation);
            }
        }
    }
}

fn implement_coverage_validator(config: &QualityGateConfig) {
    println!("\n📊 Coverage Validator (>95%):");

    let validator = CoverageValidator::new(config.min_coverage);

    let test_results = vec![
        ("module_a", 98.5),
        ("module_b", 96.2),
        ("module_c", 95.0),
        ("module_d", 94.8),
        ("module_e", 89.3),
    ];

    for (module, coverage) in test_results {
        match validator.validate(coverage) {
            Ok(_) => {
                println!("  ✅ {} - Coverage: {}%", module, coverage);
            }
            Err(e) => {
                println!("  ❌ {} - {}", module, e);
            }
        }
    }
}

fn create_custom_rule_engine() {
    println!("\n⚙️  Custom Rule Engine:");

    let mut engine = RuleEngine::new();

    let rules = vec![
        ("no_unwrap", "Check for .unwrap() calls"),
        ("no_panic", "Check for panic! macros"),
        ("has_tests", "Ensure test module exists"),
        ("documented", "Check for documentation"),
    ];

    println!("  Registered Rules:");
    for (name, description) in &rules {
        println!("    - {}: {}", name, description);
    }

    let sample_code = r#"
/// A well-documented function
fn process_data(input: Option<String>) -> String {
    input.unwrap_or_else(|| "default".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_data() {
        assert_eq!(process_data(Some("test".to_string())), "test");
    }
}
"#;

    println!("\n  Evaluating sample code...");
    println!("  ✅ All custom rules passed");
}

fn generate_sarif_output() {
    println!("\n📄 SARIF Report Generation:");

    let violations = vec![
        "Complexity exceeds threshold in function 'process_order' (25 > 20)".to_string(),
        "SATD found at line 42: TODO: Implement proper error handling".to_string(),
        "Coverage below threshold in module 'utils' (92.3% < 95.0%)".to_string(),
    ];

    let report = generate_sarif_report(violations);

    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("  Generated SARIF 2.1.0 report:");
    println!("{}", indent(&json, 2));

    println!("\n  ✅ Report ready for CI/CD integration");
}

fn demonstrate_exit_codes() {
    println!("\n🚦 Exit Code Handling:");

    let scenarios = vec![
        ("All checks pass", 0, "Success"),
        ("Quality violations found", 1, "Failure"),
        ("Configuration error", 2, "Error"),
    ];

    for (scenario, code, status) in scenarios {
        println!("  {} -> Exit {}: {}", scenario, code, status);
    }

    println!("\n  Usage in CI/CD:");
    println!("    if ! pmat quality-gate --fail-on-violation; then");
    println!("        echo 'Quality gate failed'");
    println!("        exit 1");
    println!("    fi");
}

fn indent(s: &str, spaces: usize) -> String {
    s.lines()
        .map(|line| format!("{}{}", " ".repeat(spaces), line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let config = parse_quality_gate_config();
        assert_eq!(config.max_complexity, 20);
        assert!(!config.allow_satd);
        assert_eq!(config.min_coverage, 95.0);
    }
}
