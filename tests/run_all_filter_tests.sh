#!/bin/bash

echo "🚀 Running All Contact Filter API Tests..."
echo "========================================"

# Test configuration
BASE_URL="http://127.0.0.1:8081"
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_script="$2"
    
    echo ""
    echo "🧪 Running: $test_name"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if bash "$test_script"; then
        echo "✅ $test_name: PASSED"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "❌ $test_name: FAILED"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Check if server is running
echo "🔍 Checking if server is running..."
SERVER_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health" || echo "000")

if [ "$SERVER_CHECK" != "200" ]; then
    echo "❌ Server is not running or not accessible at $BASE_URL"
    echo "Please start the server with: cargo run"
    exit 1
fi

echo "✅ Server is running and accessible"

# Run all filter tests
run_test "Basic Filter Functionality" "tests/test_contact_filter_basic.sh"
run_test "Comprehensive Standard Field Tests" "tests/test_contact_filter_comprehensive.sh"

# Test Summary
echo ""
echo "🎯 Test Results Summary"
echo "========================================"
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"

if [ $FAILED_TESTS -eq 0 ]; then
    echo ""
    echo "🎉 All tests passed! Contact Filter API is working correctly."
    echo ""
    echo "✅ Verified Features:"
    echo "   • Basic filtering (equals, contains, in operators)"
    echo "   • Complex nested filters (AND/OR groups)"
    echo "   • Pagination with proper metadata"
    echo "   • Filter fields endpoint"
    echo "   • Filter validation endpoint"
    echo "   • Performance monitoring and execution time tracking"
    echo "   • Proper error handling for invalid filters"
    echo "   • Authentication and authorization"
    echo ""
    exit 0
else
    echo ""
    echo "⚠️  Some tests failed. Please review the output above."
    echo ""
    exit 1
fi
