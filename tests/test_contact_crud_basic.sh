#!/bin/bash

BASE_URL="http://127.0.0.1:8081"

echo "🧪 Basic Contact CRUD Validation..."

# Login
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}')

TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "❌ Failed to get authentication token"
    exit 1
fi

echo "✅ Authentication successful"

# Create contact
TIMESTAMP=$(date +%s)
CREATE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X POST "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "CRUD",
    "last_name": "Test",
    "email": "crud.test.'$TIMESTAMP'@example.com"
  }')

CREATE_STATUS=$(echo "$CREATE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
CREATE_BODY=$(echo "$CREATE_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

if [ "$CREATE_STATUS" = "201" ] || [ "$CREATE_STATUS" = "200" ]; then
    echo "✅ Contact creation successful"
    CONTACT_ID=$(echo "$CREATE_BODY" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
else
    echo "❌ Contact creation failed: $CREATE_STATUS"
    exit 1
fi

# Read contact
GET_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

GET_STATUS=$(echo "$GET_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$GET_STATUS" = "200" ]; then
    echo "✅ Contact retrieval successful"
else
    echo "❌ Contact retrieval failed: $GET_STATUS"
    exit 1
fi

# Update contact (PUT)
PUT_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PUT "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Updated",
    "last_name": "CRUD",
    "email": "updated.crud.test.'$TIMESTAMP'@example.com",
    "lead_status": "contacted"
  }')

PUT_STATUS=$(echo "$PUT_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$PUT_STATUS" = "200" ]; then
    echo "✅ Contact update (PUT) successful"
else
    echo "❌ Contact update (PUT) failed: $PUT_STATUS"
    exit 1
fi

# Patch contact
PATCH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X PATCH "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Patched"
  }')

PATCH_STATUS=$(echo "$PATCH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$PATCH_STATUS" = "200" ]; then
    echo "✅ Contact patch successful"
else
    echo "❌ Contact patch failed: $PATCH_STATUS"
    exit 1
fi

# Delete contact
DELETE_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X DELETE "$BASE_URL/contacts/$CONTACT_ID" \
  -H "Authorization: Bearer $TOKEN")

DELETE_STATUS=$(echo "$DELETE_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

if [ "$DELETE_STATUS" = "204" ] || [ "$DELETE_STATUS" = "200" ]; then
    echo "✅ Contact deletion successful"
else
    echo "❌ Contact deletion failed: $DELETE_STATUS"
    exit 1
fi

echo "✅ All CRUD operations completed successfully"
