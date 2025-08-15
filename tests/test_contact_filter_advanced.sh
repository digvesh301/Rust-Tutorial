#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Filter API - Advanced Features..."

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

# Step 2: Create custom fields for testing
echo ""
echo "📝 Step 2: Creating custom fields for testing..."
TIMESTAMP=$(date +%s)

# Create a number custom field
CUSTOM_FIELD1_RESPONSE=$(curl -s -X POST "$BASE_URL/custom-fields" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "field_name": "annual_revenue_'$TIMESTAMP'",
    "field_type": "number",
    "is_required": false
  }')

CUSTOM_FIELD1_ID=$(echo "$CUSTOM_FIELD1_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Create a text custom field
CUSTOM_FIELD2_RESPONSE=$(curl -s -X POST "$BASE_URL/custom-fields" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "field_name": "industry_'$TIMESTAMP'",
    "field_type": "text",
    "is_required": false
  }')

CUSTOM_FIELD2_ID=$(echo "$CUSTOM_FIELD2_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

echo "✅ Created custom fields: annual_revenue_$TIMESTAMP and industry_$TIMESTAMP"

# Step 3: Create test contacts with custom field values
echo ""
echo "📝 Step 3: Creating test contacts with custom field values..."

# Contact 1: High revenue tech company
CONTACT1_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Alice",
    "last_name": "Tech",
    "email": "alice.tech.'$TIMESTAMP'@bigtech.com",
    "company": "Big Tech Corp",
    "job_title": "CTO",
    "city": "San Francisco",
    "state": "CA",
    "lead_status": "qualified"
  }')

CONTACT1_ID=$(echo "$CONTACT1_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Add custom field values for Contact 1
curl -s -X POST "$BASE_URL/contact-custom-values" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_id": "'$CONTACT1_ID'",
    "custom_field_id": "'$CUSTOM_FIELD1_ID'",
    "value_number": 5000000
  }' > /dev/null

curl -s -X POST "$BASE_URL/contact-custom-values" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_id": "'$CONTACT1_ID'",
    "custom_field_id": "'$CUSTOM_FIELD2_ID'",
    "value": "Technology"
  }' > /dev/null

# Contact 2: Medium revenue finance company
CONTACT2_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Bob",
    "last_name": "Finance",
    "email": "bob.finance.'$TIMESTAMP'@fintech.com",
    "company": "FinTech Solutions",
    "job_title": "CFO",
    "city": "New York",
    "state": "NY",
    "lead_status": "new"
  }')

CONTACT2_ID=$(echo "$CONTACT2_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Add custom field values for Contact 2
curl -s -X POST "$BASE_URL/contact-custom-values" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_id": "'$CONTACT2_ID'",
    "custom_field_id": "'$CUSTOM_FIELD1_ID'",
    "value_number": 1000000
  }' > /dev/null

curl -s -X POST "$BASE_URL/contact-custom-values" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_id": "'$CONTACT2_ID'",
    "custom_field_id": "'$CUSTOM_FIELD2_ID'",
    "value": "Finance"
  }' > /dev/null

# Contact 3: Low revenue retail company
CONTACT3_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Carol",
    "last_name": "Retail",
    "email": "carol.retail.'$TIMESTAMP'@retail.com",
    "company": "Retail Store",
    "job_title": "Manager",
    "city": "Chicago",
    "state": "IL",
    "lead_status": "contacted"
  }')

CONTACT3_ID=$(echo "$CONTACT3_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Add custom field values for Contact 3
curl -s -X POST "$BASE_URL/contact-custom-values" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_id": "'$CONTACT3_ID'",
    "custom_field_id": "'$CUSTOM_FIELD1_ID'",
    "value_number": 250000
  }' > /dev/null

curl -s -X POST "$BASE_URL/contact-custom-values" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_id": "'$CONTACT3_ID'",
    "custom_field_id": "'$CUSTOM_FIELD2_ID'",
    "value": "Retail"
  }' > /dev/null

echo "✅ Created 3 test contacts with custom field values"

# Step 4: Test custom field number filtering (greater than)
echo ""
echo "📝 Step 4: Testing custom field number filtering (revenue > 2M)..."
CUSTOM_NUMBER_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "annual_revenue_'$TIMESTAMP'",
        "operator": "greater_than",
        "value": 2000000,
        "field_type": "number"
      }
    ],
    "page": 1,
    "limit": 10
  }')

CUSTOM_NUMBER_HTTP_STATUS=$(echo "$CUSTOM_NUMBER_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CUSTOM_NUMBER_RESPONSE_BODY=$(echo "$CUSTOM_NUMBER_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Custom Number Filter HTTP Status: $CUSTOM_NUMBER_HTTP_STATUS"

if [ "$CUSTOM_NUMBER_HTTP_STATUS" = "200" ]; then
    echo "✅ Custom field number filtering successful"
    
    # Check if we got Alice (high revenue)
    if echo "$CUSTOM_NUMBER_RESPONSE_BODY" | grep -q "Alice"; then
        echo "✅ Found high revenue contact (Alice)"
    else
        echo "❌ Did not find expected high revenue contact"
    fi
    
    # Check if we didn't get Bob or Carol (lower revenue)
    if echo "$CUSTOM_NUMBER_RESPONSE_BODY" | grep -q "Bob\|Carol"; then
        echo "❌ Found unexpected lower revenue contacts"
    else
        echo "✅ Correctly filtered out lower revenue contacts"
    fi
else
    echo "❌ Custom field number filtering failed with status: $CUSTOM_NUMBER_HTTP_STATUS"
    echo "Response: $CUSTOM_NUMBER_RESPONSE_BODY"
fi

# Step 5: Test custom field text filtering (contains)
echo ""
echo "📝 Step 5: Testing custom field text filtering (industry contains 'Tech')..."
CUSTOM_TEXT_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "industry_'$TIMESTAMP'",
        "operator": "contains",
        "value": "Tech",
        "field_type": "text"
      }
    ],
    "page": 1,
    "limit": 10
  }')

CUSTOM_TEXT_HTTP_STATUS=$(echo "$CUSTOM_TEXT_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CUSTOM_TEXT_RESPONSE_BODY=$(echo "$CUSTOM_TEXT_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Custom Text Filter HTTP Status: $CUSTOM_TEXT_HTTP_STATUS"

if [ "$CUSTOM_TEXT_HTTP_STATUS" = "200" ]; then
    echo "✅ Custom field text filtering successful"
    
    # Check if we got Alice (Technology industry)
    if echo "$CUSTOM_TEXT_RESPONSE_BODY" | grep -q "Alice"; then
        echo "✅ Found Technology industry contact (Alice)"
    else
        echo "❌ Did not find expected Technology industry contact"
    fi
else
    echo "❌ Custom field text filtering failed with status: $CUSTOM_TEXT_HTTP_STATUS"
    echo "Response: $CUSTOM_TEXT_RESPONSE_BODY"
fi

# Step 6: Test complex filter combining standard and custom fields
echo ""
echo "📝 Step 6: Testing complex filter (standard + custom fields)..."
COMPLEX_MIXED_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "lead_status",
        "operator": "in",
        "value": ["qualified", "new"]
      },
      {
        "type": "condition",
        "field": "annual_revenue_'$TIMESTAMP'",
        "operator": "greater_than",
        "value": 500000,
        "field_type": "number"
      }
    ],
    "page": 1,
    "limit": 10
  }')

COMPLEX_MIXED_HTTP_STATUS=$(echo "$COMPLEX_MIXED_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
COMPLEX_MIXED_RESPONSE_BODY=$(echo "$COMPLEX_MIXED_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Complex Mixed Filter HTTP Status: $COMPLEX_MIXED_HTTP_STATUS"

if [ "$COMPLEX_MIXED_HTTP_STATUS" = "200" ]; then
    echo "✅ Complex mixed filter successful"
    
    # Should get Alice (qualified, 5M) and Bob (new, 1M) but not Carol (contacted, 250K)
    ALICE_FOUND=$(echo "$COMPLEX_MIXED_RESPONSE_BODY" | grep -c "Alice" || echo "0")
    BOB_FOUND=$(echo "$COMPLEX_MIXED_RESPONSE_BODY" | grep -c "Bob" || echo "0")
    CAROL_FOUND=$(echo "$COMPLEX_MIXED_RESPONSE_BODY" | grep -c "Carol" || echo "0")
    
    if [ "$ALICE_FOUND" -gt 0 ] && [ "$BOB_FOUND" -gt 0 ] && [ "$CAROL_FOUND" -eq 0 ]; then
        echo "✅ Complex filter returned correct contacts (Alice and Bob, not Carol)"
    else
        echo "❌ Complex filter did not return expected contacts"
        echo "Alice found: $ALICE_FOUND, Bob found: $BOB_FOUND, Carol found: $CAROL_FOUND"
    fi
else
    echo "❌ Complex mixed filter failed with status: $COMPLEX_MIXED_HTTP_STATUS"
    echo "Response: $COMPLEX_MIXED_RESPONSE_BODY"
fi

# Step 7: Test filter fields endpoint
echo ""
echo "📝 Step 7: Testing filter fields endpoint..."
FILTER_FIELDS_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/contacts/filter/fields" \
  -H "Authorization: Bearer $TOKEN")

FILTER_FIELDS_HTTP_STATUS=$(echo "$FILTER_FIELDS_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
FILTER_FIELDS_RESPONSE_BODY=$(echo "$FILTER_FIELDS_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Filter Fields HTTP Status: $FILTER_FIELDS_HTTP_STATUS"

if [ "$FILTER_FIELDS_HTTP_STATUS" = "200" ]; then
    echo "✅ Filter fields endpoint successful"
    
    # Check if standard fields are included
    if echo "$FILTER_FIELDS_RESPONSE_BODY" | grep -q "first_name\|email\|company"; then
        echo "✅ Standard fields included in response"
    else
        echo "❌ Standard fields missing from response"
    fi
    
    # Check if custom fields are included
    if echo "$FILTER_FIELDS_RESPONSE_BODY" | grep -q "annual_revenue_$TIMESTAMP\|industry_$TIMESTAMP"; then
        echo "✅ Custom fields included in response"
    else
        echo "❌ Custom fields missing from response"
    fi
else
    echo "❌ Filter fields endpoint failed with status: $FILTER_FIELDS_HTTP_STATUS"
    echo "Response: $FILTER_FIELDS_RESPONSE_BODY"
fi

# Step 8: Test filter validation endpoint
echo ""
echo "📝 Step 8: Testing filter validation endpoint..."
FILTER_VALIDATION_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter/validate" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "email",
        "operator": "contains",
        "value": "@"
      },
      {
        "type": "group",
        "logic": "or",
        "conditions": [
          {
            "type": "condition",
            "field": "lead_status",
            "operator": "equals",
            "value": "qualified"
          },
          {
            "type": "condition",
            "field": "annual_revenue_'$TIMESTAMP'",
            "operator": "greater_than",
            "value": 1000000,
            "field_type": "number"
          }
        ]
      }
    ]
  }')

FILTER_VALIDATION_HTTP_STATUS=$(echo "$FILTER_VALIDATION_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
FILTER_VALIDATION_RESPONSE_BODY=$(echo "$FILTER_VALIDATION_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Filter Validation HTTP Status: $FILTER_VALIDATION_HTTP_STATUS"

if [ "$FILTER_VALIDATION_HTTP_STATUS" = "200" ]; then
    echo "✅ Filter validation successful"
    
    if echo "$FILTER_VALIDATION_RESPONSE_BODY" | grep -q '"valid":true'; then
        echo "✅ Filter marked as valid"
    else
        echo "❌ Filter not marked as valid"
    fi
else
    echo "❌ Filter validation failed with status: $FILTER_VALIDATION_HTTP_STATUS"
    echo "Response: $FILTER_VALIDATION_RESPONSE_BODY"
fi

# Step 9: Cleanup - Delete test contacts and custom fields
echo ""
echo "📝 Step 9: Cleaning up test data..."

# Delete contacts
for CONTACT_ID in "$CONTACT1_ID" "$CONTACT2_ID" "$CONTACT3_ID"; do
    if [ -n "$CONTACT_ID" ]; then
        curl -s -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
          -H "Authorization: Bearer $TOKEN" > /dev/null
    fi
done

# Delete custom fields
for CUSTOM_FIELD_ID in "$CUSTOM_FIELD1_ID" "$CUSTOM_FIELD2_ID"; do
    if [ -n "$CUSTOM_FIELD_ID" ]; then
        curl -s -X DELETE "$BASE_URL/custom-fields/$CUSTOM_FIELD_ID" \
          -H "Authorization: Bearer $TOKEN" > /dev/null
    fi
done

echo "✅ Test data cleaned up"

echo ""
echo "🎉 Contact Filter API Advanced Tests Complete!"
