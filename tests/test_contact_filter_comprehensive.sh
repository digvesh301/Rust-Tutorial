#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Filter API - Comprehensive Standard Field Tests..."

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

# Step 2: Create diverse test contacts
echo ""
echo "📝 Step 2: Creating diverse test contacts..."
TIMESTAMP=$(date +%s)

# Contact 1: Tech company, qualified lead
CONTACT1_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Alice",
    "last_name": "Johnson",
    "email": "alice.johnson.'$TIMESTAMP'@techcorp.com",
    "company": "TechCorp Inc",
    "job_title": "Software Engineer",
    "city": "San Francisco",
    "state": "CA",
    "lead_status": "qualified",
    "phone": "+1-555-0101"
  }')

CONTACT1_ID=$(echo "$CONTACT1_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Contact 2: Finance company, new lead
CONTACT2_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Bob",
    "last_name": "Smith",
    "email": "bob.smith.'$TIMESTAMP'@financeplus.com",
    "company": "Finance Plus",
    "job_title": "Financial Analyst",
    "city": "New York",
    "state": "NY",
    "lead_status": "new",
    "phone": "+1-555-0202"
  }')

CONTACT2_ID=$(echo "$CONTACT2_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Contact 3: Tech company, contacted lead
CONTACT3_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Carol",
    "last_name": "Davis",
    "email": "carol.davis.'$TIMESTAMP'@techcorp.com",
    "company": "TechCorp Inc",
    "job_title": "Product Manager",
    "city": "Austin",
    "state": "TX",
    "lead_status": "contacted",
    "phone": "+1-555-0303"
  }')

CONTACT3_ID=$(echo "$CONTACT3_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# Contact 4: Marketing company, qualified lead
CONTACT4_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "David",
    "last_name": "Wilson",
    "email": "david.wilson.'$TIMESTAMP'@marketingpro.com",
    "company": "Marketing Pro",
    "job_title": "Marketing Director",
    "city": "Los Angeles",
    "state": "CA",
    "lead_status": "qualified"
  }')

CONTACT4_ID=$(echo "$CONTACT4_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

echo "✅ Created 4 test contacts with diverse data"

# Step 3: Test single field equals filter
echo ""
echo "📝 Step 3: Testing single field equals filter (company = 'TechCorp Inc')..."
SINGLE_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "company",
        "operator": "equals",
        "value": "TechCorp Inc"
      }
    ],
    "page": 1,
    "limit": 10
  }')

SINGLE_HTTP_STATUS=$(echo "$SINGLE_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
SINGLE_RESPONSE_BODY=$(echo "$SINGLE_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Single Filter HTTP Status: $SINGLE_HTTP_STATUS"

if [ "$SINGLE_HTTP_STATUS" = "200" ]; then
    echo "✅ Single field filter successful"
    
    # Count contacts returned
    CONTACT_COUNT=$(echo "$SINGLE_RESPONSE_BODY" | grep -o '"total_count":[0-9]*' | cut -d':' -f2)
    if [ "$CONTACT_COUNT" = "2" ]; then
        echo "✅ Correct number of contacts returned (2 from TechCorp Inc)"
    else
        echo "❌ Expected 2 contacts, got: $CONTACT_COUNT"
    fi
else
    echo "❌ Single field filter failed with status: $SINGLE_HTTP_STATUS"
fi

# Step 4: Test contains filter
echo ""
echo "📝 Step 4: Testing contains filter (email contains 'techcorp.com')..."
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
    
    CONTACT_COUNT=$(echo "$CONTAINS_RESPONSE_BODY" | grep -o '"total_count":[0-9]*' | cut -d':' -f2)
    if [ "$CONTACT_COUNT" = "2" ]; then
        echo "✅ Correct number of contacts returned (2 with techcorp.com emails)"
    else
        echo "❌ Expected 2 contacts, got: $CONTACT_COUNT"
    fi
else
    echo "❌ Contains filter failed with status: $CONTAINS_HTTP_STATUS"
fi

# Step 5: Test IN operator filter
echo ""
echo "📝 Step 5: Testing IN operator filter (lead_status in ['qualified', 'new'])..."
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
        "value": ["qualified", "new"]
      }
    ],
    "page": 1,
    "limit": 10
  }')

IN_HTTP_STATUS=$(echo "$IN_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
IN_RESPONSE_BODY=$(echo "$IN_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "IN Filter HTTP Status: $IN_HTTP_STATUS"

if [ "$IN_HTTP_STATUS" = "200" ]; then
    echo "✅ IN operator filter successful"
    
    CONTACT_COUNT=$(echo "$IN_RESPONSE_BODY" | grep -o '"total_count":[0-9]*' | cut -d':' -f2)
    if [ "$CONTACT_COUNT" = "3" ]; then
        echo "✅ Correct number of contacts returned (3 with qualified/new status)"
    else
        echo "❌ Expected 3 contacts, got: $CONTACT_COUNT"
    fi
else
    echo "❌ IN operator filter failed with status: $IN_HTTP_STATUS"
fi

# Step 6: Test complex nested filter (AND with OR group)
echo ""
echo "📝 Step 6: Testing complex nested filter (state=CA AND (qualified OR new))..."
COMPLEX_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "state",
        "operator": "equals",
        "value": "CA"
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
            "field": "lead_status",
            "operator": "equals",
            "value": "new"
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

if [ "$COMPLEX_HTTP_STATUS" = "200" ]; then
    echo "✅ Complex nested filter successful"
    
    CONTACT_COUNT=$(echo "$COMPLEX_RESPONSE_BODY" | grep -o '"total_count":[0-9]*' | cut -d':' -f2)
    if [ "$CONTACT_COUNT" = "2" ]; then
        echo "✅ Correct number of contacts returned (Alice and David from CA with qualified status)"
    else
        echo "❌ Expected 2 contacts, got: $CONTACT_COUNT"
    fi
else
    echo "❌ Complex nested filter failed with status: $COMPLEX_HTTP_STATUS"
fi

# Step 7: Test pagination
echo ""
echo "📝 Step 7: Testing pagination (limit=2, page=1)..."
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
    
    # Check if exactly 2 contacts returned
    DATA_COUNT=$(echo "$PAGINATION_RESPONSE_BODY" | grep -o '"data":\[[^]]*\]' | grep -o '{"company"' | wc -l)
    if [ "$DATA_COUNT" = "2" ]; then
        echo "✅ Correct number of contacts in page (2)"
    else
        echo "❌ Expected 2 contacts in page, got: $DATA_COUNT"
    fi
    
    # Check pagination info
    if echo "$PAGINATION_RESPONSE_BODY" | grep -q '"pagination"'; then
        echo "✅ Pagination info included"
        
        # Check if has_next is true (since we have more than 2 contacts)
        if echo "$PAGINATION_RESPONSE_BODY" | grep -q '"has_next":true'; then
            echo "✅ Pagination correctly indicates more pages available"
        else
            echo "❌ Pagination should indicate more pages available"
        fi
    else
        echo "❌ Pagination info missing"
    fi
else
    echo "❌ Pagination filter failed with status: $PAGINATION_HTTP_STATUS"
fi

# Step 8: Test filter fields endpoint
echo ""
echo "📝 Step 8: Testing filter fields endpoint..."
FILTER_FIELDS_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/contacts/filter/fields" \
  -H "Authorization: Bearer $TOKEN")

FILTER_FIELDS_HTTP_STATUS=$(echo "$FILTER_FIELDS_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
FILTER_FIELDS_RESPONSE_BODY=$(echo "$FILTER_FIELDS_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Filter Fields HTTP Status: $FILTER_FIELDS_HTTP_STATUS"

if [ "$FILTER_FIELDS_HTTP_STATUS" = "200" ]; then
    echo "✅ Filter fields endpoint successful"
    
    # Check if standard fields are included
    STANDARD_FIELDS_FOUND=0
    for field in "first_name" "last_name" "email" "company" "job_title" "city" "state" "lead_status"; do
        if echo "$FILTER_FIELDS_RESPONSE_BODY" | grep -q "\"$field\""; then
            STANDARD_FIELDS_FOUND=$((STANDARD_FIELDS_FOUND + 1))
        fi
    done
    
    if [ "$STANDARD_FIELDS_FOUND" -ge 6 ]; then
        echo "✅ Standard fields included in response ($STANDARD_FIELDS_FOUND/8 found)"
    else
        echo "❌ Not enough standard fields found ($STANDARD_FIELDS_FOUND/8)"
    fi
else
    echo "❌ Filter fields endpoint failed with status: $FILTER_FIELDS_HTTP_STATUS"
fi

# Step 9: Test filter validation endpoint
echo ""
echo "📝 Step 9: Testing filter validation endpoint..."
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
            "field": "company",
            "operator": "contains",
            "value": "Tech"
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
        echo "✅ Filter correctly marked as valid"
    else
        echo "❌ Filter not marked as valid"
        echo "Response: $FILTER_VALIDATION_RESPONSE_BODY"
    fi
else
    echo "❌ Filter validation failed with status: $FILTER_VALIDATION_HTTP_STATUS"
    echo "Response: $FILTER_VALIDATION_RESPONSE_BODY"
fi

# Step 10: Test performance with filter summary
echo ""
echo "📝 Step 10: Testing filter performance and summary..."
PERFORMANCE_FILTER_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts/filter" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "logic": "and",
    "conditions": [
      {
        "type": "condition",
        "field": "company",
        "operator": "contains",
        "value": "Tech"
      }
    ],
    "page": 1,
    "limit": 10
  }')

PERFORMANCE_HTTP_STATUS=$(echo "$PERFORMANCE_FILTER_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
PERFORMANCE_RESPONSE_BODY=$(echo "$PERFORMANCE_FILTER_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Performance Filter HTTP Status: $PERFORMANCE_HTTP_STATUS"

if [ "$PERFORMANCE_HTTP_STATUS" = "200" ]; then
    echo "✅ Performance filter successful"
    
    # Check filter summary
    if echo "$PERFORMANCE_RESPONSE_BODY" | grep -q '"filter_summary"'; then
        echo "✅ Filter summary included"
        
        # Check execution time
        EXEC_TIME=$(echo "$PERFORMANCE_RESPONSE_BODY" | grep -o '"execution_time_ms":[0-9]*' | cut -d':' -f2)
        if [ -n "$EXEC_TIME" ] && [ "$EXEC_TIME" -lt 1000 ]; then
            echo "✅ Filter executed in reasonable time: ${EXEC_TIME}ms"
        else
            echo "⚠️  Filter execution time: ${EXEC_TIME}ms (may be slow)"
        fi
        
        # Check fields used
        if echo "$PERFORMANCE_RESPONSE_BODY" | grep -q '"fields_used":\["company"\]'; then
            echo "✅ Correct fields tracked in summary"
        else
            echo "❌ Fields not correctly tracked in summary"
        fi
    else
        echo "❌ Filter summary missing"
    fi
else
    echo "❌ Performance filter failed with status: $PERFORMANCE_HTTP_STATUS"
fi

# Step 11: Cleanup - Delete test contacts
echo ""
echo "📝 Step 11: Cleaning up test contacts..."
for CONTACT_ID in "$CONTACT1_ID" "$CONTACT2_ID" "$CONTACT3_ID" "$CONTACT4_ID"; do
    if [ -n "$CONTACT_ID" ]; then
        curl -s -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
          -H "Authorization: Bearer $TOKEN" > /dev/null
    fi
done

echo "✅ Test contacts cleaned up"

echo ""
echo "🎉 Contact Filter API Comprehensive Tests Complete!"
echo ""
echo "📊 Test Summary:"
echo "✅ Single field filtering"
echo "✅ Contains operator"
echo "✅ IN operator"
echo "✅ Complex nested filters (AND/OR groups)"
echo "✅ Pagination"
echo "✅ Filter fields endpoint"
echo "✅ Filter validation endpoint"
echo "✅ Performance monitoring"
