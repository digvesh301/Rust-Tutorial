#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Update API Custom Fields..."

# Step 1: Login to get JWT token
echo ""
echo "📝 Step 1: Login to get JWT token..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}')

TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "❌ Failed to get authentication token"
    echo "Login Response: $LOGIN_RESPONSE"
    exit 1
fi

echo "✅ Got token: ${TOKEN:0:20}..."

# Step 2: Create a test contact with initial custom fields
echo ""
echo "📝 Step 2: Creating a test contact with initial custom fields..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "CustomField",
    "last_name": "Test",
    "email": "customfield.test.'$TIMESTAMP'@example.com",
    "custom_fields": {
      "department": "Engineering",
      "priority": "high",
      "budget": "100000"
    }
  }')

echo "Create Response: $CREATE_RESPONSE"

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Get the contact to verify initial custom fields
echo ""
echo "📝 Step 3: Getting contact to verify initial custom fields..."
GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "Initial Contact: $GET_RESPONSE"

# Step 4: Update with new custom fields (should replace existing ones)
echo ""
echo "📝 Step 4: Updating with new custom fields..."
UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "custom_fields": {
      "department": "Sales",
      "priority": "medium",
      "region": "North America",
      "source": "referral"
    }
  }')

UPDATE_HTTP_STATUS=$(echo "$UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
UPDATE_RESPONSE_BODY=$(echo "$UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Update HTTP Status: $UPDATE_HTTP_STATUS"
echo "Update Response: $UPDATE_RESPONSE_BODY"

if [ "$UPDATE_HTTP_STATUS" = "200" ]; then
    echo "✅ Custom fields update successful"
else
    echo "❌ Custom fields update failed with status: $UPDATE_HTTP_STATUS"
fi

# Step 5: Get the contact again to verify custom fields were updated
echo ""
echo "📝 Step 5: Getting contact to verify updated custom fields..."
FINAL_GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "Updated Contact: $FINAL_GET_RESPONSE"

# Step 6: Update contact without custom fields (should not affect existing custom fields)
echo ""
echo "📝 Step 6: Updating contact without custom fields..."
NO_CUSTOM_FIELDS_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "UpdatedName"
  }')

NO_CUSTOM_FIELDS_HTTP_STATUS=$(echo "$NO_CUSTOM_FIELDS_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
NO_CUSTOM_FIELDS_RESPONSE_BODY=$(echo "$NO_CUSTOM_FIELDS_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "No Custom Fields Update HTTP Status: $NO_CUSTOM_FIELDS_HTTP_STATUS"
echo "No Custom Fields Update Response: $NO_CUSTOM_FIELDS_RESPONSE_BODY"

if [ "$NO_CUSTOM_FIELDS_HTTP_STATUS" = "200" ]; then
    echo "✅ Update without custom fields successful"
else
    echo "❌ Update without custom fields failed with status: $NO_CUSTOM_FIELDS_HTTP_STATUS"
fi

# Step 7: Get the contact one more time to verify custom fields are preserved
echo ""
echo "📝 Step 7: Getting contact to verify custom fields are preserved..."
PRESERVED_GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "Preserved Contact: $PRESERVED_GET_RESPONSE"

# Step 8: Clear custom fields by sending empty object
echo ""
echo "📝 Step 8: Clearing custom fields with empty object..."
CLEAR_CUSTOM_FIELDS_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "custom_fields": {}
  }')

CLEAR_HTTP_STATUS=$(echo "$CLEAR_CUSTOM_FIELDS_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CLEAR_RESPONSE_BODY=$(echo "$CLEAR_CUSTOM_FIELDS_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Clear Custom Fields HTTP Status: $CLEAR_HTTP_STATUS"
echo "Clear Custom Fields Response: $CLEAR_RESPONSE_BODY"

if [ "$CLEAR_HTTP_STATUS" = "200" ]; then
    echo "✅ Custom fields clearing successful"
else
    echo "❌ Custom fields clearing failed with status: $CLEAR_HTTP_STATUS"
fi

# Step 9: Cleanup - Delete the test contact
echo ""
echo "📝 Step 9: Cleaning up - deleting test contact..."
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_HTTP_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_HTTP_STATUS" = "204" ]; then
    echo "✅ Test contact cleaned up successfully"
else
    echo "⚠️  Failed to cleanup test contact (status: $DELETE_HTTP_STATUS)"
fi

echo ""
echo "🎉 Contact Update API Custom Fields Test Complete!"
