-- Migration: Create user_organizations table for multi-tenant user-organization relationships
-- Compatible with PostgreSQL and CockroachDB

-- First, create roles table if it doesn't exist
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    permissions JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default roles
INSERT INTO roles (name, description, permissions) VALUES 
    ('owner', 'Organization owner with full access', '["*"]'::jsonb),
    ('admin', 'Organization administrator', '["users:read", "users:write", "org:read", "org:write"]'::jsonb),
    ('member', 'Regular organization member', '["org:read", "users:read"]'::jsonb),
    ('viewer', 'Read-only access to organization', '["org:read"]'::jsonb)
ON CONFLICT (name) DO NOTHING;

-- Create user_organizations table
CREATE TABLE IF NOT EXISTS user_organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    org_id UUID NOT NULL,
    role_id UUID NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Foreign key constraints
    CONSTRAINT fk_user_organizations_user_id 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_organizations_org_id 
        FOREIGN KEY (org_id) REFERENCES organization(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_organizations_role_id 
        FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE RESTRICT,
    
    -- Unique constraint to prevent duplicate user-org relationships
    CONSTRAINT unique_user_org UNIQUE (user_id, org_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_user_organizations_user_id ON user_organizations(user_id);
CREATE INDEX IF NOT EXISTS idx_user_organizations_org_id ON user_organizations(org_id);
CREATE INDEX IF NOT EXISTS idx_user_organizations_role_id ON user_organizations(role_id);
CREATE INDEX IF NOT EXISTS idx_user_organizations_status ON user_organizations(status);
CREATE INDEX IF NOT EXISTS idx_user_organizations_joined_at ON user_organizations(joined_at);

-- Add constraint to ensure status is valid
ALTER TABLE user_organizations ADD CONSTRAINT check_user_org_status 
CHECK (status IN ('active', 'pending', 'invited', 'suspended', 'inactive'));

-- Create trigger function for updated_at timestamp
CREATE OR REPLACE FUNCTION update_user_organizations_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for automatic updated_at updates
DROP TRIGGER IF EXISTS update_user_organizations_updated_at_trigger ON user_organizations;
CREATE TRIGGER update_user_organizations_updated_at_trigger
    BEFORE UPDATE ON user_organizations
    FOR EACH ROW
    EXECUTE FUNCTION update_user_organizations_updated_at();

-- Create trigger function for roles updated_at timestamp
CREATE OR REPLACE FUNCTION update_roles_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for roles table
DROP TRIGGER IF EXISTS update_roles_updated_at_trigger ON roles;
CREATE TRIGGER update_roles_updated_at_trigger
    BEFORE UPDATE ON roles
    FOR EACH ROW
    EXECUTE FUNCTION update_roles_updated_at();
