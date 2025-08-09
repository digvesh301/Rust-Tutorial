-- CRM System Database Migration
-- Creates contacts, custom fields, and contact custom values tables
-- Compatible with both MySQL and PostgreSQL

-- =============================================
-- CONTACTS TABLE
-- =============================================
CREATE TABLE contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- PostgreSQL
    -- For MySQL, use: id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =============================================
-- CUSTOM FIELDS TABLE
-- =============================================
CREATE TABLE custom_fields (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- PostgreSQL
    -- For MySQL, use: id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    module VARCHAR(50) NOT NULL, -- 'contact', 'survey', 'organization', etc.
    label VARCHAR(255) NOT NULL, -- Display name for the field
    field_name VARCHAR(100) NOT NULL, -- Internal field name (snake_case)
    field_type VARCHAR(50) NOT NULL, -- 'text', 'number', 'email', 'phone', 'date', 'boolean', 'select', 'multi_select', 'textarea'
    is_required BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    options JSONB, -- For select/multi_select fields: {"options": ["Option 1", "Option 2"]}
    validation_rules JSONB, -- Validation rules: {"min_length": 5, "max_length": 100, "pattern": "regex"}
    default_value TEXT,
    help_text TEXT,
    display_order INTEGER DEFAULT 0,
    created_by UUID, -- Reference to users table if available
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Ensure unique field names per module
    UNIQUE(module, field_name)
);

-- =============================================
-- CONTACT CUSTOM VALUES TABLE
-- =============================================
CREATE TABLE contact_custom_values (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- PostgreSQL
    -- For MySQL, use: id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    contact_id UUID NOT NULL,
    custom_field_id UUID NOT NULL,
    value TEXT, -- Store all values as text, convert as needed
    value_json JSONB, -- For complex values like arrays, objects
    value_number DECIMAL(15,4), -- For numeric values (indexed)
    value_date DATE, -- For date values (indexed)
    value_boolean BOOLEAN, -- For boolean values (indexed)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Foreign key constraints
    CONSTRAINT fk_contact_custom_values_contact 
        FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
    CONSTRAINT fk_contact_custom_values_custom_field 
        FOREIGN KEY (custom_field_id) REFERENCES custom_fields(id) ON DELETE CASCADE,
    
    -- Ensure one value per contact per custom field
    UNIQUE(contact_id, custom_field_id)
);

-- =============================================
-- CONTACT TAGS TABLE (Many-to-Many)
-- =============================================
CREATE TABLE contact_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7) DEFAULT '#007bff', -- Hex color code
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE contact_tag_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    assigned_by UUID, -- Reference to users table if available
    assigned_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT fk_contact_tag_assignments_contact 
        FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
    CONSTRAINT fk_contact_tag_assignments_tag 
        FOREIGN KEY (tag_id) REFERENCES contact_tags(id) ON DELETE CASCADE,
    
    UNIQUE(contact_id, tag_id)
);

-- =============================================
-- CONTACT ACTIVITIES TABLE
-- =============================================
CREATE TABLE contact_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL,
    activity_type VARCHAR(50) NOT NULL, -- 'call', 'email', 'meeting', 'note', 'task'
    subject VARCHAR(255),
    description TEXT,
    activity_date TIMESTAMP WITH TIME ZONE,
    duration_minutes INTEGER,
    status VARCHAR(50) DEFAULT 'completed', -- 'completed', 'scheduled', 'cancelled'
    created_by UUID, -- Reference to users table if available
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT fk_contact_activities_contact 
        FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

-- =============================================
-- INDEXES FOR PERFORMANCE
-- =============================================

-- Contacts table indexes
CREATE INDEX idx_contacts_email ON contacts(email);
CREATE INDEX idx_contacts_phone ON contacts(phone);
CREATE INDEX idx_contacts_company ON contacts(company);
CREATE INDEX idx_contacts_lead_status ON contacts(lead_status);
CREATE INDEX idx_contacts_is_active ON contacts(is_active);
CREATE INDEX idx_contacts_created_at ON contacts(created_at);
CREATE INDEX idx_contacts_name ON contacts(first_name, last_name);

-- Custom fields table indexes
CREATE INDEX idx_custom_fields_module ON custom_fields(module);
CREATE INDEX idx_custom_fields_field_type ON custom_fields(field_type);
CREATE INDEX idx_custom_fields_is_active ON custom_fields(is_active);
CREATE INDEX idx_custom_fields_display_order ON custom_fields(display_order);

-- Contact custom values table indexes
CREATE INDEX idx_contact_custom_values_contact_id ON contact_custom_values(contact_id);
CREATE INDEX idx_contact_custom_values_custom_field_id ON contact_custom_values(custom_field_id);
CREATE INDEX idx_contact_custom_values_value_number ON contact_custom_values(value_number);
CREATE INDEX idx_contact_custom_values_value_date ON contact_custom_values(value_date);
CREATE INDEX idx_contact_custom_values_value_boolean ON contact_custom_values(value_boolean);

-- Contact tags indexes
CREATE INDEX idx_contact_tags_name ON contact_tags(name);
CREATE INDEX idx_contact_tag_assignments_contact_id ON contact_tag_assignments(contact_id);
CREATE INDEX idx_contact_tag_assignments_tag_id ON contact_tag_assignments(tag_id);

-- Contact activities indexes
CREATE INDEX idx_contact_activities_contact_id ON contact_activities(contact_id);
CREATE INDEX idx_contact_activities_type ON contact_activities(activity_type);
CREATE INDEX idx_contact_activities_date ON contact_activities(activity_date);
CREATE INDEX idx_contact_activities_status ON contact_activities(status);
CREATE INDEX idx_contact_activities_created_by ON contact_activities(created_by);

-- =============================================
-- SAMPLE CUSTOM FIELDS DATA
-- =============================================
INSERT INTO custom_fields (id, module, label, field_name, field_type, is_required, options, validation_rules, display_order) VALUES
-- Contact custom fields
('cf001-0000-0000-0000-000000000001', 'contact', 'LinkedIn Profile', 'linkedin_profile', 'text', false, 
 NULL, '{"pattern": "^https://.*linkedin\\.com/.*"}', 1),
('cf001-0000-0000-0000-000000000002', 'contact', 'Annual Revenue', 'annual_revenue', 'number', false, 
 NULL, '{"min": 0, "max": 999999999}', 2),
('cf001-0000-0000-0000-000000000003', 'contact', 'Industry', 'industry', 'select', false, 
 '{"options": ["Technology", "Healthcare", "Finance", "Education", "Manufacturing", "Retail", "Other"]}', NULL, 3),
('cf001-0000-0000-0000-000000000004', 'contact', 'Preferred Contact Method', 'preferred_contact', 'select', false, 
 '{"options": ["Email", "Phone", "SMS", "LinkedIn"]}', NULL, 4),
('cf001-0000-0000-0000-000000000005', 'contact', 'Newsletter Subscription', 'newsletter_subscription', 'boolean', false, 
 NULL, NULL, 5),
('cf001-0000-0000-0000-000000000006', 'contact', 'Last Contact Date', 'last_contact_date', 'date', false, 
 NULL, NULL, 6),
('cf001-0000-0000-0000-000000000007', 'contact', 'Skills', 'skills', 'multi_select', false, 
 '{"options": ["JavaScript", "Python", "Java", "C#", "PHP", "Ruby", "Go", "Rust"]}', NULL, 7),
('cf001-0000-0000-0000-000000000008', 'contact', 'Bio', 'bio', 'textarea', false, 
 NULL, '{"max_length": 1000}', 8);

-- =============================================
-- MYSQL SPECIFIC MODIFICATIONS (UNCOMMENT IF USING MYSQL)
-- =============================================
/*
-- For MySQL, replace UUID with CHAR(36) and gen_random_uuid() with UUID()
-- Also add ENGINE=InnoDB to all CREATE TABLE statements

-- Example for contacts table in MySQL:
CREATE TABLE contacts (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Replace JSONB with JSON in MySQL
-- Replace TIMESTAMP WITH TIME ZONE with TIMESTAMP
-- Replace gen_random_uuid() with UUID()
*/
