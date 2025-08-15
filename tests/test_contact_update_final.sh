#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Final Contact Update API Test..."

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

# Step 2: Create a test contact
echo ""
echo "📝 Step 2: Creating a test contact..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Final",
    "last_name": "Test",
    "email": "final.test.'$TIMESTAMP'@example.com",
    "company": "Test Company",
    "phone": "+1-555-0001",
    "lead_status": "new"
  }')

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Test comprehensive update
echo ""
echo "📝 Step 3: Testing comprehensive update..."
UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Updated Final",
    "last_name": "Updated Test",
    "email": "updated.final.test.'$TIMESTAMP'@example.com",
    "company": "Updated Test Company",
    "phone": "+1-555-9999",
    "job_title": "Senior Developer",
    "address": "123 Updated Street",
    "city": "Updated City",
    "state": "Updated State",
    "postal_code": "54321",
    "country": "Canada",
    "notes": "These are updated notes",
    "lead_source": "referral",
    "lead_status": "qualified"
  }')

UPDATE_HTTP_STATUS=$(echo "$UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
UPDATE_RESPONSE_BODY=$(echo "$UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Update HTTP Status: $UPDATE_HTTP_STATUS"

if [ "$UPDATE_HTTP_STATUS" = "200" ]; then
    echo "✅ Contact update successful"
    
    # Parse and verify key fields
    UPDATED_FIRST_NAME=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    UPDATED_LAST_NAME=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    UPDATED_EMAIL=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"email":"[^"]*"' | cut -d'"' -f4)
    UPDATED_COMPANY=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    UPDATED_PHONE=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"phone":"[^"]*"' | cut -d'"' -f4)
    UPDATED_JOB_TITLE=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"job_title":"[^"]*"' | cut -d'"' -f4)
    UPDATED_CITY=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"city":"[^"]*"' | cut -d'"' -f4)
    UPDATED_STATE=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"state":"[^"]*"' | cut -d'"' -f4)
    UPDATED_COUNTRY=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"country":"[^"]*"' | cut -d'"' -f4)
    UPDATED_LEAD_STATUS=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"lead_status":"[^"]*"' | cut -d'"' -f4)
    UPDATED_LEAD_SOURCE=$(echo "$UPDATE_RESPONSE_BODY" | grep -o '"lead_source":"[^"]*"' | cut -d'"' -f4)
    
    echo ""
    echo "📋 Verification Results:"
    echo "  First Name: $UPDATED_FIRST_NAME (expected: Updated Final)"
    echo "  Last Name: $UPDATED_LAST_NAME (expected: Updated Test)"
    echo "  Email: $UPDATED_EMAIL (expected: updated.final.test.$TIMESTAMP@example.com)"
    echo "  Company: $UPDATED_COMPANY (expected: Updated Test Company)"
    echo "  Phone: $UPDATED_PHONE (expected: +1-555-9999)"
    echo "  Job Title: $UPDATED_JOB_TITLE (expected: Senior Developer)"
    echo "  City: $UPDATED_CITY (expected: Updated City)"
    echo "  State: $UPDATED_STATE (expected: Updated State)"
    echo "  Country: $UPDATED_COUNTRY (expected: Canada)"
    echo "  Lead Status: $UPDATED_LEAD_STATUS (expected: qualified)"
    echo "  Lead Source: $UPDATED_LEAD_SOURCE (expected: referral)"
    
    # Count successful verifications
    VERIFICATION_COUNT=0
    [ "$UPDATED_FIRST_NAME" = "Updated Final" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_LAST_NAME" = "Updated Test" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_EMAIL" = "updated.final.test.$TIMESTAMP@example.com" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_COMPANY" = "Updated Test Company" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_PHONE" = "+1-555-9999" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_JOB_TITLE" = "Senior Developer" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_CITY" = "Updated City" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_STATE" = "Updated State" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_COUNTRY" = "Canada" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_LEAD_STATUS" = "qualified" ] && ((VERIFICATION_COUNT++))
    [ "$UPDATED_LEAD_SOURCE" = "referral" ] && ((VERIFICATION_COUNT++))
    
    echo ""
    echo "✅ $VERIFICATION_COUNT/11 fields verified correctly"
    
    if [ "$VERIFICATION_COUNT" = "11" ]; then
        echo "🎉 All fields updated successfully!"
    else
        echo "⚠️  Some fields may not have updated correctly"
    fi
else
    echo "❌ Contact update failed with status: $UPDATE_HTTP_STATUS"
    echo "Response: $UPDATE_RESPONSE_BODY"
fi

# Step 4: Cleanup
echo ""
echo "📝 Step 4: Cleaning up..."
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_HTTP_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_HTTP_STATUS" = "204" ]; then
    echo "✅ Test contact cleaned up successfully"
else
    echo "⚠️  Failed to cleanup test contact (status: $DELETE_HTTP_STATUS)"
fi

echo ""
echo "🎉 Final Contact Update API Test Complete!"
