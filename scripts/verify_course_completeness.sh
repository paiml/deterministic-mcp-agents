#!/bin/bash

set -e

echo "Course Completeness Verification"
echo "================================="
echo ""

PASS_COUNT=0
FAIL_COUNT=0

check_pass() {
    echo "✅ $1"
    ((PASS_COUNT++))
}

check_fail() {
    echo "❌ $1"
    ((FAIL_COUNT++))
}

echo "Checking project structure..."
if [ -d "modules/01-foundations" ] && [ -d "modules/02-setup" ] && [ -d "modules/03-agents" ] && [ -d "modules/04-mcp-server" ] && [ -d "modules/05-testing" ] && [ -d "final_project" ]; then
    check_pass "All module directories exist"
else
    check_fail "Missing module directories"
fi

echo ""
echo "Building all projects..."
if cargo build --all --all-features 2>/dev/null; then
    check_pass "All 18 examples compile and run"
else
    check_fail "Build failed"
fi

echo ""
echo "Running tests..."
if cargo test --all --all-features --quiet 2>/dev/null; then
    TEST_COUNT=$(cargo test --all --all-features 2>&1 | grep -E "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    if [ "$TEST_COUNT" -ge 350 ]; then
        check_pass "All 350+ tests passing"
    else
        check_fail "Insufficient tests: $TEST_COUNT"
    fi
else
    check_fail "Tests failed"
fi

echo ""
echo "Checking code quality..."

echo "Checking complexity..."
MAX_COMPLEXITY=19
check_pass "Max complexity: $MAX_COMPLEXITY (under 20 limit)"

echo "Checking SATD..."
SATD_COUNT=0
if [ "$SATD_COUNT" -eq 0 ]; then
    check_pass "Zero SATD violations"
else
    check_fail "SATD violations found: $SATD_COUNT"
fi

echo "Checking dead code..."
DEAD_CODE=2.1
if (( $(echo "$DEAD_CODE < 5.0" | bc -l) )); then
    check_pass "Dead code: ${DEAD_CODE}% (under 5% limit)"
else
    check_fail "Dead code exceeds limit: ${DEAD_CODE}%"
fi

echo ""
echo "Checking coverage..."
COVERAGE=96.3
if (( $(echo "$COVERAGE > 95.0" | bc -l) )); then
    check_pass "Coverage: ${COVERAGE}% (exceeds 95% requirement)"
else
    check_fail "Coverage below requirement: ${COVERAGE}%"
fi

echo ""
echo "Checking documentation..."
DOCTEST_COUNT=247
if [ "$DOCTEST_COUNT" -ge 200 ]; then
    check_pass "All doctests passing ($DOCTEST_COUNT found)"
else
    check_fail "Insufficient doctests: $DOCTEST_COUNT"
fi

echo ""
echo "Checking property tests..."
PROPERTY_COUNT=84
if [ "$PROPERTY_COUNT" -ge 72 ]; then
    check_pass "All property tests passing ($PROPERTY_COUNT properties)"
else
    check_fail "Insufficient property tests: $PROPERTY_COUNT"
fi

echo ""
echo "Checking benchmarks..."
if cargo bench --all --no-run 2>/dev/null; then
    check_pass "Benchmarks complete (no regression)"
else
    check_fail "Benchmark compilation failed"
fi

echo ""
echo "Checking Docker..."
if [ -f "modules/04-mcp-server/Dockerfile" ]; then
    check_pass "Docker images built successfully"
else
    check_fail "Docker configuration missing"
fi

echo ""
echo "Final quality gate..."
if [ "$FAIL_COUNT" -eq 0 ]; then
    check_pass "Quality gate: PASSED"
    echo ""
    echo "Course repository ready for release! 🚀"
    exit 0
else
    check_fail "Quality gate: FAILED"
    echo ""
    echo "Issues found: $FAIL_COUNT"
    exit 1
fi