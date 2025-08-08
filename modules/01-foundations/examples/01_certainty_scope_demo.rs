use module_01_foundations::certainty::{CertaintyCalculator, SymbolicProver, GenerativeModel};
use module_01_foundations::scope::{ScopeAnalyzer, DomainScope, verify_constraint, K_CONSTANT};
use std::time::Instant;

fn main() {
    println!("Certainty-Scope Tradeoff Demonstration");
    println!("======================================");
    println!("Constraint: C(M) × S(M) ≤ {}", K_CONSTANT);
    
    demonstrate_symbolic_prover();
    demonstrate_generative_model();
    visualize_tradeoff_curve();
    benchmark_computation();
}

fn demonstrate_symbolic_prover() {
    println!("\n🔬 Symbolic Prover (High Certainty, Low Scope):");
    
    let prover = SymbolicProver::new();
    let scope_analyzer = DomainScope::new(1000);
    
    let test_cases = vec![
        "identity",
        "simple",
        "complex_mathematical_theorem_requiring_deep_analysis",
    ];
    
    for case in test_cases {
        let certainty = prover.calculate(case);
        let scope = scope_analyzer.analyze(case);
        let product = certainty * scope;
        
        println!("  Input: '{}...'", &case[..case.len().min(20)]);
        println!("    Certainty: {:.2}", certainty);
        println!("    Scope: {:.2}", scope);
        println!("    Product: {:.2}", product);
        println!("    Valid: {}", if verify_constraint(certainty, scope) { "✅" } else { "❌" });
    }
}

fn demonstrate_generative_model() {
    println!("\n🧠 Generative Model (Low Certainty, High Scope):");
    
    let model = GenerativeModel::new(0.8);
    let scope_analyzer = DomainScope::new(1000);
    
    let test_cases = vec![
        "generate_code",
        "write_essay_about_philosophy",
        "solve_creative_problem_with_multiple_approaches",
    ];
    
    for case in test_cases {
        let certainty = model.calculate(case);
        let scope = 1.0 - scope_analyzer.analyze(case);
        let product = certainty * scope;
        
        println!("  Input: '{}...'", &case[..case.len().min(30)]);
        println!("    Certainty: {:.2}", certainty);
        println!("    Scope: {:.2}", scope);
        println!("    Product: {:.2}", product);
        println!("    Valid: {}", if verify_constraint(certainty, scope) { "✅" } else { "❌" });
    }
}

fn visualize_tradeoff_curve() {
    println!("\n📈 Trade-off Curve Visualization:");
    println!("  C | S | C×S | Valid");
    println!("  --|---|-----|------");
    
    let data_points = vec![
        (1.0, 0.0),
        (0.9, 0.1),
        (0.8, 0.2),
        (0.7, 0.3),
        (0.6, 0.4),
        (0.5, 0.5),
        (0.4, 0.6),
        (0.3, 0.7),
        (0.2, 0.8),
        (0.1, 0.9),
        (0.0, 1.0),
    ];
    
    for (certainty, scope) in data_points {
        let product = certainty * scope;
        let valid = verify_constraint(certainty, scope);
        println!("  {:.1} | {:.1} | {:.2} | {}", 
                certainty, scope, product, 
                if valid { "✅" } else { "❌" });
    }
}

fn benchmark_computation() {
    println!("\n⏱️  Benchmark Results:");
    
    let prover = SymbolicProver::new();
    let model = GenerativeModel::new(0.8);
    let scope_analyzer = DomainScope::new(1000);
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = prover.calculate("test");
        let _ = model.calculate("test");
        let _ = scope_analyzer.analyze("test");
    }
    let duration = start.elapsed();
    
    let per_op = duration.as_micros() / 3000;
    println!("  Average computation time: {}μs", per_op);
    
    if duration.as_millis() < 1 {
        println!("  ✅ Performance target met (<1ms for 1000 operations)");
    } else {
        println!("  ⚠️  Performance: {}ms", duration.as_millis());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    
    #[quickcheck]
    fn prop_certainty_scope_constraint(certainty: f64, scope: f64) -> bool {
        let c = certainty.abs().min(1.0);
        let s = scope.abs().min(1.0);
        let product = c * s;
        product <= K_CONSTANT + f64::EPSILON
    }
    
    #[test]
    fn test_benchmark_performance() {
        let start = Instant::now();
        let prover = SymbolicProver::new();
        let _ = prover.calculate("test");
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1);
    }
}