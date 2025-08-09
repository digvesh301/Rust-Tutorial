# CRM System Database Schema

## Overview
This document describes the CRM (Customer Relationship Management) database schema that extends the survey application with contact management capabilities.

## Core CRM Tables

### 1. Contacts Table
**Purpose**: Stores contact information for leads, customers, and prospects.

| Column | Type | Description | Nullable |
|--------|------|-------------|----------|
| id | UUID | Primary key | No |
| first_name | VARCHAR(100) | Contact's first name | No |
| last_name | VARCHAR(100) | Contact's last name | No |
| email | VARCHAR(255) | Email address (unique) | No |
| phone | VARCHAR(20) | Phone number | Yes |
| company | VARCHAR(255) | Company name | Yes |
| job_title | VARCHAR(100) | Job title/position | Yes |
| address | TEXT | Street address | Yes |
| city | VARCHAR(100) | City | Yes |
| state | VARCHAR(100) | State/province | Yes |
| postal_code | VARCHAR(20) | ZIP/postal code | Yes |
| country | VARCHAR(100) | Country | Yes |
| notes | TEXT | General notes about contact | Yes |
| lead_source | VARCHAR(100) | How contact was acquired | Yes |
| lead_status | VARCHAR(50) | Current lead status | Yes |
| is_active | BOOLEAN | Whether contact is active | No |
| created_at | TIMESTAMP | Creation timestamp | No |
| updated_at | TIMESTAMP | Last update timestamp | No |

**Lead Status Values**: `new`, `contacted`, `qualified`, `proposal`, `negotiation`, `closed_won`, `closed_lost`

### 2. Custom Fields Table
**Purpose**: Defines custom fields that can be added to different modules (contacts, surveys, etc.).

| Column | Type | Description | Nullable |
|--------|------|-------------|----------|
| id | UUID | Primary key | No |
| module | VARCHAR(50) | Module name (e.g., 'contact') | No |
| label | VARCHAR(255) | Display name for field | No |
| field_name | VARCHAR(100) | Internal field name | No |
| field_type | VARCHAR(50) | Field data type | No |
| is_required | BOOLEAN | Whether field is required | No |
| is_active | BOOLEAN | Whether field is active | No |
| options | JSONB | Options for select fields | Yes |
| validation_rules | JSONB | Validation rules | Yes |
| default_value | TEXT | Default field value | Yes |
| help_text | TEXT | Help text for users | Yes |
| display_order | INTEGER | Display order in forms | Yes |
| created_by | UUID | User who created field | Yes |
| created_at | TIMESTAMP | Creation timestamp | No |
| updated_at | TIMESTAMP | Last update timestamp | No |

**Field Types**:
- `text` - Single line text input
- `textarea` - Multi-line text input
- `number` - Numeric input
- `email` - Email address input
- `phone` - Phone number input
- `date` - Date picker
- `boolean` - Checkbox (true/false)
- `select` - Single selection dropdown
- `multi_select` - Multiple selection dropdown

**Options JSON Format**:
```json
{
  "options": ["Option 1", "Option 2", "Option 3"]
}
```

**Validation Rules JSON Format**:
```json
{
  "min_length": 5,
  "max_length": 100,
  "pattern": "^[A-Za-z0-9]+$",
  "min": 0,
  "max": 999999
}
```

### 3. Contact Custom Values Table
**Purpose**: Stores the actual values for custom fields assigned to contacts.

| Column | Type | Description | Nullable |
|--------|------|-------------|----------|
| id | UUID | Primary key | No |
| contact_id | UUID | FK to contacts table | No |
| custom_field_id | UUID | FK to custom_fields table | No |
| value | TEXT | Text representation of value | Yes |
| value_json | JSONB | JSON representation for complex values | Yes |
| value_number | DECIMAL(15,4) | Numeric value (indexed) | Yes |
| value_date | DATE | Date value (indexed) | Yes |
| value_boolean | BOOLEAN | Boolean value (indexed) | Yes |
| created_at | TIMESTAMP | Creation timestamp | No |
| updated_at | TIMESTAMP | Last update timestamp | No |

**Value Storage Strategy**:
- All values are stored as text in the `value` column
- Typed columns (`value_number`, `value_date`, `value_boolean`) are populated for indexing and querying
- Complex values (arrays, objects) are stored in `value_json`

### 4. Contact Tags Table
**Purpose**: Categorize contacts with tags for better organization.

| Column | Type | Description | Nullable |
|--------|------|-------------|----------|
| id | UUID | Primary key | No |
| name | VARCHAR(100) | Tag name (unique) | No |
| color | VARCHAR(7) | Hex color code | Yes |
| description | TEXT | Tag description | Yes |
| created_at | TIMESTAMP | Creation timestamp | No |
| updated_at | TIMESTAMP | Last update timestamp | No |

### 5. Contact Tag Assignments Table
**Purpose**: Many-to-many relationship between contacts and tags.

| Column | Type | Description | Nullable |
|--------|------|-------------|----------|
| id | UUID | Primary key | No |
| contact_id | UUID | FK to contacts table | No |
| tag_id | UUID | FK to contact_tags table | No |
| assigned_by | UUID | User who assigned tag | Yes |
| assigned_at | TIMESTAMP | Assignment timestamp | No |

### 6. Contact Activities Table
**Purpose**: Track interactions and activities with contacts.

| Column | Type | Description | Nullable |
|--------|------|-------------|----------|
| id | UUID | Primary key | No |
| contact_id | UUID | FK to contacts table | No |
| activity_type | VARCHAR(50) | Type of activity | No |
| subject | VARCHAR(255) | Activity subject/title | Yes |
| description | TEXT | Activity description | Yes |
| activity_date | TIMESTAMP | When activity occurred | Yes |
| duration_minutes | INTEGER | Activity duration | Yes |
| status | VARCHAR(50) | Activity status | Yes |
| created_by | UUID | User who created activity | Yes |
| created_at | TIMESTAMP | Creation timestamp | No |
| updated_at | TIMESTAMP | Last update timestamp | No |

**Activity Types**: `call`, `email`, `meeting`, `note`, `task`, `survey_sent`, `survey_completed`
**Activity Status**: `completed`, `scheduled`, `cancelled`

## Key Relationships

1. **Contacts** ↔ **Custom Fields** (Many-to-Many via contact_custom_values)
2. **Contacts** ↔ **Tags** (Many-to-Many via contact_tag_assignments)
3. **Contacts** → **Activities** (One-to-Many)
4. **Custom Fields** → **Contact Custom Values** (One-to-Many)

## Sample Queries

### 1. Get Contact with Custom Fields
```sql
SELECT 
    c.*,
    cf.label as field_label,
    ccv.value as field_value
FROM contacts c
LEFT JOIN contact_custom_values ccv ON c.id = ccv.contact_id
LEFT JOIN custom_fields cf ON ccv.custom_field_id = cf.id
WHERE c.id = 'contact-uuid'
AND cf.is_active = true;
```

### 2. Search Contacts by Custom Field
```sql
SELECT DISTINCT c.*
FROM contacts c
JOIN contact_custom_values ccv ON c.id = ccv.contact_id
JOIN custom_fields cf ON ccv.custom_field_id = cf.id
WHERE cf.field_name = 'industry'
AND ccv.value = 'Technology';
```

### 3. Get Contacts with Tags
```sql
SELECT 
    c.*,
    STRING_AGG(ct.name, ', ') as tags
FROM contacts c
LEFT JOIN contact_tag_assignments cta ON c.id = cta.contact_id
LEFT JOIN contact_tags ct ON cta.tag_id = ct.id
GROUP BY c.id;
```

### 4. Contact Activity Timeline
```sql
SELECT 
    ca.*,
    c.first_name,
    c.last_name
FROM contact_activities ca
JOIN contacts c ON ca.contact_id = c.id
WHERE ca.contact_id = 'contact-uuid'
ORDER BY ca.activity_date DESC;
```

## Integration with Survey System

The CRM system integrates with the existing survey system through:

1. **Survey Invitations**: Contacts can be invited to participate in surveys
2. **Response Tracking**: Link survey responses to specific contacts
3. **Activity Logging**: Survey interactions are logged as contact activities
4. **Custom Fields**: Survey-specific custom fields can be defined

## MySQL vs PostgreSQL Differences

### PostgreSQL (Default)
- Uses `UUID` data type with `gen_random_uuid()`
- Uses `JSONB` for JSON data
- Uses `TIMESTAMP WITH TIME ZONE`

### MySQL (Alternative)
- Uses `CHAR(36)` with `UUID()` function
- Uses `JSON` data type
- Uses `TIMESTAMP` with `ENGINE=InnoDB`
- Requires manual trigger for `updated_at` timestamps

## Performance Considerations

1. **Indexes**: All foreign keys and frequently queried columns are indexed
2. **Typed Columns**: Custom values have typed columns for efficient querying
3. **Unique Constraints**: Prevent duplicate data and improve query performance
4. **Cascading Deletes**: Properly configured to maintain data integrity
