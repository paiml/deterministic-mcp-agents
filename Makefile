.PHONY: all build test bench clean fmt clippy quality-gate quality-gate-all coverage fuzz doc run-examples run-all

# Default target
all: quality-gate-all

# Build the project
build:
	cargo build --all --all-features

# Build release version
release:
	cargo build --all --all-features --release

# Run all tests
test:
	cargo test --all --all-features

# Run benchmarks
bench:
	cargo bench --all

# Clean build artifacts
clean:
	cargo clean
	rm -rf coverage/
	rm -rf target/

# Format code
fmt:
	cargo fmt --all

# Format check
fmt-check:
	cargo fmt --all -- --check

# Run clippy linter
clippy:
	cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::pedantic

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
run-examples:
	@echo "🚀 Running All Example Programs"
	@echo "==============================="
	@echo "\n📚 Module 1: Foundations"
	cargo run --bin 00_course_overview --quiet
	cargo run --bin 01_certainty_scope_demo --quiet
	cargo run --bin 02_floridi_conjecture --quiet
	cargo run --bin 03_mcp_protocol_basics --quiet
	@echo "\n🔧 Module 2: Environment Setup"
	cargo run --bin 04_toolchain_setup --quiet
	cargo run --bin 05_pmat_validator --quiet
	cargo run --bin 06_quality_gates --quiet
	cargo run --bin 07_calculator_agent --quiet
	@echo "\n🤖 Module 3: Production Agents"
	cargo run --bin 08_fsm_builder --quiet
	cargo run --bin 09_refactor_fsm --quiet
	cargo run --bin 10_code_analysis_fsm --quiet
	cargo run --bin 11_error_boundaries --quiet
	@echo "\n🌐 Module 4: MCP Server"
	cargo run --bin 12_pmcp_server --quiet
	cargo run --bin 13_tool_composition --quiet
	cargo run --bin 14_async_handler --quiet
	cargo run --bin 15_production_server --quiet
	@echo "\n🧪 Module 5: Testing & QA"
	cargo run --bin 16_property_tests --quiet
	cargo run --bin 17_fuzzing --quiet
	cargo run --bin 18_integration_tests --quiet
	@echo "\n✅ All examples executed successfully!"

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
	@echo "Build & Test:"
	@echo "  make build          - Build all projects"
	@echo "  make test           - Run all tests"
	@echo "  make bench          - Run benchmarks"
	@echo "  make coverage       - Generate coverage report"
	@echo ""
	@echo "Quality Checks:"
	@echo "  make fmt            - Format code"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make pmat-all       - Run all PMAT checks"
	@echo "  make quality-gate   - Run quality gate checks"
	@echo "  make quality-gate-all - Run ALL quality checks"
	@echo ""
	@echo "Examples:"
	@echo "  make run-examples   - Run all example programs"
	@echo "  make run-module-N   - Run module N examples (1-5)"
	@echo ""
	@echo "CI/CD:"
	@echo "  make pre-commit     - Quick checks before commit"
	@echo "  make pre-push       - Full checks before push"
	@echo "  make run-all        - Complete CI/CD pipeline"
	@echo ""
	@echo "Reports:"
	@echo "  make quality-report - Generate quality reports"
	@echo "  make doc            - Generate documentation"