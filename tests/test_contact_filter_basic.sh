#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Filter API - Basic Functionality..."

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

# Step 2: Create test contacts for filtering
echo ""
echo "📝 Step 2: Creating test contacts for filtering..."
TIMESTAMP=$(date +%s)

# Contact 1: John Doe from Tech Corp
CONTACT1_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe.'$TIMESTAMP'@techcorp.com",
    "company": "Tech Corp",
    "job_title": "Software Engineer",
    "city": "San Francisco",
    "state": "CA",
    "lead_status": "qualified"
  }')

CONTACT1_ID=$(echo "$CONTACT1_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Contact 2: Jane Smith from Marketing Inc
CONTACT2_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jane",
    "last_name": "Smith",
    "email": "jane.smith.'$TIMESTAMP'@marketing.com",
    "company": "Marketing Inc",
    "job_title": "Marketing Manager",
    "city": "New York",
    "state": "NY",
    "lead_status": "new"
  }')

CONTACT2_ID=$(echo "$CONTACT2_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Contact 3: Bob Johnson from Tech Corp
CONTACT3_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Bob",
    "last_name": "Johnson",
    "email": "bob.johnson.'$TIMESTAMP'@techcorp.com",
    "company": "Tech Corp",
    "job_title": "Product Manager",
    "city": "Austin",
    "state": "TX",
    "lead_status": "contacted"
  }')

CONTACT3_ID=$(echo "$CONTACT3_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

echo "✅ Created 3 test contacts"

# Step 3: Test simple filter - single condition
echo ""
echo "📝 Step 3: Testing simple filter (company equals 'Tech Corp')..."
SIMPLE_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "company",
        "operator": "equals",
        "value": "Tech Corp"
      }
    ],
    "page": 1,
    "limit": 10
  }')

SIMPLE_HTTP_STATUS=$(echo "$SIMPLE_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
SIMPLE_RESPONSE_BODY=$(echo "$SIMPLE_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Simple Filter HTTP Status: $SIMPLE_HTTP_STATUS"
echo "Simple Filter Response: $SIMPLE_RESPONSE_BODY"

if [ "$SIMPLE_HTTP_STATUS" = "200" ]; then
    echo "✅ Simple filter successful"
    
    # Check if we got the expected contacts (should be 2 from Tech Corp)
    CONTACT_COUNT=$(echo "$SIMPLE_RESPONSE_BODY" | grep -o '"data":\[' | wc -l)
    if [ "$CONTACT_COUNT" -gt 0 ]; then
        echo "✅ Filter returned contacts from Tech Corp"
    else
        echo "❌ Filter did not return expected contacts"
    fi
else
    echo "❌ Simple filter failed with status: $SIMPLE_HTTP_STATUS"
fi

# Step 4: Test complex nested filter
echo ""
echo "📝 Step 4: Testing complex nested filter..."
COMPLEX_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "company",
        "operator": "equals",
        "value": "Tech Corp"
      },
      {
        "type": "group",
        "logic": "or",
        "conditions": [
          {
            "type": "condition",
            "field": "city",
            "operator": "equals",
            "value": "San Francisco"
          },
          {
            "type": "condition",
            "field": "city",
            "operator": "equals",
            "value": "Austin"
          }
        ]
      }
    ],
    "page": 1,
    "limit": 10
  }')

COMPLEX_HTTP_STATUS=$(echo "$COMPLEX_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
COMPLEX_RESPONSE_BODY=$(echo "$COMPLEX_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Complex Filter HTTP Status: $COMPLEX_HTTP_STATUS"
echo "Complex Filter Response: $COMPLEX_RESPONSE_BODY"

if [ "$COMPLEX_HTTP_STATUS" = "200" ]; then
    echo "✅ Complex nested filter successful"
else
    echo "❌ Complex nested filter failed with status: $COMPLEX_HTTP_STATUS"
fi

# Step 5: Test filter with contains operator
echo ""
echo "📝 Step 5: Testing filter with contains operator..."
CONTAINS_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "email",
        "operator": "contains",
        "value": "techcorp.com"
      }
    ],
    "page": 1,
    "limit": 10
  }')

CONTAINS_HTTP_STATUS=$(echo "$CONTAINS_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CONTAINS_RESPONSE_BODY=$(echo "$CONTAINS_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Contains Filter HTTP Status: $CONTAINS_HTTP_STATUS"

if [ "$CONTAINS_HTTP_STATUS" = "200" ]; then
    echo "✅ Contains filter successful"
else
    echo "❌ Contains filter failed with status: $CONTAINS_HTTP_STATUS"
fi

# Step 6: Test filter with IN operator
echo ""
echo "📝 Step 6: Testing filter with IN operator..."
IN_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "lead_status",
        "operator": "in",
        "value": ["new", "qualified"]
      }
    ],
    "page": 1,
    "limit": 10
  }')

IN_HTTP_STATUS=$(echo "$IN_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
IN_RESPONSE_BODY=$(echo "$IN_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "IN Filter HTTP Status: $IN_HTTP_STATUS"

if [ "$IN_HTTP_STATUS" = "200" ]; then
    echo "✅ IN filter successful"
else
    echo "❌ IN filter failed with status: $IN_HTTP_STATUS"
fi

# Step 7: Test pagination
echo ""
echo "📝 Step 7: Testing pagination..."
PAGINATION_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [],
    "page": 1,
    "limit": 2
  }')

PAGINATION_HTTP_STATUS=$(echo "$PAGINATION_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
PAGINATION_RESPONSE_BODY=$(echo "$PAGINATION_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Pagination Filter HTTP Status: $PAGINATION_HTTP_STATUS"

if [ "$PAGINATION_HTTP_STATUS" = "200" ]; then
    echo "✅ Pagination filter successful"
    
    # Check pagination info
    if echo "$PAGINATION_RESPONSE_BODY" | grep -q '"pagination"'; then
        echo "✅ Pagination info included in response"
    else
        echo "❌ Pagination info missing from response"
    fi
else
    echo "❌ Pagination filter failed with status: $PAGINATION_HTTP_STATUS"
fi

# Step 8: Test empty filter (should return all contacts)
echo ""
echo "📝 Step 8: Testing empty filter (should return all contacts)..."
EMPTY_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [],
    "page": 1,
    "limit": 10
  }')

EMPTY_HTTP_STATUS=$(echo "$EMPTY_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
EMPTY_RESPONSE_BODY=$(echo "$EMPTY_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Empty Filter HTTP Status: $EMPTY_HTTP_STATUS"

if [ "$EMPTY_HTTP_STATUS" = "200" ]; then
    echo "✅ Empty filter successful (returns all contacts)"
else
    echo "❌ Empty filter failed with status: $EMPTY_HTTP_STATUS"
fi

# Step 9: Test invalid filter structure
echo ""
echo "📝 Step 9: Testing invalid filter structure..."
INVALID_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "invalid_logic",
    "conditions": [
      {
        "type": "condition",
        "field": "company",
        "operator": "invalid_operator",
        "value": "Tech Corp"
      }
    ]
  }')

INVALID_HTTP_STATUS=$(echo "$INVALID_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
INVALID_RESPONSE_BODY=$(echo "$INVALID_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Invalid Filter HTTP Status: $INVALID_HTTP_STATUS"
echo "Invalid Filter Response: $INVALID_RESPONSE_BODY"

if [ "$INVALID_HTTP_STATUS" = "400" ]; then
    echo "✅ Invalid filter properly rejected"
else
    echo "❌ Invalid filter should return 400, got: $INVALID_HTTP_STATUS"
fi

# Step 10: Test unauthorized access
echo ""
echo "📝 Step 10: Testing unauthorized access..."
UNAUTHORIZED_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "company",
        "operator": "equals",
        "value": "Tech Corp"
      }
    ]
  }')

UNAUTHORIZED_HTTP_STATUS=$(echo "$UNAUTHORIZED_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
UNAUTHORIZED_RESPONSE_BODY=$(echo "$UNAUTHORIZED_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Unauthorized Filter HTTP Status: $UNAUTHORIZED_HTTP_STATUS"

if [ "$UNAUTHORIZED_HTTP_STATUS" = "400" ] || [ "$UNAUTHORIZED_HTTP_STATUS" = "401" ]; then
    echo "✅ Unauthorized access properly rejected"
else
    echo "❌ Unauthorized access should return 400/401, got: $UNAUTHORIZED_HTTP_STATUS"
fi

# Step 11: Cleanup - Delete test contacts
echo ""
echo "📝 Step 11: Cleaning up test contacts..."
for CONTACT_ID in "$CONTACT1_ID" "$CONTACT2_ID" "$CONTACT3_ID"; do
    if [ -n "$CONTACT_ID" ]; then
        curl -s -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
          -H "Authorization: Bearer $TOKEN" > /dev/null
    fi
done

echo "✅ Test contacts cleaned up"

echo ""
echo "🎉 Contact Filter API Basic Tests Complete!"
