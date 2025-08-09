-- Migration to fix permissions and constraints
-- Run this manually in your database

-- 1. Update role permissions to include contact permissions
UPDATE roles SET permissions = '[
  "users:read", "users:write", "users:create", "users:delete",
  "org:read", "org:write", "org:create", "org:delete",
  "contacts:read", "contacts:create", "contacts:update", "contacts:delete",
  "contacts:export", "contacts:import", "contacts:assign_owner", "contacts:bulk_update",
  "reports:read", "reports:create", "reports:export"
]'::jsonb WHERE name = 'admin';

UPDATE roles SET permissions = '[
  "org:read", "users:read",
  "contacts:read", "contacts:create", "contacts:update_own", "contacts:delete_own",
  "reports:read"
]'::jsonb WHERE name = 'member';

UPDATE roles SET permissions = '[
  "org:read", 
  "contacts:read", "contacts:read_own",
  "reports:read"
]'::jsonb WHERE name = 'viewer';

-- Owner keeps wildcard permission
-- UPDATE roles SET permissions = '["*"]'::jsonb WHERE name = 'owner';

-- 2. Fix the unique constraint to allow multiple roles per user-org
-- First, check if there are any duplicate entries that would violate the new constraint
SELECT user_id, org_id, role_id, COUNT(*) 
FROM user_organizations 
GROUP BY user_id, org_id, role_id 
HAVING COUNT(*) > 1;

-- If no duplicates, proceed with constraint change
-- Drop old constraint
ALTER TABLE user_organizations DROP CONSTRAINT IF EXISTS unique_user_org;

-- Add new constraint that allows multiple roles but prevents duplicate role assignments
ALTER TABLE user_organizations ADD CONSTRAINT unique_user_org_role 
UNIQUE (user_id, org_id, role_id);

-- 3. Add some additional useful roles
INSERT INTO roles (name, description, permissions) VALUES
  ('manager', 'Team manager with extended permissions', '[
    "org:read", "users:read", "users:write",
    "contacts:read", "contacts:create", "contacts:update", "contacts:delete",
    "contacts:assign_owner", "contacts:export",
    "reports:read", "reports:create"
  ]'::jsonb),
  ('sales', 'Sales team member', '[
    "org:read", "users:read",
    "contacts:read", "contacts:create", "contacts:update_own", "contacts:delete_own",
    "contacts:export", "reports:read"
  ]'::jsonb),
  ('readonly', 'Read-only access to all data', '[
    "org:read", "users:read", "contacts:read", "reports:read"
  ]'::jsonb)
ON CONFLICT (name) DO NOTHING;

-- 4. Create a view for easy permission checking
CREATE OR REPLACE VIEW user_permissions AS
SELECT 
    uo.user_id,
    uo.org_id,
    u.name as user_name,
    u.email as user_email,
    o.name as org_name,
    r.name as role_name,
    r.permissions,
    uo.status,
    uo.joined_at
FROM user_organizations uo
JOIN users u ON uo.user_id = u.id
JOIN organization o ON uo.org_id = o.id  
JOIN roles r ON uo.role_id = r.id
WHERE uo.status = 'active';

-- 5. Add indexes for better permission checking performance
CREATE INDEX IF NOT EXISTS idx_user_organizations_user_org ON user_organizations(user_id, org_id);
CREATE INDEX IF NOT EXISTS idx_user_organizations_status ON user_organizations(status);

-- 6. Verify the changes
SELECT 'Roles with permissions:' as info;
SELECT name, permissions FROM roles ORDER BY name;

SELECT 'User-Organization relationships:' as info;
SELECT COUNT(*) as total_relationships FROM user_organizations;

SELECT 'Active user permissions view:' as info;
SELECT user_name, org_name, role_name FROM user_permissions LIMIT 5;
