#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Update API..."

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

# Step 2: Create a test contact to update
echo ""
echo "📝 Step 2: Creating a test contact to update..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Update",
    "last_name": "Test",
    "email": "update.test.'$TIMESTAMP'@example.com",
    "company": "Original Company",
    "phone": "+1-555-0100",
    "job_title": "Original Title",
    "city": "Original City",
    "state": "Original State",
    "lead_status": "new",
    "custom_fields": {
      "department": "Engineering",
      "priority": "high"
    }
  }')

echo "Create Response: $CREATE_RESPONSE"

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Test partial update (only some fields)
echo ""
echo "📝 Step 3: Testing partial update (only some fields)..."
PARTIAL_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Updated",
    "company": "Updated Company",
    "lead_status": "contacted"
  }')

PARTIAL_HTTP_STATUS=$(echo "$PARTIAL_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
PARTIAL_RESPONSE_BODY=$(echo "$PARTIAL_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Partial Update HTTP Status: $PARTIAL_HTTP_STATUS"
echo "Partial Update Response: $PARTIAL_RESPONSE_BODY"

if [ "$PARTIAL_HTTP_STATUS" = "200" ]; then
    echo "✅ Partial update successful"
    
    # Verify the changes
    UPDATED_FIRST_NAME=$(echo "$PARTIAL_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    UPDATED_COMPANY=$(echo "$PARTIAL_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    UPDATED_LEAD_STATUS=$(echo "$PARTIAL_RESPONSE_BODY" | grep -o '"lead_status":"[^"]*"' | cut -d'"' -f4)
    UNCHANGED_LAST_NAME=$(echo "$PARTIAL_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    
    if [ "$UPDATED_FIRST_NAME" = "Updated" ] && [ "$UPDATED_COMPANY" = "Updated Company" ] && [ "$UPDATED_LEAD_STATUS" = "contacted" ] && [ "$UNCHANGED_LAST_NAME" = "Test" ]; then
        echo "✅ Partial update fields verified correctly"
    else
        echo "❌ Partial update fields not updated correctly"
        echo "Expected: first_name=Updated, company=Updated Company, lead_status=contacted, last_name=Test"
        echo "Got: first_name=$UPDATED_FIRST_NAME, company=$UPDATED_COMPANY, lead_status=$UPDATED_LEAD_STATUS, last_name=$UNCHANGED_LAST_NAME"
    fi
else
    echo "❌ Partial update failed with status: $PARTIAL_HTTP_STATUS"
fi

# Step 4: Test full update with custom fields
echo ""
echo "📝 Step 4: Testing full update with custom fields..."
FULL_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Fully",
    "last_name": "Updated",
    "email": "fully.updated.'$TIMESTAMP'@example.com",
    "company": "New Company Inc",
    "phone": "+1-555-9999",
    "job_title": "Senior Manager",
    "address": "123 New Street",
    "city": "New City",
    "state": "New State",
    "postal_code": "12345",
    "country": "USA",
    "notes": "Updated contact notes",
    "lead_source": "website",
    "lead_status": "qualified",
    "custom_fields": {
      "department": "Sales",
      "priority": "medium",
      "budget": "50000"
    }
  }')

FULL_HTTP_STATUS=$(echo "$FULL_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
FULL_RESPONSE_BODY=$(echo "$FULL_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Full Update HTTP Status: $FULL_HTTP_STATUS"
echo "Full Update Response: $FULL_RESPONSE_BODY"

if [ "$FULL_HTTP_STATUS" = "200" ]; then
    echo "✅ Full update successful"
    
    # Verify key changes
    FINAL_FIRST_NAME=$(echo "$FULL_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    FINAL_LAST_NAME=$(echo "$FULL_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    FINAL_COMPANY=$(echo "$FULL_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    FINAL_LEAD_STATUS=$(echo "$FULL_RESPONSE_BODY" | grep -o '"lead_status":"[^"]*"' | cut -d'"' -f4)
    
    if [ "$FINAL_FIRST_NAME" = "Fully" ] && [ "$FINAL_LAST_NAME" = "Updated" ] && [ "$FINAL_COMPANY" = "New Company Inc" ] && [ "$FINAL_LEAD_STATUS" = "qualified" ]; then
        echo "✅ Full update fields verified correctly"
    else
        echo "❌ Full update fields not updated correctly"
    fi
else
    echo "❌ Full update failed with status: $FULL_HTTP_STATUS"
fi

# Step 5: Test invalid lead status
echo ""
echo "📝 Step 5: Testing invalid lead status validation..."
INVALID_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "lead_status": "invalid_status"
  }')

INVALID_HTTP_STATUS=$(echo "$INVALID_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
INVALID_RESPONSE_BODY=$(echo "$INVALID_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Invalid Update HTTP Status: $INVALID_HTTP_STATUS"
echo "Invalid Update Response: $INVALID_RESPONSE_BODY"

if [ "$INVALID_HTTP_STATUS" = "400" ]; then
    echo "✅ Invalid lead status properly rejected"
else
    echo "❌ Invalid lead status should return 400, got: $INVALID_HTTP_STATUS"
fi

# Step 6: Test updating non-existent contact
echo ""
echo "📝 Step 6: Testing update of non-existent contact..."
NON_EXISTENT_ID="00000000-0000-0000-0000-000000000000"
NOT_FOUND_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$NON_EXISTENT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"first_name": "Should Fail"}')

NOT_FOUND_HTTP_STATUS=$(echo "$NOT_FOUND_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
NOT_FOUND_RESPONSE_BODY=$(echo "$NOT_FOUND_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Non-existent Contact Update HTTP Status: $NOT_FOUND_HTTP_STATUS"
echo "Non-existent Contact Update Response: $NOT_FOUND_RESPONSE_BODY"

if [ "$NOT_FOUND_HTTP_STATUS" = "404" ]; then
    echo "✅ Non-existent contact update properly returns 404"
else
    echo "❌ Non-existent contact update should return 404, got: $NOT_FOUND_HTTP_STATUS"
fi

# Step 7: Test empty update (no fields provided)
echo ""
echo "📝 Step 7: Testing empty update (no fields provided)..."
EMPTY_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{}')

EMPTY_HTTP_STATUS=$(echo "$EMPTY_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
EMPTY_RESPONSE_BODY=$(echo "$EMPTY_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Empty Update HTTP Status: $EMPTY_HTTP_STATUS"
echo "Empty Update Response: $EMPTY_RESPONSE_BODY"

if [ "$EMPTY_HTTP_STATUS" = "200" ]; then
    echo "✅ Empty update successful (no changes made)"
else
    echo "❌ Empty update failed with status: $EMPTY_HTTP_STATUS"
fi

# Step 8: Cleanup - Delete the test contact
echo ""
echo "📝 Step 8: Cleaning up - deleting test contact..."
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_HTTP_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_HTTP_STATUS" = "204" ]; then
    echo "✅ Test contact cleaned up successfully"
else
    echo "⚠️  Failed to cleanup test contact (status: $DELETE_HTTP_STATUS)"
fi

echo ""
echo "🎉 Contact Update API Test Complete!"
