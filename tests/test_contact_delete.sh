#!/bin/bash

# Test Contact Delete API
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Delete API..."

# Step 1: Login to get token
echo "📝 Step 1: Login to get JWT token..."
TOKEN=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}' | \
  grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo "❌ Failed to get token. Make sure server is running and user exists."
  exit 1
fi

echo "✅ Got token: ${TOKEN:0:20}..."

# Step 2: Create a test contact to delete
echo ""
echo "📝 Step 2: Creating a test contact to delete..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Delete",
    "last_name": "Test",
    "email": "delete.test.'$TIMESTAMP'@example.com",
    "company": "Test Company",
    "phone": "+1-555-0199",
    "job_title": "Test Manager"
  }')

echo "Create Response: $CREATE_RESPONSE"

# Extract contact ID from response
CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
  echo "❌ Failed to create test contact"
  exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Verify contact exists
echo ""
echo "📝 Step 3: Verifying contact exists before deletion..."
GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "Get Response: $GET_RESPONSE"

# Step 4: Delete the contact
echo ""
echo "📝 Step 4: Deleting the contact..."
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

HTTP_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
RESPONSE_BODY=$(echo "$DELETE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Delete HTTP Status: $HTTP_STATUS"
echo "Delete Response Body: $RESPONSE_BODY"

if [ "$HTTP_STATUS" = "204" ]; then
  echo "✅ Contact deleted successfully (HTTP 204 No Content)"
else
  echo "❌ Contact deletion failed with status: $HTTP_STATUS"
fi

# Step 5: Verify contact is no longer accessible
echo ""
echo "📝 Step 5: Verifying contact is no longer accessible..."
VERIFY_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

VERIFY_HTTP_STATUS=$(echo "$VERIFY_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
VERIFY_RESPONSE_BODY=$(echo "$VERIFY_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Verify HTTP Status: $VERIFY_HTTP_STATUS"
echo "Verify Response Body: $VERIFY_RESPONSE_BODY"

if [ "$VERIFY_HTTP_STATUS" = "404" ]; then
  echo "✅ Contact is no longer accessible (HTTP 404 Not Found) - Soft delete working correctly!"
else
  echo "❌ Contact is still accessible - Soft delete may not be working correctly"
fi

# Step 6: Try to delete non-existent contact
echo ""
echo "📝 Step 6: Testing deletion of non-existent contact..."
FAKE_ID="00000000-0000-0000-0000-000000000000"
NONEXISTENT_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/api/contacts/$FAKE_ID" \
  -H "Authorization: Bearer $TOKEN")

NONEXISTENT_HTTP_STATUS=$(echo "$NONEXISTENT_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
NONEXISTENT_RESPONSE_BODY=$(echo "$NONEXISTENT_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Non-existent Contact Delete HTTP Status: $NONEXISTENT_HTTP_STATUS"
echo "Non-existent Contact Delete Response Body: $NONEXISTENT_RESPONSE_BODY"

if [ "$NONEXISTENT_HTTP_STATUS" = "404" ]; then
  echo "✅ Non-existent contact deletion properly returns 404"
else
  echo "❌ Non-existent contact deletion returned unexpected status: $NONEXISTENT_HTTP_STATUS"
fi

echo ""
echo "🎉 Contact Delete API Test Complete!"
