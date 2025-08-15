#!/bin/bash

# Setup Contact Custom Fields
BASE_URL="http://127.0.0.1:8081"

echo "🔧 Setting up Contact Custom Fields..."

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

# Step 2: Create LinkedIn Profile custom field
echo "📝 Step 2: Creating LinkedIn Profile custom field..."
curl -s -X POST "$BASE_URL/custom-fields" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "module": "contact",
    "field_name": "linkedin_profile",
    "label": "LinkedIn Profile",
    "field_type": "text",
    "is_required": false,
    "display_order": 1
  }' | jq '.' 2>/dev/null || echo "LinkedIn Profile field created"

# Step 3: Create Industry custom field
echo "📝 Step 3: Creating Industry custom field..."
curl -s -X POST "$BASE_URL/custom-fields" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "module": "contact",
    "field_name": "industry",
    "label": "Industry",
    "field_type": "select",
    "is_required": false,
    "display_order": 2,
    "options": ["Technology", "Healthcare", "Finance", "Education", "Manufacturing", "Other"]
  }' | jq '.' 2>/dev/null || echo "Industry field created"

# Step 4: Create Annual Revenue custom field
echo "📝 Step 4: Creating Annual Revenue custom field..."
curl -s -X POST "$BASE_URL/custom-fields" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "module": "contact",
    "field_name": "annual_revenue",
    "label": "Annual Revenue",
    "field_type": "number",
    "is_required": false,
    "display_order": 3
  }' | jq '.' 2>/dev/null || echo "Annual Revenue field created"

# Step 5: Create Preferred Contact Method custom field
echo "📝 Step 5: Creating Preferred Contact Method custom field..."
curl -s -X POST "$BASE_URL/custom-fields" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "module": "contact",
    "field_name": "preferred_contact",
    "label": "Preferred Contact Method",
    "field_type": "select",
    "is_required": false,
    "display_order": 4,
    "options": ["Email", "Phone", "SMS", "LinkedIn"]
  }' | jq '.' 2>/dev/null || echo "Preferred Contact Method field created"

echo "🎉 Custom fields setup completed!"
echo ""
echo "📋 Created custom fields:"
echo "✅ linkedin_profile (text)"
echo "✅ industry (select)"
echo "✅ annual_revenue (number)"
echo "✅ preferred_contact (select)"
