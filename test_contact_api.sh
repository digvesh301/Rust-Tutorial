#!/bin/bash

# Test script for Contact API
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact API..."
echo "=========================="

# Step 1: Login to get JWT token
echo "1. Logging in to get JWT token..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

echo "Login Response: $LOGIN_RESPONSE"

# Extract token using jq (if available) or basic parsing
if command -v jq &> /dev/null; then
    TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
else
    # Basic token extraction without jq
    TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
fi

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
    echo "❌ Failed to get authentication token"
    echo "Response: $LOGIN_RESPONSE"
    exit 1
fi

echo "✅ Got authentication token: ${TOKEN:0:50}..."
echo ""

# Step 2: Test contacts health endpoint
echo "2. Testing contacts health endpoint..."
HEALTH_RESPONSE=$(curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/contacts/health")
echo "Health Response: $HEALTH_RESPONSE"
echo ""

# Step 3: List existing contacts
echo "3. Listing existing contacts..."
LIST_RESPONSE=$(curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/contacts")
echo "List Response: $LIST_RESPONSE"
echo ""

# Step 4: Create a new contact
echo "4. Creating a new contact..."
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jane",
    "last_name": "Doe",
    "email": "jane.doe@example.com",
    "phone": "+1-555-0199",
    "company": "Example Corp",
    "job_title": "Software Engineer",
    "city": "New York",
    "state": "NY",
    "country": "USA",
    "lead_source": "api_test",
    "lead_status": "new",
    "notes": "Created via API test"
  }')

echo "Create Response: $CREATE_RESPONSE"
echo ""

# Step 5: List contacts again to see the new one
echo "5. Listing contacts again to verify creation..."
LIST_RESPONSE_2=$(curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/contacts")
echo "Updated List Response: $LIST_RESPONSE_2"
echo ""

echo "🎉 Contact API test completed!"
