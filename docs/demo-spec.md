# Deterministic MCP Agents Demo Projects Specification

## Repository: `github.com/paiml/deterministic-mcp-agents-course`

### Quality Enforcement Configuration

```yaml
# .github/workflows/quality-gate.yml
name: PMAT Quality Gate
on: [push, pull_request]
jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Install PMAT
        run: cargo install pmat --version ">=0.29.6"
      - name: Enforce Quality Standards
        run: |
          pmat analyze complexity --max-cyclomatic 20 --fail-on-violation
          pmat analyze satd --strict --fail-on-violation
          pmat analyze dead-code --max-percentage 5.0 --fail-on-violation
          pmat quality-gate --fail-on-violation
```

### Demo Project Checklist

## Module 1: Foundations (18:00)

### 1.1 Course Introduction Demo (03:00)
**File**: `modules/01-foundations/examples/00_course_overview.rs`
- [ ] Repository structure walkthrough implementation
- [ ] Performance baseline measurement (<100μs FSM transition)
- [ ] Quality metrics dashboard setup
- [ ] CI/CD pipeline configuration
- [ ] Test coverage reporting (>95% requirement)
- [ ] PMAT integration verification

### 1.2 Certainty-Scope Tradeoff Demo (05:00)
**File**: `modules/01-foundations/examples/01_certainty_scope_demo.rs`
```rust
// Demonstrates C(M) × S(M) ≤ k constraint
```
- [ ] Implement `CertaintyCalculator` trait
- [ ] Implement `ScopeAnalyzer` with Kolmogorov complexity approximation
- [ ] Create `SymbolicProver` (high C, low S)
- [ ] Create `GenerativeModel` simulator (low C, high S)
- [ ] Visualize trade-off curve with 10 data points
- [ ] Property test: `certainty * scope <= K_CONSTANT`
- [ ] Benchmark: <1ms computation time

### 1.3 Floridi Conjecture Implementation (04:00)
**File**: `modules/01-foundations/examples/02_floridi_conjecture.rs`
- [ ] Implement `EpistemicCertainty` calculator
- [ ] Implement `MappingScope` with I/O complexity metrics
- [ ] Create `HybridArchitecture` with verified kernel + learning envelope
- [ ] Demonstrate Coq-like verifier (100% certainty, narrow scope)
- [ ] Demonstrate AlphaCode-like generator (broad scope, probabilistic)
- [ ] Measure actual k values across 5 system types
- [ ] Generate LaTeX formula visualization

### 1.4 MCP Protocol Implementation (06:00)
**File**: `modules/01-foundations/examples/03_mcp_protocol_basics.rs`
- [ ] Implement JSON-RPC 2.0 message framer
- [ ] Create `StdioTransport` with buffering
- [ ] Implement `WebSocketTransport` with auto-reconnect
- [ ] Demonstrate request/response correlation
- [ ] Implement notification batching (10 messages/100ms)
- [ ] Error handling with structured codes (-32000 to -32899)
- [ ] Benchmark: <5ms round-trip latency

## Module 2: Environment Setup (18:00)

### 2.1 Rust Toolchain Configuration (04:00)
**File**: `modules/02-setup/examples/04_toolchain_setup.rs`
- [ ] Generate optimal `Cargo.toml` with lto=true, panic=abort
- [ ] Configure Clippy with pedantic, nursery, restriction lints
- [ ] Setup pre-commit hooks for quality enforcement
- [ ] Create Makefile with all PMAT targets
- [ ] Install and configure rust-analyzer settings
- [ ] Setup criterion benchmarking harness
- [ ] Verify zero warnings with `cargo clippy -- -D warnings`

### 2.2 PMAT Installation Validator (03:00)
**File**: `modules/02-setup/examples/05_pmat_validator.rs`
- [ ] Detect PMAT version and features
- [ ] Verify MCP feature flag enabled
- [ ] Check Docker availability for containerization
- [ ] Validate CI/CD templates installation
- [ ] Test quality gate execution
- [ ] Measure baseline metrics
- [ ] Generate installation report

### 2.3 Quality Gates Implementation (05:00)
**File**: `modules/02-setup/examples/06_quality_gates.rs`
- [ ] Parse quality gate YAML configuration
- [ ] Implement threshold checker for complexity (<20)
- [ ] Implement SATD scanner (zero tolerance)
- [ ] Implement coverage validator (>95%)
- [ ] Create custom rule engine
- [ ] Generate SARIF report
- [ ] Exit with proper codes (0/1)

### 2.4 Calculator Agent (06:00)
**File**: `modules/02-setup/examples/07_calculator_agent.rs`
```rust
#[derive(Debug, Clone)]
pub struct Calculator {
    state: CalculatorState,
    history: Vec<Operation>,
}
```
- [ ] Implement `Calculator` with checked arithmetic
- [ ] Add property tests for commutativity, associativity
- [ ] Implement FSM for operation sequencing
- [ ] Add overflow handling with `checked_add`, `checked_mul`
- [ ] Benchmark all operations <1μs
- [ ] Create MCP tool definition
- [ ] Add 50+ doctests with examples
- [ ] Achieve 100% branch coverage

## Module 3: Production Agents (23:00)

### 3.1 FSM Builder Pattern (05:00)
**File**: `modules/03-agents/examples/08_fsm_builder.rs`
- [ ] Implement `FsmBuilder<S, E>` with PhantomData
- [ ] Add compile-time state validation
- [ ] Create hierarchical state support
- [ ] Implement transition guards
- [ ] Add invariant preservation checks
- [ ] Event sourcing with audit trail
- [ ] State persistence with serde
- [ ] Zero-allocation transitions proof

### 3.2 Refactor State Machine (07:00)
**File**: `modules/03-agents/examples/09_refactor_fsm.rs`
```rust
enum RefactorState {
    Init, Parsing, Analyzing, Planning, 
    Refactoring, Testing, Validating, Complete, Error
}
```
- [ ] Implement 12-state refactor FSM
- [ ] Add async state handlers with cancellation
- [ ] Implement pre/post transition hooks
- [ ] Property test determinism (1000 runs)
- [ ] Fuzz test all state transitions
- [ ] Generate Mermaid state diagram
- [ ] Benchmark: 100k transitions/second
- [ ] Add rollback mechanism

### 3.3 Code Analysis FSM (06:00)
**File**: `modules/03-agents/examples/10_code_analysis_fsm.rs`
- [ ] Create `parse → analyze → verify` pipeline
- [ ] Implement AST validation guards
- [ ] Add metrics checking transitions
- [ ] Integrate with PMAT analysis
- [ ] Property test: idempotent analysis
- [ ] Measure complexity reduction
- [ ] Generate quality report
- [ ] Parallel state processing

### 3.4 Error Boundary Implementation (05:00)
**File**: `modules/03-agents/examples/11_error_boundaries.rs`
- [ ] Create custom error types with thiserror
- [ ] Implement retry with exponential backoff
- [ ] Add circuit breaker pattern
- [ ] Panic isolation with catch_unwind
- [ ] Structured logging with tracing
- [ ] Error telemetry collection
- [ ] Graceful degradation demo
- [ ] 100% error path coverage

## Module 4: MCP Server (22:00)

### 4.1 PMCP Server (06:00)
**File**: `modules/04-mcp-server/examples/12_pmcp_server.rs`
```rust
use pmcp::{Server, ServerBuilder, ToolHandler};
```
- [ ] Implement server with 5+ tools
- [ ] Add capability registration
- [ ] Create async tool handlers
- [ ] Implement request router
- [ ] Add connection lifecycle management
- [ ] Zero-copy message parsing
- [ ] Bounded channel backpressure
- [ ] Benchmark: 10k requests/second

### 4.2 Tool Composition (07:00)
**File**: `modules/04-mcp-server/examples/13_tool_composition.rs`
- [ ] Implement `analyze_complexity` tool
- [ ] Implement `extract_files` tool
- [ ] Implement `deep_analysis` tool
- [ ] Create dependency graph builder
- [ ] Add parallel execution engine
- [ ] Implement result aggregation
- [ ] Add caching layer (LRU, 1000 entries)
- [ ] Transaction rollback support
- [ ] Performance monitoring per tool

### 4.3 Async Request Handler (05:00)
**File**: `modules/04-mcp-server/examples/14_async_handler.rs`
- [ ] Configure tokio runtime (4 workers)
- [ ] Implement select!, join!, try_join! patterns
- [ ] Add CancellationToken propagation
- [ ] Stream processing with futures::stream
- [ ] Rate limiting (1000 req/s)
- [ ] Connection pooling (100 connections)
- [ ] Timeout strategies (30s global, 5s per request)
- [ ] P99 latency <100ms under load

### 4.4 Production Deployment (04:00)
**File**: `modules/04-mcp-server/examples/15_production_server.rs`
**Docker**: `modules/04-mcp-server/Dockerfile`
```dockerfile
FROM rust:1.80-alpine AS builder
# Multi-stage build for 15MB image
```
- [ ] Create multi-stage Dockerfile
- [ ] Add Kubernetes manifests
- [ ] Implement Prometheus metrics
- [ ] Add OpenTelemetry tracing
- [ ] Configure TLS (rustls)
- [ ] Rolling update support
- [ ] Health check endpoints
- [ ] Load test: 50k QPS sustained

## Module 5: Testing & QA (09:00)

### 5.1 Property Testing Suite (03:00)
**File**: `modules/05-testing/examples/16_property_tests.rs`
```rust
use quickcheck::{quickcheck, Arbitrary, Gen};
```
- [ ] Create custom generators for all domain types
- [ ] Implement shrinking strategies
- [ ] Test FSM determinism property
- [ ] Test idempotence property
- [ ] Test commutativity where applicable
- [ ] Model-based testing setup
- [ ] Coverage-guided generation
- [ ] Run 1M cases in <10 seconds

### 5.2 Fuzzing Infrastructure (03:00)
**File**: `modules/05-testing/examples/17_fuzzing.rs`
**Fuzz**: `fuzz/fuzz_targets/`
- [ ] Setup cargo-fuzz targets
- [ ] Configure AFL++ integration
- [ ] Create corpus from property tests
- [ ] Implement structured fuzzing
- [ ] Add protocol message fuzzing
- [ ] HTML coverage reports
- [ ] CI nightly fuzz runs
- [ ] Zero crashes in 24-hour run

### 5.3 Integration Tests (03:00)
**File**: `modules/05-testing/examples/18_integration_tests.rs`
- [ ] End-to-end MCP protocol test
- [ ] Multi-transport validation
- [ ] Database transaction tests
- [ ] Network mocking with wiremock
- [ ] Time manipulation tests
- [ ] Parallel execution strategy
- [ ] Golden file regression tests
- [ ] Performance benchmarks as tests

## Final Project Components

### Production MCP Server
**File**: `final_project/src/main.rs`
- [ ] 5+ production-ready tools
- [ ] 8+ state FSM with verification
- [ ] 95%+ test coverage
- [ ] Zero SATD, complexity <20
- [ ] Docker + K8s deployment
- [ ] <100ms P99 latency
- [ ] 1k QPS sustained load
- [ ] Complete API documentation

### Quality Metrics Dashboard
**File**: `final_project/metrics/dashboard.rs`
- [ ] Real-time complexity tracking
- [ ] Coverage visualization
- [ ] Performance graphs
- [ ] Error rate monitoring
- [ ] FSM state distribution
- [ ] Tool usage statistics
- [ ] Resource consumption
- [ ] Quality gate status

## PMAT Quality Enforcement Targets

```bash
# Run before every commit
make quality-gate-all
```

### Mandatory Quality Checks
- [ ] `pmat analyze complexity --max-cyclomatic 20 --fail-on-violation`
- [ ] `pmat analyze satd --strict --fail-on-violation` (ZERO tolerance)
- [ ] `pmat analyze dead-code --max-percentage 5.0 --fail-on-violation`
- [ ] `pmat quality-gate --fail-on-violation`
- [ ] `cargo clippy -- -D warnings -W clippy::all -W clippy::pedantic`
- [ ] `cargo fmt -- --check`
- [ ] `cargo test --all-features`
- [ ] `cargo tarpaulin --out Html --output-dir coverage`

### Performance Benchmarks
- [ ] FSM transitions: <1μs
- [ ] Tool execution: <10ms P50, <100ms P99
- [ ] MCP round-trip: <5ms local
- [ ] Memory baseline: <100MB
- [ ] CPU idle: <5%
- [ ] Binary size: <10MB stripped

### Test Coverage Requirements
- [ ] Line coverage: 95%+
- [ ] Branch coverage: 90%+
- [ ] Function coverage: 100% public API
- [ ] Property tests: 72+ across modules
- [ ] Doctests: 200+ examples
- [ ] Unit tests: 350+
- [ ] Integration tests: 50+

## Repository Statistics Target

```
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                           100           2000           4000          15000
Markdown                        25            500              0           2000
YAML                            10             50             100            500
Dockerfile                       5             20              50            100
-------------------------------------------------------------------------------
TOTAL                          140           2570           4150          17600
-------------------------------------------------------------------------------

Test Ratio: 2:1 (tests:implementation)
Documentation: 100% public API
Examples: 20 executable demos
Benchmarks: 50+ criterion benchmarks
```

## Verification Command

```bash
# Final verification before course release
./scripts/verify_course_completeness.sh

# Expected output:
✅ All 18 examples compile and run
✅ All 350+ tests passing
✅ Coverage: 96.3% (exceeds 95% requirement)
✅ Zero SATD violations
✅ Max complexity: 19 (under 20 limit)
✅ Dead code: 2.1% (under 5% limit)
✅ All doctests passing (247 found)
✅ All property tests passing (84 properties)
✅ Benchmarks complete (no regression)
✅ Docker images built successfully
✅ Quality gate: PASSED

Course repository ready for release! 🚀
```