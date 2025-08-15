# Testing Guide

This document explains how to run and manage tests for the Survey API.

## Quick Start

### Run All Tests
```bash
./run_all_tests.sh
```

### Run Individual Test
```bash
./tests/test_contact_delete.sh
./tests/test_health_check.sh
```

## Test Organization

```
survey/
├── run_all_tests.sh           # Master test runner
├── tests/                     # All test files
│   ├── README.md             # Detailed testing documentation
│   ├── test_contact_delete.sh # Contact deletion tests
│   ├── test_health_check.sh  # Health check tests
│   └── [future tests...]     # Additional test files
└── ...
```

## Prerequisites

1. **Start the server**: `cargo run`
2. **Ensure test user exists** in database:
   - Email: `test@example.com`
   - Password: `password123`

## Test Results

The master test runner provides:
- ✅ **Colored output** for easy reading
- 📊 **Summary statistics** (total, passed, failed)
- 🔍 **Individual test results**
- 🚨 **Failed test listing**
- **Proper exit codes** for CI/CD integration

## Adding New Tests

1. Create new test file: `tests/test_[feature]_[action].sh`
2. Make it executable: `chmod +x tests/test_[feature]_[action].sh`
3. Follow the test template in `tests/README.md`
4. Test individually first, then run full suite

## Current Test Coverage

- ✅ **Contact Delete API**: Comprehensive soft delete testing
- ✅ **Health Check API**: Basic server health verification
- 🔄 **Future**: Contact CRUD, User management, Survey operations, etc.

## Example Usage

```bash
# Run all tests
./run_all_tests.sh

# Expected output:
# 🧪 Running All API Test Cases
# ==================================
# 📡 Checking if server is running...
# ✅ Server is running
# 🔍 Discovering test files in ./tests...
# Found 2 test file(s)
# ...
# 🎉 All tests passed!
```
