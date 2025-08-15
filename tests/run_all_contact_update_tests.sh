#!/bin/bash

echo "🚀 Running All Contact Update API Tests..."
echo "=========================================="

# Test 1: Basic functionality
echo ""
echo "🧪 Test 1: Basic Contact Update Functionality"
echo "----------------------------------------------"
./tests/test_contact_update.sh

# Test 2: Validation tests
echo ""
echo "🧪 Test 2: Contact Update Validation"
echo "------------------------------------"
./tests/test_contact_update_validation.sh

# Test 3: Custom fields tests
echo ""
echo "🧪 Test 3: Contact Update Custom Fields"
echo "---------------------------------------"
./tests/test_contact_update_custom_fields.sh

# Test 4: Final comprehensive test
echo ""
echo "🧪 Test 4: Final Comprehensive Test"
echo "-----------------------------------"
./tests/test_contact_update_final.sh

# Test 5: Clear fields test
echo ""
echo "🧪 Test 5: Clear Fields Test"
echo "----------------------------"
./tests/test_contact_update_clear_fields.sh

echo ""
echo "🎉 All Contact Update API Tests Complete!"
echo "=========================================="
