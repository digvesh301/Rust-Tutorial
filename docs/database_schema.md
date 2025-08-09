# Survey Application Database Schema

## Overview
This document describes the complete database schema for the survey application, focusing on user roles, organizations, and survey management.

## Core Tables

### 1. Organizations
**Purpose**: Represents companies, institutions, or groups that use the survey system.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| name | VARCHAR(255) | Organization name |
| description | TEXT | Optional description |
| website | VARCHAR(255) | Organization website |
| logo_url | VARCHAR(500) | URL to organization logo |
| address | TEXT | Physical address |
| phone | VARCHAR(20) | Contact phone |
| email | VARCHAR(255) | Contact email |
| is_active | BOOLEAN | Whether organization is active |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

### 2. Roles
**Purpose**: Defines system-wide roles with specific permissions.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| name | VARCHAR(100) | Role name (unique) |
| description | TEXT | Role description |
| permissions | JSONB | JSON array of permissions |
| is_system_role | BOOLEAN | True for built-in roles |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

**Default Roles**:
- `super_admin`: Full system access
- `org_admin`: Organization administration
- `survey_creator`: Can create and manage surveys
- `survey_manager`: Can manage assigned surveys
- `respondent`: Can respond to surveys
- `viewer`: Read-only access

### 3. Users
**Purpose**: Stores user account information.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| organization_id | UUID | FK to organizations (nullable) |
| email | VARCHAR(255) | Unique email address |
| username | VARCHAR(100) | Unique username (optional) |
| password_hash | VARCHAR(255) | Hashed password |
| first_name | VARCHAR(100) | User's first name |
| last_name | VARCHAR(100) | User's last name |
| phone | VARCHAR(20) | Phone number |
| avatar_url | VARCHAR(500) | Profile picture URL |
| is_active | BOOLEAN | Account status |
| is_verified | BOOLEAN | Email verification status |
| last_login_at | TIMESTAMP | Last login time |
| created_at | TIMESTAMP | Account creation time |
| updated_at | TIMESTAMP | Last update time |

### 4. User Roles (Junction Table)
**Purpose**: Many-to-many relationship between users and roles, scoped by organization.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| user_id | UUID | FK to users |
| role_id | UUID | FK to roles |
| organization_id | UUID | FK to organizations (nullable) |
| assigned_by | UUID | FK to users (who assigned) |
| assigned_at | TIMESTAMP | Assignment timestamp |
| expires_at | TIMESTAMP | Role expiration (nullable) |
| is_active | BOOLEAN | Whether assignment is active |

### 5. Organization Members
**Purpose**: Tracks user membership in organizations.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| organization_id | UUID | FK to organizations |
| user_id | UUID | FK to users |
| member_type | VARCHAR(50) | 'owner', 'admin', 'member', 'guest' |
| joined_at | TIMESTAMP | Membership start date |
| invited_by | UUID | FK to users (who invited) |
| is_active | BOOLEAN | Membership status |

## Survey Tables

### 6. Surveys
**Purpose**: Stores survey definitions and metadata.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| organization_id | UUID | FK to organizations |
| created_by | UUID | FK to users (creator) |
| title | VARCHAR(255) | Survey title |
| description | TEXT | Survey description |
| instructions | TEXT | Instructions for respondents |
| is_public | BOOLEAN | Public visibility |
| is_active | BOOLEAN | Survey status |
| allow_anonymous | BOOLEAN | Allow anonymous responses |
| max_responses | INTEGER | Maximum response limit |
| starts_at | TIMESTAMP | Survey start time |
| ends_at | TIMESTAMP | Survey end time |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

### 7. Survey Permissions
**Purpose**: Fine-grained access control for surveys.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| survey_id | UUID | FK to surveys |
| user_id | UUID | FK to users (nullable) |
| role_id | UUID | FK to roles (nullable) |
| permission_type | VARCHAR(50) | 'view', 'edit', 'manage', 'respond' |
| granted_by | UUID | FK to users (who granted) |
| granted_at | TIMESTAMP | Permission grant time |
| expires_at | TIMESTAMP | Permission expiration |

### 8. Questions
**Purpose**: Stores individual survey questions.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| survey_id | UUID | FK to surveys |
| question_text | TEXT | The question text |
| question_type | VARCHAR(50) | Question type |
| options | JSONB | Question options (for choice questions) |
| is_required | BOOLEAN | Whether answer is required |
| order_index | INTEGER | Question order in survey |
| validation_rules | JSONB | Validation rules |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

**Question Types**:
- `multiple_choice`: Multiple selections allowed
- `single_choice`: Single selection only
- `text`: Free text input
- `rating`: Numeric rating scale
- `boolean`: Yes/No question

### 9. Survey Responses
**Purpose**: Tracks individual survey submissions.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| survey_id | UUID | FK to surveys |
| respondent_id | UUID | FK to users (nullable for anonymous) |
| respondent_email | VARCHAR(255) | Email for anonymous responses |
| ip_address | INET | Respondent IP address |
| user_agent | TEXT | Browser/client information |
| is_complete | BOOLEAN | Whether response is complete |
| submitted_at | TIMESTAMP | Submission timestamp |
| created_at | TIMESTAMP | Response start time |
| updated_at | TIMESTAMP | Last update time |

### 10. Question Responses
**Purpose**: Stores answers to individual questions.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| survey_response_id | UUID | FK to survey_responses |
| question_id | UUID | FK to questions |
| answer_text | TEXT | Text answer |
| answer_number | DECIMAL | Numeric answer |
| answer_boolean | BOOLEAN | Boolean answer |
| answer_json | JSONB | Complex answers (arrays, objects) |
| created_at | TIMESTAMP | Answer timestamp |

## Audit and Logging

### 11. Audit Logs
**Purpose**: Tracks all changes to important data.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| table_name | VARCHAR(100) | Name of affected table |
| record_id | UUID | ID of affected record |
| action | VARCHAR(50) | 'INSERT', 'UPDATE', 'DELETE' |
| old_values | JSONB | Previous values |
| new_values | JSONB | New values |
| changed_by | UUID | FK to users (who made change) |
| changed_at | TIMESTAMP | Change timestamp |
| ip_address | INET | IP address of change |
| user_agent | TEXT | Client information |

## Relationships Summary

1. **Organizations** have many **Users** (via organization_id)
2. **Users** can belong to multiple **Organizations** (via organization_members)
3. **Users** can have multiple **Roles** (via user_roles)
4. **Roles** can be assigned to multiple **Users** (via user_roles)
5. **Organizations** can have many **Surveys**
6. **Users** can create many **Surveys**
7. **Surveys** can have many **Questions**
8. **Surveys** can have many **Survey Responses**
9. **Questions** can have many **Question Responses**
10. **Survey Permissions** control access to specific surveys

## Key Use Cases and Queries

### 1. User Authentication and Authorization
```sql
-- Get user with roles for a specific organization
SELECT u.*, r.name as role_name, r.permissions
FROM users u
JOIN user_roles ur ON u.id = ur.user_id
JOIN roles r ON ur.role_id = r.id
WHERE u.email = 'user@example.com'
AND ur.organization_id = 'org-uuid'
AND ur.is_active = true;
```

### 2. Organization Management
```sql
-- Get all members of an organization with their roles
SELECT u.first_name, u.last_name, u.email, om.member_type, r.name as role_name
FROM organization_members om
JOIN users u ON om.user_id = u.id
LEFT JOIN user_roles ur ON u.id = ur.user_id AND ur.organization_id = om.organization_id
LEFT JOIN roles r ON ur.role_id = r.id
WHERE om.organization_id = 'org-uuid' AND om.is_active = true;
```

### 3. Survey Access Control
```sql
-- Check if user can access a survey
SELECT DISTINCT 'allowed' as access
FROM surveys s
LEFT JOIN survey_permissions sp ON s.id = sp.survey_id
LEFT JOIN user_roles ur ON sp.role_id = ur.role_id
WHERE s.id = 'survey-uuid'
AND (
    s.is_public = true OR
    s.created_by = 'user-uuid' OR
    sp.user_id = 'user-uuid' OR
    (ur.user_id = 'user-uuid' AND ur.is_active = true)
);
```

### 4. Survey Analytics
```sql
-- Get survey response statistics
SELECT
    s.title,
    COUNT(sr.id) as total_responses,
    COUNT(CASE WHEN sr.is_complete THEN 1 END) as complete_responses,
    COUNT(CASE WHEN sr.respondent_id IS NULL THEN 1 END) as anonymous_responses
FROM surveys s
LEFT JOIN survey_responses sr ON s.id = sr.survey_id
WHERE s.organization_id = 'org-uuid'
GROUP BY s.id, s.title;
```

## Permission System

### Role-Based Access Control (RBAC)
The system implements a flexible RBAC model where:

1. **Users** can have multiple **Roles**
2. **Roles** are scoped to **Organizations** (or global)
3. **Permissions** are stored as JSON arrays in roles
4. **Survey-specific permissions** override role permissions

### Permission Hierarchy
1. **System Level**: Super admin access
2. **Organization Level**: Organization-scoped permissions
3. **Survey Level**: Survey-specific permissions
4. **User Level**: Individual user permissions

### Sample Permission Strings
- `system:*` - Full system access
- `org:manage` - Manage organization
- `survey:create` - Create surveys
- `survey:manage` - Manage surveys
- `survey:view` - View surveys
- `survey:respond` - Respond to surveys
- `user:manage` - Manage users

## Data Flow Examples

### Creating a Survey
1. User must have `survey:create` permission in organization
2. Survey is created with `organization_id` and `created_by`
3. Creator automatically gets `manage` permission
4. Additional permissions can be granted via `survey_permissions`

### Responding to a Survey
1. Check if survey is public OR user has `respond` permission
2. Create `survey_response` record
3. For each question, create `question_response` record
4. Mark survey_response as complete when finished

### Managing Organization
1. User must be organization member with `owner` or `admin` type
2. OR have `org:manage` permission via role
3. Can invite users, assign roles, manage surveys

## Indexes
All foreign keys and frequently queried columns have indexes for optimal performance.
