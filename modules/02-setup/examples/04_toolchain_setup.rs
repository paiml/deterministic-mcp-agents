use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    println!("Rust Toolchain Configuration");
    println!("============================\n");

    generate_optimal_cargo_toml();
    configure_clippy();
    setup_pre_commit_hooks();
    create_makefile_targets();
    configure_rust_analyzer();
    setup_criterion_benchmarking();
    verify_zero_warnings();
}

fn generate_optimal_cargo_toml() {
    println!("📦 Generating Optimal Cargo.toml:");

    let cargo_toml = r#"[package]
name = "optimized-project"
version = "0.1.0"
edition = "2021"

[profile.release]
lto = true           # Link-time optimization
panic = "abort"      # Smaller binary, no unwinding
codegen-units = 1    # Better optimization
strip = true         # Strip symbols
opt-level = 3        # Maximum optimization

[profile.bench]
inherits = "release"

[profile.dev]
opt-level = 0        # Fast compilation
debug = true

[profile.test]
opt-level = 0        # Fast test compilation"#;

    println!("{}", indent(cargo_toml, 2));
    println!("  ✅ Optimization settings configured");
}

fn configure_clippy() {
    println!("\n📎 Clippy Configuration (.clippy.toml):");

    let clippy_config = r#"avoid-breaking-exported-api = false
msrv = "1.75.0"
warn-on-all-wildcard-imports = true
allow-expect-in-tests = true
allow-unwrap-in-tests = true
allow-dbg-in-tests = true
allow-print-in-tests = true

[[disallowed-methods]]
path = "std::option::Option::unwrap"
reason = "use expect() or proper error handling"

[[disallowed-methods]]
path = "std::result::Result::unwrap"
reason = "use expect() or proper error handling""#;

    println!("{}", indent(clippy_config, 2));

    let lints = vec![
        "clippy::all",
        "clippy::pedantic",
        "clippy::nursery",
        "clippy::cargo",
        "clippy::restriction",
    ];

    println!("\n  Recommended lints:");
    for lint in lints {
        println!("    - {}", lint);
    }
}

fn setup_pre_commit_hooks() {
    println!("\n🪝 Pre-commit Hooks (.git/hooks/pre-commit):");

    let pre_commit = r#"#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Format check
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "❌ Formatting issues found. Run 'cargo fmt'"
    exit 1
fi

# Clippy
cargo clippy --all-targets --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy warnings found"
    exit 1
fi

# Tests
cargo test --all --all-features
if [ $? -ne 0 ]; then
    echo "❌ Tests failed"
    exit 1
fi

# PMAT checks
pmat analyze complexity --max-cyclomatic 20 --fail-on-violation
pmat analyze satd --strict --fail-on-violation
pmat analyze dead-code --max-percentage 5.0 --fail-on-violation

echo "✅ All pre-commit checks passed!"
exit 0"#;

    println!("{}", indent(pre_commit, 2));
}

fn create_makefile_targets() {
    println!("\n🎯 Makefile Targets:");

    let targets = vec![
        ("build", "cargo build --all --all-features"),
        ("test", "cargo test --all --all-features"),
        ("bench", "cargo bench --all"),
        ("fmt", "cargo fmt --all"),
        (
            "clippy",
            "cargo clippy --all-targets --all-features -- -D warnings",
        ),
        ("quality", "pmat quality-gate --fail-on-violation"),
        (
            "coverage",
            "cargo tarpaulin --out Html --output-dir coverage",
        ),
        ("doc", "cargo doc --all --all-features --no-deps --open"),
    ];

    for (name, command) in targets {
        println!("  {:-12} : {}", name, command);
    }
}

fn configure_rust_analyzer() {
    println!("\n🔧 Rust Analyzer Settings (.vscode/settings.json):");

    let settings = r#"{
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": [
    "--all-targets",
    "--all-features",
    "--",
    "-D",
    "warnings"
  ],
  "rust-analyzer.inlayHints.enable": true,
  "rust-analyzer.inlayHints.chainingHints": true,
  "rust-analyzer.inlayHints.parameterHints": true,
  "rust-analyzer.inlayHints.typeHints": true,
  "rust-analyzer.lens.enable": true,
  "rust-analyzer.lens.run": true,
  "rust-analyzer.lens.debug": true,
  "rust-analyzer.lens.implementations": true,
  "rust-analyzer.procMacro.enable": true
}"#;

    println!("{}", indent(settings, 2));
}

fn setup_criterion_benchmarking() {
    println!("\n📊 Criterion Benchmarking Setup:");

    let bench_example = r#"use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);"#;

    println!("{}", indent(bench_example, 2));
    println!("\n  ✅ Criterion harness configured");
}

fn verify_zero_warnings() {
    println!("\n✅ Zero Warnings Verification:");
    println!("  Command: cargo clippy -- -D warnings");
    println!("  Expected: No output (all checks pass)");
    println!("  Status: Ready for CI/CD integration");
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
    fn test_indent() {
        let text = "line1\nline2";
        let indented = indent(text, 2);
        assert!(indented.contains("  line1"));
        assert!(indented.contains("  line2"));
    }
}
