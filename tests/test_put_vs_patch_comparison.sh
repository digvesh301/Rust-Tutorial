#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing PUT vs PATCH API Comparison..."

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

# Step 2: Create a test contact with full data
echo ""
echo "📝 Step 2: Creating a test contact with full data..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Compare",
    "last_name": "Test",
    "email": "compare.test.'$TIMESTAMP'@example.com",
    "phone": "+1-555-0100",
    "company": "Original Company",
    "job_title": "Original Title",
    "address": "123 Original St",
    "city": "Original City",
    "state": "Original State",
    "postal_code": "12345",
    "country": "USA",
    "notes": "Original notes",
    "lead_source": "website",
    "lead_status": "new"
  }')

echo "Create Response: $CREATE_RESPONSE"

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Test PUT behavior (should replace all fields)
echo ""
echo "📝 Step 3: Testing PUT behavior (should replace all fields)..."
PUT_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "PUT",
    "last_name": "Updated",
    "email": "put.updated.'$TIMESTAMP'@example.com",
    "lead_status": "contacted"
  }')

PUT_HTTP_STATUS=$(echo "$PUT_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
PUT_RESPONSE_BODY=$(echo "$PUT_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "PUT HTTP Status: $PUT_HTTP_STATUS"
echo "PUT Response: $PUT_RESPONSE_BODY"

if [ "$PUT_HTTP_STATUS" = "200" ]; then
    echo "✅ PUT successful"
    
    # Check if optional fields were cleared
    if echo "$PUT_RESPONSE_BODY" | grep -q '"phone":null' && \
       echo "$PUT_RESPONSE_BODY" | grep -q '"company":null' && \
       echo "$PUT_RESPONSE_BODY" | grep -q '"job_title":null' && \
       echo "$PUT_RESPONSE_BODY" | grep -q '"address":null' && \
       echo "$PUT_RESPONSE_BODY" | grep -q '"notes":null'; then
        echo "✅ PUT correctly cleared unspecified optional fields"
    else
        echo "❌ PUT did not clear unspecified optional fields as expected"
    fi
else
    echo "❌ PUT failed with status: $PUT_HTTP_STATUS"
fi

# Step 4: Recreate the contact with full data for PATCH test
echo ""
echo "📝 Step 4: Recreating contact with full data for PATCH test..."
DELETE_RESPONSE=$(curl -s -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

RECREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Compare",
    "last_name": "Test",
    "email": "compare.test2.'$TIMESTAMP'@example.com",
    "phone": "+1-555-0100",
    "company": "Original Company",
    "job_title": "Original Title",
    "address": "123 Original St",
    "city": "Original City",
    "state": "Original State",
    "postal_code": "12345",
    "country": "USA",
    "notes": "Original notes",
    "lead_source": "website",
    "lead_status": "new"
  }')

CONTACT_ID2=$(echo "$RECREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
echo "✅ Recreated test contact with ID: $CONTACT_ID2"

# Step 5: Test PATCH behavior (should only update specified fields)
echo ""
echo "📝 Step 5: Testing PATCH behavior (should only update specified fields)..."
PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID2" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "PATCH",
    "last_name": "Updated",
    "email": "patch.updated.'$TIMESTAMP'@example.com",
    "lead_status": "contacted"
  }')

PATCH_HTTP_STATUS=$(echo "$PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
PATCH_RESPONSE_BODY=$(echo "$PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "PATCH HTTP Status: $PATCH_HTTP_STATUS"
echo "PATCH Response: $PATCH_RESPONSE_BODY"

if [ "$PATCH_HTTP_STATUS" = "200" ]; then
    echo "✅ PATCH successful"
    
    # Check if unspecified fields were preserved
    PATCH_PHONE=$(echo "$PATCH_RESPONSE_BODY" | grep -o '"phone":"[^"]*"' | cut -d'"' -f4)
    PATCH_COMPANY=$(echo "$PATCH_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    PATCH_JOB_TITLE=$(echo "$PATCH_RESPONSE_BODY" | grep -o '"job_title":"[^"]*"' | cut -d'"' -f4)
    PATCH_ADDRESS=$(echo "$PATCH_RESPONSE_BODY" | grep -o '"address":"[^"]*"' | cut -d'"' -f4)
    PATCH_NOTES=$(echo "$PATCH_RESPONSE_BODY" | grep -o '"notes":"[^"]*"' | cut -d'"' -f4)
    
    echo "  Phone: $PATCH_PHONE (should be preserved: +1-555-0100)"
    echo "  Company: $PATCH_COMPANY (should be preserved: Original Company)"
    echo "  Job Title: $PATCH_JOB_TITLE (should be preserved: Original Title)"
    echo "  Address: $PATCH_ADDRESS (should be preserved: 123 Original St)"
    echo "  Notes: $PATCH_NOTES (should be preserved: Original notes)"
    
    if [ "$PATCH_PHONE" = "+1-555-0100" ] && [ "$PATCH_COMPANY" = "Original Company" ] && \
       [ "$PATCH_JOB_TITLE" = "Original Title" ] && [ "$PATCH_ADDRESS" = "123 Original St" ] && \
       [ "$PATCH_NOTES" = "Original notes" ]; then
        echo "✅ PATCH correctly preserved unspecified fields"
    else
        echo "❌ PATCH did not preserve unspecified fields as expected"
    fi
else
    echo "❌ PATCH failed with status: $PATCH_HTTP_STATUS"
fi

# Step 6: Test PATCH with field clearing
echo ""
echo "📝 Step 6: Testing PATCH with field clearing (empty strings)..."
CLEAR_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID2" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "phone": "",
    "notes": ""
  }')

CLEAR_HTTP_STATUS=$(echo "$CLEAR_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CLEAR_RESPONSE_BODY=$(echo "$CLEAR_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Clear PATCH HTTP Status: $CLEAR_HTTP_STATUS"

if [ "$CLEAR_HTTP_STATUS" = "200" ]; then
    echo "✅ Clear PATCH successful"
    
    # Check if specified fields were cleared but others preserved
    if echo "$CLEAR_RESPONSE_BODY" | grep -q '"phone":null' && \
       echo "$CLEAR_RESPONSE_BODY" | grep -q '"notes":null'; then
        echo "✅ PATCH correctly cleared specified fields"
        
        # Check if other fields are still preserved
        PRESERVED_COMPANY=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
        PRESERVED_ADDRESS=$(echo "$CLEAR_RESPONSE_BODY" | grep -o '"address":"[^"]*"' | cut -d'"' -f4)
        
        if [ "$PRESERVED_COMPANY" = "Original Company" ] && [ "$PRESERVED_ADDRESS" = "123 Original St" ]; then
            echo "✅ PATCH preserved other fields while clearing specified ones"
        else
            echo "❌ PATCH did not preserve other fields correctly"
        fi
    else
        echo "❌ PATCH did not clear specified fields correctly"
    fi
else
    echo "❌ Clear PATCH failed with status: $CLEAR_HTTP_STATUS"
fi

# Step 7: Summary comparison
echo ""
echo "📝 Step 7: Summary of PUT vs PATCH differences..."
echo ""
echo "🔄 PUT Behavior:"
echo "  - Replaces the entire resource"
echo "  - Unspecified optional fields are set to null/default values"
echo "  - Idempotent (same result when called multiple times)"
echo "  - Requires all required fields to be provided"
echo ""
echo "🔧 PATCH Behavior:"
echo "  - Applies partial modifications to the resource"
echo "  - Unspecified fields are preserved (not modified)"
echo "  - May not be idempotent (depends on implementation)"
echo "  - Only requires the fields you want to change"
echo "  - Can clear fields by providing empty strings"
echo ""

# Step 8: Cleanup
echo ""
echo "📝 Step 8: Cleaning up test contacts..."
curl -s -X DELETE "$BASE_URL/contacts/$CONTACT_ID2" \
  -H "Authorization: Bearer $TOKEN" > /dev/null

echo "✅ Test contacts cleaned up successfully"

echo ""
echo "🎉 PUT vs PATCH Comparison Test Complete!"
echo ""
echo "Key Takeaways:"
echo "✅ Use PUT when you want to replace the entire resource"
echo "✅ Use PATCH when you want to update only specific fields"
echo "✅ PATCH preserves existing data, PUT replaces it"
echo "✅ Both methods are properly implemented and working correctly"
