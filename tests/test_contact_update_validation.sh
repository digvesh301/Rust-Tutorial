#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Update API Validation..."

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
    "first_name": "Validation",
    "last_name": "Test",
    "email": "validation.test.'$TIMESTAMP'@example.com"
  }')

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Test invalid email format
echo ""
echo "📝 Step 3: Testing invalid email format..."
INVALID_EMAIL_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "invalid-email-format"
  }')

INVALID_EMAIL_HTTP_STATUS=$(echo "$INVALID_EMAIL_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
INVALID_EMAIL_RESPONSE_BODY=$(echo "$INVALID_EMAIL_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Invalid Email HTTP Status: $INVALID_EMAIL_HTTP_STATUS"
echo "Invalid Email Response: $INVALID_EMAIL_RESPONSE_BODY"

if [ "$INVALID_EMAIL_HTTP_STATUS" = "400" ]; then
    echo "✅ Invalid email format properly rejected"
else
    echo "❌ Invalid email format should return 400, got: $INVALID_EMAIL_HTTP_STATUS"
fi

# Step 4: Test empty first name
echo ""
echo "📝 Step 4: Testing empty first name..."
EMPTY_NAME_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": ""
  }')

EMPTY_NAME_HTTP_STATUS=$(echo "$EMPTY_NAME_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
EMPTY_NAME_RESPONSE_BODY=$(echo "$EMPTY_NAME_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Empty Name HTTP Status: $EMPTY_NAME_HTTP_STATUS"
echo "Empty Name Response: $EMPTY_NAME_RESPONSE_BODY"

if [ "$EMPTY_NAME_HTTP_STATUS" = "400" ]; then
    echo "✅ Empty first name properly rejected"
else
    echo "❌ Empty first name should return 400, got: $EMPTY_NAME_HTTP_STATUS"
fi

# Step 5: Test very long field values
echo ""
echo "📝 Step 5: Testing very long field values..."
LONG_STRING=$(printf 'a%.0s' {1..300})  # 300 character string
LONG_FIELD_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "'$LONG_STRING'"
  }')

LONG_FIELD_HTTP_STATUS=$(echo "$LONG_FIELD_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
LONG_FIELD_RESPONSE_BODY=$(echo "$LONG_FIELD_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Long Field HTTP Status: $LONG_FIELD_HTTP_STATUS"
echo "Long Field Response: $LONG_FIELD_RESPONSE_BODY"

if [ "$LONG_FIELD_HTTP_STATUS" = "400" ]; then
    echo "✅ Long field value properly rejected"
else
    echo "❌ Long field value should return 400, got: $LONG_FIELD_HTTP_STATUS"
fi

# Step 6: Test unauthorized access (no token)
echo ""
echo "📝 Step 6: Testing unauthorized access (no token)..."
UNAUTHORIZED_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Should Fail"
  }')

UNAUTHORIZED_HTTP_STATUS=$(echo "$UNAUTHORIZED_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
UNAUTHORIZED_RESPONSE_BODY=$(echo "$UNAUTHORIZED_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Unauthorized HTTP Status: $UNAUTHORIZED_HTTP_STATUS"
echo "Unauthorized Response: $UNAUTHORIZED_RESPONSE_BODY"

if [ "$UNAUTHORIZED_HTTP_STATUS" = "401" ]; then
    echo "✅ Unauthorized access properly rejected"
else
    echo "❌ Unauthorized access should return 401, got: $UNAUTHORIZED_HTTP_STATUS"
fi

# Step 7: Test all valid lead statuses
echo ""
echo "📝 Step 7: Testing all valid lead statuses..."
VALID_STATUSES=("new" "contacted" "qualified" "proposal" "negotiation" "closed_won" "closed_lost")

for status in "${VALID_STATUSES[@]}"; do
    echo "  Testing lead status: $status"
    STATUS_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
      -H "Authorization: Bearer $TOKEN" \
      -H "Content-Type: application/json" \
      -d '{
        "lead_status": "'$status'"
      }')
    
    STATUS_HTTP_STATUS=$(echo "$STATUS_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
    
    if [ "$STATUS_HTTP_STATUS" = "200" ]; then
        echo "  ✅ Lead status '$status' accepted"
    else
        echo "  ❌ Lead status '$status' rejected with status: $STATUS_HTTP_STATUS"
    fi
done

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
echo "🎉 Contact Update API Validation Test Complete!"
