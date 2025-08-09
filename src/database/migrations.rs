// Database migrations runner

use sqlx::{PgPool, Row};
use crate::errors::AppError;

pub struct MigrationRunner;

impl MigrationRunner {
    /// Run all migrations
    pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
        tracing::info!("Starting database migrations...");

        // Create migrations table to track applied migrations
        Self::create_migrations_table(pool).await?;

        // Run individual migrations
        Self::run_migration_001_create_users_table(pool).await?;
        Self::run_migration_002_create_user_organizations_table_force(pool).await?;
        Self::run_migration_003_create_crm_tables(pool).await?;
        Self::run_migration_004_create_crm_indexes(pool).await?;
        Self::run_migration_005_insert_sample_crm_data(pool).await?;
        Self::run_migration_006_add_owner_to_contacts(pool).await?;
        Self::run_migration_007_add_contact_permissions(pool).await?;

        tracing::info!("All migrations completed successfully");
        Ok(())
    }

    /// Create migrations tracking table
    async fn create_migrations_table(pool: &PgPool) -> Result<(), AppError> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS _migrations (
                id SERIAL PRIMARY KEY,
                migration_name VARCHAR(255) NOT NULL UNIQUE,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#;

        sqlx::query(query).execute(pool).await?;
        tracing::info!("Migrations tracking table created");
        Ok(())
    }

    /// Check if migration has been applied
    async fn is_migration_applied(pool: &PgPool, migration_name: &str) -> Result<bool, AppError> {
        let result = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE migration_name = $1) as exists"
        )
        .bind(migration_name)
        .fetch_one(pool)
        .await?;

        let exists: bool = result.get("exists");
        Ok(exists)
    }

    /// Mark migration as applied
    async fn mark_migration_applied(pool: &PgPool, migration_name: &str) -> Result<(), AppError> {
        sqlx::query("INSERT INTO _migrations (migration_name) VALUES ($1)")
            .bind(migration_name)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Migration 001: Create users table
    async fn run_migration_001_create_users_table(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "001_create_users_table";

        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Create users table
        let create_table_query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                status VARCHAR(50) NOT NULL DEFAULT 'active',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#;

        sqlx::query(create_table_query).execute(pool).await?;

        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)",
            "CREATE INDEX IF NOT EXISTS idx_users_status ON users(status)",
            "CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at)",
        ];

        for index_query in indexes {
            sqlx::query(index_query).execute(pool).await?;
        }

        // Add constraints
        let constraints = vec![
            "ALTER TABLE users ADD CONSTRAINT IF NOT EXISTS check_user_status CHECK (status IN ('active', 'inactive', 'suspended', 'pending'))",
            "ALTER TABLE users ADD CONSTRAINT IF NOT EXISTS check_email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$')",
        ];

        for constraint_query in constraints {
            // Use a transaction to handle constraint errors gracefully
            let result = sqlx::query(constraint_query).execute(pool).await;
            if let Err(e) = result {
                tracing::warn!("Constraint may already exist: {}", e);
                // Continue with other constraints
            }
        }

        // Create trigger function for updated_at
        let trigger_function = r#"
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql'
        "#;

        sqlx::query(trigger_function).execute(pool).await?;

        // Drop existing trigger if it exists
        let drop_trigger = "DROP TRIGGER IF EXISTS update_users_updated_at ON users";
        sqlx::query(drop_trigger).execute(pool).await?;

        // Create trigger
        let create_trigger = r#"
            CREATE TRIGGER update_users_updated_at
                BEFORE UPDATE ON users
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column()
        "#;

        sqlx::query(create_trigger).execute(pool).await?;

        // Mark migration as applied
        Self::mark_migration_applied(pool, migration_name).await?;

        tracing::info!("Migration {} completed successfully", migration_name);
        Ok(())
    }

    /// Migration 002: Create user_organizations table
    async fn run_migration_002_create_user_organizations_table(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "002_create_user_organizations_table";

        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Create roles table
        let create_roles_table = r#"
            CREATE TABLE IF NOT EXISTS roles (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(50) NOT NULL UNIQUE,
                description TEXT,
                permissions JSONB DEFAULT '[]'::jsonb,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#;

        sqlx::query(create_roles_table).execute(pool).await?;
        tracing::info!("Roles table created successfully");

        // Insert default roles
        let insert_roles = r#"
            INSERT INTO roles (name, description, permissions) VALUES
                ('owner', 'Organization owner with full access', '["*"]'::jsonb),
                ('admin', 'Organization administrator', '["users:read", "users:write", "org:read", "org:write"]'::jsonb),
                ('member', 'Regular organization member', '["org:read", "users:read"]'::jsonb),
                ('viewer', 'Read-only access to organization', '["org:read"]'::jsonb)
            ON CONFLICT (name) DO NOTHING
        "#;

        sqlx::query(insert_roles).execute(pool).await?;
        tracing::info!("Default roles inserted successfully");

        // Create user_organizations table
        let create_user_organizations_table = r#"
            CREATE TABLE IF NOT EXISTS user_organizations (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL,
                org_id UUID NOT NULL,
                role_id UUID NOT NULL,
                status VARCHAR(20) NOT NULL DEFAULT 'active',
                joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                CONSTRAINT fk_user_organizations_user_id
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                CONSTRAINT fk_user_organizations_org_id
                    FOREIGN KEY (org_id) REFERENCES organization(id) ON DELETE CASCADE,
                CONSTRAINT fk_user_organizations_role_id
                    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE RESTRICT,

                CONSTRAINT unique_user_org UNIQUE (user_id, org_id)
            )
        "#;

        sqlx::query(create_user_organizations_table).execute(pool).await?;
        tracing::info!("User organizations table created successfully");

        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_user_id ON user_organizations(user_id)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_org_id ON user_organizations(org_id)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_role_id ON user_organizations(role_id)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_status ON user_organizations(status)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_joined_at ON user_organizations(joined_at)",
        ];

        for index_query in indexes {
            sqlx::query(index_query).execute(pool).await?;
        }
        tracing::info!("Indexes created successfully");

        // Add status constraint
        let status_constraint = r#"
            ALTER TABLE user_organizations ADD CONSTRAINT IF NOT EXISTS check_user_org_status
            CHECK (status IN ('active', 'pending', 'invited', 'suspended', 'inactive'))
        "#;

        let result = sqlx::query(status_constraint).execute(pool).await;
        if let Err(e) = result {
            tracing::warn!("Status constraint may already exist: {}", e);
        }

        // Drop existing trigger if it exists (must be done before dropping function)
        let drop_trigger = "DROP TRIGGER IF EXISTS update_user_organizations_updated_at_trigger ON user_organizations";
        sqlx::query(drop_trigger).execute(pool).await?;

        // Drop existing function if it exists
        let drop_function = "DROP FUNCTION IF EXISTS update_user_organizations_updated_at()";
        sqlx::query(drop_function).execute(pool).await?;

        // Create trigger function for user_organizations
        let trigger_function = r#"
            CREATE FUNCTION update_user_organizations_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql'
        "#;

        sqlx::query(trigger_function).execute(pool).await?;

        // Create trigger
        let create_trigger = r#"
            CREATE TRIGGER update_user_organizations_updated_at_trigger
                BEFORE UPDATE ON user_organizations
                FOR EACH ROW
                EXECUTE FUNCTION update_user_organizations_updated_at()
        "#;

        sqlx::query(create_trigger).execute(pool).await?;

        // Drop existing roles trigger if it exists (must be done before dropping function)
        let drop_roles_trigger = "DROP TRIGGER IF EXISTS update_roles_updated_at_trigger ON roles";
        sqlx::query(drop_roles_trigger).execute(pool).await?;

        // Drop existing roles function if it exists
        let drop_roles_function = "DROP FUNCTION IF EXISTS update_roles_updated_at()";
        sqlx::query(drop_roles_function).execute(pool).await?;

        // Create trigger function for roles
        let roles_trigger_function = r#"
            CREATE FUNCTION update_roles_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql'
        "#;

        sqlx::query(roles_trigger_function).execute(pool).await?;

        // Create trigger for roles
        let create_roles_trigger = r#"
            CREATE TRIGGER update_roles_updated_at_trigger
                BEFORE UPDATE ON roles
                FOR EACH ROW
                EXECUTE FUNCTION update_roles_updated_at()
        "#;

        sqlx::query(create_roles_trigger).execute(pool).await?;

        tracing::info!("Triggers created successfully");

        // Mark migration as applied
        Self::mark_migration_applied(pool, migration_name).await?;

        tracing::info!("Migration {} completed successfully", migration_name);
        Ok(())
    }

    /// Migration 002: Create user_organizations table (Force version - ignores if already applied)
    async fn run_migration_002_create_user_organizations_table_force(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "002_create_user_organizations_table";

        tracing::info!("Force running migration: {}", migration_name);

        // Create roles table
        let create_roles_table = r#"
            CREATE TABLE IF NOT EXISTS roles (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(50) NOT NULL UNIQUE,
                description TEXT,
                permissions JSONB DEFAULT '[]'::jsonb,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#;

        sqlx::query(create_roles_table).execute(pool).await?;
        tracing::info!("Roles table created successfully");

        // Insert default roles
        let insert_roles = r#"
            INSERT INTO roles (name, description, permissions) VALUES
                ('owner', 'Organization owner with full access', '["*"]'::jsonb),
                ('admin', 'Organization administrator', '["users:read", "users:write", "org:read", "org:write"]'::jsonb),
                ('member', 'Regular organization member', '["org:read", "users:read"]'::jsonb),
                ('viewer', 'Read-only access to organization', '["org:read"]'::jsonb)
            ON CONFLICT (name) DO NOTHING
        "#;

        sqlx::query(insert_roles).execute(pool).await?;
        tracing::info!("Default roles inserted successfully");

        // Create user_organizations table
        let create_user_organizations_table = r#"
            CREATE TABLE IF NOT EXISTS user_organizations (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL,
                org_id UUID NOT NULL,
                role_id UUID NOT NULL,
                status VARCHAR(20) NOT NULL DEFAULT 'active',
                joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                CONSTRAINT fk_user_organizations_user_id
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                CONSTRAINT fk_user_organizations_org_id
                    FOREIGN KEY (org_id) REFERENCES organization(id) ON DELETE CASCADE,
                CONSTRAINT fk_user_organizations_role_id
                    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE RESTRICT,

                CONSTRAINT unique_user_org UNIQUE (user_id, org_id)
            )
        "#;

        sqlx::query(create_user_organizations_table).execute(pool).await?;
        tracing::info!("User organizations table created successfully");

        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_user_id ON user_organizations(user_id)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_org_id ON user_organizations(org_id)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_role_id ON user_organizations(role_id)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_status ON user_organizations(status)",
            "CREATE INDEX IF NOT EXISTS idx_user_organizations_joined_at ON user_organizations(joined_at)",
        ];

        for index_query in indexes {
            sqlx::query(index_query).execute(pool).await?;
        }
        tracing::info!("Indexes created successfully");

        // Add status constraint
        let status_constraint = r#"
            ALTER TABLE user_organizations ADD CONSTRAINT IF NOT EXISTS check_user_org_status
            CHECK (status IN ('active', 'pending', 'invited', 'suspended', 'inactive'))
        "#;

        let result = sqlx::query(status_constraint).execute(pool).await;
        if let Err(e) = result {
            tracing::warn!("Status constraint may already exist: {}", e);
        }

        // Drop existing trigger if it exists (must be done before dropping function)
        let drop_trigger = "DROP TRIGGER IF EXISTS update_user_organizations_updated_at_trigger ON user_organizations";
        sqlx::query(drop_trigger).execute(pool).await?;

        // Drop existing function if it exists
        let drop_function = "DROP FUNCTION IF EXISTS update_user_organizations_updated_at()";
        sqlx::query(drop_function).execute(pool).await?;

        // Create trigger function for user_organizations
        let trigger_function = r#"
            CREATE FUNCTION update_user_organizations_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql'
        "#;

        sqlx::query(trigger_function).execute(pool).await?;

        // Create trigger
        let create_trigger = r#"
            CREATE TRIGGER update_user_organizations_updated_at_trigger
                BEFORE UPDATE ON user_organizations
                FOR EACH ROW
                EXECUTE FUNCTION update_user_organizations_updated_at()
        "#;

        sqlx::query(create_trigger).execute(pool).await?;

        // Drop existing roles trigger if it exists (must be done before dropping function)
        let drop_roles_trigger = "DROP TRIGGER IF EXISTS update_roles_updated_at_trigger ON roles";
        sqlx::query(drop_roles_trigger).execute(pool).await?;

        // Drop existing roles function if it exists
        let drop_roles_function = "DROP FUNCTION IF EXISTS update_roles_updated_at()";
        sqlx::query(drop_roles_function).execute(pool).await?;

        // Create trigger function for roles
        let roles_trigger_function = r#"
            CREATE FUNCTION update_roles_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql'
        "#;

        sqlx::query(roles_trigger_function).execute(pool).await?;

        // Create trigger for roles
        let create_roles_trigger = r#"
            CREATE TRIGGER update_roles_updated_at_trigger
                BEFORE UPDATE ON roles
                FOR EACH ROW
                EXECUTE FUNCTION update_roles_updated_at()
        "#;

        sqlx::query(create_roles_trigger).execute(pool).await?;

        tracing::info!("Triggers created successfully");

        // Mark migration as applied (only if not already marked)
        if !Self::is_migration_applied(pool, migration_name).await? {
            Self::mark_migration_applied(pool, migration_name).await?;
        }

        tracing::info!("Migration {} completed successfully", migration_name);
        Ok(())
    }

    /// Migration 003: Create CRM tables
    async fn run_migration_003_create_crm_tables(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "003_create_crm_tables";

        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Create contacts table
        let create_contacts_table = r#"
            CREATE TABLE IF NOT EXISTS contacts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                first_name VARCHAR(100) NOT NULL,
                last_name VARCHAR(100) NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                phone VARCHAR(20),
                company VARCHAR(255),
                job_title VARCHAR(100),
                address TEXT,
                city VARCHAR(100),
                state VARCHAR(100),
                postal_code VARCHAR(20),
                country VARCHAR(100),
                notes TEXT,
                lead_source VARCHAR(100),
                lead_status VARCHAR(50) DEFAULT 'new',
                is_active BOOLEAN DEFAULT true,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
        "#;

        sqlx::query(create_contacts_table).execute(pool).await?;
        tracing::info!("Contacts table created successfully");

        // Create custom_fields table
        let create_custom_fields_table = r#"
            CREATE TABLE IF NOT EXISTS custom_fields (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                module VARCHAR(50) NOT NULL,
                label VARCHAR(255) NOT NULL,
                field_name VARCHAR(100) NOT NULL,
                field_type VARCHAR(50) NOT NULL,
                is_required BOOLEAN DEFAULT false,
                is_active BOOLEAN DEFAULT true,
                options JSONB,
                validation_rules JSONB,
                default_value TEXT,
                help_text TEXT,
                display_order INTEGER DEFAULT 0,
                created_by UUID,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                UNIQUE(module, field_name)
            )
        "#;

        sqlx::query(create_custom_fields_table).execute(pool).await?;
        tracing::info!("Custom fields table created successfully");

        // Create contact_custom_values table
        let create_contact_custom_values_table = r#"
            CREATE TABLE IF NOT EXISTS contact_custom_values (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                contact_id UUID NOT NULL,
                custom_field_id UUID NOT NULL,
                value TEXT,
                value_json JSONB,
                value_number DECIMAL(15,4),
                value_date DATE,
                value_boolean BOOLEAN,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                CONSTRAINT fk_contact_custom_values_contact
                    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
                CONSTRAINT fk_contact_custom_values_custom_field
                    FOREIGN KEY (custom_field_id) REFERENCES custom_fields(id) ON DELETE CASCADE,
                UNIQUE(contact_id, custom_field_id)
            )
        "#;

        sqlx::query(create_contact_custom_values_table).execute(pool).await?;
        tracing::info!("Contact custom values table created successfully");

        // Create contact_tags table
        let create_contact_tags_table = r#"
            CREATE TABLE IF NOT EXISTS contact_tags (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(100) NOT NULL UNIQUE,
                color VARCHAR(7) DEFAULT '#007bff',
                description TEXT,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
        "#;

        sqlx::query(create_contact_tags_table).execute(pool).await?;
        tracing::info!("Contact tags table created successfully");

        // Create contact_tag_assignments table
        let create_contact_tag_assignments_table = r#"
            CREATE TABLE IF NOT EXISTS contact_tag_assignments (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                contact_id UUID NOT NULL,
                tag_id UUID NOT NULL,
                assigned_by UUID,
                assigned_at TIMESTAMPTZ DEFAULT NOW(),
                CONSTRAINT fk_contact_tag_assignments_contact
                    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
                CONSTRAINT fk_contact_tag_assignments_tag
                    FOREIGN KEY (tag_id) REFERENCES contact_tags(id) ON DELETE CASCADE,
                UNIQUE(contact_id, tag_id)
            )
        "#;

        sqlx::query(create_contact_tag_assignments_table).execute(pool).await?;
        tracing::info!("Contact tag assignments table created successfully");

        // Create contact_activities table
        let create_contact_activities_table = r#"
            CREATE TABLE IF NOT EXISTS contact_activities (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                contact_id UUID NOT NULL,
                activity_type VARCHAR(50) NOT NULL,
                subject VARCHAR(255),
                description TEXT,
                activity_date TIMESTAMPTZ,
                duration_minutes INTEGER,
                status VARCHAR(50) DEFAULT 'completed',
                created_by UUID,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                CONSTRAINT fk_contact_activities_contact
                    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
            )
        "#;

        sqlx::query(create_contact_activities_table).execute(pool).await?;
        tracing::info!("Contact activities table created successfully");

        // Mark migration as applied
        Self::mark_migration_applied(pool, migration_name).await?;

        tracing::info!("Migration {} completed successfully", migration_name);
        Ok(())
    }

    /// Migration 004: Create CRM indexes
    async fn run_migration_004_create_crm_indexes(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "004_create_crm_indexes";

        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Create indexes for contacts table
        let contact_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_contacts_email ON contacts(email)",
            "CREATE INDEX IF NOT EXISTS idx_contacts_phone ON contacts(phone)",
            "CREATE INDEX IF NOT EXISTS idx_contacts_company ON contacts(company)",
            "CREATE INDEX IF NOT EXISTS idx_contacts_lead_status ON contacts(lead_status)",
            "CREATE INDEX IF NOT EXISTS idx_contacts_is_active ON contacts(is_active)",
            "CREATE INDEX IF NOT EXISTS idx_contacts_created_at ON contacts(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_contacts_name ON contacts(first_name, last_name)",
        ];

        for index in contact_indexes {
            sqlx::query(index).execute(pool).await?;
        }
        tracing::info!("Contact indexes created successfully");

        // Create indexes for custom_fields table
        let custom_field_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_custom_fields_module ON custom_fields(module)",
            "CREATE INDEX IF NOT EXISTS idx_custom_fields_field_type ON custom_fields(field_type)",
            "CREATE INDEX IF NOT EXISTS idx_custom_fields_is_active ON custom_fields(is_active)",
            "CREATE INDEX IF NOT EXISTS idx_custom_fields_display_order ON custom_fields(display_order)",
        ];

        for index in custom_field_indexes {
            sqlx::query(index).execute(pool).await?;
        }
        tracing::info!("Custom field indexes created successfully");

        // Create indexes for contact_custom_values table
        let custom_value_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_contact_custom_values_contact_id ON contact_custom_values(contact_id)",
            "CREATE INDEX IF NOT EXISTS idx_contact_custom_values_custom_field_id ON contact_custom_values(custom_field_id)",
            "CREATE INDEX IF NOT EXISTS idx_contact_custom_values_value_number ON contact_custom_values(value_number)",
            "CREATE INDEX IF NOT EXISTS idx_contact_custom_values_value_date ON contact_custom_values(value_date)",
            "CREATE INDEX IF NOT EXISTS idx_contact_custom_values_value_boolean ON contact_custom_values(value_boolean)",
        ];

        for index in custom_value_indexes {
            sqlx::query(index).execute(pool).await?;
        }
        tracing::info!("Contact custom value indexes created successfully");

        // Create indexes for contact_tags table
        let tag_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_contact_tags_name ON contact_tags(name)",
        ];

        for index in tag_indexes {
            sqlx::query(index).execute(pool).await?;
        }
        tracing::info!("Contact tag indexes created successfully");

        // Create indexes for contact_tag_assignments table
        let tag_assignment_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_contact_tag_assignments_contact_id ON contact_tag_assignments(contact_id)",
            "CREATE INDEX IF NOT EXISTS idx_contact_tag_assignments_tag_id ON contact_tag_assignments(tag_id)",
        ];

        for index in tag_assignment_indexes {
            sqlx::query(index).execute(pool).await?;
        }
        tracing::info!("Contact tag assignment indexes created successfully");

        // Create indexes for contact_activities table
        let activity_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_contact_activities_contact_id ON contact_activities(contact_id)",
            "CREATE INDEX IF NOT EXISTS idx_contact_activities_type ON contact_activities(activity_type)",
            "CREATE INDEX IF NOT EXISTS idx_contact_activities_date ON contact_activities(activity_date)",
            "CREATE INDEX IF NOT EXISTS idx_contact_activities_status ON contact_activities(status)",
            "CREATE INDEX IF NOT EXISTS idx_contact_activities_created_by ON contact_activities(created_by)",
        ];

        for index in activity_indexes {
            sqlx::query(index).execute(pool).await?;
        }
        tracing::info!("Contact activity indexes created successfully");

        // Mark migration as applied
        Self::mark_migration_applied(pool, migration_name).await?;

        tracing::info!("Migration {} completed successfully", migration_name);
        Ok(())
    }

    /// Migration 005: Insert sample CRM data
    async fn run_migration_005_insert_sample_crm_data(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "005_insert_sample_crm_data";

        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Insert sample custom fields
        let insert_custom_fields = r#"
            INSERT INTO custom_fields (module, label, field_name, field_type, is_required, options, validation_rules, display_order) VALUES
            ('contact', 'LinkedIn Profile', 'linkedin_profile', 'text', false,
             NULL, '{"pattern": "^https://.*linkedin\\.com/.*"}', 1),
            ('contact', 'Annual Revenue', 'annual_revenue', 'number', false,
             NULL, '{"min": 0, "max": 999999999}', 2),
            ('contact', 'Industry', 'industry', 'select', false,
             '{"options": ["Technology", "Healthcare", "Finance", "Education", "Manufacturing", "Retail", "Other"]}', NULL, 3),
            ('contact', 'Preferred Contact Method', 'preferred_contact', 'select', false,
             '{"options": ["Email", "Phone", "SMS", "LinkedIn"]}', NULL, 4),
            ('contact', 'Newsletter Subscription', 'newsletter_subscription', 'boolean', false,
             NULL, NULL, 5),
            ('contact', 'Last Contact Date', 'last_contact_date', 'date', false,
             NULL, NULL, 6),
            ('contact', 'Skills', 'skills', 'multi_select', false,
             '{"options": ["JavaScript", "Python", "Java", "C#", "PHP", "Ruby", "Go", "Rust"]}', NULL, 7),
            ('contact', 'Bio', 'bio', 'textarea', false,
             NULL, '{"max_length": 1000}', 8)
            ON CONFLICT (module, field_name) DO NOTHING
        "#;

        sqlx::query(insert_custom_fields).execute(pool).await?;
        tracing::info!("Sample custom fields inserted successfully");

        // Insert sample contact tags
        let insert_contact_tags = r#"
            INSERT INTO contact_tags (name, color, description) VALUES
            ('Enterprise', '#007bff', 'Large enterprise clients'),
            ('SMB', '#28a745', 'Small to medium business'),
            ('Startup', '#ffc107', 'Early stage startups'),
            ('Healthcare', '#dc3545', 'Healthcare industry'),
            ('Education', '#6f42c1', 'Educational institutions'),
            ('Technology', '#17a2b8', 'Technology companies'),
            ('Hot Lead', '#fd7e14', 'High priority prospects'),
            ('VIP', '#e83e8c', 'VIP customers')
            ON CONFLICT (name) DO NOTHING
        "#;

        sqlx::query(insert_contact_tags).execute(pool).await?;
        tracing::info!("Sample contact tags inserted successfully");

        // Insert sample contacts
        let insert_contacts = r#"
            INSERT INTO contacts (first_name, last_name, email, phone, company, job_title, city, state, country, lead_source, lead_status, notes) VALUES
            ('John', 'Smith', 'john.smith@techcorp.com', '+1-555-0101', 'TechCorp Solutions', 'CTO', 'San Francisco', 'CA', 'USA', 'website', 'qualified', 'Interested in enterprise survey solutions'),
            ('Sarah', 'Johnson', 'sarah.j@healthplus.com', '+1-555-0102', 'HealthPlus Medical', 'Director of Operations', 'Boston', 'MA', 'USA', 'referral', 'proposal', 'Looking for patient feedback system'),
            ('Michael', 'Chen', 'mchen@edutech.org', '+1-555-0103', 'EduTech Institute', 'Research Manager', 'Austin', 'TX', 'USA', 'conference', 'new', 'Met at EdTech conference 2024'),
            ('Emily', 'Davis', 'emily.davis@retailco.com', '+1-555-0104', 'RetailCo Inc', 'Marketing Manager', 'Chicago', 'IL', 'USA', 'linkedin', 'contacted', 'Interested in customer satisfaction surveys'),
            ('David', 'Wilson', 'dwilson@startup.io', '+1-555-0105', 'StartupIO', 'Founder', 'Seattle', 'WA', 'USA', 'website', 'negotiation', 'Early stage startup, price sensitive')
            ON CONFLICT (email) DO NOTHING
        "#;

        sqlx::query(insert_contacts).execute(pool).await?;
        tracing::info!("Sample contacts inserted successfully");

        // Mark migration as applied
        Self::mark_migration_applied(pool, migration_name).await?;

        tracing::info!("Migration {} completed successfully", migration_name);
        Ok(())
    }

    /// Migration 006: Add owner column to contacts table
    async fn run_migration_006_add_owner_to_contacts(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "006_add_owner_to_contacts";

        // Check if migration already applied
        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Add owner column to contacts table
        let add_owner_column = r#"
            ALTER TABLE contacts
            ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES users(id) ON DELETE SET NULL
        "#;

        sqlx::query(add_owner_column)
            .execute(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        tracing::info!("Added owner_id column to contacts table");

        // Create index on owner_id for better query performance
        let create_owner_index = r#"
            CREATE INDEX IF NOT EXISTS idx_contacts_owner_id ON contacts(owner_id)
        "#;

        sqlx::query(create_owner_index)
            .execute(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        tracing::info!("Created index on contacts.owner_id");

        // Update existing contacts to have an owner (set to first user if exists)
        let update_existing_contacts = r#"
            UPDATE contacts
            SET owner_id = (
                SELECT id FROM users
                WHERE status = 'active'
                ORDER BY created_at ASC
                LIMIT 1
            )
            WHERE owner_id IS NULL
        "#;

        sqlx::query(update_existing_contacts)
            .execute(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        tracing::info!("Updated existing contacts with owner");

        // Mark migration as completed
        Self::mark_migration_applied(pool, migration_name).await?;
        tracing::info!("Migration {} completed successfully", migration_name);

        Ok(())
    }

    /// Migration 007: Add contact permissions to roles and create contact-specific roles
    async fn run_migration_007_add_contact_permissions(pool: &PgPool) -> Result<(), AppError> {
        let migration_name = "007_add_contact_permissions";

        // Check if migration already applied
        if Self::is_migration_applied(pool, migration_name).await? {
            tracing::info!("Migration {} already applied, skipping", migration_name);
            return Ok(());
        }

        tracing::info!("Running migration: {}", migration_name);

        // Update existing roles with contact permissions
        let update_admin_role = r#"
            UPDATE roles SET permissions = '[
                "users:read", "users:write", "users:create", "users:delete",
                "org:read", "org:write", "org:create", "org:delete",
                "contacts:read", "contacts:create", "contacts:update", "contacts:delete",
                "contacts:export", "contacts:import", "contacts:assign_owner", "contacts:bulk_update",
                "reports:read", "reports:create", "reports:export"
            ]'::jsonb WHERE name = 'admin'
        "#;

        sqlx::query(update_admin_role).execute(pool).await?;
        tracing::info!("Updated admin role with contact permissions");

        let update_member_role = r#"
            UPDATE roles SET permissions = '[
                "org:read", "users:read",
                "contacts:read", "contacts:create", "contacts:update_own", "contacts:delete_own",
                "reports:read"
            ]'::jsonb WHERE name = 'member'
        "#;

        sqlx::query(update_member_role).execute(pool).await?;
        tracing::info!("Updated member role with contact permissions");

        let update_viewer_role = r#"
            UPDATE roles SET permissions = '[
                "org:read",
                "contacts:read", "contacts:read_own",
                "reports:read"
            ]'::jsonb WHERE name = 'viewer'
        "#;

        sqlx::query(update_viewer_role).execute(pool).await?;
        tracing::info!("Updated viewer role with contact permissions");

        // Add new contact-specific roles
        let insert_contact_roles = r#"
            INSERT INTO roles (name, description, permissions) VALUES
                ('contact_manager', 'Contact management specialist', '[
                    "org:read", "users:read",
                    "contacts:read", "contacts:create", "contacts:update", "contacts:delete",
                    "contacts:export", "contacts:import", "contacts:assign_owner",
                    "reports:read", "reports:create"
                ]'::jsonb),
                ('sales_rep', 'Sales representative', '[
                    "org:read", "users:read",
                    "contacts:read", "contacts:create", "contacts:update_own", "contacts:delete_own",
                    "contacts:export", "reports:read"
                ]'::jsonb),
                ('marketing_user', 'Marketing team member', '[
                    "org:read", "users:read",
                    "contacts:read", "contacts:create", "contacts:update_own",
                    "contacts:export", "contacts:bulk_update", "reports:read"
                ]'::jsonb),
                ('support_agent', 'Customer support agent', '[
                    "org:read", "users:read",
                    "contacts:read", "contacts:update", "reports:read"
                ]'::jsonb),
                ('readonly_user', 'Read-only access to all contacts', '[
                    "org:read", "users:read", "contacts:read", "reports:read"
                ]'::jsonb)
            ON CONFLICT (name) DO NOTHING
        "#;

        sqlx::query(insert_contact_roles).execute(pool).await?;
        tracing::info!("Added new contact-specific roles");

        // Mark migration as completed
        Self::mark_migration_applied(pool, migration_name).await?;
        tracing::info!("Migration {} completed successfully", migration_name);

        Ok(())
    }
}
