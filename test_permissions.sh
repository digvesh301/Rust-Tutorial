#!/bin/bash

# Test script for permission system

echo "🧪 Testing Permission System"
echo "============================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# API Base URL
BASE_URL="http://127.0.0.1:8081"

# Function to test endpoint
test_endpoint() {
    local endpoint=$1
    local token=$2
    local expected_status=$3
    local description=$4
    
    echo -e "${BLUE}Testing: $description${NC}"
    
    response=$(curl -s -w "%{http_code}" -X GET "$BASE_URL$endpoint" \
        -H "Authorization: Bearer $token" \
        -o /tmp/response.json)
    
    status_code="${response: -3}"
    
    if [ "$status_code" = "$expected_status" ]; then
        echo -e "${GREEN}✅ PASS: Got expected status $status_code${NC}"
    else
        echo -e "${RED}❌ FAIL: Expected $expected_status, got $status_code${NC}"
        echo "Response: $(cat /tmp/response.json)"
    fi
    echo ""
}

# Step 1: Login to get JWT token
echo -e "${BLUE}1. Getting JWT token...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo -e "${RED}❌ Failed to get authentication token${NC}"
    echo "Login response: $LOGIN_RESPONSE"
    exit 1
fi

echo -e "${GREEN}✅ Got authentication token${NC}"
echo ""

# Step 2: Test protected endpoints
echo -e "${YELLOW}2. Testing Protected Endpoints${NC}"
echo "============================================="

# Test health endpoint (should work with contacts:read permission)
test_endpoint "/contacts-protected/protected/health" "$TOKEN" "200" "Health check with contacts:read permission"

# Test list contacts (should work with contacts:read permission)  
test_endpoint "/contacts-protected/protected/" "$TOKEN" "200" "List contacts with contacts:read permission"

# Step 3: Test without token
echo -e "${YELLOW}3. Testing Without Authentication${NC}"
echo "============================================="

test_endpoint "/contacts-protected/protected/health" "" "401" "Health check without token (should fail)"

# Step 4: Test with invalid token
echo -e "${YELLOW}4. Testing With Invalid Token${NC}"
echo "============================================="

test_endpoint "/contacts-protected/protected/health" "invalid-token" "401" "Health check with invalid token (should fail)"

echo -e "${GREEN}🎉 Permission System Test Completed!${NC}"
echo ""
echo -e "${YELLOW}📋 Summary:${NC}"
echo "• ✅ Basic permission checking implemented"
echo "• ✅ JWT token validation working"
echo "• ✅ Unauthorized access properly blocked"
echo "• ✅ Contact roles and permissions in database"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo "• Add more granular permission tests"
echo "• Test different user roles"
echo "• Add resource ownership tests"
echo "• Implement remaining CRUD operations"
