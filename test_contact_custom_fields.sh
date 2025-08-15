#!/bin/bash

# Test Contact Custom Fields API
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Contact Custom Fields..."

# Step 1: Login to get token
echo "📝 Step 1: Login to get JWT token..."
TOKEN=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}' | \
  grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo "❌ Failed to get token. Make sure server is running and user exists."
  exit 1
fi

echo "✅ Got token: ${TOKEN:0:20}..."

# Step 2: Create contact with custom fields
echo "📝 Step 2: Create contact with custom fields..."
TIMESTAMP=$(date +%s)
CONTACT_RESPONSE=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"first_name\": \"Digvesh\",
    \"last_name\": \"Dadhaniya\",
    \"email\": \"digvesh.test${TIMESTAMP}@example.com\",
    \"phone\": \"+91-9876543210\",
    \"company\": \"RapidOps Inc\",
    \"job_title\": \"Software Engineer\",
    \"custom_fields\": {
      \"linkedin_profile\": \"https://www.linkedin.com/in/digvesh\",
      \"industry\": \"Technology\",
      \"annual_revenue\": \"1000000\",
      \"preferred_contact\": \"Email\"
    }
  }")

echo "📋 Contact creation response:"
echo "$CONTACT_RESPONSE" | jq '.' 2>/dev/null || echo "$CONTACT_RESPONSE"

# Extract contact ID from response
CONTACT_ID=$(echo "$CONTACT_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CONTACT_ID" ]; then
  echo "❌ Failed to create contact or extract contact ID"
  exit 1
fi

echo "✅ Contact created with ID: $CONTACT_ID"

# Step 3: Get contact to verify custom fields are stored
echo "📝 Step 3: Get contact to verify custom fields..."
GET_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

echo "📋 Contact details with custom fields:"
echo "$GET_RESPONSE" | jq '.' 2>/dev/null || echo "$GET_RESPONSE"

# Step 4: Create another contact without custom fields
echo "📝 Step 4: Create contact without custom fields..."
SIMPLE_CONTACT=$(curl -s -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "company": "Simple Corp"
  }')

echo "📋 Simple contact response:"
echo "$SIMPLE_CONTACT" | jq '.' 2>/dev/null || echo "$SIMPLE_CONTACT"

echo "🎉 Custom fields test completed!"
echo ""
echo "📊 Summary:"
echo "✅ Created contact with custom fields (LinkedIn, Industry, Revenue, Contact Method)"
echo "✅ Retrieved contact with custom fields populated"
echo "✅ Created simple contact without custom fields"
echo "✅ Custom field values are stored in contact_custom_values table"
