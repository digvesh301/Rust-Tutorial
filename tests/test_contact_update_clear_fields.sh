#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Update API - Clearing Fields with Empty Strings..."

# Step 1: Login to get JWT token
echo ""
echo "📝 Step 1: Login to get JWT token..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}')

TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "❌ Failed to get authentication token"
    exit 1
fi

echo "✅ Got token: ${TOKEN:0:20}..."

# Step 2: Create a test contact with all fields populated
echo ""
echo "📝 Step 2: Creating a test contact with all fields populated..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Clear",
    "last_name": "Test",
    "email": "clear.test.'$TIMESTAMP'@example.com",
    "company": "Original Company",
    "phone": "+1-555-0001",
    "job_title": "Original Title",
    "address": "123 Original Street",
    "city": "Original City",
    "state": "Original State",
    "postal_code": "12345",
    "country": "USA",
    "notes": "Original notes",
    "lead_source": "website",
    "lead_status": "new"
  }')

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Update with empty strings to clear optional fields
echo ""
echo "📝 Step 3: Testing update with empty strings to clear optional fields..."
CLEAR_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "phone": "",
    "company": "",
    "job_title": "",
    "address": "",
    "city": "",
    "state": "",
    "postal_code": "",
    "country": "",
    "notes": "",
    "lead_source": ""
  }')

CLEAR_HTTP_STATUS=$(echo "$CLEAR_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CLEAR_RESPONSE_BODY=$(echo "$CLEAR_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Clear Update HTTP Status: $CLEAR_HTTP_STATUS"

if [ "$CLEAR_HTTP_STATUS" = "200" ]; then
    echo "✅ Clear fields update successful"
    
    echo ""
    echo "📋 Checking if optional fields were cleared with empty strings:"
    
    # Check if fields are empty strings or null
    PHONE_VALUE=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"phone":"[^"]*"' | cut -d'"' -f4)
    COMPANY_VALUE=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    JOB_TITLE_VALUE=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"job_title":"[^"]*"' | cut -d'"' -f4)
    NOTES_VALUE=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"notes":"[^"]*"' | cut -d'"' -f4)
    
    echo "  Phone: '$PHONE_VALUE'"
    echo "  Company: '$COMPANY_VALUE'"
    echo "  Job Title: '$JOB_TITLE_VALUE'"
    echo "  Notes: '$NOTES_VALUE'"
    
    # Verify required fields are still present
    FIRST_NAME=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    LAST_NAME=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    EMAIL=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"email":"[^"]*"' | cut -d'"' -f4)
    
    echo ""
    echo "📋 Required fields preserved:"
    echo "  First Name: $FIRST_NAME (should be: Clear)"
    echo "  Last Name: $LAST_NAME (should be: Test)"
    echo "  Email: $EMAIL (should be: clear.test.$TIMESTAMP@example.com)"
    
    if [ "$FIRST_NAME" = "Clear" ] && [ "$LAST_NAME" = "Test" ] && [ "$EMAIL" = "clear.test.$TIMESTAMP@example.com" ]; then
        echo "  ✅ Required fields preserved correctly"
    else
        echo "  ❌ Required fields not preserved correctly"
    fi
    
else
    echo "❌ Clear fields update failed with status: $CLEAR_HTTP_STATUS"
    echo "Response: $CLEAR_RESPONSE_BODY"
fi

# Step 4: Test updating only one field to ensure others remain unchanged
echo ""
echo "📝 Step 4: Testing single field update (others should remain unchanged)..."
SINGLE_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "lead_status": "contacted"
  }')

SINGLE_HTTP_STATUS=$(echo "$SINGLE_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
SINGLE_RESPONSE_BODY=$(echo "$SINGLE_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Single Update HTTP Status: $SINGLE_HTTP_STATUS"

if [ "$SINGLE_HTTP_STATUS" = "200" ]; then
    echo "✅ Single field update successful"
    
    # Verify lead status changed but other fields remained the same
    FINAL_LEAD_STATUS=$(echo "$SINGLE_RESPONSE_BODY" | grep -o '"lead_status":"[^"]*"' | cut -d'"' -f4)
    FINAL_FIRST_NAME=$(echo "$SINGLE_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    
    echo "  Lead Status: $FINAL_LEAD_STATUS (should be: contacted)"
    echo "  First Name: $FINAL_FIRST_NAME (should be: Clear)"
    
    if [ "$FINAL_LEAD_STATUS" = "contacted" ] && [ "$FINAL_FIRST_NAME" = "Clear" ]; then
        echo "  ✅ Single field update verified correctly"
    else
        echo "  ❌ Single field update not working correctly"
    fi
else
    echo "❌ Single field update failed with status: $SINGLE_HTTP_STATUS"
fi

# Step 5: Cleanup
echo ""
echo "📝 Step 5: Cleaning up..."
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_HTTP_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_HTTP_STATUS" = "204" ]; then
    echo "✅ Test contact cleaned up successfully"
else
    echo "⚠️  Failed to cleanup test contact (status: $DELETE_HTTP_STATUS)"
fi

echo ""
echo "🎉 Contact Update API Clear Fields Test Complete!"
