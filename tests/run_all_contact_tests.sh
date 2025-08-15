#!/bin/bash

# Comprehensive Contact API Test Suite
echo "🚀 Running Comprehensive Contact API Test Suite..."
echo "=================================================="

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
    
    if [ -f "$test_script" ]; then
        chmod +x "$test_script"
        if ./"$test_script"; then
            echo "✅ $test_name: PASSED"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo "❌ $test_name: FAILED"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo "❌ $test_name: TEST SCRIPT NOT FOUND ($test_script)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Check if server is running
echo ""
echo "🔍 Checking if server is running..."
SERVER_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health" 2>/dev/null || echo "000")

if [ "$SERVER_CHECK" != "200" ]; then
    echo "❌ Server is not running on $BASE_URL"
    echo "Please start the server with: cargo run"
    exit 1
fi

echo "✅ Server is running on $BASE_URL"

# Run individual test suites
run_test "Basic Contact CRUD Operations" "tests/test_contact_crud.sh"
run_test "Contact PATCH API" "tests/test_contact_patch.sh"
run_test "Contact PATCH Custom Fields" "tests/test_contact_patch_custom_fields.sh"
run_test "PUT vs PATCH Comparison" "tests/test_put_vs_patch_comparison.sh"

# Create a quick validation test if basic CRUD doesn't exist
if [ ! -f "tests/test_contact_crud.sh" ]; then
    echo ""
    echo "📝 Creating basic CRUD validation test..."
    
    cat > tests/test_contact_crud_basic.sh << 'EOF'
#!/bin/bash

BASE_URL="http://127.0.0.1:8081"

echo "🧪 Basic Contact CRUD Validation..."

# Login
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}')

TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "❌ Failed to get authentication token"
    exit 1
fi

echo "✅ Authentication successful"

# Create contact
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "CRUD",
    "last_name": "Test",
    "email": "crud.test.'$TIMESTAMP'@example.com"
  }')

CREATE_STATUS=$(echo "$CREATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CREATE_BODY=$(echo "$CREATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

if [ "$CREATE_STATUS" = "201" ] || [ "$CREATE_STATUS" = "200" ]; then
    echo "✅ Contact creation successful"
    CONTACT_ID=$(echo "$CREATE_BODY" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
else
    echo "❌ Contact creation failed: $CREATE_STATUS"
    exit 1
fi

# Read contact
GET_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

GET_STATUS=$(echo "$GET_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$GET_STATUS" = "200" ]; then
    echo "✅ Contact retrieval successful"
else
    echo "❌ Contact retrieval failed: $GET_STATUS"
    exit 1
fi

# Update contact (PUT)
PUT_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Updated",
    "last_name": "CRUD",
    "email": "updated.crud.test.'$TIMESTAMP'@example.com",
    "lead_status": "contacted"
  }')

PUT_STATUS=$(echo "$PUT_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$PUT_STATUS" = "200" ]; then
    echo "✅ Contact update (PUT) successful"
else
    echo "❌ Contact update (PUT) failed: $PUT_STATUS"
    exit 1
fi

# Patch contact
PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Patched"
  }')

PATCH_STATUS=$(echo "$PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$PATCH_STATUS" = "200" ]; then
    echo "✅ Contact patch successful"
else
    echo "❌ Contact patch failed: $PATCH_STATUS"
    exit 1
fi

# Delete contact
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_STATUS" = "204" ] || [ "$DELETE_STATUS" = "200" ]; then
    echo "✅ Contact deletion successful"
else
    echo "❌ Contact deletion failed: $DELETE_STATUS"
    exit 1
fi

echo "✅ All CRUD operations completed successfully"
EOF

    run_test "Basic Contact CRUD Operations (Generated)" "tests/test_contact_crud_basic.sh"
fi

# Summary
echo ""
echo "=================================================="
echo "🎯 Test Suite Summary"
echo "=================================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo "🎉 All tests passed! Contact API is working perfectly."
    echo ""
    echo "✅ Key Features Verified:"
    echo "  - Contact creation with validation"
    echo "  - Contact retrieval by ID"
    echo "  - Contact updates using PUT (full replacement)"
    echo "  - Contact updates using PATCH (partial updates)"
    echo "  - Contact deletion (soft delete)"
    echo "  - Proper error handling and status codes"
    echo "  - Authentication and authorization"
    echo "  - Field validation and constraints"
    echo ""
    echo "🔧 PATCH API Features:"
    echo "  - Partial updates (only specified fields changed)"
    echo "  - Field preservation (unspecified fields kept)"
    echo "  - Field clearing (empty strings set fields to null)"
    echo "  - Merge semantics for custom fields"
    echo "  - Proper validation and error handling"
    echo ""
    exit 0
else
    echo "❌ Some tests failed. Please review the output above."
    echo ""
    echo "Common issues to check:"
    echo "  - Is the server running on $BASE_URL?"
    echo "  - Are the test user credentials correct?"
    echo "  - Are all required database tables created?"
    echo "  - Are the API endpoints properly configured?"
    echo ""
    exit 1
fi
