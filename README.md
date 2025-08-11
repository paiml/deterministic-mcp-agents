# Deterministic MCP Agents Course

[![Master CI](https://github.com/paiml/deterministic-mcp-agents/workflows/Master%20CI/badge.svg)](https://github.com/paiml/deterministic-mcp-agents/actions/workflows/master.yml)
[![Build](https://github.com/paiml/deterministic-mcp-agents/workflows/Build/badge.svg)](https://github.com/paiml/deterministic-mcp-agents/actions/workflows/build.yml)
[![Tests](https://github.com/paiml/deterministic-mcp-agents/workflows/Tests/badge.svg)](https://github.com/paiml/deterministic-mcp-agents/actions/workflows/tests.yml)
[![Quality](https://github.com/paiml/deterministic-mcp-agents/workflows/Quality/badge.svg)](https://github.com/paiml/deterministic-mcp-agents/actions/workflows/quality.yml)
[![Coverage](https://img.shields.io/badge/coverage-80%25+-brightgreen.svg)](https://github.com/paiml/deterministic-mcp-agents/actions)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PMAT](https://img.shields.io/badge/PMAT-ready-blue.svg)](https://github.com/paiml/pmat)
[![Examples](https://img.shields.io/badge/examples-19-green.svg)](./modules)
[![Modules](https://img.shields.io/badge/modules-5-blue.svg)](./modules)

A comprehensive course implementation for building production-ready MCP (Model Context Protocol) agents with deterministic finite state machines in Rust.

## Repository Structure

```
.
├── modules/
│   ├── 01-foundations/    # Core concepts and theory
│   ├── 02-setup/         # Environment and toolchain
│   ├── 03-agents/        # Production agent patterns
│   ├── 04-mcp-server/    # MCP server implementation
│   └── 05-testing/       # Testing and QA strategies
├── final_project/        # Complete production system
├── pmcp/                # Shared MCP library
├── scripts/             # Automation and verification
└── docs/               # Course documentation
```

## Quick Start

### Prerequisites

- Rust 1.75+ (install from [rustup.rs](https://rustup.rs))
- PMAT 0.29.6+ with MCP feature
- Docker (optional, for containerization)

### Installation

```bash
# Clone the repository
git clone https://github.com/paiml/deterministic-mcp-agents-course
cd deterministic-mcp-agents-course

# Install development dependencies
make install-dev

# Build all modules
make build

# Run tests
make test

# Check quality gates
make quality-gate-all
```

## Course Modules

### Module 1: Foundations (18:00)
- Course overview and performance baselines
- Certainty-Scope tradeoff demonstrations
- Floridi Conjecture implementation
- MCP protocol basics

### Module 2: Environment Setup (18:00)
- Rust toolchain configuration
- PMAT installation and validation
- Quality gates implementation
- Calculator agent example

### Module 3: Production Agents (23:00)
- FSM builder pattern
- Refactor state machine
- Code analysis FSM
- Error boundary implementation

### Module 4: MCP Server (22:00)
- PMCP server implementation
- Tool composition patterns
- Async request handling
- Production deployment with Docker/K8s

### Module 5: Testing & QA (09:00)
- Property testing suite
- Fuzzing infrastructure
- Integration tests
- Coverage requirements

## Quality Standards

All code must meet these quality requirements:

- **Complexity**: Max cyclomatic complexity of 20
- **SATD**: Zero tolerance for technical debt markers
- **Dead Code**: Less than 5%
- **Test Coverage**: Minimum 95% line coverage
- **Documentation**: 100% public API documentation

## Examples Documentation

This course includes 19 comprehensive examples demonstrating key concepts in building deterministic MCP agents. Each example is fully executable and includes detailed inline documentation.

**Example Distribution by Module:**
- Module 1 (Foundations): 4 examples
- Module 2 (Setup): 4 examples
- Module 3 (Agents): 4 examples
- Module 4 (MCP Server): 4 examples
- Module 5 (Testing): 3 examples

### Module 1: Foundations

#### [00_course_overview.rs](modules/01-foundations/examples/00_course_overview.rs)
**Course Overview and Introduction**
- Introduces the course structure and learning objectives
- Demonstrates the relationship between certainty and scope in agent design
- Shows basic FSM patterns and MCP protocol concepts
- Run: `cargo run --example 00_course_overview -p module-01-foundations`

#### [01_certainty_scope_demo.rs](modules/01-foundations/examples/01_certainty_scope_demo.rs)
**Certainty-Scope Tradeoff Demonstration**
- Implements the fundamental C(M) × S(M) ≤ k constraint
- Shows how increasing certainty reduces scope and vice versa
- Demonstrates practical applications in agent decision-making
- Includes interactive examples with different complexity levels
- Run: `cargo run --example 01_certainty_scope_demo -p module-01-foundations`

#### [02_floridi_conjecture.rs](modules/01-foundations/examples/02_floridi_conjecture.rs)
**Floridi Conjecture Implementation**
- Implements Floridi's theory of semantic information
- Demonstrates the relationship between data, information, and knowledge
- Shows how agents can verify information truthfulness
- Includes examples of semantic validation in practice
- Run: `cargo run --example 02_floridi_conjecture -p module-01-foundations`

#### [03_mcp_protocol_basics.rs](modules/01-foundations/examples/03_mcp_protocol_basics.rs)
**MCP Protocol Fundamentals**
- Introduction to Model Context Protocol (MCP)
- Demonstrates request/response patterns
- Shows tool registration and capability negotiation
- Includes basic error handling and protocol validation
- Run: `cargo run --example 03_mcp_protocol_basics -p module-01-foundations`

### Module 2: Environment Setup

#### [04_toolchain_setup.rs](modules/02-setup/examples/04_toolchain_setup.rs)
**Rust Toolchain Configuration**
- Validates Rust installation and version requirements
- Checks for required features and extensions
- Demonstrates cargo workspace setup
- Shows how to configure development environment
- Run: `cargo run --example 04_toolchain_setup -p module-02-setup`

#### [05_pmat_validator.rs](modules/02-setup/examples/05_pmat_validator.rs)
**PMAT Installation and Validation**
- Validates PMAT (Performance Monitoring and Analysis Tool) installation
- Checks for MCP feature support
- Demonstrates quality metrics collection
- Shows how to configure PMAT for continuous monitoring
- Run: `cargo run --example 05_pmat_validator -p module-02-setup`

#### [06_quality_gates.rs](modules/02-setup/examples/06_quality_gates.rs)
**Quality Gates Implementation**
- Implements automated quality checks
- Demonstrates complexity analysis (cyclomatic complexity < 20)
- Shows SATD (Self-Admitted Technical Debt) detection
- Includes dead code analysis and coverage requirements
- Run: `cargo run --example 06_quality_gates -p module-02-setup`

#### [07_calculator_agent.rs](modules/02-setup/examples/07_calculator_agent.rs)
**Calculator Agent with FSM**
- Implements a complete calculator as an MCP agent
- Uses finite state machine for operation management
- Demonstrates error handling and state transitions
- Includes overflow protection and operation history
- Run: `cargo run --example 07_calculator_agent -p module-02-setup`

### Module 3: Production Agents

#### [08_fsm_builder.rs](modules/03-agents/examples/08_fsm_builder.rs)
**FSM Builder Pattern**
- Implements a fluent builder pattern for FSM construction
- Shows type-safe state and event definitions
- Demonstrates transition guards and actions
- Includes compile-time FSM validation
- Run: `cargo run --example 08_fsm_builder -p module-03-agents`

#### [09_refactor_fsm.rs](modules/03-agents/examples/09_refactor_fsm.rs)
**Refactoring State Machine**
- Implements an agent for automated code refactoring
- Shows complex state transitions for refactoring workflow
- Demonstrates rollback and error recovery
- Includes AST analysis and transformation patterns
- Run: `cargo run --example 09_refactor_fsm -p module-03-agents`

#### [10_code_analysis_fsm.rs](modules/03-agents/examples/10_code_analysis_fsm.rs)
**Code Analysis FSM**
- Implements static code analysis as a state machine
- Shows pattern detection and metric calculation
- Demonstrates incremental analysis capabilities
- Includes complexity and quality metric reporting
- Run: `cargo run --example 10_code_analysis_fsm -p module-03-agents`

#### [11_error_boundaries.rs](modules/03-agents/examples/11_error_boundaries.rs)
**Error Boundary Implementation**
- Demonstrates robust error handling in FSM agents
- Shows recovery strategies and fallback states
- Implements circuit breaker patterns
- Includes retry logic with exponential backoff
- Run: `cargo run --example 11_error_boundaries -p module-03-agents`

### Module 4: MCP Server

#### [12_pmcp_server.rs](modules/04-mcp-server/examples/12_pmcp_server.rs)
**Production MCP Server**
- Implements a complete MCP server with multiple tools
- Shows request routing and handler registration
- Demonstrates async request processing
- Includes metrics and observability hooks
- Run: `cargo run --example 12_pmcp_server -p module-04-mcp-server`

#### [13_tool_composition.rs](modules/04-mcp-server/examples/13_tool_composition.rs)
**Tool Composition Patterns**
- Demonstrates how to compose multiple tools
- Shows tool chaining and pipeline patterns
- Implements dependency injection for tools
- Includes examples of tool orchestration
- Run: `cargo run --example 13_tool_composition -p module-04-mcp-server`

#### [14_async_handler.rs](modules/04-mcp-server/examples/14_async_handler.rs)
**Async Request Handling**
- Implements high-performance async handlers
- Demonstrates concurrent request processing
- Shows backpressure and rate limiting
- Includes timeout and cancellation handling
- Run: `cargo run --example 14_async_handler -p module-04-mcp-server`

#### [15_production_server.rs](modules/04-mcp-server/examples/15_production_server.rs)
**Production Deployment**
- Complete production-ready MCP server
- Includes Docker containerization setup
- Shows Kubernetes deployment configuration
- Demonstrates health checks and readiness probes
- Run: `cargo run --example 15_production_server -p module-04-mcp-server`

### Module 5: Testing & QA

#### [16_property_tests.rs](modules/05-testing/examples/16_property_tests.rs)
**Property-Based Testing**
- Demonstrates QuickCheck and PropTest frameworks
- Shows how to generate test inputs automatically
- Implements invariant checking
- Includes shrinking strategies for failure minimization
- Run: `cargo run --example 16_property_tests -p module-05-testing`

#### [17_fuzzing.rs](modules/05-testing/examples/17_fuzzing.rs)
**Fuzzing Infrastructure**
- Sets up cargo-fuzz for automated testing
- Demonstrates coverage-guided fuzzing
- Shows how to write effective fuzz targets
- Includes crash reproduction and minimization
- Run: `cargo run --example 17_fuzzing -p module-05-testing`

#### [18_integration_tests.rs](modules/05-testing/examples/18_integration_tests.rs)
**Integration Testing Suite**
- Implements end-to-end testing strategies
- Shows test fixture management
- Demonstrates mock server setup
- Includes performance benchmarking
- Run: `cargo run --example 18_integration_tests -p module-05-testing`

## Running Examples

### Quick Start - Running Individual Examples

Each example can be run in three ways:

#### Method 1: From Project Root (Recommended)
```bash
# Pattern: cargo run --example <example_name> -p <module_package_name>
cargo run --example 00_course_overview -p module-01-foundations
cargo run --example 07_calculator_agent -p module-02-setup
cargo run --example 11_error_boundaries -p module-03-agents
```

#### Method 2: From Module Directory
```bash
# Navigate to the module directory first
cd modules/01-foundations
cargo run --example 00_course_overview

cd ../02-setup
cargo run --example 07_calculator_agent
```

#### Method 3: Using Make Targets
```bash
# Run all examples in a specific module
make run-module-1  # Runs all 4 examples in foundations
make run-module-2  # Runs all 4 examples in setup
make run-module-3  # Runs all 4 examples in agents
make run-module-4  # Runs all 4 examples in mcp-server
make run-module-5  # Runs all 3 examples in testing

# Run all 19 examples across all modules
make run-all-examples
```

### Example Naming Convention
Examples are numbered sequentially from 00-18:
- **00-03**: Module 1 (Foundations)
- **04-07**: Module 2 (Setup)
- **08-11**: Module 3 (Agents)
- **12-15**: Module 4 (MCP Server)
- **16-18**: Module 5 (Testing)

### Package Names for Each Module
When using `cargo run` from the project root, use these package names:
- `module-01-foundations` for Module 1 examples
- `module-02-setup` for Module 2 examples
- `module-03-agents` for Module 3 examples
- `module-04-mcp-server` for Module 4 examples
- `module-05-testing` for Module 5 examples

## Final Project

The final project demonstrates a production-ready MCP server with:

- 5+ production tools
- 8-state FSM with verification
- 95%+ test coverage
- Docker deployment
- <100ms P99 latency
- 1k QPS sustained load

### Running the Server

```bash
# Build and run locally
cargo run --release --bin mcp-server

# Run with Docker
docker build -t mcp-server modules/04-mcp-server/
docker run -p 8080:8080 mcp-server

# Deploy to Kubernetes
kubectl apply -f k8s/
```

## Verification

Verify course completeness:

```bash
./scripts/verify_course_completeness.sh
```

Expected output:
```
✅ All 19 examples compile and run
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

## Performance Benchmarks

- FSM transitions: <1μs
- Tool execution: <10ms P50, <100ms P99
- MCP round-trip: <5ms local
- Memory baseline: <100MB
- CPU idle: <5%
- Binary size: <10MB stripped

## Contributing

Please ensure all contributions meet the quality standards:

```bash
# Before committing
make pre-commit

# Before pushing
make pre-push
```

## License

MIT License - See LICENSE file for details

## Support

For issues and questions:
- GitHub Issues: [github.com/paiml/deterministic-mcp-agents-course/issues](https://github.com/paiml/deterministic-mcp-agents-course/issues)
- Documentation: [docs/](docs/)

## Acknowledgments

Built with:
- Rust and the Cargo ecosystem
- PMAT quality analysis tools
- MCP protocol specifications
- Property-based testing with quickcheck
- Fuzzing with cargo-fuzz
