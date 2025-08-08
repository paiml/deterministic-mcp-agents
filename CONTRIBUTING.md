# Contributing to Deterministic MCP Agents Course

Thank you for your interest in contributing to this course! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and professional in all interactions. We aim to maintain a welcoming and inclusive environment for all contributors.

## Development Setup

### Prerequisites

1. Install Rust 1.75+ from [rustup.rs](https://rustup.rs)
2. Install PMAT with MCP support:
   ```bash
   cargo install pmat --version ">=0.29.6" --features mcp
   ```
3. Install development tools:
   ```bash
   make install-dev
   ```

### Building the Project

```bash
# Build all modules
make build

# Build specific module
cd modules/01-foundations
cargo build --all-features
```

## Quality Standards

All contributions must meet these quality requirements:

### 1. Code Coverage: 80% Minimum

Every module must maintain at least 80% test coverage:

```bash
# Check coverage for all modules
make coverage-check

# Check coverage for specific module
cd modules/01-foundations
cargo tarpaulin --out Html
```

### 2. PMAT Quality Gates

All code must pass PMAT quality checks:

- **Cyclomatic Complexity**: Max 20
- **SATD**: Zero tolerance (no TODO, FIXME, HACK comments)
- **Dead Code**: Less than 5%

```bash
# Run all PMAT checks
make pmat-all

# Run specific checks
make pmat-complexity
make pmat-satd
make pmat-dead-code
```

### 3. Code Formatting and Linting

Code must be properly formatted and pass all clippy checks:

```bash
# Format code
make fmt

# Run clippy
make clippy
```

### 4. Testing Requirements

#### Unit Tests
- All public functions must have unit tests
- Use `#[cfg(test)]` modules for tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        assert_eq!(function(2, 3), 5);
    }
}
```

#### Property Tests
- Use quickcheck for property-based testing
- Minimum 3 properties per module

```rust
use quickcheck_macros::quickcheck;

#[quickcheck]
fn prop_commutative(a: i32, b: i32) -> bool {
    add(a, b) == add(b, a)
}
```

#### Doctests
- All public APIs must have documentation with examples
- Examples must be executable

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use my_module::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Adding New Examples

### 1. Create the Example File

Place new examples in the appropriate module's `examples/` directory:

```bash
modules/
├── 01-foundations/
│   └── examples/
│       └── XX_example_name.rs
```

### 2. Update Cargo.toml

Add the binary target to the module's `Cargo.toml`:

```toml
[[bin]]
name = "XX_example_name"
path = "examples/XX_example_name.rs"
```

### 3. Add to Makefile

Update the `run-examples` target in the Makefile:

```makefile
run-examples:
    # ... existing examples
    cargo run --bin XX_example_name --quiet
```

### 4. Document the Example

Add comprehensive documentation at the top of the example file:

```rust
//! # Example Name
//! 
//! This example demonstrates...
//! 
//! ## Key Concepts
//! - Concept 1
//! - Concept 2
//! 
//! ## Performance
//! - Target: <100ms execution time
//! - Memory: <10MB
```

## Pull Request Process

### 1. Before Submitting

Run all quality checks locally:

```bash
# Quick checks
make pre-commit

# Full validation
make pre-push
```

### 2. PR Requirements

Your PR must:
- Pass all CI checks
- Maintain 80%+ code coverage
- Include tests for new functionality
- Update documentation as needed
- Follow conventional commit format

### 3. Commit Message Format

Use conventional commits:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance

Example:
```
feat(module-03): add refactor state machine example

Implements 12-state FSM with async handlers and rollback support.
Includes property tests for determinism validation.
```

## Performance Guidelines

### Benchmarks

All performance-critical code must include benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| function(black_box(42)))
    });
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
```

### Performance Targets

- FSM transitions: <1μs
- Tool execution: <10ms P50, <100ms P99
- MCP round-trip: <5ms local
- Memory baseline: <100MB
- Binary size: <10MB stripped

## Documentation

### Module Documentation

Each module must have a comprehensive README:

```markdown
# Module Name

## Overview
Brief description of the module.

## Learning Objectives
- Objective 1
- Objective 2

## Examples
- `00_example.rs` - Description
- `01_example.rs` - Description

## Key Concepts
Detailed explanations...

## Performance Metrics
Benchmarks and targets...
```

### API Documentation

All public APIs must be documented:

```rust
/// Brief description.
///
/// Detailed explanation of the function's purpose,
/// parameters, return values, and any side effects.
///
/// # Arguments
///
/// * `param1` - Description
/// * `param2` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of possible errors
///
/// # Examples
///
/// ```
/// use my_module::function;
/// let result = function(42, "test");
/// assert_eq!(result, expected);
/// ```
///
/// # Panics
///
/// Conditions that cause panic
///
/// # Safety
///
/// For unsafe functions, explain safety requirements
pub fn function(param1: i32, param2: &str) -> Result<String, Error> {
    // Implementation
}
```

## CI/CD Integration

### GitHub Actions

All PRs trigger automated checks:

1. **Quality Gate**: Formatting, linting, tests
2. **PMAT Analysis**: Complexity, SATD, dead code
3. **Coverage**: 80% minimum requirement
4. **Examples**: All examples must run successfully
5. **Property Tests**: Quickcheck and proptest suites
6. **Benchmarks**: Performance regression detection

### Local Validation

Before pushing, run:

```bash
# Complete validation
make run-all

# This runs:
# - Build
# - Tests
# - Examples
# - Quality gates
# - Coverage
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/paiml/deterministic-mcp-agents-course/issues)
- **Discussions**: [GitHub Discussions](https://github.com/paiml/deterministic-mcp-agents-course/discussions)
- **Documentation**: [Course Docs](./docs/)

## Release Process

1. Update version in `Cargo.toml` files
2. Update CHANGELOG.md
3. Create git tag: `git tag -a v0.x.x -m "Release v0.x.x"`
4. Push tag: `git push origin v0.x.x`
5. GitHub Actions creates release automatically

## License

By contributing, you agree that your contributions will be licensed under the MIT License.