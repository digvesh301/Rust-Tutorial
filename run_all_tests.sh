#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
BASE_URL="http://127.0.0.1:8081"
TESTS_DIR="./tests"

echo -e "${BLUE}🧪 Running All API Test Cases${NC}"
echo "=================================="
echo ""

# Check if server is running
echo -e "${YELLOW}📡 Checking if server is running...${NC}"
if ! curl -s "$BASE_URL/health" > /dev/null 2>&1; then
    echo -e "${RED}❌ Server is not running at $BASE_URL${NC}"
    echo -e "${YELLOW}💡 Please start the server with: cargo run${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Server is running${NC}"
echo ""

# Initialize test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
FAILED_TEST_NAMES=()

# Function to run a single test
run_test() {
    local test_file="$1"
    local test_name=$(basename "$test_file" .sh)

    echo -e "${BLUE}🔍 Running: $test_name${NC}"
    echo "----------------------------------------"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Run the test and capture output
    if bash "$test_file"; then
        echo -e "${GREEN}✅ PASSED: $test_name${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}❌ FAILED: $test_name${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        FAILED_TEST_NAMES+=("$test_name")
    fi

    echo ""
}

# Find and run all test files
echo -e "${YELLOW}🔍 Discovering test files in $TESTS_DIR...${NC}"

if [ ! -d "$TESTS_DIR" ]; then
    echo -e "${RED}❌ Tests directory not found: $TESTS_DIR${NC}"
    exit 1
fi

# Find all .sh files in tests directory
TEST_FILES=$(find "$TESTS_DIR" -name "*.sh" -type f | sort)

if [ -z "$TEST_FILES" ]; then
    echo -e "${YELLOW}⚠️  No test files found in $TESTS_DIR${NC}"
    exit 0
fi

echo -e "${GREEN}Found $(echo "$TEST_FILES" | wc -l) test file(s)${NC}"
echo ""

# Run each test
for test_file in $TEST_FILES; do
    run_test "$test_file"
done

# Print summary
echo "=========================================="
echo -e "${BLUE}📊 TEST SUMMARY${NC}"
echo "=========================================="
echo -e "Total Tests: ${YELLOW}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -gt 0 ]; then
    echo ""
    echo -e "${RED}❌ Failed Tests:${NC}"
    for failed_test in "${FAILED_TEST_NAMES[@]}"; do
        echo -e "  - ${RED}$failed_test${NC}"
    done
    echo ""
    echo -e "${RED}🚨 Some tests failed!${NC}"
    exit 1
else
    echo ""
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
fi
