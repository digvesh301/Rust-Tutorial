# API Test Suite

This directory contains all API test cases for the Survey application.

## Structure

```
tests/
├── README.md                    # This file
├── test_contact_delete.sh       # Contact deletion API tests
└── [future test files]          # Additional test files as needed
```

## Running Tests

### Run All Tests
From the project root directory:
```bash
./run_all_tests.sh
```

### Run Individual Tests
From the project root directory:
```bash
./tests/test_contact_delete.sh
```

## Test File Naming Convention

- All test files should be named with the pattern: `test_[feature]_[action].sh`
- Examples:
  - `test_contact_delete.sh`
  - `test_contact_create.sh`
  - `test_user_authentication.sh`
  - `test_survey_crud.sh`

## Test File Requirements

Each test file should:

1. **Be executable**: `chmod +x test_file.sh`
2. **Include proper headers**: Shebang line `#!/bin/bash`
3. **Use consistent output format**: 
   - Use emojis and colors for better readability
   - Include step-by-step descriptions
   - Show clear success/failure indicators
4. **Return proper exit codes**:
   - `0` for success
   - `1` for failure
5. **Clean up after themselves**: Remove any test data created during the test
6. **Be independent**: Each test should be able to run standalone

## Prerequisites

Before running tests, ensure:

1. **Server is running**: Start the server with `cargo run`
2. **Database is accessible**: Ensure the database connection is working
3. **Test data exists**: Ensure required test users/data exist in the database

## Test Data

Tests use the following test user for authentication:
- Email: `test@example.com`
- Password: `password123`

Make sure this user exists in your database before running tests.

## Adding New Tests

To add a new test:

1. Create a new `.sh` file in the `tests/` directory
2. Follow the naming convention: `test_[feature]_[action].sh`
3. Make it executable: `chmod +x tests/test_[feature]_[action].sh`
4. Follow the test file requirements above
5. Test it individually first, then run the full test suite

## Example Test Structure

```bash
#!/bin/bash

# Test configuration
BASE_URL="http://127.0.0.1:8081"

echo "🧪 Testing [Feature] [Action] API..."

# Step 1: Setup/Authentication
echo "📝 Step 1: [Description]..."
# ... test code ...

# Step 2: Main test logic
echo "📝 Step 2: [Description]..."
# ... test code ...

# Step 3: Verification
echo "📝 Step 3: [Description]..."
# ... test code ...

# Step 4: Cleanup (if needed)
echo "📝 Step 4: [Description]..."
# ... cleanup code ...

echo "🎉 [Feature] [Action] API Test Complete!"
```
