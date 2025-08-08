# Deterministic MCP Agents Course

[![PMAT Quality Gate](https://github.com/paiml/deterministic-mcp-agents-course/workflows/PMAT%20Quality%20Gate/badge.svg)](https://github.com/paiml/deterministic-mcp-agents-course/actions)
[![Coverage](https://img.shields.io/badge/coverage-80%25+-brightgreen.svg)](https://github.com/paiml/deterministic-mcp-agents-course/actions)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PMAT](https://img.shields.io/badge/PMAT-0.29.6%2B-blue.svg)](https://github.com/paiml/pmat)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://hub.docker.com)
[![Examples](https://img.shields.io/badge/examples-19-green.svg)](./modules)

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

## Running Examples

Each module contains executable examples:

```bash
# Run Module 1 examples
cargo run --bin 00_course_overview
cargo run --bin 01_certainty_scope_demo
cargo run --bin 02_floridi_conjecture
cargo run --bin 03_mcp_protocol_basics

# Run Module 2 examples
cargo run --bin 04_toolchain_setup
cargo run --bin 05_pmat_validator
cargo run --bin 06_quality_gates
cargo run --bin 07_calculator_agent
```

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
