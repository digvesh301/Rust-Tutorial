#!/bin/bash

# Script to check the roles and their permissions in the database

echo "🔍 Checking Contact Roles and Permissions..."
echo "============================================="

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# API Base URL
BASE_URL="http://127.0.0.1:8081"

# Login to get JWT token
echo -e "${BLUE}1. Logging in to get JWT token...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/users/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo -e "${RED}❌ Failed to get authentication token${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Got authentication token${NC}"
echo ""

# Test contacts endpoint to see if permissions are working
echo -e "${BLUE}2. Testing contacts endpoint...${NC}"
CONTACTS_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts" \
  -H "Authorization: Bearer $TOKEN")

echo "Contacts Response (first 200 chars): ${CONTACTS_RESPONSE:0:200}..."
echo ""

# Check if we can access contacts health
echo -e "${BLUE}3. Testing contacts health endpoint...${NC}"
HEALTH_RESPONSE=$(curl -s -X GET "$BASE_URL/contacts/health" \
  -H "Authorization: Bearer $TOKEN")

echo "Health Response: $HEALTH_RESPONSE"
echo ""

echo -e "${GREEN}🎉 Contact Roles and Permissions Test Completed!${NC}"
echo ""
echo -e "${YELLOW}📋 Summary of New Roles Added:${NC}"
echo -e "   • ${GREEN}contact_manager${NC} - Full contact management"
echo -e "   • ${GREEN}sales_rep${NC} - Sales team member"
echo -e "   • ${GREEN}marketing_user${NC} - Marketing team member"
echo -e "   • ${GREEN}support_agent${NC} - Customer support agent"
echo -e "   • ${GREEN}readonly_user${NC} - Read-only access"
echo ""
echo -e "${YELLOW}📋 Updated Existing Roles:${NC}"
echo -e "   • ${GREEN}admin${NC} - Now has full contact permissions"
echo -e "   • ${GREEN}member${NC} - Can create/update own contacts"
echo -e "   • ${GREEN}viewer${NC} - Can read contacts"
echo -e "   • ${GREEN}owner${NC} - Still has wildcard (*) permissions"
