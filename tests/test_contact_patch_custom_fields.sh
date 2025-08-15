#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact PATCH API Custom Fields..."

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
    "first_name": "PatchCustom",
    "last_name": "Test",
    "email": "patchcustom.test.'$TIMESTAMP'@example.com",
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

# Step 3: Patch with new custom fields (should merge)
echo ""
echo "📝 Step 3: Patching with new custom fields (merge semantics)..."
PATCH_CUSTOM_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "custom_fields": {
      "department": "Sales",
      "region": "North America",
      "source": "referral"
    }
  }')

PATCH_HTTP_STATUS=$(echo "$PATCH_CUSTOM_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
PATCH_RESPONSE_BODY=$(echo "$PATCH_CUSTOM_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Patch Custom Fields HTTP Status: $PATCH_HTTP_STATUS"
echo "Patch Custom Fields Response: $PATCH_RESPONSE_BODY"

if [ "$PATCH_HTTP_STATUS" = "200" ]; then
    echo "✅ Custom fields patch successful"
else
    echo "❌ Custom fields patch failed with status: $PATCH_HTTP_STATUS"
fi

# Step 4: Get the contact to verify custom fields were merged
echo ""
echo "📝 Step 4: Getting contact to verify custom fields merge..."
GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "Updated Contact: $GET_RESPONSE"

# Step 5: Patch to remove a custom field (using empty string)
echo ""
echo "📝 Step 5: Patching to remove a custom field (using empty string)..."
REMOVE_CUSTOM_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "custom_fields": {
      "budget": ""
    }
  }')

REMOVE_HTTP_STATUS=$(echo "$REMOVE_CUSTOM_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
REMOVE_RESPONSE_BODY=$(echo "$REMOVE_CUSTOM_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Remove Custom Field HTTP Status: $REMOVE_HTTP_STATUS"
echo "Remove Custom Field Response: $REMOVE_RESPONSE_BODY"

if [ "$REMOVE_HTTP_STATUS" = "200" ]; then
    echo "✅ Custom field removal patch successful"
else
    echo "❌ Custom field removal patch failed with status: $REMOVE_HTTP_STATUS"
fi

# Step 6: Patch contact without custom fields (should not affect existing custom fields)
echo ""
echo "📝 Step 6: Patching contact without custom fields..."
NO_CUSTOM_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "PatchedCustom"
  }')

NO_CUSTOM_HTTP_STATUS=$(echo "$NO_CUSTOM_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
NO_CUSTOM_RESPONSE_BODY=$(echo "$NO_CUSTOM_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "No Custom Fields Patch HTTP Status: $NO_CUSTOM_HTTP_STATUS"

if [ "$NO_CUSTOM_HTTP_STATUS" = "200" ]; then
    echo "✅ Patch without custom fields successful"
    
    # Verify first name changed
    UPDATED_FIRST_NAME=$(echo "$NO_CUSTOM_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    if [ "$UPDATED_FIRST_NAME" = "PatchedCustom" ]; then
        echo "✅ First name updated correctly"
    else
        echo "❌ First name not updated correctly: $UPDATED_FIRST_NAME"
    fi
else
    echo "❌ Patch without custom fields failed with status: $NO_CUSTOM_HTTP_STATUS"
fi

# Step 7: Get the contact one more time to verify custom fields are preserved
echo ""
echo "📝 Step 7: Getting contact to verify custom fields are preserved..."
FINAL_GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "Final Contact: $FINAL_GET_RESPONSE"

# Step 8: Test patch with both regular fields and custom fields
echo ""
echo "📝 Step 8: Testing patch with both regular fields and custom fields..."
COMBINED_PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "last_name": "CombinedPatch",
    "company": "Combined Company",
    "lead_status": "qualified",
    "custom_fields": {
      "priority": "medium",
      "status": "active"
    }
  }')

COMBINED_HTTP_STATUS=$(echo "$COMBINED_PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
COMBINED_RESPONSE_BODY=$(echo "$COMBINED_PATCH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Combined Patch HTTP Status: $COMBINED_HTTP_STATUS"

if [ "$COMBINED_HTTP_STATUS" = "200" ]; then
    echo "✅ Combined patch successful"
    
    # Verify changes
    COMBINED_LAST_NAME=$(echo "$COMBINED_RESPONSE_BODY" | grep -o '"last_name":"[^"]*"' | cut -d'"' -f4)
    COMBINED_COMPANY=$(echo "$COMBINED_RESPONSE_BODY" | grep -o '"company":"[^"]*"' | cut -d'"' -f4)
    COMBINED_LEAD_STATUS=$(echo "$COMBINED_RESPONSE_BODY" | grep -o '"lead_status":"[^"]*"' | cut -d'"' -f4)
    PRESERVED_FIRST_NAME=$(echo "$COMBINED_RESPONSE_BODY" | grep -o '"first_name":"[^"]*"' | cut -d'"' -f4)
    
    echo "  Last Name: $COMBINED_LAST_NAME (expected: CombinedPatch)"
    echo "  Company: $COMBINED_COMPANY (expected: Combined Company)"
    echo "  Lead Status: $COMBINED_LEAD_STATUS (expected: qualified)"
    echo "  First Name: $PRESERVED_FIRST_NAME (should be preserved: PatchedCustom)"
    
    if [ "$COMBINED_LAST_NAME" = "CombinedPatch" ] && [ "$COMBINED_COMPANY" = "Combined Company" ] && \
       [ "$COMBINED_LEAD_STATUS" = "qualified" ] && [ "$PRESERVED_FIRST_NAME" = "PatchedCustom" ]; then
        echo "✅ Combined patch verified correctly"
    else
        echo "❌ Combined patch not working correctly"
    fi
else
    echo "❌ Combined patch failed with status: $COMBINED_HTTP_STATUS"
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
echo "🎉 Contact PATCH API Custom Fields Test Complete!"
