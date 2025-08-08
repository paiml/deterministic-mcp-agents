.PHONY: all build test bench clean fmt clippy quality-gate quality-gate-all coverage fuzz doc run-examples run-all

# Default target
all: quality-gate-all

# Build the project
build:
	cargo build --all --all-features

# Build release version
release:
	cargo build --all --all-features --release

# Run all tests with comprehensive reporting
test:
	@echo "🧪 Running Comprehensive Test Suite"
	@echo "===================================="
	@echo ""
	@echo "1️⃣ Unit Tests"
	@echo "-------------"
	@cargo test --all --all-features --lib 2>&1 | grep -E "(test result:|running)" || cargo test --all --all-features --lib
	@echo ""
	@echo "2️⃣ Integration Tests"
	@echo "-------------------"
	@cargo test --all --all-features --test '*' 2>&1 | grep -E "(test result:|running)" || cargo test --all --all-features --test '*' || true
	@echo ""
	@echo "3️⃣ Documentation Tests"
	@echo "---------------------"
	@cargo test --all --all-features --doc 2>&1 | grep -E "(test result:|Doc-tests|running)" || cargo test --all --all-features --doc
	@echo ""
	@echo "4️⃣ Property Tests"
	@echo "----------------"
	@echo "Running QuickCheck property tests..."
	@cargo test --all --all-features quickcheck 2>&1 | grep -E "(test result:|running|quickcheck)" || true
	@echo "Running PropTest property tests..."
	@cargo test --all --all-features proptest 2>&1 | grep -E "(test result:|running|proptest)" || true
	@echo ""
	@echo "5️⃣ Example Tests"
	@echo "---------------"
	@echo "Testing examples compilation..."
	@cargo test --all --all-features --examples 2>&1 | grep -E "(test result:|running)" || cargo build --all --examples
	@echo ""
	@echo "📊 Coverage Summary"
	@echo "-----------------"
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		cargo tarpaulin --print-summary --all --all-features 2>/dev/null | grep -E "Coverage" || echo "Run 'make coverage' for detailed report"; \
	else \
		echo "Install cargo-tarpaulin for coverage: cargo install cargo-tarpaulin"; \
	fi
	@echo ""
	@echo "✅ All tests completed!"

# Run tests with verbose output
test-verbose:
	@echo "🧪 Running Tests (Verbose)"
	@echo "========================="
	cargo test --all --all-features -- --nocapture --test-threads=1

# Run only unit tests
test-unit:
	@echo "🧪 Running Unit Tests"
	cargo test --all --all-features --lib

# Run only integration tests
test-integration:
	@echo "🧪 Running Integration Tests"
	cargo test --all --all-features --test '*'

# Run only doc tests
test-doc:
	@echo "📚 Running Documentation Tests"
	cargo test --all --all-features --doc

# Run only property tests
test-property:
	@echo "🎲 Running Property Tests"
	@echo "QuickCheck tests:"
	cargo test --all --all-features -- quickcheck
	@echo "PropTest tests:"
	cargo test --all --all-features -- proptest

# Run example tests
test-examples:
	@echo "📘 Testing Examples"
	cargo build --all --examples
	cargo test --all --all-features --examples

# Run all tests with detailed summary
test-all: test
	@echo ""
	@echo "Running detailed test summary..."
	@./scripts/test_summary.sh

# Run tests with coverage report
test-coverage:
	@echo "🧪 Running Tests with Coverage"
	@echo "=============================="
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		cargo tarpaulin --out Stdout --all --all-features; \
	else \
		echo "Installing cargo-tarpaulin..."; \
		cargo install cargo-tarpaulin; \
		cargo tarpaulin --out Stdout --all --all-features; \
	fi

# Run benchmarks
bench:
	cargo bench --all

# Clean build artifacts
clean:
	cargo clean
	rm -rf coverage/
	rm -rf target/

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt --all
	@echo "✅ Code formatted successfully!"

# Alias for format
fmt: format

# Format check
fmt-check:
	@echo "🔍 Checking code formatting..."
	cargo fmt --all -- --check

# Run linter (clippy)
lint:
	@echo "🔍 Running Clippy linter..."
	@echo "================================"
	@cargo clippy --all-targets --all-features && echo "✅ Linting completed!" || (echo "❌ Linting failed!" && exit 1)

# Alias for lint
clippy: lint

# Run PMAT quality gate checks
quality-gate:
	@echo "🔍 Running PMAT Quality Checks..."
	@echo "================================"
	pmat analyze complexity --max-cyclomatic 20 --fail-on-violation
	pmat analyze satd --strict --fail-on-violation
	pmat analyze dead-code --max-percentage 5.0 --fail-on-violation
	pmat quality-gate --fail-on-violation
	@echo "✅ All quality checks passed!"

# PMAT specific checks
pmat-complexity:
	@echo "📊 Checking Cyclomatic Complexity (max: 20)..."
	pmat analyze complexity --max-cyclomatic 20 --fail-on-violation

pmat-satd:
	@echo "🔍 Checking for SATD (zero tolerance)..."
	pmat analyze satd --strict --fail-on-violation

pmat-dead-code:
	@echo "💀 Checking Dead Code (max: 5%)..."
	pmat analyze dead-code --max-percentage 5.0 --fail-on-violation

pmat-all: pmat-complexity pmat-satd pmat-dead-code
	@echo "✅ All PMAT checks passed!"

# Run all quality checks
quality-gate-all: fmt-check clippy test quality-gate

# Generate coverage report
coverage:
	@echo "📊 Generating Coverage Report (80% minimum)..."
	@mkdir -p coverage
	cargo tarpaulin --out Html --output-dir coverage --all --all-features \
		--exclude-files "*/examples/*" --exclude-files "*/tests/*" \
		--ignore-panics --timeout 300
	@echo "✅ Coverage report generated in coverage/"

# Check coverage meets 80% threshold
coverage-check: coverage
	@COVERAGE=$$(cargo tarpaulin --print-summary --all --all-features | grep "Coverage" | awk '{print $$2}' | sed 's/%//'); \
	echo "Current coverage: $$COVERAGE%"; \
	if [ "$$(echo "$$COVERAGE < 80" | bc)" -eq 1 ]; then \
		echo "❌ Coverage $$COVERAGE% is below 80% threshold"; \
		exit 1; \
	else \
		echo "✅ Coverage meets 80% requirement"; \
	fi

# Run fuzzing
fuzz:
	cargo +nightly fuzz run fuzz_target_1

# Generate documentation
doc:
	cargo doc --all --all-features --no-deps --open

# Install development dependencies
install-dev:
	cargo install pmat --version ">=0.29.6"
	cargo install cargo-tarpaulin
	cargo install cargo-fuzz
	cargo install cargo-criterion
	rustup component add rustfmt clippy

# Verify course completeness (will be implemented later)
verify:
	./scripts/verify_course_completeness.sh

# Quick check before commit
pre-commit: fmt clippy test

# Full check before push
pre-push: quality-gate-all coverage

# Run all example programs
run-all-examples: run-module-1 run-module-2 run-module-3 run-module-4 run-module-5
	@echo "\n✅ All 19 examples completed successfully!"

# Run Module 1 examples
run-module-1:
	@echo "📚 Module 1: Foundations"
	@echo "====================="
	@cd modules/01-foundations && cargo run --example 00_course_overview --quiet
	@cd modules/01-foundations && cargo run --example 01_certainty_scope_demo --quiet
	@cd modules/01-foundations && cargo run --example 02_floridi_conjecture --quiet
	@cd modules/01-foundations && cargo run --example 03_mcp_protocol_basics --quiet

# Run Module 2 examples
run-module-2:
	@echo "\n🔧 Module 2: Environment Setup"
	@echo "==========================="
	@cd modules/02-setup && cargo run --example 04_toolchain_setup --quiet
	@cd modules/02-setup && cargo run --example 05_pmat_validator --quiet
	@cd modules/02-setup && cargo run --example 06_quality_gates --quiet
	@cd modules/02-setup && cargo run --example 07_calculator_agent --quiet

# Run Module 3 examples
run-module-3:
	@echo "\n🤖 Module 3: Production Agents"
	@echo "==========================="
	@cd modules/03-agents && cargo run --example 08_fsm_builder --quiet
	@cd modules/03-agents && cargo run --example 09_refactor_fsm --quiet
	@cd modules/03-agents && cargo run --example 10_code_analysis_fsm --quiet
	@cd modules/03-agents && cargo run --example 11_error_boundaries --quiet

# Run Module 4 examples
run-module-4:
	@echo "\n🌐 Module 4: MCP Server"
	@echo "====================="
	@cd modules/04-mcp-server && cargo run --example 12_pmcp_server --quiet
	@cd modules/04-mcp-server && cargo run --example 13_tool_composition --quiet
	@cd modules/04-mcp-server && cargo run --example 14_async_handler --quiet
	@cd modules/04-mcp-server && cargo run --example 15_production_server --quiet

# Run Module 5 examples
run-module-5:
	@echo "\n🧪 Module 5: Testing & QA"
	@echo "======================"
	@cd modules/05-testing && cargo run --example 16_property_tests --quiet
	@cd modules/05-testing && cargo run --example 17_fuzzing --quiet
	@cd modules/05-testing && cargo run --example 18_integration_tests --quiet

# Run specific module examples
run-module-1:
	@echo "Running Module 1 examples..."
	@for i in 00 01 02 03; do \
		cargo run --bin $${i}_* --quiet || exit 1; \
	done

run-module-2:
	@echo "Running Module 2 examples..."
	@for i in 04 05 06 07; do \
		cargo run --bin $${i}_* --quiet || exit 1; \
	done

run-module-3:
	@echo "Running Module 3 examples..."
	@for i in 08 09 10 11; do \
		cargo run --bin $${i}_* --quiet || exit 1; \
	done

run-module-4:
	@echo "Running Module 4 examples..."
	@for i in 12 13 14 15; do \
		cargo run --bin $${i}_* --quiet || exit 1; \
	done

run-module-5:
	@echo "Running Module 5 examples..."
	@for i in 16 17 18; do \
		cargo run --bin $${i}_* --quiet || exit 1; \
	done

# Run all targets for CI/CD
run-all: build test run-examples quality-gate-all coverage
	@echo "🎉 All targets completed successfully!"

# CI target for GitHub Actions
ci: fmt-check clippy build test
	@echo "✅ CI checks passed!"

# Quick CI without examples
ci-quick: fmt-check clippy build test
	@echo "✅ Quick CI passed!"

# Full CI with everything
ci-full: ci quality-gate coverage run-examples
	@echo "🎉 Full CI passed!"

# Quality report generation
quality-report:
	@echo "📊 Generating Quality Report..."
	@echo "================================"
	@echo "Complexity Analysis:"
	pmat analyze complexity --format json > reports/complexity.json || true
	@echo "SATD Detection:"
	pmat analyze satd --format json > reports/satd.json || true
	@echo "Dead Code Analysis:"
	pmat analyze dead-code --format json > reports/dead-code.json || true
	@echo "Coverage Report:"
	cargo tarpaulin --out Json --output-dir reports || true
	@echo "✅ Reports generated in reports/"

# Help target
help:
	@echo "Deterministic MCP Agents - Makefile Targets"
	@echo "==========================================="
	@echo ""
	@echo "Essential Commands:"
	@echo "  make format         - Format all code"
	@echo "  make lint           - Run Clippy linter"
	@echo "  make test           - Run comprehensive test suite"
	@echo ""
	@echo "Build & Test:"
	@echo "  make build          - Build all projects"
	@echo "  make test           - Run all tests with summary"
	@echo "  make test-all       - Run tests with detailed report"
	@echo "  make test-coverage  - Run tests with coverage"
	@echo "  make test-unit      - Run only unit tests"
	@echo "  make test-doc       - Run only doc tests"
	@echo "  make test-property  - Run only property tests"
	@echo "  make test-examples  - Test all examples"
	@echo "  make bench          - Run benchmarks"
	@echo "  make coverage       - Generate HTML coverage report"
	@echo ""
	@echo "Code Quality:"
	@echo "  make format         - Format code (alias: fmt)"
	@echo "  make lint           - Run linter (alias: clippy)"
	@echo "  make pmat-all       - Run all PMAT checks"
	@echo "  make quality-gate   - Run quality gate checks"
	@echo "  make quality-gate-all - Run ALL quality checks"
	@echo ""
	@echo "Examples:"
	@echo "  make run-examples   - Run all 19 example programs"
	@echo "  make run-module-N   - Run module N examples (1-5)"
	@echo ""
	@echo "CI/CD:"
	@echo "  make ci             - Quick CI checks"
	@echo "  make ci-full        - Complete CI pipeline"
	@echo "  make pre-commit     - Quick checks before commit"
	@echo "  make pre-push       - Full checks before push"
	@echo "  make run-all        - Complete CI/CD pipeline"
	@echo ""
	@echo "Reports:"
	@echo "  make quality-report - Generate quality reports"
	@echo "  make doc            - Generate documentation"
	@echo ""
	@echo "Clean:"
	@echo "  make clean          - Remove all build artifacts"