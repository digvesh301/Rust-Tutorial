# Contact PATCH API Implementation

## Overview

This document describes the implementation of the PATCH API for contacts in the Rust survey application. The PATCH API provides partial update functionality with proper merge semantics, following REST conventions.

## Implementation Details

### 1. Data Transfer Object (DTO)

**File**: `src/dto/contact_dto.rs`

Added `PatchContactRequest` struct with the same fields as `UpdateContactRequest` but with different semantics:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct PatchContactRequest {
    // All fields are optional for partial updates
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    // ... other fields
    pub custom_fields: Option<HashMap<String, String>>,
}
```

### 2. Service Layer

**File**: `src/services/contact_service.rs`

Implemented `patch_contact` method with merge semantics:

- **Partial Updates**: Only updates fields that are provided in the request
- **Field Preservation**: Unspecified fields remain unchanged
- **Field Clearing**: Empty strings set fields to `null`
- **Custom Field Merging**: Custom fields are merged, not replaced
- **Validation**: Same validation rules as PUT requests

Key features:
- Retrieves existing contact first
- Applies only the provided changes
- Preserves all unspecified fields
- Handles custom fields with merge semantics
- Proper error handling and logging

### 3. Controller Layer

**File**: `src/controllers/contact_controller.rs`

Added `patch_contact` controller function:

```rust
pub async fn patch_contact(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(contact_id): Path<Uuid>,
    Json(request): Json<PatchContactRequest>,
) -> Result<Json<Value>, AppError>
```

- Uses same permission as PUT (`contacts:update`)
- Proper authentication and authorization
- Consistent error handling and response format

### 4. Routing

**File**: `src/routes/contact_routes.rs`

Added PATCH route:

```rust
.route("/contacts/:id", patch(patch_contact))
```

### 5. Repository Layer

**File**: `src/repository/contact_custom_value_repository.rs`

Added `delete_by_contact_and_field` method for custom field removal:

```rust
pub async fn delete_by_contact_and_field(
    pool: &PgPool,
    contact_id: Uuid,
    custom_field_id: Uuid,
) -> Result<(), AppError>
```

## API Usage

### Endpoint

```
PATCH /api/contacts/{id}
```

### Authentication

Requires Bearer token in Authorization header:

```
Authorization: Bearer <jwt_token>
```

### Request Body Examples

#### Minimal Update (Single Field)

```json
{
  "first_name": "Updated Name"
}
```

#### Multiple Fields Update

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "company": "New Company",
  "lead_status": "contacted"
}
```

#### Clear Fields (Set to null)

```json
{
  "phone": "",
  "notes": "",
  "company": ""
}
```

#### Custom Fields (Merge Semantics)

```json
{
  "custom_fields": {
    "department": "Sales",
    "priority": "high",
    "budget": ""  // This will remove the budget field
  }
}
```

#### Combined Update

```json
{
  "first_name": "John",
  "company": "Updated Company",
  "lead_status": "qualified",
  "custom_fields": {
    "department": "Engineering",
    "status": "active"
  }
}
```

### Response Format

Success (200 OK):

```json
{
  "success": true,
  "message": "Contact patched successfully",
  "data": {
    "id": "uuid",
    "first_name": "Updated Name",
    "last_name": "Original Last Name",
    // ... other fields
    "custom_fields": {
      "department": "Engineering",
      "status": "active"
    }
  }
}
```

## PUT vs PATCH Comparison

| Aspect | PUT | PATCH |
|--------|-----|-------|
| **Purpose** | Replace entire resource | Apply partial modifications |
| **Unspecified Fields** | Set to default/null values | Preserved unchanged |
| **Idempotency** | Always idempotent | May not be idempotent |
| **Required Fields** | All required fields must be provided | Only fields to change |
| **Custom Fields** | Replaces all custom fields | Merges custom fields |
| **Use Case** | Full resource replacement | Partial updates |

## Testing

Comprehensive test suite includes:

1. **Basic PATCH Tests** (`tests/test_contact_patch.sh`)
   - Minimal patch (single field)
   - Multiple field patch
   - Field clearing with empty strings
   - Empty patch (no changes)
   - Error handling (invalid data, not found, unauthorized)

2. **Custom Fields Tests** (`tests/test_contact_patch_custom_fields.sh`)
   - Custom field merging
   - Custom field removal
   - Combined regular and custom field updates

3. **PUT vs PATCH Comparison** (`tests/test_put_vs_patch_comparison.sh`)
   - Side-by-side comparison of behaviors
   - Field preservation verification

4. **Comprehensive Test Suite** (`tests/run_all_contact_tests.sh`)
   - Runs all contact API tests
   - Provides summary and validation

## Key Benefits

1. **Efficiency**: Only send fields that need to be changed
2. **Safety**: Preserves existing data by default
3. **Flexibility**: Can update any combination of fields
4. **Consistency**: Follows REST conventions and HTTP standards
5. **Merge Semantics**: Intelligent handling of custom fields
6. **Validation**: Same validation rules as other endpoints
7. **Security**: Same authentication and authorization as PUT

## Error Handling

- **400 Bad Request**: Invalid field values or validation errors
- **401 Unauthorized**: Missing or invalid authentication token
- **404 Not Found**: Contact does not exist or is inactive
- **500 Internal Server Error**: Database or server errors

## Performance Considerations

- Retrieves existing contact before applying changes
- Only updates fields that are actually changing
- Efficient database queries with proper indexing
- Minimal data transfer (only changed fields in request)

## Future Enhancements

1. **Optimistic Locking**: Add version field for concurrent update protection
2. **Audit Trail**: Track what fields were changed and when
3. **Bulk PATCH**: Support patching multiple contacts in one request
4. **Field-level Permissions**: Different permissions for different fields
5. **Conditional PATCH**: Support If-Match headers for conditional updates

## Conclusion

The PATCH API implementation provides a robust, efficient, and user-friendly way to perform partial updates on contacts. It follows REST conventions, maintains data integrity, and provides excellent developer experience with comprehensive testing and documentation.
