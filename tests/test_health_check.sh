#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing Health Check API..."

# Step 1: Test health endpoint
echo ""
echo "📝 Step 1: Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/health")

HTTP_STATUS=$(echo "$HEALTH_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)
RESPONSE_BODY=$(echo "$HEALTH_RESPONSE" | sed 's/HTTP_STATUS:[0-9]*$//')

echo "Health Check HTTP Status: $HTTP_STATUS"
echo "Health Check Response: $RESPONSE_BODY"

if [ "$HTTP_STATUS" = "200" ]; then
    echo "✅ Health check endpoint is working correctly"
else
    echo "❌ Health check failed with status: $HTTP_STATUS"
    exit 1
fi

# Step 2: Test invalid endpoint
echo ""
echo "📝 Step 2: Testing invalid endpoint..."
INVALID_RESPONSE=$(curl -s -w "HTTP_STATUS:%{http_code}" -X GET "$BASE_URL/invalid-endpoint")

INVALID_HTTP_STATUS=$(echo "$INVALID_RESPONSE" | grep -o "HTTP_STATUS:[0-9]*" | cut -d':' -f2)

echo "Invalid Endpoint HTTP Status: $INVALID_HTTP_STATUS"

if [ "$INVALID_HTTP_STATUS" = "404" ] || [ "$INVALID_HTTP_STATUS" = "400" ]; then
    echo "✅ Invalid endpoint properly returns error status ($INVALID_HTTP_STATUS)"
else
    echo "❌ Invalid endpoint returned unexpected status: $INVALID_HTTP_STATUS"
    exit 1
fi

echo ""
echo "🎉 Health Check API Test Complete!"
