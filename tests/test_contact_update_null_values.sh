#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Update API with Null Values..."

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
    "first_name": "Null",
    "last_name": "Test",
    "email": "null.test.'$TIMESTAMP'@example.com",
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

# Step 3: Update with null values to clear optional fields
echo ""
echo "📝 Step 3: Testing update with null values to clear optional fields..."
NULL_UPDATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "phone": null,
    "company": null,
    "job_title": null,
    "address": null,
    "city": null,
    "state": null,
    "postal_code": null,
    "country": null,
    "notes": null,
    "lead_source": null
  }')

NULL_HTTP_STATUS=$(echo "$NULL_UPDATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
NULL_RESPONSE_BODY=$(echo "$NULL_UPDATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Null Update HTTP Status: $NULL_HTTP_STATUS"

if [ "$NULL_HTTP_STATUS" = "200" ]; then
    echo "✅ Null values update successful"
    
    # Verify that optional fields are now null
    echo ""
    echo "📋 Checking if optional fields were cleared:"
    
    # Check if phone is null
    if echo "$NULL_RESPONSE_BODY" | grep -q '"phone":null'; then
        echo "  ✅ Phone cleared to null"
    else
        echo "  ❌ Phone not cleared to null"
    fi
    
    # Check if company is null
    if echo "$NULL_RESPONSE_BODY" | grep -q '"company":null'; then
        echo "  ✅ Company cleared to null"
    else
        echo "  ❌ Company not cleared to null"
    fi
    
    # Check if job_title is null
    if echo "$NULL_RESPONSE_BODY" | grep -q '"job_title":null'; then
        echo "  ✅ Job title cleared to null"
    else
        echo "  ❌ Job title not cleared to null"
    fi
    
    # Check if notes is null
    if echo "$NULL_RESPONSE_BODY" | grep -q '"notes":null'; then
        echo "  ✅ Notes cleared to null"
    else
        echo "  ❌ Notes not cleared to null"
    fi
    
    # Verify required fields are still present
    FIRST_NAME=$(echo "$NULL_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    LAST_NAME=$(echo "$NULL_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    EMAIL=$(echo "$NULL_RESPONSE_BODY" | grep -o '"email":"[^"]*"' | cut -d'"' -f4)
    
    echo ""
    echo "📋 Required fields preserved:"
    echo "  First Name: $FIRST_NAME (should be: Null)"
    echo "  Last Name: $LAST_NAME (should be: Test)"
    echo "  Email: $EMAIL (should be: null.test.$TIMESTAMP@example.com)"
    
    if [ "$FIRST_NAME" = "Null" ] && [ "$LAST_NAME" = "Test" ] && [ "$EMAIL" = "null.test.$TIMESTAMP@example.com" ]; then
        echo "  ✅ Required fields preserved correctly"
    else
        echo "  ❌ Required fields not preserved correctly"
    fi
    
else
    echo "❌ Null values update failed with status: $NULL_HTTP_STATUS"
    echo "Response: $NULL_RESPONSE_BODY"
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
echo "🎉 Contact Update API Null Values Test Complete!"
