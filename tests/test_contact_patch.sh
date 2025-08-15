#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact PATCH API..."

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

# Step 2: Create a test contact to patch
echo ""
echo "📝 Step 2: Creating a test contact to patch..."
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Patch",
    "last_name": "Test",
    "email": "patch.test.'$TIMESTAMP'@example.com",
    "company": "Original Company",
    "phone": "+1-555-0100",
    "job_title": "Original Title",
    "city": "Original City",
    "state": "Original State",
    "lead_status": "new"
  }')

echo "Create Response: $CREATE_RESPONSE"

CONTACT_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
    echo "❌ Failed to create test contact"
    exit 1
fi

echo "✅ Created test contact with ID: $CONTACT_ID"

# Step 3: Test minimal patch (only one field)
echo ""
echo "📝 Step 3: Testing minimal patch (only first name)..."
MINIMAL_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Patched"
  }')

MINIMAL_HTTP_STATUS=$(echo "$MINIMAL_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
MINIMAL_RESPONSE_BODY=$(echo "$MINIMAL_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Minimal Patch HTTP Status: $MINIMAL_HTTP_STATUS"
echo "Minimal Patch Response: $MINIMAL_RESPONSE_BODY"

if [ "$MINIMAL_HTTP_STATUS" = "200" ]; then
    echo "✅ Minimal patch successful"
    
    # Verify the change
    PATCHED_FIRST_NAME=$(echo "$MINIMAL_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    UNCHANGED_LAST_NAME=$(echo "$MINIMAL_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    UNCHANGED_COMPANY=$(echo "$MINIMAL_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    
    if [ "$PATCHED_FIRST_NAME" = "Patched" ] && [ "$UNCHANGED_LAST_NAME" = "Test" ] && [ "$UNCHANGED_COMPANY" = "Original Company" ]; then
        echo "✅ Minimal patch verified correctly - only first name changed"
    else
        echo "❌ Minimal patch not working correctly"
        echo "Expected: first_name=Patched, last_name=Test, company=Original Company"
        echo "Got: first_name=$PATCHED_FIRST_NAME, last_name=$UNCHANGED_LAST_NAME, company=$UNCHANGED_COMPANY"
    fi
else
    echo "❌ Minimal patch failed with status: $MINIMAL_HTTP_STATUS"
fi

# Step 4: Test clearing fields with empty strings
echo ""
echo "📝 Step 4: Testing clearing fields with empty strings..."
CLEAR_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "phone": "",
    "company": "",
    "job_title": ""
  }')

CLEAR_HTTP_STATUS=$(echo "$CLEAR_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CLEAR_RESPONSE_BODY=$(echo "$CLEAR_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Clear Patch HTTP Status: $CLEAR_HTTP_STATUS"

if [ "$CLEAR_HTTP_STATUS" = "200" ]; then
    echo "✅ Clear fields patch successful"
    
    # Verify fields are cleared
    if echo "$CLEAR_RESPONSE_BODY" | grep -q '"phone":null' && \
       echo "$CLEAR_RESPONSE_BODY" | grep -q '"company":null' && \
       echo "$CLEAR_RESPONSE_BODY" | grep -q '"job_title":null'; then
        echo "✅ Fields cleared correctly with empty strings"
    else
        echo "❌ Fields not cleared correctly"
        echo "Response: $CLEAR_RESPONSE_BODY"
    fi
else
    echo "❌ Clear fields patch failed with status: $CLEAR_HTTP_STATUS"
fi

# Step 5: Test multiple field patch
echo ""
echo "📝 Step 5: Testing multiple field patch..."
MULTI_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "last_name": "MultiPatched",
    "company": "New Company",
    "city": "New City",
    "lead_status": "contacted"
  }')

MULTI_HTTP_STATUS=$(echo "$MULTI_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
MULTI_RESPONSE_BODY=$(echo "$MULTI_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Multi Patch HTTP Status: $MULTI_HTTP_STATUS"

if [ "$MULTI_HTTP_STATUS" = "200" ]; then
    echo "✅ Multiple field patch successful"
    
    # Verify the changes
    MULTI_LAST_NAME=$(echo "$MULTI_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    MULTI_COMPANY=$(echo "$MULTI_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    MULTI_CITY=$(echo "$MULTI_RESPONSE_BODY" | grep -o '"city":"[^"]*"' | cut -d'"' -f4)
    MULTI_LEAD_STATUS=$(echo "$MULTI_RESPONSE_BODY" | grep -o '"lead_status":"[^"]*"' | cut -d'"' -f4)
    PRESERVED_FIRST_NAME=$(echo "$MULTI_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    
    echo "  Last Name: $MULTI_LAST_NAME (expected: MultiPatched)"
    echo "  Company: $MULTI_COMPANY (expected: New Company)"
    echo "  City: $MULTI_CITY (expected: New City)"
    echo "  Lead Status: $MULTI_LEAD_STATUS (expected: contacted)"
    echo "  First Name: $PRESERVED_FIRST_NAME (should be preserved: Patched)"
    
    if [ "$MULTI_LAST_NAME" = "MultiPatched" ] && [ "$MULTI_COMPANY" = "New Company" ] && \
       [ "$MULTI_CITY" = "New City" ] && [ "$MULTI_LEAD_STATUS" = "contacted" ] && \
       [ "$PRESERVED_FIRST_NAME" = "Patched" ]; then
        echo "✅ Multiple field patch verified correctly"
    else
        echo "❌ Multiple field patch not working correctly"
    fi
else
    echo "❌ Multiple field patch failed with status: $MULTI_HTTP_STATUS"
fi

# Step 6: Test invalid lead status validation
echo ""
echo "📝 Step 6: Testing invalid lead status validation..."
INVALID_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "lead_status": "invalid_status"
  }')

INVALID_HTTP_STATUS=$(echo "$INVALID_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
INVALID_RESPONSE_BODY=$(echo "$INVALID_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Invalid Patch HTTP Status: $INVALID_HTTP_STATUS"
echo "Invalid Patch Response: $INVALID_RESPONSE_BODY"

if [ "$INVALID_HTTP_STATUS" = "400" ]; then
    echo "✅ Invalid lead status properly rejected"
else
    echo "❌ Invalid lead status should return 400, got: $INVALID_HTTP_STATUS"
fi

# Step 7: Test empty patch (no fields provided)
echo ""
echo "📝 Step 7: Testing empty patch (no fields provided)..."
EMPTY_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{}')

EMPTY_HTTP_STATUS=$(echo "$EMPTY_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
EMPTY_RESPONSE_BODY=$(echo "$EMPTY_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Empty Patch HTTP Status: $EMPTY_HTTP_STATUS"

if [ "$EMPTY_HTTP_STATUS" = "200" ]; then
    echo "✅ Empty patch successful (no changes made)"
else
    echo "❌ Empty patch failed with status: $EMPTY_HTTP_STATUS"
fi

# Step 8: Test patching non-existent contact
echo ""
echo "📝 Step 8: Testing patch of non-existent contact..."
NON_EXISTENT_ID="00000000-0000-0000-0000-000000000000"
NOT_FOUND_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$NON_EXISTENT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"first_name": "Should Fail"}')

NOT_FOUND_HTTP_STATUS=$(echo "$NOT_FOUND_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
NOT_FOUND_RESPONSE_BODY=$(echo "$NOT_FOUND_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Non-existent Contact Patch HTTP Status: $NOT_FOUND_HTTP_STATUS"
echo "Non-existent Contact Patch Response: $NOT_FOUND_RESPONSE_BODY"

if [ "$NOT_FOUND_HTTP_STATUS" = "404" ]; then
    echo "✅ Non-existent contact patch properly returns 404"
else
    echo "❌ Non-existent contact patch should return 404, got: $NOT_FOUND_HTTP_STATUS"
fi

# Step 9: Test unauthorized access (no token)
echo ""
echo "📝 Step 9: Testing unauthorized access (no token)..."
UNAUTHORIZED_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Should Fail"
  }')

UNAUTHORIZED_HTTP_STATUS=$(echo "$UNAUTHORIZED_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
UNAUTHORIZED_RESPONSE_BODY=$(echo "$UNAUTHORIZED_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Unauthorized Patch HTTP Status: $UNAUTHORIZED_HTTP_STATUS"
echo "Unauthorized Patch Response: $UNAUTHORIZED_RESPONSE_BODY"

if [ "$UNAUTHORIZED_HTTP_STATUS" = "401" ]; then
    echo "✅ Unauthorized access properly rejected"
else
    echo "❌ Unauthorized access should return 401, got: $UNAUTHORIZED_HTTP_STATUS"
fi

# Step 10: Cleanup - Delete the test contact
echo ""
echo "📝 Step 10: Cleaning up - deleting test contact..."
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_HTTP_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_HTTP_STATUS" = "204" ]; then
    echo "✅ Test contact cleaned up successfully"
else
    echo "⚠️  Failed to cleanup test contact (status: $DELETE_HTTP_STATUS)"
fi

echo ""
echo "🎉 Contact PATCH API Test Complete!"
